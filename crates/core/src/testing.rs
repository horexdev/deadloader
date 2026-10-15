//! Helpers shared by tests: minimal VPK files and temporary folders.

use std::path::{Path, PathBuf};

/// A minimal VPK v2 directory with the given files (path, CRC32) and no data.
pub fn vpk(files: &[(&str, u32)]) -> Vec<u8> {
    let mut tree = Vec::new();
    for (path, crc) in files {
        let (dir, rest) = path.rsplit_once('/').unwrap_or((" ", path));
        let (name, ext) = rest.rsplit_once('.').unwrap_or((rest, " "));
        for part in [ext, dir, name] {
            tree.extend_from_slice(part.as_bytes());
            tree.push(0);
        }
        tree.extend_from_slice(&crc.to_le_bytes());
        tree.extend_from_slice(&0u16.to_le_bytes());
        tree.extend_from_slice(&0x7FFFu16.to_le_bytes());
        tree.extend_from_slice(&0u32.to_le_bytes());
        tree.extend_from_slice(&0u32.to_le_bytes());
        tree.extend_from_slice(&0xFFFFu16.to_le_bytes());
        tree.extend_from_slice(&[0, 0]); // end of this extension's names and directories
    }
    tree.push(0); // end of extensions
    let mut out = Vec::new();
    out.extend_from_slice(&crate::vpk::SIGNATURE.to_le_bytes());
    out.extend_from_slice(&2u32.to_le_bytes());
    out.extend_from_slice(&(tree.len() as u32).to_le_bytes());
    out.extend_from_slice(&[0u8; 16]);
    out.extend_from_slice(&tree);
    out
}

/// A fresh, empty folder under the system temp folder, unique to this process.
pub fn temp_dir(prefix: &str, name: &str) -> PathBuf {
    let dir =
        std::env::temp_dir().join(format!("deadloader-{prefix}-{name}-{}", std::process::id()));
    remove_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp dir");
    dir
}

/// Removes a test folder, clearing the read-only attribute library files carry.
pub fn remove_all(dir: &Path) {
    fn walk(p: &Path) {
        if let Ok(entries) = std::fs::read_dir(p) {
            for e in entries.flatten() {
                let path = e.path();
                if path.is_dir() {
                    walk(&path);
                } else if let Ok(meta) = std::fs::metadata(&path) {
                    let mut perm = meta.permissions();
                    #[allow(clippy::permissions_set_readonly_false)]
                    perm.set_readonly(false);
                    let _ = std::fs::set_permissions(&path, perm);
                }
            }
        }
    }
    walk(dir);
    let _ = std::fs::remove_dir_all(dir);
}

/// A logs folder in memory that counts how many bytes were read, so tests can tell a part was read, not the whole.
#[derive(Debug, Default)]
pub struct MemLogDir {
    pub files: std::sync::Mutex<std::collections::BTreeMap<String, Vec<u8>>>,
    pub bytes_read: std::sync::atomic::AtomicU64,
}

impl MemLogDir {
    /// Adds `text` to the end of `name` as it is (lines carry their own newlines).
    pub fn push(&self, name: &str, text: &str) {
        self.files
            .lock()
            .expect("files")
            .entry(name.to_owned())
            .or_default()
            .extend_from_slice(text.as_bytes());
    }

    pub fn read_so_far(&self) -> u64 {
        self.bytes_read.load(std::sync::atomic::Ordering::Relaxed)
    }
}

impl crate::log::LogDir for MemLogDir {
    fn append_line(&self, name: &str, line: &str) -> std::io::Result<()> {
        self.push(name, &format!("{line}\n"));
        Ok(())
    }

    fn list(&self) -> std::io::Result<Vec<crate::fs::DirEntry>> {
        Ok(self
            .files
            .lock()
            .expect("files")
            .iter()
            .map(|(name, data)| crate::fs::DirEntry {
                name: name.clone(),
                is_dir: false,
                len: data.len() as u64,
            })
            .collect())
    }

    fn remove(&self, name: &str) -> std::io::Result<()> {
        self.files.lock().expect("files").remove(name);
        Ok(())
    }

    fn read(&self, name: &str) -> std::io::Result<Vec<u8>> {
        let data = self
            .files
            .lock()
            .expect("files")
            .get(name)
            .cloned()
            .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::NotFound))?;
        self.bytes_read
            .fetch_add(data.len() as u64, std::sync::atomic::Ordering::Relaxed);
        Ok(data)
    }

    fn file_len(&self, name: &str) -> std::io::Result<u64> {
        self.files
            .lock()
            .expect("files")
            .get(name)
            .map(|data| data.len() as u64)
            .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::NotFound))
    }

    fn read_range(&self, name: &str, start: u64, len: u64) -> std::io::Result<Vec<u8>> {
        let files = self.files.lock().expect("files");
        let data = files
            .get(name)
            .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::NotFound))?;
        let from = usize::try_from(start).unwrap_or(usize::MAX).min(data.len());
        let to = from
            .saturating_add(usize::try_from(len).unwrap_or(usize::MAX))
            .min(data.len());
        self.bytes_read
            .fetch_add((to - from) as u64, std::sync::atomic::Ordering::Relaxed);
        Ok(data[from..to].to_vec())
    }
}
