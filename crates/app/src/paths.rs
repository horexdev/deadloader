//! Where deadloader keeps its data: `deadloader-data\` next to the exe and nowhere else (ADR 0003).
//!
//! If that folder cannot be written there is no fallback to `%APPDATA%` (decision 57): the user is told to move the
//! exe, because data written somewhere else would end up where nobody looks.

use std::fmt;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

pub const DATA_DIR: &str = "deadloader-data";

/// The folders inside `deadloader-data\` that this part of the app uses.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataLayout {
    root: PathBuf,
}

impl DataLayout {
    /// The layout next to the executable at `exe`.
    pub fn beside(exe: &Path) -> Result<Self, DataDirError> {
        match exe.parent() {
            Some(dir) if !dir.as_os_str().is_empty() => Ok(Self {
                root: dir.join(DATA_DIR),
            }),
            _ => Err(DataDirError::NoExeFolder {
                exe: exe.to_path_buf(),
            }),
        }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    /// The WebView2 profile (ADR 0001).
    pub fn webview(&self) -> PathBuf {
        self.root.join("webview")
    }

    pub fn logs(&self) -> PathBuf {
        self.root.join("logs")
    }

    /// Creates the data folder if needed and proves it can be written by creating and deleting a probe file.
    pub fn ensure_writable(&self) -> Result<(), DataDirError> {
        let not_writable = |error| DataDirError::NotWritable {
            dir: self.root.clone(),
            error,
        };
        std::fs::create_dir_all(&self.root).map_err(not_writable)?;
        // Creating the folder can succeed where creating files does not (a read-only share, folder permissions).
        let probe = self
            .root
            .join(format!(".write-test-{}", std::process::id()));
        let written = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&probe)
            .and_then(|mut file| file.write_all(b"deadloader"));
        let removed = std::fs::remove_file(&probe);
        written.map_err(not_writable)?;
        removed.map_err(not_writable)
    }
}

#[derive(Debug)]
pub enum DataDirError {
    /// The executable's path has no parent folder.
    NoExeFolder { exe: PathBuf },
    /// The data folder could not be created or written.
    NotWritable { dir: PathBuf, error: io::Error },
}

impl fmt::Display for DataDirError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoExeFolder { exe } => write!(
                f,
                "deadloader could not tell which folder it was started from ({}). Start deadloader.exe directly \
                 from its folder.",
                exe.display()
            ),
            Self::NotWritable { dir, error } => write!(
                f,
                "deadloader cannot save its data in {} ({error}). It keeps everything in that folder next to \
                 deadloader.exe and nowhere else, so it cannot run without it. Move deadloader.exe to a folder you \
                 can write to, for example on the same drive as Deadlock (not Program Files), and start it again.",
                dir.display()
            ),
        }
    }
}

impl std::error::Error for DataDirError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::NoExeFolder { .. } => None,
            Self::NotWritable { error, .. } => Some(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_dir(name: &str) -> PathBuf {
        let dir =
            std::env::temp_dir().join(format!("deadloader-app-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        dir
    }

    #[test]
    fn data_lives_next_to_the_exe() {
        let layout =
            DataLayout::beside(Path::new(r"D:\Games\deadloader\deadloader.exe")).expect("layout");
        assert_eq!(
            layout.root(),
            Path::new(r"D:\Games\deadloader\deadloader-data")
        );
        assert_eq!(
            layout.webview(),
            Path::new(r"D:\Games\deadloader\deadloader-data\webview")
        );
        assert_eq!(
            layout.logs(),
            Path::new(r"D:\Games\deadloader\deadloader-data\logs")
        );
    }

    #[test]
    fn works_for_an_exe_in_a_drive_root() {
        let layout = DataLayout::beside(Path::new(r"E:\deadloader.exe")).expect("layout");
        assert_eq!(layout.root(), Path::new(r"E:\deadloader-data"));
    }

    #[test]
    fn an_exe_path_without_a_folder_is_an_error() {
        let err = DataLayout::beside(Path::new("deadloader.exe")).expect_err("no folder");
        assert!(matches!(err, DataDirError::NoExeFolder { .. }), "{err:?}");
    }

    #[test]
    fn ensure_writable_creates_the_folder_and_leaves_no_probe_behind() {
        let exe_dir = temp_dir("writable");
        std::fs::create_dir_all(&exe_dir).expect("exe dir");
        let layout = DataLayout::beside(&exe_dir.join("deadloader.exe")).expect("layout");
        layout.ensure_writable().expect("writable");
        assert!(layout.root().is_dir());
        let left: Vec<_> = std::fs::read_dir(layout.root()).expect("list").collect();
        assert!(left.is_empty(), "{left:?}");
        // A second start finds the folder already there.
        layout.ensure_writable().expect("writable again");
        let _ = std::fs::remove_dir_all(&exe_dir);
    }

    #[test]
    fn a_data_folder_that_cannot_be_created_is_reported_with_its_path() {
        let exe_dir = temp_dir("blocked");
        std::fs::create_dir_all(&exe_dir).expect("exe dir");
        // A file where the folder should be: the folder cannot be created.
        std::fs::write(exe_dir.join(DATA_DIR), b"").expect("blocker");
        let layout = DataLayout::beside(&exe_dir.join("deadloader.exe")).expect("layout");
        let err = layout.ensure_writable().expect_err("not writable");
        match &err {
            DataDirError::NotWritable { dir, .. } => assert_eq!(dir, layout.root()),
            other => panic!("unexpected {other:?}"),
        }
        let _ = std::fs::remove_dir_all(&exe_dir);
    }

    #[test]
    fn the_error_says_what_happened_why_and_what_to_do() {
        let err = DataDirError::NotWritable {
            dir: PathBuf::from(r"C:\Program Files\deadloader\deadloader-data"),
            error: io::Error::from(io::ErrorKind::PermissionDenied),
        };
        let text = err.to_string();
        assert!(
            text.contains(r"C:\Program Files\deadloader\deadloader-data"),
            "{text}"
        );
        assert!(text.contains("next to deadloader.exe"), "{text}");
        assert!(text.contains("Move deadloader.exe"), "{text}");
    }
}
