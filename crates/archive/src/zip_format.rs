//! Zip backend. Only Stored and Deflate are built in: they are what Windows, 7-Zip and WinRAR write by default;
//! anything else gets a clear "unsupported method" error instead of extra decoders in the binary.
//!
//! Non-VPK entries are never decompressed — their data is not touched at all.

use crate::error::ArchiveError;
use crate::limits::Unit;
use crate::output::ExtractedVpk;
use crate::{Entry, Job};
use std::fs::File;
use std::io::BufReader;
use zip::result::ZipError;
use zip::{CompressionMethod, ZipArchive};

type Archive = ZipArchive<BufReader<File>>;

fn open(file: File) -> Result<Archive, ArchiveError> {
    ZipArchive::new(BufReader::new(file)).map_err(|e| map_error(e, None))
}

fn entries(archive: &mut Archive) -> Result<Vec<Entry>, ArchiveError> {
    (0..archive.len())
        .map(|i| {
            let f = archive.by_index_raw(i).map_err(|e| map_error(e, None))?;
            Ok(Entry {
                // zip 9: a name that can't be decoded is a damaged archive, like any other read error.
                name: f.name().map_err(|e| map_error(e, None))?.into_owned(),
                size: f.size(),
                is_dir: f.is_dir(),
            })
        })
        .collect()
}

pub(crate) fn list(file: File) -> Result<Vec<Entry>, ArchiveError> {
    entries(&mut open(file)?)
}

pub(crate) fn extract(
    file: File,
    job: &Job<'_>,
) -> Result<(Vec<Entry>, Vec<ExtractedVpk>), ArchiveError> {
    let mut archive = open(file)?;
    let entries = entries(&mut archive)?;
    let vpks: Vec<usize> = (0..entries.len())
        .filter(|&i| entries[i].is_vpk())
        .collect();
    job.preflight(&entries)?;
    // Refuse before writing anything: a protected or exotic VPK would fail halfway otherwise.
    let mut compressed = Vec::with_capacity(vpks.len());
    for &i in &vpks {
        let f = archive.by_index_raw(i).map_err(|e| map_error(e, None))?;
        let entry = Some(f.name().map_err(|e| map_error(e, None))?.into_owned());
        if f.encrypted() {
            return Err(ArchiveError::Encrypted { entry });
        }
        let method = f.compression();
        if method != CompressionMethod::Stored && method != CompressionMethod::Deflated {
            return Err(ArchiveError::UnsupportedMethod {
                entry,
                method: method.to_string(),
            });
        }
        compressed.push(f.compressed_size());
    }

    let mut meter = job.meter();
    let mut staging = job.staging();
    for (&i, compressed) in vpks.iter().zip(compressed) {
        let name = entries[i].name.as_str();
        let mut reader = archive.by_index(i).map_err(|e| map_error(e, Some(name)))?;
        staging.write(name, &mut reader, &mut meter, &mut Unit::new(compressed))?;
    }
    Ok((entries, staging.commit()?))
}

fn map_error(e: ZipError, entry: Option<&str>) -> ArchiveError {
    let entry = entry.map(str::to_owned);
    match e {
        ZipError::UnsupportedArchive(ZipError::PASSWORD_REQUIRED) | ZipError::InvalidPassword => {
            ArchiveError::Encrypted { entry }
        }
        ZipError::UnsupportedArchive(what) => ArchiveError::UnsupportedMethod {
            entry,
            method: what.to_owned(),
        },
        ZipError::CompressionMethodNotSupported(m) => ArchiveError::UnsupportedMethod {
            entry,
            method: format!("zip method {m}"),
        },
        ZipError::Io(e) => ArchiveError::Damaged {
            entry,
            detail: e.to_string(),
        },
        other => ArchiveError::Damaged {
            entry,
            detail: other.to_string(),
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::error::LimitExceeded;
    use crate::test_support::{RecordingHooks, TempDir, files_under, zip_bytes};
    use crate::{ArchiveError, Format, Limits, NoHooks, extract_vpks, list};
    use sha2::{Digest, Sha256};
    use std::time::{Duration, Instant};

    #[test]
    fn only_vpks_are_extracted_under_our_names_and_the_rest_is_reported() {
        let dir = TempDir::new("zip-basic");
        let archive = dir.file(
            "mod.zip",
            &zip_bytes(
                &[
                    ("readme.txt", b"hello"),
                    ("variant A/", b""),
                    ("variant A/pak01_dir.vpk", b"vpk A"),
                    ("Variant B/PAK01_DIR.VPK", b"vpk B"),
                    ("inner.zip", &zip_bytes(&[("hidden.vpk", b"nested")], false)),
                ],
                false,
            ),
        );
        let out = dir.path().join("out");
        let x = extract_vpks(&archive, &out, &Limits::default(), &NoHooks).expect("extracted");

        assert_eq!(x.format, Format::Zip);
        assert_eq!(x.skipped, ["readme.txt", "inner.zip"]);
        assert_eq!(files_under(&out), ["0.vpk", "1.vpk"]);
        assert_eq!(x.vpks.len(), 2);
        assert_eq!(x.vpks[0].archive_path, "variant A/pak01_dir.vpk");
        assert_eq!(x.vpks[1].archive_path, "Variant B/PAK01_DIR.VPK");
        assert_eq!(x.vpks[1].output, out.join("1.vpk"));
        assert_eq!(x.vpks[1].size, 5);
        assert_eq!(x.vpks[1].sha256, <[u8; 32]>::from(Sha256::digest(b"vpk B")));
        assert_eq!(std::fs::read(out.join("0.vpk")).expect("read"), b"vpk A");
    }

    #[test]
    fn stored_entries_work_too() {
        let dir = TempDir::new("zip-stored");
        let archive = dir.file("mod.zip", &zip_bytes(&[("a.vpk", b"plain")], true));
        let out = dir.path().join("out");
        let x = extract_vpks(&archive, &out, &Limits::default(), &NoHooks).expect("extracted");
        assert_eq!(x.vpks[0].size, 5);
    }

    #[test]
    fn listing_shows_every_entry_without_extracting() {
        let dir = TempDir::new("zip-list");
        let archive = dir.file(
            "mod.zip",
            &zip_bytes(
                &[("docs/", b""), ("docs/a.txt", b"12"), ("x.vpk", b"123")],
                false,
            ),
        );
        let listing = list(&archive).expect("listed");
        assert_eq!(listing.format, Format::Zip);
        let names: Vec<_> = listing
            .entries
            .iter()
            .map(|e| (e.name.as_str(), e.size, e.is_dir, e.is_vpk()))
            .collect();
        assert_eq!(
            names,
            [
                ("docs/", 0, true, false),
                ("docs/a.txt", 2, false, false),
                ("x.vpk", 3, false, true)
            ]
        );
        assert_eq!(files_under(dir.path()), ["mod.zip"]);
    }

    #[test]
    fn hostile_names_cannot_place_files_outside_the_output_directory() {
        let hostile = [
            "../x.vpk",
            "..\\x.vpk",
            "../../../../../../x.vpk",
            "C:\\x.vpk",
            "C:x.vpk",
            "/x.vpk",
            "\\\\server\\share\\x.vpk",
            "\\\\?\\C:\\x.vpk",
            "CON.vpk",
            "NUL.vpk",
            "COM1.vpk",
            "a.vpk:stream",
            "a.vpk::$DATA",
        ];
        let dir = TempDir::new("zip-hostile");
        let entries: Vec<(&str, &[u8])> = hostile.iter().map(|n| (*n, &b"evil"[..])).collect();
        let archive = dir.file("mod.zip", &zip_bytes(&entries, false));
        let out = dir.path().join("lib").join("out");
        let x = extract_vpks(&archive, &out, &Limits::default(), &NoHooks).expect("extracted");

        let expected_vpks = hostile.len() - 2;
        let mut expected: Vec<String> = (0..expected_vpks)
            .map(|n| format!("lib/out/{n}.vpk"))
            .collect();
        expected.push("mod.zip".into());
        expected.sort();
        assert_eq!(files_under(dir.path()), expected, "nothing outside lib/out");
        assert_eq!(x.skipped, ["a.vpk:stream", "a.vpk::$DATA"]);
        let shown: Vec<_> = x.vpks.iter().map(|v| v.archive_path.as_str()).collect();
        assert_eq!(shown, hostile[..expected_vpks]);
        for (n, v) in x.vpks.iter().enumerate() {
            assert_eq!(v.output, out.join(format!("{n}.vpk")));
        }
    }

    #[test]
    fn a_zip_bomb_stops_at_the_limit_quickly_and_leaves_nothing() {
        let dir = TempDir::new("zip-bomb");
        let zeros = vec![0u8; 64 << 20];
        let bytes = zip_bytes(&[("small.vpk", b"ok"), ("bomb.vpk", &zeros)], false);
        let archive = dir.file("bomb.zip", &bytes);
        let limit = Limits::default().total_for(bytes.len() as u64);
        assert!(limit < 8 << 20, "the 20x cap is far below 64 MB: {limit}");
        let hooks = RecordingHooks::default();
        let out = dir.path().join("out");

        let started = Instant::now();
        let err = extract_vpks(&archive, &out, &Limits::default(), &hooks).expect_err("bomb");
        assert!(
            started.elapsed() < Duration::from_secs(5),
            "{:?}",
            started.elapsed()
        );
        match err {
            ArchiveError::Limit(LimitExceeded::TotalSize { limit: l, entry }) => {
                assert_eq!((l, entry.as_str()), (limit, "bomb.vpk"));
            }
            other => panic!("{other:?}"),
        }
        assert!(files_under(&out).is_empty(), "partial output removed");
        assert!(hooks.events().iter().any(|e| e.starts_with("removed")));
    }

    #[test]
    fn a_highly_compressed_entry_trips_the_ratio_limit() {
        let dir = TempDir::new("zip-ratio");
        let zeros = vec![0u8; 4 << 20];
        let archive = dir.file("r.zip", &zip_bytes(&[("flat.vpk", &zeros)], false));
        let limits = Limits {
            max_total_factor: u64::MAX,
            ..Limits::default()
        };
        let out = dir.path().join("out");
        let err = extract_vpks(&archive, &out, &limits, &NoHooks).expect_err("ratio");
        match err {
            ArchiveError::Limit(LimitExceeded::Ratio { limit, entry }) => {
                assert_eq!((limit, entry.as_str()), (100, "flat.vpk"));
            }
            other => panic!("{other:?}"),
        }
        assert!(files_under(&out).is_empty());
    }

    #[test]
    fn too_many_vpks_are_refused_before_anything_is_written() {
        let dir = TempDir::new("zip-count");
        let names: Vec<String> = (0..257).map(|n| format!("p{n}.vpk")).collect();
        let entries: Vec<(&str, &[u8])> = names.iter().map(|n| (n.as_str(), &b"v"[..])).collect();
        let archive = dir.file("pack.zip", &zip_bytes(&entries, true));
        let out = dir.path().join("out");
        let err = extract_vpks(&archive, &out, &Limits::default(), &NoHooks).expect_err("count");
        match err {
            ArchiveError::Limit(LimitExceeded::TooManyVpks { limit, found }) => {
                assert_eq!((limit, found), (256, 257));
            }
            other => panic!("{other:?}"),
        }
        assert!(!out.exists() || files_under(&out).is_empty());

        let ok = dir.file("ok.zip", &zip_bytes(&entries[..256], true));
        let x = extract_vpks(&ok, &dir.path().join("ok"), &Limits::default(), &NoHooks)
            .expect("256 is fine");
        assert_eq!(x.vpks.len(), 256);
    }

    #[test]
    fn free_space_is_checked_before_starting() {
        let dir = TempDir::new("zip-space");
        let data = vec![7u8; 10_000];
        let archive = dir.file("m.zip", &zip_bytes(&[("a.vpk", &data)], true));
        let out = dir.path().join("out");
        let err = extract_vpks(
            &archive,
            &out,
            &Limits::default(),
            &RecordingHooks::with_free(10_500),
        )
        .expect_err("needs 10 % margin");
        match err {
            ArchiveError::InsufficientSpace { needed, available } => {
                assert_eq!((needed, available), (11_000, 10_500));
            }
            other => panic!("{other:?}"),
        }
        assert!(files_under(dir.path()) == ["m.zip"]);
        extract_vpks(
            &archive,
            &out,
            &Limits::default(),
            &RecordingHooks::with_free(11_000),
        )
        .expect("enough");
    }

    #[test]
    fn a_password_protected_vpk_is_reported_as_such() {
        let mut bytes = zip_bytes(
            &[("readme.txt", b"public"), ("locked.vpk", b"secret vpk")],
            true,
        );
        set_encrypted_flag(&mut bytes, "locked.vpk");
        let dir = TempDir::new("zip-encrypted");
        let archive = dir.file("e.zip", &bytes);
        let err = extract_vpks(
            &archive,
            &dir.path().join("out"),
            &Limits::default(),
            &NoHooks,
        )
        .expect_err("encrypted");
        match err {
            ArchiveError::Encrypted { entry } => assert_eq!(entry.as_deref(), Some("locked.vpk")),
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn an_unsupported_method_is_named() {
        let mut bytes = zip_bytes(&[("a.vpk", b"data")], true);
        set_method(&mut bytes, 12);
        let dir = TempDir::new("zip-method");
        let archive = dir.file("b.zip", &bytes);
        let err = extract_vpks(
            &archive,
            &dir.path().join("out"),
            &Limits::default(),
            &NoHooks,
        )
        .expect_err("bzip2");
        match err {
            ArchiveError::UnsupportedMethod { entry, method } => {
                assert_eq!(entry.as_deref(), Some("a.vpk"));
                assert!(method.to_lowercase().contains("bzip2"), "{method}");
            }
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn corrupted_data_is_reported_as_damaged() {
        let mut bytes = zip_bytes(&[("a.vpk", b"original data")], true);
        let at = find(&bytes, b"original").expect("data");
        bytes[at] = b'O';
        let dir = TempDir::new("zip-crc");
        let archive = dir.file("c.zip", &bytes);
        let out = dir.path().join("out");
        let err = extract_vpks(&archive, &out, &Limits::default(), &NoHooks).expect_err("crc");
        assert!(
            matches!(&err, ArchiveError::Damaged { entry: Some(e), .. } if e == "a.vpk"),
            "{err:?}"
        );
        assert!(files_under(&out).is_empty());
    }

    #[test]
    fn a_truncated_download_is_damaged() {
        let bytes = zip_bytes(&[("a.vpk", b"data")], true);
        let dir = TempDir::new("zip-truncated");
        let archive = dir.file("t.zip", &bytes[..bytes.len() - 10]);
        let err = extract_vpks(
            &archive,
            &dir.path().join("out"),
            &Limits::default(),
            &NoHooks,
        )
        .expect_err("truncated");
        assert!(matches!(err, ArchiveError::Damaged { .. }), "{err:?}");
    }

    fn find(hay: &[u8], needle: &[u8]) -> Option<usize> {
        hay.windows(needle.len()).position(|w| w == needle)
    }

    /// Sets general-purpose flag bit 0 ("encrypted") in the local and central headers of `name`.
    fn set_encrypted_flag(bytes: &mut [u8], name: &str) {
        let mut i = 0;
        while i + 4 <= bytes.len() {
            let (flags_at, name_at) = match &bytes[i..i + 4] {
                b"PK\x03\x04" => (i + 6, i + 30),
                b"PK\x01\x02" => (i + 8, i + 46),
                _ => {
                    i += 1;
                    continue;
                }
            };
            if bytes[name_at..].starts_with(name.as_bytes()) {
                bytes[flags_at] |= 1;
            }
            i += 1;
        }
    }

    /// Rewrites the compression method in every local and central header.
    fn set_method(bytes: &mut [u8], method: u16) {
        let mut i = 0;
        while i + 4 <= bytes.len() {
            match &bytes[i..i + 4] {
                b"PK\x03\x04" => bytes[i + 8..i + 10].copy_from_slice(&method.to_le_bytes()),
                b"PK\x01\x02" => bytes[i + 10..i + 12].copy_from_slice(&method.to_le_bytes()),
                _ => {}
            }
            i += 1;
        }
    }
}
