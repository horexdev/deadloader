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
