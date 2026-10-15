//! Extraction of mod archives (zip, 7z, RAR) with size, count and ratio limits. Only `.vpk` entries are
//! extracted, always under names chosen by us (ADR 0008).
//!
//! Archives come from strangers, so the design assumes hostile input:
//! - the path stored for an entry is display text only; outputs are `<out>\<n>.vpk` (zip slip impossible);
//! - nothing but `.vpk` entries is unpacked, so nested archives are never opened;
//! - limits are enforced on bytes as they are unpacked, never on sizes the headers declare;
//! - output is written to temp names and renamed only when every VPK is complete; any failure removes it all.
//!
//! The format is recognised by the first bytes, not the file extension. RAR is read by libarchive's RAR and RAR5
//! readers compiled into the crate (ADR 0008); builds for targets other than MSVC refuse it with
//! [`ArchiveError::Rar`].

mod error;
#[cfg(target_env = "msvc")]
mod libarchive;
mod limits;
mod output;
#[cfg(target_env = "msvc")]
mod rar;
mod sevenz;
mod zip_format;

#[cfg(test)]
mod fixtures;
#[cfg(test)]
mod test_support;

pub use error::{ArchiveError, LimitExceeded};
pub use limits::Limits;
pub use output::{ExtractedVpk, FileEvent, Hooks, NoHooks};

use limits::Meter;
use output::Staging;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Zip,
    SevenZip,
    Rar,
}

/// One entry as the archive describes it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    /// The path stored in the archive, for display only.
    pub name: String,
    /// The size the header declares; not trusted for anything but display and the free-space estimate.
    pub size: u64,
    pub is_dir: bool,
}

impl Entry {
    pub fn is_vpk(&self) -> bool {
        !self.is_dir && is_vpk_name(&self.name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Listing {
    pub format: Format,
    pub entries: Vec<Entry>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Extraction {
    pub format: Format,
    /// In output order: `vpks[n].output` is `<out>\<n>.vpk`.
    pub vpks: Vec<ExtractedVpk>,
    /// Names of the files that were not extracted (everything that is not a `.vpk`), in archive order.
    pub skipped: Vec<String>,
}

/// `true` for names ending in `.vpk`, any case. `a.vpk:stream` is not a VPK.
pub fn is_vpk_name(name: &str) -> bool {
    let bytes = name.as_bytes();
    bytes.len() >= 4 && bytes[bytes.len() - 4..].eq_ignore_ascii_case(b".vpk")
}

/// The archive format from the file's first bytes.
pub fn detect(archive: &Path) -> Result<Format, ArchiveError> {
    let mut file = File::open(archive).map_err(ArchiveError::Io)?;
    sniff(&mut file)
}

/// Every entry of the archive, without extracting anything.
pub fn list(archive: &Path) -> Result<Listing, ArchiveError> {
    let mut file = File::open(archive).map_err(ArchiveError::Io)?;
    let format = sniff(&mut file)?;
    let entries = match format {
        Format::Zip => zip_format::list(file)?,
        Format::SevenZip => sevenz::list(file)?,
        Format::Rar => rar_list(file)?,
    };
    Ok(Listing { format, entries })
}

/// Extracts every `.vpk` of `archive` into `out_dir` (created if missing) as `0.vpk`, `1.vpk`, …
///
/// Fails without leaving any file behind if a limit is hit, the archive is damaged, protected or uses an
/// unsupported method, or writing fails. The output names must not exist yet.
pub fn extract_vpks(
    archive: &Path,
    out_dir: &Path,
    limits: &Limits,
    hooks: &dyn Hooks,
) -> Result<Extraction, ArchiveError> {
    let mut file = File::open(archive).map_err(ArchiveError::Io)?;
    let format = sniff(&mut file)?;
    let job = Job {
        limits,
        archive_size: file.metadata().map_err(ArchiveError::Io)?.len(),
        out_dir,
        hooks,
    };
    let (entries, vpks) = match format {
        Format::Zip => zip_format::extract(file, &job)?,
        Format::SevenZip => sevenz::extract(file, &job)?,
        Format::Rar => rar_extract(file, &job)?,
    };
    let skipped = entries
        .into_iter()
        .filter(|e| !e.is_dir && !e.is_vpk())
        .map(|e| e.name)
        .collect();
    Ok(Extraction {
        format,
        vpks,
        skipped,
    })
}

fn sniff(file: &mut File) -> Result<Format, ArchiveError> {
    let mut head = [0u8; 8];
    let mut filled = 0;
    while filled < head.len() {
        match file.read(&mut head[filled..]) {
            Ok(0) => break,
            Ok(n) => filled += n,
            Err(e) if e.kind() == io::ErrorKind::Interrupted => {}
            Err(e) => return Err(ArchiveError::Io(e)),
        }
    }
    file.seek(SeekFrom::Start(0)).map_err(ArchiveError::Io)?;
    format_of(&head[..filled])
}

fn format_of(head: &[u8]) -> Result<Format, ArchiveError> {
    const SEVEN_Z: &[u8] = b"7z\xBC\xAF\x27\x1C";
    if head.starts_with(SEVEN_Z) {
        Ok(Format::SevenZip)
    } else if [b"PK\x03\x04", b"PK\x05\x06", b"PK\x07\x08"]
        .iter()
        .any(|m| head.starts_with(*m))
    {
        Ok(Format::Zip)
    } else if head.starts_with(b"Rar!\x1A\x07") {
        Ok(Format::Rar)
    } else {
        Err(ArchiveError::NotAnArchive)
    }
}

#[cfg(target_env = "msvc")]
use rar::{extract as rar_extract, list as rar_list};

#[cfg(not(target_env = "msvc"))]
fn rar_list(_: File) -> Result<Vec<Entry>, ArchiveError> {
    Err(ArchiveError::Rar)
}

#[cfg(not(target_env = "msvc"))]
fn rar_extract(_: File, _: &Job<'_>) -> Result<(Vec<Entry>, Vec<ExtractedVpk>), ArchiveError> {
    Err(ArchiveError::Rar)
}

/// What every backend needs for one extraction.
pub(crate) struct Job<'a> {
    pub(crate) limits: &'a Limits,
    pub(crate) archive_size: u64,
    pub(crate) out_dir: &'a Path,
    pub(crate) hooks: &'a dyn Hooks,
}

impl<'a> Job<'a> {
    /// Checks made from the file list before anything is unpacked: VPK count, free space, output names.
    pub(crate) fn preflight(&self, entries: &[Entry]) -> Result<(), ArchiveError> {
        let vpks: Vec<&Entry> = entries.iter().filter(|e| e.is_vpk()).collect();
        if vpks.len() > self.limits.max_vpk_entries {
            return Err(LimitExceeded::TooManyVpks {
                limit: self.limits.max_vpk_entries,
                found: vpks.len(),
            }
            .into());
        }
        std::fs::create_dir_all(self.out_dir).map_err(ArchiveError::Io)?;
        if let Some(available) = self.hooks.available_bytes(self.out_dir) {
            let declared = vpks
                .iter()
                .fold(0u64, |sum, e| sum.saturating_add(e.size))
                .min(self.limits.total_for(self.archive_size));
            let needed = declared.saturating_add(declared / 10);
            if needed > available {
                return Err(ArchiveError::InsufficientSpace { needed, available });
            }
        }
        output::ensure_free_names(self.out_dir, vpks.len())
    }

    pub(crate) fn meter(&self) -> Meter<'a> {
        Meter::new(self.limits, self.archive_size).watching(self.hooks)
    }

    pub(crate) fn staging(&self) -> Staging<'_> {
        Staging::new(self.out_dir, self.hooks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_support::{TempDir, files_under, zip_bytes};

    #[test]
    fn vpk_names_are_matched_case_insensitively_on_the_extension_only() {
        for name in ["a.vpk", "A.VPK", "dir/pak01_dir.Vpk", ".vpk"] {
            assert!(is_vpk_name(name), "{name}");
        }
        for name in [
            "a.vpk.zip",
            "a.vpk:stream",
            "a.vpk ",
            "a.vpk.",
            "vpk",
            "a_vpk",
            "",
            "ой.впк",
        ] {
            assert!(!is_vpk_name(name), "{name}");
        }
    }

    #[test]
    fn directories_named_like_vpks_are_not_vpks() {
        let dir = Entry {
            name: "x.vpk".into(),
            size: 0,
            is_dir: true,
        };
        assert!(!dir.is_vpk());
    }

    #[test]
    fn the_format_comes_from_the_first_bytes() {
        assert_eq!(format_of(b"PK\x03\x04rest").expect("zip"), Format::Zip);
        assert_eq!(
            format_of(b"PK\x05\x06\0\0").expect("empty zip"),
            Format::Zip
        );
        assert_eq!(
            format_of(b"7z\xBC\xAF\x27\x1C\0\x04").expect("7z"),
            Format::SevenZip
        );
        assert_eq!(
            format_of(b"Rar!\x1A\x07\x01\x00").expect("rar5"),
            Format::Rar
        );
        assert_eq!(
            format_of(b"Rar!\x1A\x07\x00\xcf").expect("rar4"),
            Format::Rar
        );
        assert!(matches!(
            format_of(b"<html>"),
            Err(ArchiveError::NotAnArchive)
        ));
        assert!(matches!(format_of(b""), Err(ArchiveError::NotAnArchive)));
    }

    #[test]
    fn a_renamed_file_is_recognised_by_content() {
        let dir = TempDir::new("detect");
        let zip = dir.file("actually-a-zip.7z", &zip_bytes(&[("a.vpk", b"x")], true));
        assert_eq!(detect(&zip).expect("zip"), Format::Zip);
        let html = dir.file("mod.zip", b"<!doctype html><title>404</title>");
        assert!(matches!(
            extract_vpks(&html, &dir.path().join("out"), &Limits::default(), &NoHooks),
            Err(ArchiveError::NotAnArchive)
        ));
        let rar = dir.file("mod.zip", b"Rar!\x1A\x07\x01\x00more");
        assert_eq!(detect(&rar).expect("rar"), Format::Rar);
    }

    #[test]
    fn a_missing_archive_is_an_io_error() {
        let dir = TempDir::new("missing");
        let err = extract_vpks(
            &dir.path().join("nope.zip"),
            &dir.path().join("out"),
            &Limits::default(),
            &NoHooks,
        )
        .expect_err("missing");
        assert!(matches!(&err, ArchiveError::Io(e) if e.kind() == io::ErrorKind::NotFound));
    }

    #[test]
    fn existing_output_names_are_refused_before_unpacking() {
        let dir = TempDir::new("taken");
        let archive = dir.file(
            "m.zip",
            &zip_bytes(&[("a.vpk", b"new"), ("b.vpk", b"new")], true),
        );
        let out = dir.path().join("out");
        std::fs::create_dir_all(&out).expect("dir");
        std::fs::write(out.join("1.vpk"), b"old").expect("write");
        let err = extract_vpks(&archive, &out, &Limits::default(), &NoHooks).expect_err("taken");
        assert!(matches!(&err, ArchiveError::Io(e) if e.kind() == io::ErrorKind::AlreadyExists));
        assert_eq!(files_under(&out), ["1.vpk"]);
        assert_eq!(std::fs::read(out.join("1.vpk")).expect("read"), b"old");
    }

    /// Asks to stop once `after` chunks were asked about.
    struct StopAfter {
        after: usize,
        asked: std::sync::atomic::AtomicUsize,
    }

    impl Hooks for StopAfter {
        fn cancelled(&self) -> bool {
            self.asked.fetch_add(1, std::sync::atomic::Ordering::SeqCst) >= self.after
        }
    }

    #[test]
    fn a_cancel_stops_unpacking_between_chunks_and_leaves_nothing() {
        let dir = TempDir::new("cancel");
        // Noise: it doesn't compress, so no limit stops it first.
        let mut seed = 0x2545_f491_u32;
        let big: Vec<u8> = (0..300 * 1024)
            .map(|_| {
                seed ^= seed << 13;
                seed ^= seed >> 17;
                seed ^= seed << 5;
                seed.to_le_bytes()[0]
            })
            .collect();
        let entries: [(&str, &[u8]); 2] = [("a.vpk", &big), ("b.vpk", &big)];
        let archives = [
            dir.file("m.zip", &zip_bytes(&entries, false)),
            dir.file("m.7z", &test_support::seven_z_bytes(&entries, true)),
        ];
        for archive in archives {
            let out = dir.path().join("out");
            let hooks = StopAfter {
                after: 3,
                asked: std::sync::atomic::AtomicUsize::new(0),
            };
            let err =
                extract_vpks(&archive, &out, &Limits::default(), &hooks).expect_err("cancelled");
            assert!(matches!(err, ArchiveError::Cancelled), "{err:?}");
            assert!(files_under(&out).is_empty(), "{archive:?}");
            assert_eq!(
                hooks.asked.load(std::sync::atomic::Ordering::SeqCst),
                4,
                "stopped at once"
            );
        }
    }

    #[test]
    fn an_archive_without_vpks_extracts_nothing_and_lists_what_it_skipped() {
        let dir = TempDir::new("no-vpk");
        let archive = dir.file(
            "m.zip",
            &zip_bytes(&[("readme.txt", b"hi"), ("img/", b"")], true),
        );
        let out = dir.path().join("out");
        let x = extract_vpks(&archive, &out, &Limits::default(), &NoHooks).expect("ok");
        assert!(x.vpks.is_empty());
        assert_eq!(x.skipped, ["readme.txt"]);
        assert!(files_under(&out).is_empty());
    }
}
