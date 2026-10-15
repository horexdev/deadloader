//! The file access the core needs, as traits, so logic can be tested with in-memory fakes.

use sha2::{Digest, Sha256};
use std::io::{self, Read, Write};
use std::path::Path;

pub trait ReadText {
    /// The whole file as UTF-8 text.
    fn read_text(&self, path: &Path) -> io::Result<String>;
}

/// File operations used to change the game folder.
pub trait FileOps {
    fn hard_link(&self, original: &Path, link: &Path) -> io::Result<()>;
    /// Copies `from` to the new file `to`, flushes it to disk and returns its size and SHA-256.
    fn copy_verified(&self, from: &Path, to: &Path) -> io::Result<(u64, [u8; 32])>;
    fn remove_file(&self, path: &Path) -> io::Result<()>;
    /// Renames without replacing: fails with `AlreadyExists` if `to` exists.
    fn rename(&self, from: &Path, to: &Path) -> io::Result<()>;
    fn exists(&self, path: &Path) -> io::Result<bool>;
    /// The entries directly inside `dir`; `NotFound` if it doesn't exist.
    fn list_dir(&self, dir: &Path) -> io::Result<Vec<DirEntry>>;
    fn create_dir_all(&self, dir: &Path) -> io::Result<()>;
    /// Creates a new file (never overwrites) with `bytes` and flushes it to disk.
    fn write_new(&self, path: &Path, bytes: &[u8]) -> io::Result<()>;
    /// Atomically puts `from` in place of `to`, replacing it. For files we fully rewrite, like gameinfo.gi.
    fn replace(&self, from: &Path, to: &Path) -> io::Result<()>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirEntry {
    pub name: String,
    pub is_dir: bool,
    pub len: u64,
}

#[cfg(test)]
mod md5_tests {
    #[test]
    fn md5_of_known_inputs() {
        assert_eq!(super::md5_hex(b""), "d41d8cd98f00b204e9800998ecf8427e");
        assert_eq!(super::md5_hex(b"abc"), "900150983cd24fb0d6963f7d28e17f72");
    }

    #[test]
    fn md5_of_a_file_is_streamed() {
        let dir = crate::testing::temp_dir("fs", "md5");
        let path = dir.join("x.bin");
        std::fs::write(&path, vec![7u8; 3 * 65_536 + 5]).expect("write");
        assert_eq!(
            super::md5_file(&path).expect("read"),
            super::md5_hex(&vec![7u8; 3 * 65_536 + 5])
        );
        crate::testing::remove_all(&dir);
    }
}

/// Lowercase hex md5, as GameBanana lists it.
pub fn md5_hex(bytes: &[u8]) -> String {
    lower_hex(&md5::Md5::digest(bytes))
}

/// [`md5_hex`] of a file, read in chunks.
pub fn md5_file(path: &Path) -> io::Result<String> {
    let mut hasher = md5::Md5::new();
    let mut file = std::fs::File::open(path)?;
    let mut buf = vec![0u8; 64 * 1024];
    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(lower_hex(&hasher.finalize()))
}

fn lower_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

pub fn sha256(bytes: &[u8]) -> [u8; 32] {
    Sha256::digest(bytes).into()
}

/// The real filesystem.
#[derive(Debug, Default, Clone, Copy)]
pub struct Disk;

impl ReadText for Disk {
    fn read_text(&self, path: &Path) -> io::Result<String> {
        std::fs::read_to_string(path)
    }
}

impl FileOps for Disk {
    fn hard_link(&self, original: &Path, link: &Path) -> io::Result<()> {
        std::fs::hard_link(original, link)
    }

    fn copy_verified(&self, from: &Path, to: &Path) -> io::Result<(u64, [u8; 32])> {
        let mut src = std::fs::File::open(from)?;
        let mut dst = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(to)?;
        let mut hasher = Sha256::new();
        let mut buf = vec![0u8; 1 << 20];
        let mut total = 0u64;
        loop {
            let n = src.read(&mut buf)?;
            if n == 0 {
                break;
            }
            hasher.update(&buf[..n]);
            dst.write_all(&buf[..n])?;
            total += n as u64;
        }
        dst.sync_all()?;
        Ok((total, hasher.finalize().into()))
    }

    fn remove_file(&self, path: &Path) -> io::Result<()> {
        std::fs::remove_file(path)
    }

    fn rename(&self, from: &Path, to: &Path) -> io::Result<()> {
        // std::fs::rename replaces an existing target on Windows; we never want that.
        if to.try_exists()? {
            return Err(io::Error::from(io::ErrorKind::AlreadyExists));
        }
        std::fs::rename(from, to)
    }

    fn exists(&self, path: &Path) -> io::Result<bool> {
        path.try_exists()
    }

    fn list_dir(&self, dir: &Path) -> io::Result<Vec<DirEntry>> {
        let mut out = Vec::new();
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let meta = entry.metadata()?;
            out.push(DirEntry {
                name: entry.file_name().to_string_lossy().into_owned(),
                is_dir: meta.is_dir(),
                len: meta.len(),
            });
        }
        Ok(out)
    }

    fn create_dir_all(&self, dir: &Path) -> io::Result<()> {
        std::fs::create_dir_all(dir)
    }

    fn write_new(&self, path: &Path, bytes: &[u8]) -> io::Result<()> {
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)?;
        f.write_all(bytes)?;
        f.sync_all()
    }

    fn replace(&self, from: &Path, to: &Path) -> io::Result<()> {
        // On Windows this is MoveFileEx with MOVEFILE_REPLACE_EXISTING: the target is swapped in one step.
        std::fs::rename(from, to)
    }
}

/// File operations that report each change to the files log (ADR 0009) and then return its result unchanged.
/// Reads are not logged.
pub struct Logged<'a, F, L: ?Sized> {
    pub fs: &'a F,
    pub log: &'a L,
}

impl<F: std::fmt::Debug, L: ?Sized> std::fmt::Debug for Logged<'_, F, L> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Logged")
            .field("fs", self.fs)
            .finish_non_exhaustive()
    }
}

impl<F, L: crate::log::FileLog + ?Sized> Logged<'_, F, L> {
    fn report<T>(
        &self,
        event: crate::log::FileEvent,
        result: io::Result<T>,
        detail: impl FnOnce(&T, &mut crate::log::FileEvent),
    ) -> io::Result<T> {
        use crate::log::Outcome;
        let mut event = event;
        match &result {
            Ok(value) => detail(value, &mut event),
            Err(e) => {
                event.result = Outcome::Failed;
                event.error = Some(e.to_string());
            }
        }
        let msg = match event.result {
            Outcome::Ok => "file operation",
            Outcome::Failed => "file operation failed",
        };
        self.log.record(msg, &event);
        result
    }
}

fn event(op: crate::log::FileOp, src: Option<&Path>, dst: Option<&Path>) -> crate::log::FileEvent {
    crate::log::FileEvent {
        src: src.map(|p| p.display().to_string()),
        dst: dst.map(|p| p.display().to_string()),
        ..crate::log::FileEvent::new(op)
    }
}

impl<F: FileOps, L: crate::log::FileLog + ?Sized> FileOps for Logged<'_, F, L> {
    fn hard_link(&self, original: &Path, link: &Path) -> io::Result<()> {
        use crate::log::FileOp;
        let e = event(FileOp::Hardlink, Some(original), Some(link));
        self.report(e, self.fs.hard_link(original, link), |_, _| {})
    }

    fn copy_verified(&self, from: &Path, to: &Path) -> io::Result<(u64, [u8; 32])> {
        use crate::log::FileOp;
        let e = event(FileOp::Copy, Some(from), Some(to));
        self.report(e, self.fs.copy_verified(from, to), |(len, hash), e| {
            e.bytes = Some(*len);
            e.sha256 = Some(*hash);
        })
    }

    fn remove_file(&self, path: &Path) -> io::Result<()> {
        use crate::log::FileOp;
        let e = event(FileOp::Delete, Some(path), None);
        self.report(e, self.fs.remove_file(path), |_, _| {})
    }

    fn rename(&self, from: &Path, to: &Path) -> io::Result<()> {
        use crate::log::FileOp;
        let e = event(FileOp::Rename, Some(from), Some(to));
        self.report(e, self.fs.rename(from, to), |_, _| {})
    }

    fn exists(&self, path: &Path) -> io::Result<bool> {
        self.fs.exists(path)
    }

    fn list_dir(&self, dir: &Path) -> io::Result<Vec<DirEntry>> {
        self.fs.list_dir(dir)
    }

    fn create_dir_all(&self, dir: &Path) -> io::Result<()> {
        self.fs.create_dir_all(dir)
    }

    fn write_new(&self, path: &Path, bytes: &[u8]) -> io::Result<()> {
        use crate::log::FileOp;
        let mut e = event(FileOp::Write, None, Some(path));
        e.bytes = Some(bytes.len() as u64);
        self.report(e, self.fs.write_new(path, bytes), |_, _| {})
    }

    fn replace(&self, from: &Path, to: &Path) -> io::Result<()> {
        use crate::log::FileOp;
        let e = event(FileOp::Rename, Some(from), Some(to));
        self.report(e, self.fs.replace(from, to), |_, _| {})
    }
}

impl<F: ReadText, L: ?Sized> ReadText for Logged<'_, F, L> {
    fn read_text(&self, path: &Path) -> io::Result<String> {
        self.fs.read_text(path)
    }
}

/// Counts the hard links and copies made through it, so an apply can tell whether the game's files take extra disk
/// space (decision 124's copy fallback, the library's "copied" badge).
pub struct Counted<'a, F> {
    fs: &'a F,
    linked: std::cell::Cell<usize>,
    copied: std::cell::Cell<usize>,
}

impl<F> std::fmt::Debug for Counted<'_, F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Counted")
            .field("linked", &self.linked.get())
            .field("copied", &self.copied.get())
            .finish_non_exhaustive()
    }
}

impl<'a, F> Counted<'a, F> {
    pub fn new(fs: &'a F) -> Self {
        Self {
            fs,
            linked: std::cell::Cell::new(0),
            copied: std::cell::Cell::new(0),
        }
    }

    pub fn linked(&self) -> usize {
        self.linked.get()
    }

    pub fn copied(&self) -> usize {
        self.copied.get()
    }
}

impl<F: FileOps> FileOps for Counted<'_, F> {
    fn hard_link(&self, original: &Path, link: &Path) -> io::Result<()> {
        let result = self.fs.hard_link(original, link);
        if result.is_ok() {
            self.linked.set(self.linked.get() + 1);
        }
        result
    }

    fn copy_verified(&self, from: &Path, to: &Path) -> io::Result<(u64, [u8; 32])> {
        let result = self.fs.copy_verified(from, to);
        if result.is_ok() {
            self.copied.set(self.copied.get() + 1);
        }
        result
    }

    fn remove_file(&self, path: &Path) -> io::Result<()> {
        self.fs.remove_file(path)
    }

    fn rename(&self, from: &Path, to: &Path) -> io::Result<()> {
        self.fs.rename(from, to)
    }

    fn exists(&self, path: &Path) -> io::Result<bool> {
        self.fs.exists(path)
    }

    fn list_dir(&self, dir: &Path) -> io::Result<Vec<DirEntry>> {
        self.fs.list_dir(dir)
    }

    fn create_dir_all(&self, dir: &Path) -> io::Result<()> {
        self.fs.create_dir_all(dir)
    }

    fn write_new(&self, path: &Path, bytes: &[u8]) -> io::Result<()> {
        self.fs.write_new(path, bytes)
    }

    fn replace(&self, from: &Path, to: &Path) -> io::Result<()> {
        self.fs.replace(from, to)
    }
}

impl<F: ReadText> ReadText for Counted<'_, F> {
    fn read_text(&self, path: &Path) -> io::Result<String> {
        self.fs.read_text(path)
    }
}

/// Replaces `path` with `text` so a crash leaves either version: a flushed `<name>.tmp`, the current file copied to
/// `<name>.bak`, then a rename over it. For the JSON files users may read and edit (manifest, profiles, settings).
pub fn write_atomic(path: &Path, text: &str) -> io::Result<()> {
    let with_suffix = |suffix: &str| {
        let mut name = path.file_name().unwrap_or_default().to_os_string();
        name.push(suffix);
        path.with_file_name(name)
    };
    let tmp = with_suffix(".tmp");
    {
        let mut f = std::fs::File::create(&tmp)?;
        f.write_all(text.as_bytes())?;
        f.sync_all()?;
    }
    if path.exists() {
        std::fs::copy(path, with_suffix(".bak"))?;
    }
    std::fs::rename(&tmp, path)
}

#[cfg(test)]
mod counted_tests {
    use super::fake::MemFs;
    use super::*;
    use std::path::Path;

    #[test]
    fn links_and_copies_made_through_it_are_counted() {
        let fs = MemFs::default()
            .volume(r"C:\", true)
            .volume(r"E:\", false)
            .with(r"C:\lib\a.vpk", b"aaa")
            .with(r"C:\lib\b.vpk", b"bbb");
        let counted = Counted::new(&fs);
        crate::place::place(
            &counted,
            Path::new(r"C:\lib\a.vpk"),
            Path::new(r"C:\game\a.vpk"),
            &sha256(b"aaa"),
        )
        .expect("linked");
        crate::place::place(
            &counted,
            Path::new(r"C:\lib\b.vpk"),
            Path::new(r"E:\game\b.vpk"),
            &sha256(b"bbb"),
        )
        .expect("copied");
        assert_eq!((counted.linked(), counted.copied()), (1, 1));
    }
}

#[cfg(test)]
pub mod fake {
    //! An in-memory filesystem with volumes (to provoke cross-volume links), filesystems without hard links,
    //! write-protected folders and a simulated power cut. Paths are compared like on Windows: case-insensitively.

    use super::*;
    use std::cell::{Cell, RefCell};
    use std::collections::{BTreeMap, HashMap};
    use std::rc::Rc;

    /// Shared budget of operations before the "power" goes out; every later operation fails.
    #[derive(Debug, Clone, Default)]
    pub struct Power(Rc<Cell<Option<usize>>>);

    impl Power {
        pub fn cut_after(&self, operations: usize) {
            self.0.set(Some(operations));
        }

        pub fn restore(&self) {
            self.0.set(None);
        }

        /// Spends one operation, or fails if the power is out.
        pub fn spend(&self) -> io::Result<()> {
            match self.0.get() {
                Some(0) => Err(io::Error::other("power cut")),
                Some(n) => {
                    self.0.set(Some(n - 1));
                    Ok(())
                }
                None => Ok(()),
            }
        }
    }

    #[derive(Debug, Default)]
    pub struct MemFs {
        /// Volume prefix (e.g. `D:`) and whether it supports hard links.
        volumes: Vec<(String, bool)>,
        files: RefCell<HashMap<String, Rc<Vec<u8>>>>,
        denied: Vec<String>,
        power: Power,
    }

    fn key(path: impl AsRef<Path>) -> String {
        path.as_ref().to_string_lossy().to_ascii_lowercase()
    }

    impl MemFs {
        pub fn volume(mut self, prefix: &str, hard_links: bool) -> Self {
            self.volumes.push((prefix.to_ascii_lowercase(), hard_links));
            self
        }

        pub fn with(self, path: &str, bytes: &[u8]) -> Self {
            self.files
                .borrow_mut()
                .insert(key(path), Rc::new(bytes.to_vec()));
            self
        }

        pub fn deny_writes_under(mut self, prefix: &str) -> Self {
            self.denied.push(prefix.to_ascii_lowercase());
            self
        }

        pub fn power(&self) -> Power {
            self.power.clone()
        }

        /// Every file and its content, for comparing whole states.
        pub fn snapshot(&self) -> BTreeMap<String, Vec<u8>> {
            self.files
                .borrow()
                .iter()
                .map(|(k, v)| (k.clone(), v.to_vec()))
                .collect()
        }

        pub fn read(&self, path: &str) -> Option<Vec<u8>> {
            self.files.borrow().get(&key(path)).map(|b| b.to_vec())
        }

        /// Whether both names are hard links to the same data.
        pub fn same_file(&self, a: &str, b: &str) -> bool {
            let files = self.files.borrow();
            match (files.get(&key(a)), files.get(&key(b))) {
                (Some(x), Some(y)) => Rc::ptr_eq(x, y),
                _ => false,
            }
        }

        fn volume_of(&self, path: &Path) -> Option<&(String, bool)> {
            let k = key(path);
            self.volumes
                .iter()
                .find(|(prefix, _)| k.starts_with(prefix.as_str()))
        }

        fn check_writable(&self, path: &Path) -> io::Result<()> {
            let k = key(path);
            if self.denied.iter().any(|d| k.starts_with(d.as_str())) {
                return Err(io::Error::from(io::ErrorKind::PermissionDenied));
            }
            if self.files.borrow().contains_key(&k) {
                return Err(io::Error::from(io::ErrorKind::AlreadyExists));
            }
            Ok(())
        }

        fn source(&self, path: &Path) -> io::Result<Rc<Vec<u8>>> {
            self.files
                .borrow()
                .get(&key(path))
                .cloned()
                .ok_or_else(|| io::Error::from(io::ErrorKind::NotFound))
        }
    }

    impl FileOps for MemFs {
        fn hard_link(&self, original: &Path, link: &Path) -> io::Result<()> {
            self.power.spend()?;
            self.check_writable(link)?;
            let data = self.source(original)?;
            match (self.volume_of(original), self.volume_of(link)) {
                (Some(a), Some(b)) if a.0 != b.0 => {
                    return Err(io::Error::from(io::ErrorKind::CrossesDevices));
                }
                (Some((_, false)), _) => return Err(io::Error::from(io::ErrorKind::Unsupported)),
                _ => {}
            }
            self.files.borrow_mut().insert(key(link), data);
            Ok(())
        }

        fn copy_verified(&self, from: &Path, to: &Path) -> io::Result<(u64, [u8; 32])> {
            self.power.spend()?;
            self.check_writable(to)?;
            let data = self.source(from)?;
            let hash = sha256(&data);
            let len = data.len() as u64;
            self.files
                .borrow_mut()
                .insert(key(to), Rc::new(data.to_vec()));
            Ok((len, hash))
        }

        fn remove_file(&self, path: &Path) -> io::Result<()> {
            self.power.spend()?;
            self.files
                .borrow_mut()
                .remove(&key(path))
                .map(|_| ())
                .ok_or_else(|| io::Error::from(io::ErrorKind::NotFound))
        }

        fn rename(&self, from: &Path, to: &Path) -> io::Result<()> {
            self.power.spend()?;
            self.check_writable(to)?;
            let data = self
                .files
                .borrow_mut()
                .remove(&key(from))
                .ok_or_else(|| io::Error::from(io::ErrorKind::NotFound))?;
            self.files.borrow_mut().insert(key(to), data);
            Ok(())
        }

        fn exists(&self, path: &Path) -> io::Result<bool> {
            Ok(self.files.borrow().contains_key(&key(path)))
        }

        fn list_dir(&self, dir: &Path) -> io::Result<Vec<DirEntry>> {
            let prefix = format!("{}\\", key(dir).trim_end_matches('\\'));
            let mut entries: BTreeMap<String, DirEntry> = BTreeMap::new();
            for (k, data) in self.files.borrow().iter() {
                let Some(rest) = k.strip_prefix(&prefix) else {
                    continue;
                };
                match rest.split_once('\\') {
                    Some((sub, _)) => {
                        entries.insert(
                            sub.to_owned(),
                            DirEntry {
                                name: sub.to_owned(),
                                is_dir: true,
                                len: 0,
                            },
                        );
                    }
                    None => {
                        entries.insert(
                            rest.to_owned(),
                            DirEntry {
                                name: rest.to_owned(),
                                is_dir: false,
                                len: data.len() as u64,
                            },
                        );
                    }
                }
            }
            if entries.is_empty() {
                return Err(io::Error::from(io::ErrorKind::NotFound));
            }
            Ok(entries.into_values().collect())
        }

        fn create_dir_all(&self, _dir: &Path) -> io::Result<()> {
            // Folders are implicit in this fake: a file's path is enough.
            Ok(())
        }

        fn write_new(&self, path: &Path, bytes: &[u8]) -> io::Result<()> {
            self.power.spend()?;
            self.check_writable(path)?;
            self.files
                .borrow_mut()
                .insert(key(path), Rc::new(bytes.to_vec()));
            Ok(())
        }

        fn replace(&self, from: &Path, to: &Path) -> io::Result<()> {
            self.power.spend()?;
            let k = key(to);
            if self.denied.iter().any(|d| k.starts_with(d.as_str())) {
                return Err(io::Error::from(io::ErrorKind::PermissionDenied));
            }
            let data = self
                .files
                .borrow_mut()
                .remove(&key(from))
                .ok_or_else(|| io::Error::from(io::ErrorKind::NotFound))?;
            self.files.borrow_mut().insert(k, data);
            Ok(())
        }
    }

    impl ReadText for MemFs {
        fn read_text(&self, path: &Path) -> io::Result<String> {
            let data = self.source(path)?;
            String::from_utf8(data.to_vec())
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_dir(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("deadloader-fs-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        dir
    }

    #[test]
    fn logged_operations_report_what_they_did() {
        use crate::log::{FileOp, MemFileLog, Outcome};
        let mem = fake::MemFs::default()
            .volume("D:", true)
            .with(r"D:\lib\a.vpk", b"abc");
        let log = MemFileLog::default();
        let fs = Logged {
            fs: &mem,
            log: &log,
        };
        fs.hard_link(
            Path::new(r"D:\lib\a.vpk"),
            Path::new(r"D:\game\pak01_dir.vpk"),
        )
        .expect("link");
        fs.copy_verified(Path::new(r"D:\lib\a.vpk"), Path::new(r"D:\game\copy.vpk"))
            .expect("copy");
        fs.rename(
            Path::new(r"D:\game\copy.vpk"),
            Path::new(r"D:\game\pak02_dir.vpk"),
        )
        .expect("rename");
        fs.write_new(Path::new(r"D:\game\gameinfo.tmp"), b"xyz")
            .expect("write");
        fs.replace(
            Path::new(r"D:\game\gameinfo.tmp"),
            Path::new(r"D:\game\gameinfo.gi"),
        )
        .expect("replace");
        fs.remove_file(Path::new(r"D:\game\pak02_dir.vpk"))
            .expect("delete");
        let events = log.events();
        let ops: Vec<FileOp> = events.iter().map(|(_, e)| e.op).collect();
        assert_eq!(
            ops,
            [
                FileOp::Hardlink,
                FileOp::Copy,
                FileOp::Rename,
                FileOp::Write,
                FileOp::Rename,
                FileOp::Delete
            ]
        );
        let copy = &events[1].1;
        assert_eq!(copy.src.as_deref(), Some(r"D:\lib\a.vpk"));
        assert_eq!(copy.dst.as_deref(), Some(r"D:\game\copy.vpk"));
        assert_eq!((copy.bytes, copy.sha256), (Some(3), Some(sha256(b"abc"))));
        assert_eq!(events[3].1.bytes, Some(3));
        assert!(events.iter().all(|(_, e)| e.result == Outcome::Ok));
    }

    #[test]
    fn a_failed_operation_is_logged_with_its_error_and_still_fails() {
        use crate::log::{FileOp, MemFileLog, Outcome};
        let mem = fake::MemFs::default();
        let log = MemFileLog::default();
        let fs = Logged {
            fs: &mem,
            log: &log,
        };
        let err = fs
            .remove_file(Path::new(r"D:\game\missing.vpk"))
            .expect_err("missing");
        assert_eq!(err.kind(), io::ErrorKind::NotFound);
        let events = log.events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].1.op, FileOp::Delete);
        assert_eq!(events[0].1.result, Outcome::Failed);
        assert!(events[0].1.error.is_some());
    }

    #[test]
    fn reads_are_not_logged() {
        use crate::log::MemFileLog;
        let mem = fake::MemFs::default().with(r"D:\game\x.vpk", b"1");
        let log = MemFileLog::default();
        let fs = Logged {
            fs: &mem,
            log: &log,
        };
        assert!(fs.exists(Path::new(r"D:\game\x.vpk")).expect("exists"));
        fs.list_dir(Path::new(r"D:\game")).expect("list");
        assert!(log.events().is_empty());
    }

    #[test]
    fn an_atomic_write_keeps_the_previous_version_as_bak() {
        let dir = temp_dir("atomic");
        std::fs::create_dir_all(&dir).expect("dir");
        let path = dir.join("manifest.json");
        write_atomic(&path, "one").expect("first");
        assert!(
            !dir.join("manifest.json.bak").exists(),
            "nothing to back up yet"
        );
        write_atomic(&path, "two").expect("second");
        assert_eq!(std::fs::read_to_string(&path).expect("read"), "two");
        assert_eq!(
            std::fs::read_to_string(dir.join("manifest.json.bak")).expect("bak"),
            "one"
        );
        assert!(!dir.join("manifest.json.tmp").exists());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn creating_folders_is_idempotent() {
        let root = temp_dir("mkdir");
        let dir = root.join("a").join("b");
        Disk.create_dir_all(&dir).expect("create");
        Disk.create_dir_all(&dir).expect("again");
        assert!(dir.is_dir());
        std::fs::remove_dir_all(&root).expect("cleanup");
    }

    #[test]
    fn writing_a_new_file_never_overwrites() {
        let dir = temp_dir("write");
        Disk.create_dir_all(&dir).expect("dir");
        let file = dir.join("x.tmp");
        Disk.write_new(&file, b"first").expect("write");
        let err = Disk.write_new(&file, b"second").expect_err("exists");
        assert_eq!(err.kind(), io::ErrorKind::AlreadyExists);
        assert_eq!(std::fs::read(&file).expect("read"), b"first");
        std::fs::remove_dir_all(&dir).expect("cleanup");
    }

    #[test]
    fn replacing_swaps_in_the_new_content() {
        let dir = temp_dir("replace");
        Disk.create_dir_all(&dir).expect("dir");
        let target = dir.join("gameinfo.gi");
        let staged = dir.join("gameinfo.gi.tmp");
        std::fs::write(&target, b"old").expect("old");
        Disk.write_new(&staged, b"new").expect("staged");
        Disk.replace(&staged, &target).expect("replace");
        assert_eq!(std::fs::read(&target).expect("read"), b"new");
        assert!(!staged.exists());
        std::fs::remove_dir_all(&dir).expect("cleanup");
    }
}
