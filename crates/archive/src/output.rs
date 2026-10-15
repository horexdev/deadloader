//! Writing extracted VPKs: temp names first, our own final names, all-or-nothing.
//!
//! Every output file is named `<n>.vpk` by its position among the archive's VPKs; the path stored in the archive
//! is kept for display only and never becomes part of a filesystem path, so a hostile name (`../`, `C:\`, UNC,
//! `CON`, `name:stream`) cannot steer a write anywhere (ADR 0008). Data goes to `<n>.vpk.part`; only when every
//! VPK has been written are the temp files renamed. Any failure removes everything this extraction created.

use crate::error::ArchiveError;
use crate::limits::{Meter, Unit};
use sha2::{Digest, Sha256};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

/// What the caller supplies besides limits: free-space information and a file-operation log.
pub trait Hooks {
    /// Free bytes on the volume holding `dir`; `None` when unknown, which skips the check.
    fn available_bytes(&self, dir: &Path) -> Option<u64> {
        let _ = dir;
        None
    }

    /// Called for every file created, renamed or removed, so the caller can log it.
    fn file_event(&self, event: &FileEvent<'_>) {
        let _ = event;
    }

    /// Asked before every chunk is unpacked: `true` stops the extraction with [`ArchiveError::Cancelled`] and
    /// removes what it wrote. Must be quick.
    fn cancelled(&self) -> bool {
        false
    }
}

/// Free space unknown, nothing logged.
#[derive(Debug, Default, Clone, Copy)]
pub struct NoHooks;

impl Hooks for NoHooks {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileEvent<'a> {
    Created(&'a Path),
    Renamed { from: &'a Path, to: &'a Path },
    Removed(&'a Path),
}

/// One VPK taken out of an archive.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtractedVpk {
    /// The path stored in the archive: shown to the user, never used to build a path.
    pub archive_path: String,
    /// `<output dir>\<n>.vpk`.
    pub output: PathBuf,
    pub size: u64,
    pub sha256: [u8; 32],
}

pub(crate) fn output_name(n: usize) -> String {
    format!("{n}.vpk")
}

/// Fails if any of the first `count` output names is already taken, before anything is unpacked.
pub(crate) fn ensure_free_names(dir: &Path, count: usize) -> Result<(), ArchiveError> {
    for n in 0..count {
        let path = dir.join(output_name(n));
        if path.try_exists().map_err(ArchiveError::Io)? {
            return Err(ArchiveError::Io(io::Error::new(
                io::ErrorKind::AlreadyExists,
                format!("{} already exists", path.display()),
            )));
        }
    }
    Ok(())
}

/// Why streaming one entry stopped.
#[derive(Debug)]
pub(crate) enum PumpError {
    /// Decompression failed: the archive's fault.
    Read(io::Error),
    /// Writing failed: the disk's fault.
    Write(io::Error),
    Limit(crate::error::LimitExceeded),
    /// The caller cancelled ([`Hooks::cancelled`]).
    Cancelled,
}

const CHUNK: usize = 64 * 1024;

/// Copies `reader` into `sink` chunk by chunk, asking the meter before every write.
pub(crate) fn pump(
    reader: &mut dyn Read,
    meter: &mut Meter,
    unit: &mut Unit,
    entry: &str,
    buf: &mut [u8],
    mut sink: impl FnMut(&[u8]) -> io::Result<()>,
) -> Result<u64, PumpError> {
    let mut copied = 0u64;
    loop {
        if meter.cancelled() {
            return Err(PumpError::Cancelled);
        }
        let n = match reader.read(buf) {
            Ok(0) => return Ok(copied),
            Ok(n) => n,
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(PumpError::Read(e)),
        };
        meter
            .admit(unit, n as u64, entry)
            .map_err(PumpError::Limit)?;
        sink(&buf[..n]).map_err(PumpError::Write)?;
        copied += n as u64;
    }
}

/// Decodes and discards an entry that must be read past (solid 7z blocks); it still counts against limits.
pub(crate) fn drain(
    reader: &mut dyn Read,
    meter: &mut Meter,
    unit: &mut Unit,
    entry: &str,
    buf: &mut [u8],
) -> Result<(), PumpError> {
    pump(reader, meter, unit, entry, buf, |_| Ok(())).map(|_| ())
}

pub(crate) fn is_disk_full(e: &io::Error) -> bool {
    // ERROR_HANDLE_DISK_FULL, ERROR_DISK_FULL
    e.kind() == io::ErrorKind::StorageFull
        || (cfg!(windows) && matches!(e.raw_os_error(), Some(39 | 112)))
}

#[derive(Debug)]
struct Staged {
    archive_path: String,
    temp: PathBuf,
    output: PathBuf,
    size: u64,
    sha256: [u8; 32],
    renamed: bool,
}

/// The files of one extraction. Dropping it without [`Staging::commit`] removes all of them.
pub(crate) struct Staging<'h> {
    dir: PathBuf,
    hooks: &'h dyn Hooks,
    staged: Vec<Staged>,
    buf: Vec<u8>,
    committed: bool,
}

impl std::fmt::Debug for Staging<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Staging")
            .field("dir", &self.dir)
            .field("staged", &self.staged)
            .finish_non_exhaustive()
    }
}

impl<'h> Staging<'h> {
    pub(crate) fn new(dir: &Path, hooks: &'h dyn Hooks) -> Self {
        Self {
            dir: dir.to_owned(),
            hooks,
            staged: Vec::new(),
            buf: vec![0; CHUNK],
            committed: false,
        }
    }

    /// Streams one VPK into `<n>.vpk.part`, where `n` is the number of VPKs written so far.
    pub(crate) fn write(
        &mut self,
        archive_path: &str,
        reader: &mut dyn Read,
        meter: &mut Meter,
        unit: &mut Unit,
    ) -> Result<(), ArchiveError> {
        let name = output_name(self.staged.len());
        let output = self.dir.join(&name);
        let temp = self.dir.join(format!("{name}.part"));
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temp)
            .map_err(write_error)?;
        self.hooks.file_event(&FileEvent::Created(&temp));
        self.staged.push(Staged {
            archive_path: archive_path.to_owned(),
            temp,
            output,
            size: 0,
            sha256: [0; 32],
            renamed: false,
        });

        let mut hasher = Sha256::new();
        let size = pump(reader, meter, unit, archive_path, &mut self.buf, |chunk| {
            hasher.update(chunk);
            file.write_all(chunk)
        })
        .map_err(|e| match e {
            PumpError::Read(e) => ArchiveError::Damaged {
                entry: Some(archive_path.to_owned()),
                detail: e.to_string(),
            },
            PumpError::Write(e) => write_error(e),
            PumpError::Limit(l) => ArchiveError::Limit(l),
            PumpError::Cancelled => ArchiveError::Cancelled,
        })?;
        finish(file).map_err(write_error)?;

        if let Some(last) = self.staged.last_mut() {
            last.size = size;
            last.sha256 = hasher.finalize().into();
        }
        Ok(())
    }

    /// Gives every temp file its final name; on any failure nothing of this extraction remains.
    pub(crate) fn commit(mut self) -> Result<Vec<ExtractedVpk>, ArchiveError> {
        for i in 0..self.staged.len() {
            let s = &self.staged[i];
            if s.output.try_exists().map_err(ArchiveError::Io)? {
                return Err(ArchiveError::Io(io::Error::new(
                    io::ErrorKind::AlreadyExists,
                    format!("{} already exists", s.output.display()),
                )));
            }
            std::fs::rename(&s.temp, &s.output).map_err(ArchiveError::Io)?;
            self.hooks.file_event(&FileEvent::Renamed {
                from: &s.temp,
                to: &s.output,
            });
            self.staged[i].renamed = true;
        }
        self.committed = true;
        Ok(std::mem::take(&mut self.staged)
            .into_iter()
            .map(|s| ExtractedVpk {
                archive_path: s.archive_path,
                output: s.output,
                size: s.size,
                sha256: s.sha256,
            })
            .collect())
    }
}

impl Drop for Staging<'_> {
    fn drop(&mut self) {
        if self.committed {
            return;
        }
        for s in &self.staged {
            let path = if s.renamed { &s.output } else { &s.temp };
            // Best effort: a leftover `.part` is removed by the intent journal on the next start.
            if std::fs::remove_file(path).is_ok() {
                self.hooks.file_event(&FileEvent::Removed(path));
            }
        }
    }
}

fn finish(file: File) -> io::Result<()> {
    file.sync_all()
}

fn write_error(e: io::Error) -> ArchiveError {
    if is_disk_full(&e) {
        ArchiveError::DiskFull
    } else {
        ArchiveError::Io(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::LimitExceeded;
    use crate::limits::Limits;
    use crate::test_support::{RecordingHooks, TempDir, files_under};

    /// An endless stream of zeros: what a bomb's decompressor looks like from the outside.
    struct Zeros;

    impl Read for Zeros {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            buf.fill(0);
            Ok(buf.len())
        }
    }

    struct Failing;

    impl Read for Failing {
        fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "invalid checksum",
            ))
        }
    }

    fn meter(total: u64) -> Meter<'static> {
        let limits = Limits {
            max_total_bytes: total,
            max_total_factor: u64::MAX,
            max_entry_ratio: u64::MAX,
            ..Limits::default()
        };
        Meter::new(&limits, 1)
    }

    #[test]
    fn written_files_get_our_names_sizes_and_hashes() {
        let dir = TempDir::new("stage-ok");
        let hooks = RecordingHooks::default();
        let mut staging = Staging::new(dir.path(), &hooks);
        let mut m = meter(1 << 20);
        for data in [&b"first"[..], b"second"] {
            staging
                .write("x/../evil.vpk", &mut &data[..], &mut m, &mut Unit::new(100))
                .expect("written");
        }
        let out = staging.commit().expect("committed");

        assert_eq!(files_under(dir.path()), ["0.vpk", "1.vpk"]);
        assert_eq!(out[0].output, dir.path().join("0.vpk"));
        assert_eq!(out[0].archive_path, "x/../evil.vpk");
        assert_eq!(out[1].size, 6);
        assert_eq!(out[1].sha256, <[u8; 32]>::from(Sha256::digest(b"second")));
        assert_eq!(
            std::fs::read(dir.path().join("1.vpk")).expect("read"),
            b"second"
        );
        assert_eq!(
            hooks.events(),
            [
                format!("created {}", dir.path().join("0.vpk.part").display()),
                format!("created {}", dir.path().join("1.vpk.part").display()),
                format!(
                    "renamed {} -> {}",
                    dir.path().join("0.vpk.part").display(),
                    dir.path().join("0.vpk").display()
                ),
                format!(
                    "renamed {} -> {}",
                    dir.path().join("1.vpk.part").display(),
                    dir.path().join("1.vpk").display()
                ),
            ]
        );
    }

    #[test]
    fn an_endless_stream_stops_at_the_limit_and_leaves_nothing() {
        let dir = TempDir::new("stage-bomb");
        let hooks = RecordingHooks::default();
        let mut staging = Staging::new(dir.path(), &hooks);
        let mut m = meter(1_000_000);
        staging
            .write("ok.vpk", &mut &b"fine"[..], &mut m, &mut Unit::new(4))
            .expect("first");
        let err = staging
            .write("bomb.vpk", &mut Zeros, &mut m, &mut Unit::new(1))
            .expect_err("bomb");
        match err {
            ArchiveError::Limit(LimitExceeded::TotalSize { limit, entry }) => {
                assert_eq!((limit, entry.as_str()), (1_000_000, "bomb.vpk"));
            }
            other => panic!("{other:?}"),
        }
        let written = std::fs::metadata(dir.path().join("1.vpk.part"))
            .expect("still there until dropped")
            .len();
        assert!(written <= 1_000_000 - 4, "wrote {written}");
        drop(staging);
        assert!(files_under(dir.path()).is_empty());
        assert!(
            hooks
                .events()
                .iter()
                .any(|e| e.starts_with("removed") && e.ends_with("1.vpk.part"))
        );
    }

    #[test]
    fn a_read_error_means_the_archive_is_damaged() {
        let dir = TempDir::new("stage-damaged");
        let mut staging = Staging::new(dir.path(), &NoHooks);
        let err = staging
            .write("a.vpk", &mut Failing, &mut meter(100), &mut Unit::new(1))
            .expect_err("damaged");
        match err {
            ArchiveError::Damaged { entry, detail } => {
                assert_eq!(entry.as_deref(), Some("a.vpk"));
                assert!(detail.contains("invalid checksum"), "{detail}");
            }
            other => panic!("{other:?}"),
        }
        drop(staging);
        assert!(files_under(dir.path()).is_empty());
    }

    #[test]
    fn commit_refuses_to_replace_an_existing_file_and_cleans_up() {
        let dir = TempDir::new("stage-exists");
        let mut staging = Staging::new(dir.path(), &NoHooks);
        let mut m = meter(100);
        for _ in 0..2 {
            staging
                .write("a.vpk", &mut &b"new"[..], &mut m, &mut Unit::new(3))
                .expect("written");
        }
        std::fs::write(dir.path().join("1.vpk"), b"someone else's").expect("write");
        let err = staging.commit().expect_err("exists");
        assert!(matches!(&err, ArchiveError::Io(e) if e.kind() == io::ErrorKind::AlreadyExists));
        assert_eq!(
            files_under(dir.path()),
            ["1.vpk"],
            "only the foreign file stays"
        );
        assert_eq!(
            std::fs::read(dir.path().join("1.vpk")).expect("read"),
            b"someone else's"
        );
    }

    #[test]
    fn taken_output_names_are_found_before_unpacking() {
        let dir = TempDir::new("stage-names");
        ensure_free_names(dir.path(), 3).expect("empty dir");
        std::fs::write(dir.path().join("2.vpk"), b"x").expect("write");
        ensure_free_names(dir.path(), 2).expect("only 0 and 1 matter");
        assert!(ensure_free_names(dir.path(), 3).is_err());
    }

    #[test]
    fn disk_full_is_recognised() {
        assert!(is_disk_full(&io::Error::from(io::ErrorKind::StorageFull)));
        assert!(!is_disk_full(&io::Error::from(
            io::ErrorKind::PermissionDenied
        )));
        if cfg!(windows) {
            assert!(is_disk_full(&io::Error::from_raw_os_error(112)));
            assert!(is_disk_full(&io::Error::from_raw_os_error(39)));
        }
        assert!(matches!(
            write_error(io::Error::from(io::ErrorKind::StorageFull)),
            ArchiveError::DiskFull
        ));
    }

    #[test]
    fn draining_counts_against_the_limit_but_writes_nothing() {
        let mut m = meter(10_000);
        let mut buf = vec![0; 4096];
        let err = drain(
            &mut Zeros,
            &mut m,
            &mut Unit::new(1),
            "skipped.dds",
            &mut buf,
        )
        .expect_err("bomb in a skipped file");
        assert!(matches!(
            err,
            PumpError::Limit(LimitExceeded::TotalSize { .. })
        ));
        assert!(m.total() <= 10_000);
    }
}
