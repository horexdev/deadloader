//! Helpers for tests: temp directories, archives built in memory, a recording [`Hooks`].

use crate::output::{FileEvent, Hooks};
use std::cell::RefCell;
use std::io::{Cursor, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

/// A fresh directory under the system temp dir, removed on drop.
#[derive(Debug)]
pub struct TempDir(PathBuf);

impl TempDir {
    pub fn new(tag: &str) -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let n = NEXT.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "deadloader-archive-{}-{tag}-{n}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).expect("temp dir");
        Self(path)
    }

    pub fn path(&self) -> &Path {
        &self.0
    }

    pub fn file(&self, name: &str, bytes: &[u8]) -> PathBuf {
        let path = self.0.join(name);
        std::fs::write(&path, bytes).expect("write fixture");
        path
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// Every file under `dir`, relative, with `/` separators, sorted.
pub fn files_under(dir: &Path) -> Vec<String> {
    fn walk(root: &Path, dir: &Path, out: &mut Vec<String>) {
        for entry in std::fs::read_dir(dir).expect("read dir") {
            let entry = entry.expect("entry");
            let path = entry.path();
            if path.is_dir() {
                walk(root, &path, out);
            } else {
                let rel = path.strip_prefix(root).expect("under root");
                out.push(rel.to_string_lossy().replace('\\', "/"));
            }
        }
    }
    let mut out = Vec::new();
    walk(dir, dir, &mut out);
    out.sort();
    out
}

#[derive(Debug, Default)]
pub struct RecordingHooks {
    pub free: Option<u64>,
    events: RefCell<Vec<String>>,
}

impl RecordingHooks {
    pub fn with_free(free: u64) -> Self {
        Self {
            free: Some(free),
            ..Self::default()
        }
    }

    pub fn events(&self) -> Vec<String> {
        self.events.borrow().clone()
    }
}

impl Hooks for RecordingHooks {
    fn available_bytes(&self, _: &Path) -> Option<u64> {
        self.free
    }

    fn file_event(&self, event: &FileEvent<'_>) {
        let line = match event {
            FileEvent::Created(p) => format!("created {}", p.display()),
            FileEvent::Renamed { from, to } => {
                format!("renamed {} -> {}", from.display(), to.display())
            }
            FileEvent::Removed(p) => format!("removed {}", p.display()),
        };
        self.events.borrow_mut().push(line);
    }
}

/// A zip with `(name, data)` entries, deflated unless `stored`; names ending in `/` become directories.
pub fn zip_bytes(entries: &[(&str, &[u8])], stored: bool) -> Vec<u8> {
    use zip::write::SimpleFileOptions;
    let method = if stored {
        zip::CompressionMethod::Stored
    } else {
        zip::CompressionMethod::Deflated
    };
    let options = SimpleFileOptions::default()
        .compression_method(method)
        .large_file(false);
    let mut w = zip::ZipWriter::new(Cursor::new(Vec::new()));
    for (name, data) in entries {
        if name.ends_with('/') {
            w.add_directory(*name, options).expect("dir");
        } else {
            w.start_file(*name, options).expect("start");
            w.write_all(data).expect("data");
        }
    }
    w.finish().expect("finish").into_inner()
}

/// A 7z with `(name, data)` entries: one block per file, or one solid block for all.
pub fn seven_z_bytes(entries: &[(&str, &[u8])], solid: bool) -> Vec<u8> {
    use sevenz_rust2::{ArchiveEntry, ArchiveWriter, SourceReader};
    let mut w = ArchiveWriter::new(Cursor::new(Vec::new())).expect("writer");
    if solid {
        let names = entries
            .iter()
            .map(|(n, _)| ArchiveEntry::new_file(n))
            .collect();
        let readers = entries.iter().map(|(_, d)| SourceReader::new(*d)).collect();
        w.push_archive_entries(names, readers).expect("solid");
    } else {
        for (name, data) in entries {
            if name.ends_with('/') {
                w.push_archive_entry::<&[u8]>(ArchiveEntry::new_directory(name), None)
                    .expect("dir");
            } else {
                w.push_archive_entry(ArchiveEntry::new_file(name), Some(*data))
                    .expect("entry");
            }
        }
    }
    w.finish().expect("finish").into_inner()
}
