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
}

#[cfg(test)]
pub mod fake {
    //! An in-memory filesystem with volumes (to provoke cross-volume links), filesystems without hard links and
    //! write-protected folders. Paths are compared like on Windows: case-insensitively.

    use super::*;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;

    #[derive(Debug, Default)]
    pub struct MemFs {
        /// Volume prefix (e.g. `D:`) and whether it supports hard links.
        volumes: Vec<(String, bool)>,
        files: RefCell<HashMap<String, Rc<Vec<u8>>>>,
        denied: Vec<String>,
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
            self.files
                .borrow_mut()
                .remove(&key(path))
                .map(|_| ())
                .ok_or_else(|| io::Error::from(io::ErrorKind::NotFound))
        }
    }
}
