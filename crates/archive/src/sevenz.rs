//! 7z backend on `sevenz-rust2` (pure Rust). Built-in methods: LZMA, LZMA2 (7-Zip's default), Deflate, the
//! BCJ filters, Delta and Copy; PPMd, BZip2 and the rest get an "unsupported method" error.
//!
//! 7z compresses *blocks*: a solid block holds many files back to back, and reaching a VPK means decoding every
//! file before it. So blocks without a VPK are never decoded, decoding a block stops after its last VPK, and the
//! bytes of skipped files decoded on the way count against the size limit like written ones. The ratio limit is
//! measured per block (packed size vs. bytes decoded), since 7z has no per-file compressed size.
//!
//! Each block's dictionary is checked before decoding: the LZMA decoders allocate it in full up front, so a
//! header announcing a 4 GB dictionary would otherwise cost 4 GB of memory before a single byte is checked.

use crate::error::{ArchiveError, LimitExceeded};
use crate::limits::Unit;
use crate::output::{ExtractedVpk, PumpError, drain};
use crate::{Entry, Job};
use sevenz_rust2::{Archive, BlockDecoder, EncoderMethod, Error, Password};
use std::fs::File;
use std::io::{BufReader, Read};

fn open(reader: &mut BufReader<File>) -> Result<Archive, ArchiveError> {
    Archive::read(reader, &Password::empty()).map_err(|e| map_error(e, None))
}

fn entries(archive: &Archive) -> Vec<Entry> {
    archive
        .files
        .iter()
        .filter(|f| !f.is_anti_item)
        .map(|f| Entry {
            name: f.name.clone(),
            size: f.size,
            is_dir: f.is_directory,
        })
        .collect()
}

pub(crate) fn list(file: File) -> Result<Vec<Entry>, ArchiveError> {
    let mut reader = BufReader::new(file);
    Ok(entries(&open(&mut reader)?))
}

pub(crate) fn extract(
    file: File,
    job: &Job<'_>,
) -> Result<(Vec<Entry>, Vec<ExtractedVpk>), ArchiveError> {
    let mut reader = BufReader::new(file);
    let archive = open(&mut reader)?;
    let listed = entries(&archive);
    job.preflight(&listed)?;

    let is_vpk = |i: usize| {
        let f = &archive.files[i];
        !f.is_anti_item && !f.is_directory && crate::is_vpk_name(&f.name)
    };
    // VPKs with a data stream are decoded from their block; those without one are simply empty.
    let has_data = |i: usize| is_vpk(i) && archive.files[i].has_stream;
    let file_count = archive.files.len();
    let block_of = |i: usize| {
        archive
            .stream_map
            .file_block_index
            .get(i)
            .copied()
            .flatten()
    };
    let mut blocks: Vec<usize> = (0..file_count)
        .filter(|&i| has_data(i))
        .filter_map(block_of)
        .collect();
    blocks.sort_unstable();
    blocks.dedup();
    for &b in &blocks {
        check_block(&archive, b, job.limits.max_dictionary_bytes)?;
    }

    let mut meter = job.meter();
    let mut staging = job.staging();
    let mut buf = vec![0u8; 64 * 1024];
    let mut seen = vec![false; file_count];
    let password = Password::empty();

    for &b in &blocks {
        let last_vpk = (0..file_count)
            .rfind(|&i| has_data(i) && block_of(i) == Some(b))
            .unwrap_or(0);
        let mut unit = Unit::new(packed_size(&archive, b));
        let mut expected = archive
            .stream_map
            .block_first_file_index
            .get(b)
            .copied()
            .unwrap_or(0);
        let mut failure: Option<ArchiveError> = None;

        let decoder = BlockDecoder::new(1, b, &archive, &password, &mut reader);
        let result = decoder.for_each_entries(&mut |entry, data: &mut dyn Read| {
            let Some(i) = index_of(&archive, entry, expected) else {
                failure = Some(damaged(
                    Some(&entry.name),
                    "the file list does not match the data",
                ));
                return Err(Error::Other("aborted".into()));
            };
            expected = i + 1;
            let step = if has_data(i) {
                seen[i] = true;
                staging.write(&entry.name, data, &mut meter, &mut unit)
            } else if is_vpk(i) {
                Ok(())
            } else {
                drain(data, &mut meter, &mut unit, &entry.name, &mut buf).map_err(|e| match e {
                    PumpError::Limit(l) => ArchiveError::Limit(l),
                    PumpError::Read(e) | PumpError::Write(e) => {
                        damaged(Some(&entry.name), &e.to_string())
                    }
                })
            };
            match step {
                Ok(()) => Ok(i < last_vpk),
                Err(e) => {
                    failure = Some(e);
                    Err(Error::Other("aborted".into()))
                }
            }
        });
        if let Some(e) = failure {
            return Err(e);
        }
        result.map_err(|e| map_error(e, None))?;
    }

    if let Some(i) = (0..file_count).find(|&i| has_data(i) && !seen[i]) {
        return Err(damaged(
            Some(&archive.files[i].name),
            "the file was not found in the data",
        ));
    }

    // VPKs without data (empty files) are numbered after the others.
    for i in (0..file_count).filter(|&i| is_vpk(i) && !has_data(i)) {
        let name = &archive.files[i].name;
        staging.write(name, &mut std::io::empty(), &mut meter, &mut Unit::new(0))?;
    }
    Ok((listed, staging.commit()?))
}

/// The file index of `entry`: `expected` (the library hands files over in order) or, failing that, a search.
fn index_of(
    archive: &Archive,
    entry: &sevenz_rust2::ArchiveEntry,
    expected: usize,
) -> Option<usize> {
    if archive
        .files
        .get(expected)
        .is_some_and(|f| std::ptr::eq(f, entry))
    {
        return Some(expected);
    }
    archive.files.iter().position(|f| std::ptr::eq(f, entry))
}

fn packed_size(archive: &Archive, block: usize) -> u64 {
    let first = archive.stream_map.block_first_pack_stream_index();
    let sizes = archive.pack_sizes();
    let start = first.get(block).copied().unwrap_or(sizes.len());
    let end = first.get(block + 1).copied().unwrap_or(sizes.len());
    sizes
        .get(start..end.max(start))
        .map_or(0, |s| s.iter().sum())
}

/// Refuses encrypted blocks, unsupported methods and dictionaries over the limit before decoding starts.
fn check_block(archive: &Archive, block: usize, max_dictionary: u64) -> Result<(), ArchiveError> {
    let block_data = &archive.blocks[block];
    for coder in &block_data.coders {
        let id = coder.encoder_method_id();
        if id == EncoderMethod::ID_AES256_SHA256 {
            return Err(ArchiveError::Encrypted { entry: None });
        }
        let dictionary = match id {
            EncoderMethod::ID_LZMA => lzma_dictionary(coder.properties())?
                .min(block_data.get_unpack_size_for_coder(coder)),
            EncoderMethod::ID_LZMA2 => lzma2_dictionary(coder.properties())?,
            _ if SUPPORTED.contains(&id) => 0,
            _ => {
                return Err(ArchiveError::UnsupportedMethod {
                    entry: None,
                    method: method_name(id),
                });
            }
        };
        if dictionary > max_dictionary {
            return Err(LimitExceeded::Dictionary {
                limit: max_dictionary,
                needed: dictionary,
            }
            .into());
        }
    }
    Ok(())
}

const SUPPORTED: [&[u8]; 11] = [
    EncoderMethod::ID_COPY,
    EncoderMethod::ID_DELTA,
    EncoderMethod::ID_DEFLATE,
    EncoderMethod::ID_BCJ_X86,
    EncoderMethod::ID_BCJ_ARM,
    EncoderMethod::ID_BCJ_ARM64,
    EncoderMethod::ID_BCJ_ARM_THUMB,
    EncoderMethod::ID_BCJ_PPC,
    EncoderMethod::ID_BCJ_IA64,
    EncoderMethod::ID_BCJ_SPARC,
    EncoderMethod::ID_BCJ_RISCV,
];

fn method_name(id: &[u8]) -> String {
    EncoderMethod::by_id(id).map_or_else(|| format!("7z method {id:02x?}"), |m| m.name().to_owned())
}

/// LZMA properties: one byte lc/lp/pb, then the dictionary size as little-endian u32.
fn lzma_dictionary(props: &[u8]) -> Result<u64, ArchiveError> {
    match props.get(1..5) {
        Some(&[a, b, c, d]) => Ok(u64::from(u32::from_le_bytes([a, b, c, d]))),
        _ => Err(damaged(None, "LZMA properties are too short")),
    }
}

/// LZMA2 property byte `p`: dictionary `(2 | (p & 1)) << (p / 2 + 11)`; 40 means 4 GB − 1.
fn lzma2_dictionary(props: &[u8]) -> Result<u64, ArchiveError> {
    match props.first() {
        Some(&40) => Ok(u64::from(u32::MAX)),
        Some(&p) if p < 40 => Ok((2 | u64::from(p & 1)) << (p / 2 + 11)),
        _ => Err(damaged(None, "invalid LZMA2 properties")),
    }
}

fn damaged(entry: Option<&str>, detail: &str) -> ArchiveError {
    ArchiveError::Damaged {
        entry: entry.map(str::to_owned),
        detail: detail.to_owned(),
    }
}

fn map_error(e: Error, entry: Option<&str>) -> ArchiveError {
    let entry = entry.map(str::to_owned);
    match e {
        Error::PasswordRequired | Error::MaybeBadPassword(_) => ArchiveError::Encrypted { entry },
        // Built without AES: an encrypted header surfaces as an unknown method.
        Error::UnsupportedCompressionMethod(m) if m.to_ascii_uppercase().contains("AES") => {
            ArchiveError::Encrypted { entry }
        }
        Error::UnsupportedCompressionMethod(method) => {
            ArchiveError::UnsupportedMethod { entry, method }
        }
        Error::ExternalUnsupported => ArchiveError::UnsupportedMethod {
            entry,
            method: "external streams".into(),
        },
        Error::MaxMemLimited { max_kb, actaul_kb } => LimitExceeded::Dictionary {
            limit: (max_kb as u64).saturating_mul(1024),
            needed: (actaul_kb as u64).saturating_mul(1024),
        }
        .into(),
        other => ArchiveError::Damaged {
            entry,
            detail: other.to_string(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{RecordingHooks, TempDir, files_under, seven_z_bytes};
    use crate::{Format, Limits, NoHooks, extract_vpks, list};
    use sha2::{Digest, Sha256};
    use std::time::{Duration, Instant};

    #[test]
    fn only_vpks_are_extracted_from_separate_blocks() {
        let dir = TempDir::new("7z-basic");
        let archive = dir.file(
            "mod.7z",
            &seven_z_bytes(
                &[
                    ("readme.txt", b"hello"),
                    ("A/", b""),
                    ("A/pak01_dir.vpk", b"vpk A"),
                    ("B/Pak01_Dir.VPK", b"vpk B"),
                    ("inner.7z", &seven_z_bytes(&[("hidden.vpk", b"x")], false)),
                ],
                false,
            ),
        );
        let out = dir.path().join("out");
        let x = extract_vpks(&archive, &out, &Limits::default(), &NoHooks).expect("extracted");
        assert_eq!(x.format, Format::SevenZip);
        assert_eq!(x.skipped, ["readme.txt", "inner.7z"]);
        assert_eq!(files_under(&out), ["0.vpk", "1.vpk"]);
        assert_eq!(x.vpks[0].archive_path, "A/pak01_dir.vpk");
        assert_eq!(x.vpks[1].archive_path, "B/Pak01_Dir.VPK");
        assert_eq!(x.vpks[1].sha256, <[u8; 32]>::from(Sha256::digest(b"vpk B")));
        assert_eq!(std::fs::read(out.join("0.vpk")).expect("read"), b"vpk A");
    }

    #[test]
    fn vpks_inside_a_solid_block_are_found_between_skipped_files() {
        let dir = TempDir::new("7z-solid");
        let archive = dir.file(
            "solid.7z",
            &seven_z_bytes(
                &[
                    ("preview.png", b"not a vpk at all"),
                    ("one.vpk", b"first vpk"),
                    ("notes.txt", b"between"),
                    ("two.vpk", b"second vpk"),
                    ("after.txt", b"never decoded"),
                ],
                true,
            ),
        );
        let out = dir.path().join("out");
        let x = extract_vpks(&archive, &out, &Limits::default(), &NoHooks).expect("extracted");
        assert_eq!(x.skipped, ["preview.png", "notes.txt", "after.txt"]);
        assert_eq!(
            std::fs::read(out.join("0.vpk")).expect("read"),
            b"first vpk"
        );
        assert_eq!(
            std::fs::read(out.join("1.vpk")).expect("read"),
            b"second vpk"
        );
        assert_eq!(x.vpks[1].size, 10);
    }

    #[test]
    fn an_empty_vpk_is_written_empty() {
        let dir = TempDir::new("7z-empty");
        let archive = dir.file(
            "e.7z",
            &seven_z_bytes(&[("empty.vpk", b""), ("a.vpk", b"a")], false),
        );
        let out = dir.path().join("out");
        let x = extract_vpks(&archive, &out, &Limits::default(), &NoHooks).expect("extracted");
        let shown: Vec<_> = x
            .vpks
            .iter()
            .map(|v| (v.archive_path.as_str(), v.size))
            .collect();
        assert_eq!(shown, [("empty.vpk", 0), ("a.vpk", 1)]);
        assert_eq!(files_under(&out), ["0.vpk", "1.vpk"]);
        for v in &x.vpks {
            assert_eq!(std::fs::metadata(&v.output).expect("exists").len(), v.size);
        }
    }

    #[test]
    fn a_vpk_without_a_data_stream_is_written_empty_after_the_others() {
        use sevenz_rust2::{ArchiveEntry, ArchiveWriter};
        let mut w = ArchiveWriter::new(std::io::Cursor::new(Vec::new())).expect("writer");
        w.push_archive_entry::<&[u8]>(ArchiveEntry::new_file("nostream.vpk"), None)
            .expect("no stream");
        w.push_archive_entry(ArchiveEntry::new_file("a.vpk"), Some(&b"a"[..]))
            .expect("entry");
        let bytes = w.finish().expect("finish").into_inner();
        let dir = TempDir::new("7z-nostream");
        let archive = dir.file("n.7z", &bytes);
        let out = dir.path().join("out");
        let x = extract_vpks(&archive, &out, &Limits::default(), &NoHooks).expect("extracted");
        let shown: Vec<_> = x
            .vpks
            .iter()
            .map(|v| (v.archive_path.as_str(), v.size))
            .collect();
        assert_eq!(shown, [("a.vpk", 1), ("nostream.vpk", 0)]);
        assert_eq!(
            std::fs::metadata(out.join("1.vpk")).expect("exists").len(),
            0
        );
    }

    #[test]
    fn listing_shows_every_entry() {
        let dir = TempDir::new("7z-list");
        let archive = dir.file(
            "l.7z",
            &seven_z_bytes(&[("d/", b""), ("d/x.vpk", b"123")], false),
        );
        let listing = list(&archive).expect("listed");
        assert_eq!(listing.format, Format::SevenZip);
        let shown: Vec<_> = listing
            .entries
            .iter()
            .map(|e| (e.name.as_str(), e.size, e.is_dir, e.is_vpk()))
            .collect();
        assert_eq!(shown, [("d/", 0, true, false), ("d/x.vpk", 3, false, true)]);
    }

    #[test]
    fn hostile_names_cannot_place_files_outside_the_output_directory() {
        let hostile = [
            "../x.vpk",
            "..\\x.vpk",
            "C:\\x.vpk",
            "/x.vpk",
            "\\\\server\\share\\x.vpk",
            "CON.vpk",
            "NUL.vpk",
            "a.vpk:stream",
        ];
        let dir = TempDir::new("7z-hostile");
        let entries: Vec<(&str, &[u8])> = hostile.iter().map(|n| (*n, &b"evil"[..])).collect();
        let archive = dir.file("mod.7z", &seven_z_bytes(&entries, true));
        let out = dir.path().join("lib").join("out");
        let x = extract_vpks(&archive, &out, &Limits::default(), &NoHooks).expect("extracted");
        let mut expected: Vec<String> = (0..7).map(|n| format!("lib/out/{n}.vpk")).collect();
        expected.push("mod.7z".into());
        expected.sort();
        assert_eq!(files_under(dir.path()), expected);
        assert_eq!(x.skipped, ["a.vpk:stream"]);
        let shown: Vec<_> = x.vpks.iter().map(|v| v.archive_path.as_str()).collect();
        assert_eq!(shown, hostile[..7]);
    }

    #[test]
    fn a_bomb_in_a_skipped_file_of_a_solid_block_still_stops_at_the_limit() {
        let dir = TempDir::new("7z-bomb");
        let zeros = vec![0u8; 4 << 20];
        let bytes = seven_z_bytes(&[("filler.bin", &zeros), ("late.vpk", b"vpk")], true);
        let archive = dir.file("bomb.7z", &bytes);
        let limit = Limits::default().total_for(bytes.len() as u64);
        assert!(limit < 1 << 20, "{limit}");
        let out = dir.path().join("out");
        let started = Instant::now();
        let err = extract_vpks(&archive, &out, &Limits::default(), &NoHooks).expect_err("bomb");
        assert!(
            started.elapsed() < Duration::from_secs(5),
            "{:?}",
            started.elapsed()
        );
        match err {
            ArchiveError::Limit(LimitExceeded::TotalSize { limit: l, entry }) => {
                assert_eq!((l, entry.as_str()), (limit, "filler.bin"));
            }
            other => panic!("{other:?}"),
        }
        assert!(files_under(&out).is_empty());
    }

    #[test]
    fn a_bomb_vpk_stops_at_the_limit_and_removes_earlier_output() {
        let dir = TempDir::new("7z-bomb-vpk");
        let zeros = vec![0u8; 4 << 20];
        let bytes = seven_z_bytes(&[("ok.vpk", b"fine"), ("bomb.vpk", &zeros)], false);
        let archive = dir.file("bomb.7z", &bytes);
        let hooks = RecordingHooks::default();
        let out = dir.path().join("out");
        let err = extract_vpks(&archive, &out, &Limits::default(), &hooks).expect_err("bomb");
        assert!(
            matches!(&err, ArchiveError::Limit(LimitExceeded::TotalSize { entry, .. }) if entry == "bomb.vpk"),
            "{err:?}"
        );
        assert!(files_under(&out).is_empty());
        assert_eq!(
            hooks
                .events()
                .iter()
                .filter(|e| e.starts_with("removed"))
                .count(),
            2
        );
    }

    #[test]
    fn the_ratio_is_measured_per_block() {
        let dir = TempDir::new("7z-ratio");
        let zeros = vec![0u8; 4 << 20];
        let archive = dir.file("r.7z", &seven_z_bytes(&[("flat.vpk", &zeros)], false));
        let limits = Limits {
            max_total_factor: u64::MAX,
            ..Limits::default()
        };
        let err =
            extract_vpks(&archive, &dir.path().join("out"), &limits, &NoHooks).expect_err("ratio");
        assert!(
            matches!(&err, ArchiveError::Limit(LimitExceeded::Ratio { limit: 100, entry }) if entry == "flat.vpk"),
            "{err:?}"
        );
    }

    #[test]
    fn too_many_vpks_are_refused() {
        let dir = TempDir::new("7z-count");
        let names: Vec<String> = (0..4).map(|n| format!("p{n}.vpk")).collect();
        let entries: Vec<(&str, &[u8])> = names.iter().map(|n| (n.as_str(), &b"v"[..])).collect();
        let archive = dir.file("pack.7z", &seven_z_bytes(&entries, true));
        let limits = Limits {
            max_vpk_entries: 3,
            ..Limits::default()
        };
        let err =
            extract_vpks(&archive, &dir.path().join("out"), &limits, &NoHooks).expect_err("count");
        assert!(
            matches!(
                err,
                ArchiveError::Limit(LimitExceeded::TooManyVpks { limit: 3, found: 4 })
            ),
            "{err:?}"
        );
    }

    #[test]
    fn a_dictionary_over_the_limit_is_refused_before_decoding() {
        let dir = TempDir::new("7z-dict");
        let archive = dir.file("d.7z", &seven_z_bytes(&[("a.vpk", b"data")], false));
        let limits = Limits {
            max_dictionary_bytes: 64 << 10,
            ..Limits::default()
        };
        let out = dir.path().join("out");
        let err = extract_vpks(&archive, &out, &limits, &NoHooks).expect_err("dictionary");
        assert!(
            matches!(err, ArchiveError::Limit(LimitExceeded::Dictionary { limit, needed }) if limit == 64 << 10 && needed > limit),
            "{err:?}"
        );
        assert!(!out.exists() || files_under(&out).is_empty());
    }

    #[test]
    fn dictionary_sizes_are_read_from_properties() {
        assert_eq!(lzma2_dictionary(&[0]).expect("p0"), 4 << 10);
        assert_eq!(lzma2_dictionary(&[1]).expect("p1"), 6 << 10);
        assert_eq!(lzma2_dictionary(&[18]).expect("p18"), 2 << 20);
        assert_eq!(lzma2_dictionary(&[24]).expect("p24"), 16 << 20);
        assert_eq!(lzma2_dictionary(&[40]).expect("p40"), u64::from(u32::MAX));
        assert!(lzma2_dictionary(&[41]).is_err());
        assert!(lzma2_dictionary(&[]).is_err());
        assert_eq!(
            lzma_dictionary(&[0x5d, 0, 0, 0, 1]).expect("16 MB"),
            16 << 20
        );
        assert!(lzma_dictionary(&[0x5d, 0]).is_err());
    }

    #[test]
    fn encrypted_content_is_reported_as_password_protected() {
        let dir = TempDir::new("7z-aes");
        let archive = dir.file("e.7z", crate::fixtures::ENCRYPTED_CONTENT_7Z);
        let err = extract_vpks(
            &archive,
            &dir.path().join("out"),
            &Limits::default(),
            &NoHooks,
        )
        .expect_err("encrypted");
        assert!(matches!(err, ArchiveError::Encrypted { .. }), "{err:?}");
    }

    #[test]
    fn an_encrypted_file_list_is_reported_as_password_protected() {
        let dir = TempDir::new("7z-aes-header");
        let archive = dir.file("e.7z", crate::fixtures::ENCRYPTED_HEADER_7Z);
        let err = list(&archive).expect_err("encrypted header");
        assert!(
            matches!(err, ArchiveError::Encrypted { entry: None }),
            "{err:?}"
        );
    }

    #[test]
    fn an_unsupported_method_is_named() {
        let dir = TempDir::new("7z-ppmd");
        let archive = dir.file("p.7z", crate::fixtures::PPMD_7Z);
        let err = extract_vpks(
            &archive,
            &dir.path().join("out"),
            &Limits::default(),
            &NoHooks,
        )
        .expect_err("ppmd");
        match err {
            ArchiveError::UnsupportedMethod { method, .. } => {
                assert!(method.to_uppercase().contains("PPMD"), "{method}");
            }
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn corrupted_data_is_reported_as_damaged() {
        let mut bytes = seven_z_bytes(&[("a.vpk", &[0x55; 1000])], false);
        // Packed data starts right after the 32-byte signature header.
        for b in &mut bytes[40..60] {
            *b ^= 0xff;
        }
        let dir = TempDir::new("7z-corrupt");
        let archive = dir.file("c.7z", &bytes);
        let out = dir.path().join("out");
        let err = extract_vpks(&archive, &out, &Limits::default(), &NoHooks).expect_err("corrupt");
        assert!(matches!(err, ArchiveError::Damaged { .. }), "{err:?}");
        assert!(!out.exists() || files_under(&out).is_empty());
    }

    #[test]
    fn a_truncated_download_is_damaged() {
        let bytes = seven_z_bytes(&[("a.vpk", b"data")], false);
        let dir = TempDir::new("7z-truncated");
        let archive = dir.file("t.7z", &bytes[..bytes.len() - 8]);
        let err = list(&archive).expect_err("truncated");
        assert!(matches!(err, ArchiveError::Damaged { .. }), "{err:?}");
    }
}
