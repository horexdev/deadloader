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
