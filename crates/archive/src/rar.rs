//! RAR backend (RAR 2.9–4.x and RAR5) on libarchive's readers compiled into the crate (ADR 0008).
//!
//! RAR has no central file list: headers sit between the files' data, so the list comes from walking the whole
//! archive. Extraction therefore takes two passes over the file: the first collects every header (the checks before
//! anything is written need them), the second unpacks the VPKs.
//!
//! In a *solid* archive every file continues the previous one's compressed stream, so reaching a file means
//! decoding all files before it. libarchive would do that silently when asked to skip, with no limit on the work;
//! here the skipped files of a solid archive are decoded through the meter like 7z solid blocks, so a bomb among
//! them stops at the size limit. The ratio limit is measured over the whole solid stream, as for one 7z block. In a
//! non-solid archive skipping is a seek that decodes nothing, and the ratio is measured per file against the bytes
//! the file occupies in the archive.
//!
//! Refused before anything is written: password-protected VPKs, links named like VPKs, and parts of multi-volume
//! archives (a part holds only pieces of its files). Every VPK must unpack to exactly the size its header declares.

use crate::error::ArchiveError;
use crate::libarchive::{Failure, Header, Kind, Reader};
use crate::limits::{Limits, Meter, Unit};
use crate::output::{ExtractedVpk, PumpError, drain};
use crate::{Entry, Job};
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};

/// What the main archive header says, read by us before libarchive opens the file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Layout {
    solid: bool,
    volume: bool,
}

/// An unknown layout (encrypted or unreadable main header) counts as solid: decoding everything through the meter
/// is the safe side, and libarchive reports what is actually wrong.
const UNKNOWN: Layout = Layout {
    solid: true,
    volume: false,
};

const CHUNK: usize = 64 * 1024;

pub(crate) fn list(mut file: File) -> Result<Vec<Entry>, ArchiveError> {
    let archive_size = file.metadata().map_err(ArchiveError::Io)?.len();
    // Listing a solid archive decodes it; this only bounds that work. The limits that protect the disk apply
    // when extracting.
    let bound = Limits {
        max_total_factor: u64::MAX,
        max_entry_ratio: u64::MAX,
        ..Limits::default()
    };
    let mut meter = Meter::new(&bound, archive_size);
    let headers = scan(&mut file, &mut meter, archive_size)?.1;
    Ok(headers.iter().map(entry_of).collect())
}

pub(crate) fn extract(
    mut file: File,
    job: &Job<'_>,
) -> Result<(Vec<Entry>, Vec<ExtractedVpk>), ArchiveError> {
    let (layout, headers) = scan(&mut file, &mut job.meter(), job.archive_size)?;
    let entries: Vec<Entry> = headers.iter().map(entry_of).collect();
    job.preflight(&entries)?;
    for h in headers.iter().filter(|h| is_vpk(h)) {
        if h.encrypted {
            return Err(ArchiveError::Encrypted {
                entry: Some(h.name.clone()),
            });
        }
        if h.kind == Kind::Link {
            return Err(ArchiveError::UnsupportedMethod {
                entry: Some(h.name.clone()),
                method: "a link instead of a file".into(),
            });
        }
    }
    let Some(last_vpk) = headers.iter().rposition(is_vpk) else {
        return Ok((entries, Vec::new()));
    };

    let mut reader = Reader::open(&mut file).map_err(|f| failure(f, None))?;
    let mut meter = job.meter();
    let mut staging = job.staging();
    let mut solid_unit = Unit::new(job.archive_size);
    let mut buf = vec![0u8; CHUNK];
    for (i, expected) in headers[..=last_vpk].iter().enumerate() {
        let found = reader.next().map_err(|f| failure(f, None))?;
        if found.as_ref().map(|h| &h.name) != Some(&expected.name) {
            return Err(damaged(
                Some(&expected.name),
                "the file list changed between two reads",
            ));
        }
        let name = expected.name.as_str();
        if !is_vpk(expected) {
            pass_over(
                &mut reader,
                expected,
                layout,
                &mut meter,
                &mut solid_unit,
                &mut buf,
            )?;
            continue;
        }
        let mut own_unit = Unit::new(span(&headers, i, job.archive_size));
        let unit = if layout.solid {
            &mut solid_unit
        } else {
            &mut own_unit
        };
        let mut data = EntryData::new(&mut reader, expected.size);
        let written = staging.write(name, &mut data, &mut meter, unit);
        if let (Err(ArchiveError::Damaged { .. }), Some(f)) = (&written, data.failure.take()) {
            return Err(failure(f, Some(name)));
        }
        written?;
    }
    Ok((entries, staging.commit()?))
}

/// The first pass: the layout and every header, in archive order. Leaves `file` at its start.
fn scan(
    file: &mut File,
    meter: &mut Meter,
    archive_size: u64,
) -> Result<(Layout, Vec<Header>), ArchiveError> {
    let layout = layout_of(file)?;
    if layout.volume {
        return Err(ArchiveError::UnsupportedMethod {
            entry: None,
            method: "a part of a multi-volume RAR archive; join the parts into one archive first"
                .into(),
        });
    }
    let mut reader = Reader::open(&mut *file).map_err(|f| failure(f, None))?;
    let mut unit = Unit::new(archive_size);
    let mut buf = vec![0u8; CHUNK];
    let mut headers = Vec::new();
    while let Some(h) = reader.next().map_err(|f| failure(f, None))? {
        pass_over(&mut reader, &h, layout, meter, &mut unit, &mut buf)?;
        headers.push(h);
    }
    drop(reader);
    if has_end_marker(file, archive_size).map_err(ArchiveError::Io)? == Some(false) {
        return Err(damaged(
            None,
            "the archive ends early: its end-of-archive marker is missing",
        ));
    }
    file.seek(SeekFrom::Start(0)).map_err(ArchiveError::Io)?;
    Ok((layout, headers))
}

/// Whether the archive ends with an end-of-archive block; `None` when that cannot be told (encrypted headers).
///
/// libarchive treats a file that stops between two headers as a complete archive, so a download cut at the wrong
/// place would look like a smaller mod. Walks the block headers only, never their data.
fn has_end_marker(file: &mut File, size: u64) -> io::Result<Option<bool>> {
    let mut head = [0u8; 64];
    let mut read_at = |at: u64, buf: &mut [u8; 64]| -> io::Result<usize> {
        file.seek(SeekFrom::Start(at))?;
        let mut n = 0;
        while n < buf.len() {
            match file.read(&mut buf[n..])? {
                0 => break,
                k => n += k,
            }
        }
        Ok(n)
    };
    let n = read_at(0, &mut head)?;
    let rar5 = head[..n].starts_with(b"Rar!\x1A\x07\x01\x00");
    let mut at: u64 = if rar5 { 8 } else { 7 };
    loop {
        let n = read_at(at, &mut head)?;
        let block = if rar5 {
            rar5_block(&head[..n])
        } else {
            rar4_block(&head[..n])
        };
        let Some(block) = block else {
            return Ok(Some(false));
        };
        match block.kind {
            BlockKind::End => return Ok(Some(true)),
            BlockKind::Encrypted => return Ok(None),
            BlockKind::Other => {}
        }
        at = match at.checked_add(block.length) {
            Some(next) if next <= size => next,
            _ => return Ok(Some(false)),
        };
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BlockKind {
    End,
    /// The rest of the headers is encrypted and cannot be walked.
    Encrypted,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Block {
    kind: BlockKind,
    /// Header and data together; at least one byte.
    length: u64,
}

/// A RAR5 block at the start of `b`: CRC32, header size, type, header flags, [extra size], [data size].
fn rar5_block(b: &[u8]) -> Option<Block> {
    let mut at = 4;
    let size = vint(b, &mut at)?;
    let header_end = (at as u64).checked_add(size)?;
    let kind = vint(b, &mut at)?;
    let flags = vint(b, &mut at)?;
    if flags & 1 != 0 {
        vint(b, &mut at)?;
    }
    let data = if flags & 2 != 0 { vint(b, &mut at)? } else { 0 };
    Some(Block {
        kind: match kind {
            4 => BlockKind::Encrypted,
            5 => BlockKind::End,
            _ => BlockKind::Other,
        },
        length: header_end.checked_add(data)?,
    })
}

/// A RAR 2.9–4.x block at the start of `b`: CRC16, type, flags, header size, [data size (low, high for files)].
fn rar4_block(b: &[u8]) -> Option<Block> {
    let u16_at = |i: usize| Some(u16::from_le_bytes([*b.get(i)?, *b.get(i + 1)?]));
    let u32_at = |i: usize| {
        Some(u32::from_le_bytes([
            *b.get(i)?,
            *b.get(i + 1)?,
            *b.get(i + 2)?,
            *b.get(i + 3)?,
        ]))
    };
    let kind = *b.get(2)?;
    let flags = u16_at(3)?;
    let size = u16_at(5)?;
    if size < 7 {
        return None;
    }
    let mut data = 0u64;
    if flags & 0x8000 != 0 {
        data = u64::from(u32_at(7)?);
        // A file header with LHD_LARGE carries the high 32 bits of its packed size.
        if kind == 0x74 && flags & 0x0100 != 0 {
            data |= u64::from(u32_at(32)?) << 32;
        }
    }
    Some(Block {
        kind: match kind {
            0x7b => BlockKind::End,
            // The main header with MHD_PASSWORD: everything after it is encrypted.
            0x73 if flags & 0x0080 != 0 => BlockKind::Encrypted,
            _ => BlockKind::Other,
        },
        length: u64::from(size).checked_add(data)?,
    })
}

/// Moves past an entry that is not extracted: decoded through the meter in a solid archive, skipped otherwise.
fn pass_over<R: Read + Seek>(
    reader: &mut Reader<R>,
    h: &Header,
    layout: Layout,
    meter: &mut Meter,
    unit: &mut Unit,
    buf: &mut [u8],
) -> Result<(), ArchiveError> {
    // Protected data cannot be decoded; libarchive moves past it without trying.
    if !layout.solid || h.kind != Kind::File || h.encrypted {
        return reader.skip().map_err(|f| failure(f, Some(&h.name)));
    }
    let mut data = EntryData::new(reader, h.size);
    drain(&mut data, meter, unit, &h.name, buf).map_err(|e| match (e, data.failure.take()) {
        (PumpError::Limit(l), _) => ArchiveError::Limit(l),
        (PumpError::Cancelled, _) => ArchiveError::Cancelled,
        (_, Some(f)) => failure(f, Some(&h.name)),
        (PumpError::Read(e) | PumpError::Write(e), None) => damaged(Some(&h.name), &e.to_string()),
    })
}

/// Bytes entry `i` occupies in the archive (its header and data): the compressed size its ratio is measured on.
fn span(headers: &[Header], i: usize, archive_size: u64) -> u64 {
    let end = headers.get(i + 1).map_or(archive_size, |h| h.position);
    end.saturating_sub(headers[i].position)
}

fn is_vpk(h: &Header) -> bool {
    h.kind != Kind::Dir && crate::is_vpk_name(&h.name)
}

fn entry_of(h: &Header) -> Entry {
    Entry {
        name: h.name.clone(),
        size: h.size.unwrap_or(0),
        is_dir: h.kind == Kind::Dir,
    }
}

/// The current entry's data as a `Read` that fails unless the data ends at exactly the declared size.
struct EntryData<'r, R: Read + Seek> {
    reader: &'r mut Reader<R>,
    declared: Option<u64>,
    read: u64,
    /// The libarchive failure behind the last `io::Error`, to report it precisely.
    failure: Option<Failure>,
}

impl<'r, R: Read + Seek> EntryData<'r, R> {
    fn new(reader: &'r mut Reader<R>, declared: Option<u64>) -> Self {
        Self {
            reader,
            declared,
            read: 0,
            failure: None,
        }
    }
}

impl<R: Read + Seek> Read for EntryData<'_, R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = match self.reader.read(buf) {
            Ok(n) => n,
            Err(f) => {
                let e = io::Error::new(io::ErrorKind::InvalidData, f.to_string());
                self.failure = Some(f);
                return Err(e);
            }
        };
        self.read += n as u64;
        match self.declared {
            Some(declared) if self.read > declared || (n == 0 && self.read != declared) => {
                Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "the file unpacks to {} bytes but its header declares {declared}",
                        self.read
                    ),
                ))
            }
            _ => Ok(n),
        }
    }
}

/// Reads the main header's flags. Leaves `file` at its start.
fn layout_of(file: &mut File) -> Result<Layout, ArchiveError> {
    let mut head = Vec::with_capacity(64);
    (&mut *file)
        .take(64)
        .read_to_end(&mut head)
        .map_err(ArchiveError::Io)?;
    file.seek(SeekFrom::Start(0)).map_err(ArchiveError::Io)?;
    Ok(parse_layout(&head).unwrap_or(UNKNOWN))
}

/// The main header of RAR5 or RAR 2.9–4.x, right after the signature.
fn parse_layout(head: &[u8]) -> Option<Layout> {
    if let Some(rest) = head.strip_prefix(b"Rar!\x1A\x07\x01\x00") {
        // CRC32, header size, type (1 = main), header flags, [extra size], [data size], archive flags.
        let mut at = 4;
        vint(rest, &mut at)?;
        if vint(rest, &mut at)? != 1 {
            return None;
        }
        let flags = vint(rest, &mut at)?;
        for present in [flags & 1 != 0, flags & 2 != 0] {
            if present {
                vint(rest, &mut at)?;
            }
        }
        let archive = vint(rest, &mut at)?;
        Some(Layout {
            volume: archive & 0x01 != 0,
            solid: archive & 0x04 != 0,
        })
    } else if let Some(rest) = head.strip_prefix(b"Rar!\x1A\x07\x00") {
        // CRC16, type (0x73 = main), flags: 0x0001 volume, 0x0008 solid.
        match rest.get(..5)? {
            &[_, _, 0x73, lo, hi] => {
                let flags = u16::from_le_bytes([lo, hi]);
                Some(Layout {
                    volume: flags & 0x0001 != 0,
                    solid: flags & 0x0008 != 0,
                })
            }
            _ => None,
        }
    } else {
        None
    }
}

/// A RAR5 variable-length integer: 7 bits per byte, the high bit set on all but the last byte.
fn vint(bytes: &[u8], at: &mut usize) -> Option<u64> {
    let mut value = 0u64;
    for shift in (0..64).step_by(7) {
        let b = *bytes.get(*at)?;
        *at += 1;
        value |= u64::from(b & 0x7f) << shift;
        if b & 0x80 == 0 {
            return Some(value);
        }
    }
    None
}

/// The user-facing error for a libarchive failure.
fn failure(f: Failure, entry: Option<&str>) -> ArchiveError {
    let entry = entry.map(str::to_owned);
    match f {
        Failure::Encrypted => ArchiveError::Encrypted { entry },
        Failure::Input(e) => ArchiveError::Damaged {
            entry,
            detail: format!("reading the archive failed: {e}"),
        },
        Failure::Library(message)
            if message.contains("nsupported") || message.contains("not supported") =>
        {
            ArchiveError::UnsupportedMethod {
                entry,
                method: message,
            }
        }
        Failure::Library(detail) => ArchiveError::Damaged { entry, detail },
    }
}

fn damaged(entry: Option<&str>, detail: &str) -> ArchiveError {
    ArchiveError::Damaged {
        entry: entry.map(str::to_owned),
        detail: detail.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    //! The archives come from libarchive 3.8.9's test suite (`libarchive/test/*.rar.uu`, decoded; BSD-2-Clause,
    //! see `vendor/libarchive/COPYING`). Entry names are renamed in place to `.vpk` where a test needs VPKs, and
    //! the CRC of the changed header is recomputed, so the archives stay valid. Expected hashes were taken from
    //! 7-Zip's extraction of the original files.
    use crate::error::LimitExceeded;
    use crate::test_support::{RecordingHooks, TempDir, files_under};
    use crate::{ArchiveError, Extraction, Format, Limits, NoHooks, extract_vpks, list};
    use sha2::{Digest, Sha256};
    use std::path::{Path, PathBuf};
    use std::time::{Duration, Instant};

    const TEST1_BIN: &str = "7d89f86f9f69d744ffff3fc043e15bf89fc3ffc134ffcbb31d164a99bb8b67b0";
    const TEST2_BIN: &str = "f81e6fceeeab366306b23466bf6bb3aac2875e0906dc20a8652be0696ceb15a2";
    const TEST3_BIN: &str = "5e621f2b6ce8fed758c3df8221f994eda55d1e432c7cc4349c34a30ec2e1c43d";
    const TEST5_BIN: &str = "b0622b648b174abd9c5f3965155bbcc82c642f997ab8949add0a8632bf94e636";
    const TEST_BIN: &str = "588870a2dade35c2650fbb7898c9a9c7f21fce7c281198604e8d0c9737f2c375";
    const TEST_TXT: &str = "5a5f16e01faf8adf92eb4499a2d3e93010c4b41dbb7f698f4a8466d9f58e6dd2";

    fn fixture(name: &str) -> Vec<u8> {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/rar")
            .join(name);
        std::fs::read(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
    }

    fn hex(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{b:02x}")).collect()
    }

    fn sha_of(path: &Path) -> String {
        hex(&Sha256::digest(std::fs::read(path).expect("read")))
    }

    fn crc32(data: &[u8]) -> u32 {
        let mut crc = !0u32;
        for &b in data {
            crc ^= u32::from(b);
            for _ in 0..8 {
                crc = if crc & 1 != 0 {
                    (crc >> 1) ^ 0xEDB8_8320
                } else {
                    crc >> 1
                };
            }
        }
        !crc
    }

    /// RAR5 variable-length integer at `at`: (value, length in bytes).
    fn vint(bytes: &[u8], at: usize) -> (u64, usize) {
        let mut value = 0u64;
        for i in 0..10 {
            let b = bytes[at + i];
            value |= u64::from(b & 0x7f) << (7 * i);
            if b & 0x80 == 0 {
                return (value, i + 1);
            }
        }
        panic!("bad vint at {at}");
    }

    /// One header block: where it starts, where its fields start (after the size field), where it ends.
    #[derive(Debug, Clone, Copy)]
    struct Block {
        start: usize,
        fields: usize,
        end: usize,
    }

    /// The header blocks of a RAR5 or RAR4 archive, in file order.
    fn blocks(bytes: &[u8]) -> Vec<Block> {
        let mut out = Vec::new();
        if bytes.starts_with(b"Rar!\x1A\x07\x01\x00") {
            let mut p = 8;
            while p + 4 < bytes.len() {
                let (size, len) = vint(bytes, p + 4);
                let fields = p + 4 + len;
                let end = fields + usize::try_from(size).expect("size");
                let (_, l_type) = vint(bytes, fields);
                let (flags, l_flags) = vint(bytes, fields + l_type);
                let mut q = fields + l_type + l_flags;
                if flags & 1 != 0 {
                    q += vint(bytes, q).1;
                }
                let data = if flags & 2 != 0 { vint(bytes, q).0 } else { 0 };
                out.push(Block {
                    start: p,
                    fields,
                    end,
                });
                p = end + usize::try_from(data).expect("data");
            }
        } else {
            let mut p = 7;
            while p + 7 <= bytes.len() {
                let flags = u16::from_le_bytes([bytes[p + 3], bytes[p + 4]]);
                let size = usize::from(u16::from_le_bytes([bytes[p + 5], bytes[p + 6]]));
                let data = if flags & 0x8000 != 0 {
                    u32::from_le_bytes([bytes[p + 7], bytes[p + 8], bytes[p + 9], bytes[p + 10]])
                } else {
                    0
                };
                out.push(Block {
                    start: p,
                    fields: p + 7,
                    end: p + size,
                });
                p += size + usize::try_from(data).expect("data");
            }
        }
        out
    }

    /// Recomputes the CRC of the header block holding byte `at`.
    fn fix_crc(bytes: &mut [u8], at: usize) {
        let block = blocks(bytes)
            .into_iter()
            .find(|b| (b.start..b.end).contains(&at))
            .expect("inside a header");
        if bytes.starts_with(b"Rar!\x1A\x07\x01\x00") {
            let crc = crc32(&bytes[block.start + 4..block.end]);
            bytes[block.start..block.start + 4].copy_from_slice(&crc.to_le_bytes());
        } else {
            let crc = (crc32(&bytes[block.start + 2..block.end]) & 0xffff) as u16;
            bytes[block.start..block.start + 2].copy_from_slice(&crc.to_le_bytes());
        }
    }

    /// Replaces the first `from` inside a header with `to` (same length) and fixes that header's CRC.
    fn rename(bytes: &mut [u8], from: &str, to: &str) {
        assert_eq!(from.len(), to.len());
        let headers = blocks(bytes);
        let at = bytes
            .windows(from.len())
            .enumerate()
            .position(|(i, w)| {
                w == from.as_bytes() && headers.iter().any(|b| (b.start..b.end).contains(&i))
            })
            .unwrap_or_else(|| panic!("{from} not found in a header"));
        bytes[at..at + to.len()].copy_from_slice(to.as_bytes());
        fix_crc(bytes, at);
    }

    fn renamed(name: &str, renames: &[(&str, &str)]) -> Vec<u8> {
        let mut bytes = fixture(name);
        for (from, to) in renames {
            rename(&mut bytes, from, to);
        }
        bytes
    }

    fn extract(
        dir: &TempDir,
        bytes: &[u8],
        limits: &Limits,
    ) -> (PathBuf, Result<Extraction, ArchiveError>) {
        let archive = dir.file("mod.rar", bytes);
        let out = dir.path().join("out");
        let result = extract_vpks(&archive, &out, limits, &NoHooks);
        (out, result)
    }

    fn assert_nothing_written(out: &Path) {
        assert!(
            !out.exists() || files_under(out).is_empty(),
            "{:?}",
            files_under(out)
        );
    }

    #[test]
    fn rar5_vpks_are_extracted_under_our_names_and_the_rest_is_skipped() {
        let dir = TempDir::new("rar5-basic");
        let bytes = renamed(
            "test_read_format_rar5_multiple_files.rar",
            &[("test1.bin", "test1.vpk"), ("test3.bin", "test3.vpk")],
        );
        let (out, result) = extract(&dir, &bytes, &Limits::default());
        let x = result.expect("extracted");
        assert_eq!(x.format, Format::Rar);
        assert_eq!(x.skipped, ["test2.bin", "test4.bin"]);
        assert_eq!(files_under(&out), ["0.vpk", "1.vpk"]);
        let shown: Vec<_> = x
            .vpks
            .iter()
            .map(|v| (v.archive_path.as_str(), v.size))
            .collect();
        assert_eq!(shown, [("test1.vpk", 4096), ("test3.vpk", 4096)]);
        assert_eq!(sha_of(&out.join("0.vpk")), TEST1_BIN);
        assert_eq!(sha_of(&out.join("1.vpk")), TEST3_BIN);
        assert_eq!(hex(&x.vpks[1].sha256), TEST3_BIN);
    }

    #[test]
    fn vpks_inside_a_solid_rar5_archive_are_found_between_skipped_files() {
        let dir = TempDir::new("rar5-solid");
        let bytes = renamed(
            "test_read_format_rar5_solid.rar",
            &[("test2.bin", "test2.vpk"), ("test5.bin", "test5.vpk")],
        );
        // The test data is far more compressible than mods: 25 KB from 1 KB.
        let limits = Limits {
            max_total_factor: 100,
            ..Limits::default()
        };
        let (out, result) = extract(&dir, &bytes, &limits);
        let x = result.expect("extracted");
        assert_eq!(
            x.skipped,
            [
                "test.bin",
                "test1.bin",
                "test3.bin",
                "test4.bin",
                "test6.bin"
            ]
        );
        assert_eq!(sha_of(&out.join("0.vpk")), TEST2_BIN);
        assert_eq!(sha_of(&out.join("1.vpk")), TEST5_BIN);
    }

    #[test]
    fn rar4_vpks_are_extracted() {
        let dir = TempDir::new("rar4-basic");
        let bytes = renamed(
            "test_read_format_rar_compress_normal.rar",
            &[("test.txt", "test.vpk")],
        );
        let (out, result) = extract(&dir, &bytes, &Limits::default());
        let x = result.expect("extracted");
        assert_eq!(x.vpks.len(), 1);
        assert_eq!(x.vpks[0].archive_path, "testdir/test.vpk");
        assert_eq!(x.vpks[0].size, 20);
        assert_eq!(sha_of(&out.join("0.vpk")), TEST_TXT);
        assert_eq!(
            x.skipped,
            [
                "LibarchiveAddingTest.html",
                "testlink",
                "testdir/LibarchiveAddingTest.html"
            ]
        );
    }

    #[test]
    fn listing_shows_every_entry_without_extracting() {
        let dir = TempDir::new("rar-list");
        let archive = dir.file("l.rar", &fixture("test_read_format_rar.rar"));
        let listing = list(&archive).expect("listed");
        assert_eq!(listing.format, Format::Rar);
        let shown: Vec<_> = listing
            .entries
            .iter()
            .map(|e| (e.name.as_str(), e.size, e.is_dir))
            .collect();
        assert_eq!(
            shown,
            [
                ("test.txt", 20, false),
                ("testlink", 0, false),
                ("testdir/test.txt", 20, false),
                ("testdir", 0, true),
                ("testemptydir", 0, true),
            ]
        );
        assert_eq!(files_under(dir.path()), ["l.rar"]);

        // Solid: listing decodes every file; 25 KB from 1 KB is over the extraction limits, not the listing's.
        let archive = dir.file("s.rar", &fixture("test_read_format_rar5_solid.rar"));
        assert_eq!(list(&archive).expect("solid listed").entries.len(), 7);

        let archive = dir.file("c.rar", &fixture("test_read_format_rar5_compressed.rar"));
        let listing = list(&archive).expect("rar5 listed");
        assert_eq!(listing.entries.len(), 1);
        assert_eq!(
            (listing.entries[0].name.as_str(), listing.entries[0].size),
            ("test.bin", 1200)
        );
    }

    #[test]
    fn hostile_names_cannot_place_files_outside_the_output_directory() {
        let dir = TempDir::new("rar-hostile");
        let bytes = renamed(
            "test_read_format_rar_compress_normal.rar",
            &[("testdir\\test.txt", "..\\..\\..\\..\\.vpk")],
        );
        let archive = dir.file("mod.rar", &bytes);
        let out = dir.path().join("lib").join("out");
        let x = extract_vpks(&archive, &out, &Limits::default(), &NoHooks).expect("extracted");
        assert_eq!(files_under(dir.path()), ["lib/out/0.vpk", "mod.rar"]);
        assert_eq!(x.vpks[0].archive_path, "../../../../.vpk");
        assert_eq!(sha_of(&out.join("0.vpk")), TEST_TXT);
    }

    #[test]
    fn a_password_protected_vpk_is_refused_before_anything_is_written() {
        for name in [
            "test_read_format_rar5_encrypted.rar",
            "test_read_format_rar4_encrypted.rar",
        ] {
            let dir = TempDir::new("rar-encrypted");
            let bytes = renamed(name, &[("a.txt", "a.vpk"), ("b.txt", "b.vpk")]);
            let (out, result) = extract(&dir, &bytes, &Limits::default());
            match result {
                Err(ArchiveError::Encrypted { entry }) => {
                    assert_eq!(entry.as_deref(), Some("b.vpk"), "{name}")
                }
                other => panic!("{name}: {other:?}"),
            }
            assert_nothing_written(&out);
        }
    }

    #[test]
    fn protected_files_that_are_not_vpks_do_not_stop_extraction() {
        let dir = TempDir::new("rar-encrypted-other");
        let bytes = renamed("test_read_format_rar5_encrypted.rar", &[("c.txt", "c.vpk")]);
        let (out, result) = extract(&dir, &bytes, &Limits::default());
        let x = result.expect("extracted");
        assert_eq!(x.skipped, ["a.txt", "b.txt", "d.txt"]);
        assert_eq!(files_under(&out), ["0.vpk"]);
        assert_eq!(x.vpks[0].size, 18);
    }

    #[test]
    fn an_encrypted_file_list_is_reported_as_password_protected() {
        for name in [
            "test_read_format_rar5_encrypted_filenames.rar",
            "test_read_format_rar4_encrypted_filenames.rar",
        ] {
            let dir = TempDir::new("rar-encrypted-names");
            let archive = dir.file("e.rar", &fixture(name));
            let err = list(&archive).expect_err(name);
            assert!(
                matches!(err, ArchiveError::Encrypted { entry: None }),
                "{name}: {err:?}"
            );
            let (out, result) = extract(&dir, &fixture(name), &Limits::default());
            assert!(
                matches!(result, Err(ArchiveError::Encrypted { entry: None })),
                "{name}: {result:?}"
            );
            assert_nothing_written(&out);
        }
    }

    #[test]
    fn a_part_of_a_multi_volume_archive_is_refused() {
        let dir = TempDir::new("rar-volume");
        let bytes = fixture("test_read_format_rar5_multiarchive.part01.rar");
        let archive = dir.file("m.part01.rar", &bytes);
        for result in [
            list(&archive).map(|_| ()),
            extract(&dir, &bytes, &Limits::default()).1.map(|_| ()),
        ] {
            match result {
                Err(ArchiveError::UnsupportedMethod {
                    entry: None,
                    method,
                }) => {
                    assert!(method.contains("multi-volume"), "{method}");
                }
                other => panic!("{other:?}"),
            }
        }
    }

    #[test]
    fn links_named_like_vpks_are_refused() {
        for (name, from, to) in [
            (
                "test_read_format_rar5_symlink.rar",
                "symlink.txt",
                "symlink.vpk",
            ),
            (
                "test_read_format_rar5_hardlink.rar",
                "hardlink.txt",
                "hardlink.vpk",
            ),
        ] {
            let dir = TempDir::new("rar-link");
            let bytes = renamed(name, &[(from, to)]);
            let (out, result) = extract(&dir, &bytes, &Limits::default());
            match result {
                Err(ArchiveError::UnsupportedMethod { entry, method }) => {
                    assert_eq!(entry.as_deref(), Some(to));
                    assert!(method.contains("link"), "{method}");
                }
                other => panic!("{name}: {other:?}"),
            }
            assert_nothing_written(&out);
        }
    }

    #[test]
    fn a_truncated_download_is_damaged() {
        let bytes = renamed(
            "test_read_format_rar5_solid.rar",
            &[("test5.bin", "test5.vpk")],
        );
        let dir = TempDir::new("rar-truncated");
        let (out, result) = extract(&dir, &bytes[..bytes.len() - 300], &Limits::default());
        assert!(
            matches!(result, Err(ArchiveError::Damaged { .. })),
            "{result:?}"
        );
        assert_nothing_written(&out);

        let archive = dir.file("head.rar", &bytes[..20]);
        let err = list(&archive).expect_err("only the signature");
        assert!(matches!(err, ArchiveError::Damaged { .. }), "{err:?}");

        // Cut exactly between two headers, where libarchive sees a normal end of the archive.
        for name in [
            "test_read_format_rar5_multiple_files.rar",
            "test_read_format_rar_compress_normal.rar",
        ] {
            let bytes = fixture(name);
            for block in blocks(&bytes).into_iter().skip(1) {
                let archive = dir.file("cut.rar", &bytes[..block.start]);
                let err = list(&archive).expect_err(name);
                assert!(
                    matches!(&err, ArchiveError::Damaged { .. }),
                    "{name} cut at {}: {err:?}",
                    block.start
                );
            }
        }
    }

    #[test]
    fn corrupted_data_is_damaged() {
        let mut bytes = renamed(
            "test_read_format_rar5_compressed.rar",
            &[("test.bin", "test.vpk")],
        );
        let file = blocks(&bytes)[1];
        for b in &mut bytes[file.end + 100..file.end + 140] {
            *b ^= 0x5a;
        }
        let dir = TempDir::new("rar-corrupt");
        let (out, result) = extract(&dir, &bytes, &Limits::default());
        match result {
            Err(ArchiveError::Damaged { entry, .. }) => {
                assert_eq!(entry.as_deref(), Some("test.vpk"))
            }
            other => panic!("{other:?}"),
        }
        assert_nothing_written(&out);
    }

    #[test]
    fn a_header_that_overstates_the_size_is_damaged() {
        // helloworld.txt is stored (29 bytes); its header is made to declare 30.
        let mut bytes = renamed(
            "test_read_format_rar5_stored.rar",
            &[("helloworld.txt", "helloworld.vpk")],
        );
        let file = blocks(&bytes)[1];
        let (_, l_type) = vint(&bytes, file.fields);
        let (flags, l_flags) = vint(&bytes, file.fields + l_type);
        let mut q = file.fields + l_type + l_flags;
        for bit in [1, 2] {
            if flags & bit != 0 {
                q += vint(&bytes, q).1;
            }
        }
        q += vint(&bytes, q).1; // file flags
        assert_eq!(vint(&bytes, q), (29, 2), "unpacked size");
        bytes[q] = 0x80 | 30;
        fix_crc(&mut bytes, q);
        let dir = TempDir::new("rar-size");
        let (out, result) = extract(&dir, &bytes, &Limits::default());
        match result {
            Err(ArchiveError::Damaged { entry, .. }) => {
                assert_eq!(entry.as_deref(), Some("helloworld.vpk"))
            }
            other => panic!("{other:?}"),
        }
        assert_nothing_written(&out);
    }

    #[test]
    fn a_ppmd_bomb_stops_at_the_limits_quickly() {
        // 176 KB that unpack to 241 MB.
        let bytes = renamed(
            "test_read_format_rar_ppmd_lzss_conversion.rar",
            &[("conversion_test.txt", "conversion_test.vpk")],
        );
        let dir = TempDir::new("rar-bomb");
        let started = Instant::now();
        let (out, result) = extract(&dir, &bytes, &Limits::default());
        assert!(
            started.elapsed() < Duration::from_secs(5),
            "{:?}",
            started.elapsed()
        );
        let limit = Limits::default().total_for(bytes.len() as u64);
        match result {
            Err(ArchiveError::Limit(LimitExceeded::TotalSize { limit: l, entry })) => {
                assert_eq!(
                    (l, entry.as_str()),
                    (limit, "ppmd_lzss_conversion_test.vpk")
                );
            }
            other => panic!("{other:?}"),
        }
        assert_nothing_written(&out);

        let limits = Limits {
            max_total_factor: u64::MAX,
            ..Limits::default()
        };
        let (_, result) = extract(&dir, &bytes, &limits);
        assert!(
            matches!(
                &result,
                Err(ArchiveError::Limit(LimitExceeded::Ratio { limit: 100, .. }))
            ),
            "{result:?}"
        );
    }

    #[test]
    fn a_bomb_that_is_not_a_vpk_is_never_unpacked() {
        let dir = TempDir::new("rar-bomb-skipped");
        let started = Instant::now();
        let (out, result) = extract(
            &dir,
            &fixture("test_read_format_rar_ppmd_lzss_conversion.rar"),
            &Limits::default(),
        );
        assert!(
            started.elapsed() < Duration::from_secs(1),
            "{:?}",
            started.elapsed()
        );
        let x = result.expect("nothing to extract");
        assert!(x.vpks.is_empty());
        assert_eq!(x.skipped, ["ppmd_lzss_conversion_test.txt"]);
        assert_nothing_written(&out);
    }

    #[test]
    fn skipped_files_of_a_solid_archive_count_against_the_limit() {
        let bytes = renamed(
            "test_read_format_rar5_solid.rar",
            &[("test5.bin", "test5.vpk")],
        );
        let limits = Limits {
            max_total_bytes: 3000,
            max_total_factor: u64::MAX,
            ..Limits::default()
        };
        let dir = TempDir::new("rar-solid-limit");
        let (out, result) = extract(&dir, &bytes, &limits);
        match result {
            Err(ArchiveError::Limit(LimitExceeded::TotalSize { limit, entry })) => {
                assert_eq!((limit, entry.as_str()), (3000, "test1.bin"));
            }
            other => panic!("{other:?}"),
        }
        assert_nothing_written(&out);
    }

    #[test]
    fn too_many_vpks_are_refused_before_anything_is_written() {
        let bytes = renamed(
            "test_read_format_rar5_multiple_files.rar",
            &[("test1.bin", "test1.vpk"), ("test2.bin", "test2.vpk")],
        );
        let limits = Limits {
            max_vpk_entries: 1,
            ..Limits::default()
        };
        let dir = TempDir::new("rar-count");
        let (out, result) = extract(&dir, &bytes, &limits);
        assert!(
            matches!(
                result,
                Err(ArchiveError::Limit(LimitExceeded::TooManyVpks {
                    limit: 1,
                    found: 2
                }))
            ),
            "{result:?}"
        );
        assert_nothing_written(&out);
    }

    #[test]
    fn free_space_is_checked_before_starting() {
        let bytes = renamed(
            "test_read_format_rar5_compressed.rar",
            &[("test.bin", "test.vpk")],
        );
        let dir = TempDir::new("rar-space");
        let archive = dir.file("m.rar", &bytes);
        let out = dir.path().join("out");
        let err = extract_vpks(
            &archive,
            &out,
            &Limits::default(),
            &RecordingHooks::with_free(1000),
        )
        .expect_err("needs 1320");
        assert!(
            matches!(
                err,
                ArchiveError::InsufficientSpace {
                    needed: 1320,
                    available: 1000
                }
            ),
            "{err:?}"
        );
        assert_nothing_written(&out);
        let x = extract_vpks(
            &archive,
            &out,
            &Limits::default(),
            &RecordingHooks::with_free(1320),
        )
        .expect("enough");
        assert_eq!(hex(&x.vpks[0].sha256), TEST_BIN);
    }
}
