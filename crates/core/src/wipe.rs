//! "Delete all deadloader data" (D8, F11): what `deadloader-data\` holds, and deleting it.
//!
//! The running app keeps files open in its own folder (the WebView2 profile, the logs), so the deletion is marked
//! first ([`mark_pending`]) and done when the app exits; whatever is still locked then is finished at the next start,
//! before anything else opens a file there ([`finish_pending`]). The marker is the last thing removed.
//!
//! Deleting never leaves the folder: links and junctions inside it are removed as links, never followed, and a
//! folder that isn't named `deadloader-data` is refused. Read-only library files are made writable first (they may be
//! hard links of files in a game folder; only this name is removed, the game's copy stays).

use crate::library::Manifest;
use crate::log::Kind;
use serde::Serialize;
use std::io;
use std::path::Path;

/// The data folder's name (ADR 0003); nothing else is ever deleted.
pub const DATA_DIR: &str = "deadloader-data";
/// Present while a deletion of all data is under way.
pub const PENDING_MARKER: &str = "deletion-pending";

/// What D8 lists before deleting.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct Summary {
    /// The library and `manifest.json`.
    pub library: LibraryPart,
    pub profiles: ProfilesPart,
    pub logs: LogsPart,
    /// Thumbnails, description images and catalog pages.
    pub cache_bytes: u64,
    pub settings: SettingsPart,
    /// Everything else: partial downloads and kept archives, `gameinfo.gi` backups, the window engine's data.
    pub other_bytes: u64,
    pub total_bytes: u64,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct LibraryPart {
    pub mods: usize,
    pub files: usize,
    pub bytes: u64,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct ProfilesPart {
    pub names: Vec<String>,
    pub bytes: u64,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct LogsPart {
    pub files: usize,
    /// The date of the oldest log file, `YYYY-MM-DD`.
    pub oldest: Option<String>,
    pub bytes: u64,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct SettingsPart {
    /// Dated backups of `settings.json` (decision 156).
    pub backups: usize,
    pub bytes: u64,
}

/// What a deletion removed.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Deleted {
    pub files: usize,
    pub bytes: u64,
}

pub fn summary(data_dir: &Path, manifest: &Manifest, profiles: &[String]) -> Summary {
    let size = |name: &str| size_of(&data_dir.join(name));
    let library = LibraryPart {
        mods: manifest.mods.len(),
        files: manifest.mods.iter().map(|m| m.files.len()).sum(),
        bytes: size("library") + size("manifest.json") + size("manifest.json.bak"),
    };
    let (log_files, oldest) = logs(&data_dir.join("logs"));
    let settings_file = data_dir.join(crate::settings::FILE_NAME);
    let backups = crate::backups::list(&settings_file).unwrap_or_default();
    let settings = SettingsPart {
        backups: backups.len(),
        bytes: size_of(&settings_file)
            + size("settings.json.bak")
            + backups.iter().map(|b| size_of(b)).sum::<u64>(),
    };
    let profiles = ProfilesPart {
        names: profiles.to_vec(),
        bytes: size("profiles"),
    };
    let logs = LogsPart {
        files: log_files,
        oldest,
        bytes: size("logs"),
    };
    let cache_bytes = size("cache");
    let total_bytes = size_of(data_dir);
    let listed = library.bytes + profiles.bytes + logs.bytes + cache_bytes + settings.bytes;
    Summary {
        library,
        profiles,
        logs,
        cache_bytes,
        settings,
        other_bytes: total_bytes.saturating_sub(listed),
        total_bytes,
    }
}

/// Our log files and the date of the oldest.
fn logs(dir: &Path) -> (usize, Option<String>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return (0, None);
    };
    let dates: Vec<String> = entries
        .filter_map(Result::ok)
        .filter_map(|e| Kind::parse_file_name(&e.file_name().to_string_lossy()))
        .map(|(_, date)| {
            format!(
                "{:04}-{:02}-{:02}",
                date.year(),
                u8::from(date.month()),
                date.day()
            )
        })
        .collect();
    let oldest = dates.iter().min().cloned();
    (dates.len(), oldest)
}

/// Bytes in a file or a folder, not following links.
pub fn size_of(path: &Path) -> u64 {
    let Ok(meta) = std::fs::symlink_metadata(path) else {
        return 0;
    };
    if meta.file_type().is_symlink() {
        return 0;
    }
    if !meta.is_dir() {
        return meta.len();
    }
    std::fs::read_dir(path)
        .map(|entries| {
            entries
                .filter_map(Result::ok)
                .map(|e| size_of(&e.path()))
                .sum()
        })
        .unwrap_or(0)
}

fn check(data_dir: &Path) -> io::Result<()> {
    if data_dir.file_name().and_then(|n| n.to_str()) == Some(DATA_DIR) {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "{} is not deadloader's data folder; nothing was deleted",
                data_dir.display()
            ),
        ))
    }
}

/// Records that all data is to be deleted.
pub fn mark_pending(data_dir: &Path) -> io::Result<()> {
    check(data_dir)?;
    std::fs::create_dir_all(data_dir)?;
    std::fs::write(
        data_dir.join(PENDING_MARKER),
        b"deadloader: delete all data\n",
    )
}

pub fn is_pending(data_dir: &Path) -> bool {
    data_dir.join(PENDING_MARKER).is_file()
}

/// Deletes the data folder with everything in it. On failure (a file still locked) the rest is deleted and the marker,
/// if any, stays, so [`finish_pending`] can try again.
pub fn delete_all(data_dir: &Path) -> io::Result<Deleted> {
    check(data_dir)?;
    let mut deleted = Deleted::default();
    let mut first_error = None;
    let marker = data_dir.join(PENDING_MARKER);
    match std::fs::read_dir(data_dir) {
        Ok(entries) => {
            for entry in entries.filter_map(Result::ok) {
                let path = entry.path();
                if path == marker {
                    continue;
                }
                if let Err(e) = remove_tree(&path, &mut deleted) {
                    first_error.get_or_insert(e);
                }
            }
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(deleted),
        Err(e) => return Err(e),
    }
    if let Some(e) = first_error {
        return Err(e);
    }
    match std::fs::remove_file(&marker) {
        Err(e) if e.kind() != io::ErrorKind::NotFound => return Err(e),
        _ => {}
    }
    std::fs::remove_dir(data_dir)?;
    Ok(deleted)
}

/// At start: finishes a deletion that was marked but not completed. `Ok(None)` when none was pending.
pub fn finish_pending(data_dir: &Path) -> io::Result<Option<Deleted>> {
    if !is_pending(data_dir) {
        return Ok(None);
    }
    delete_all(data_dir).map(Some)
}

/// Removes `path`: a link as a link, a file (read-only too), or a folder with its content. Keeps going after an error
/// and reports the first.
fn remove_tree(path: &Path, deleted: &mut Deleted) -> io::Result<()> {
    let meta = std::fs::symlink_metadata(path)?;
    let kind = meta.file_type();
    if kind.is_symlink() {
        return remove_link(path, &kind);
    }
    if meta.is_dir() {
        let mut first_error = None;
        for entry in std::fs::read_dir(path)?.filter_map(Result::ok) {
            if let Err(e) = remove_tree(&entry.path(), deleted) {
                first_error.get_or_insert(e);
            }
        }
        if let Some(e) = first_error {
            return Err(e);
        }
        return std::fs::remove_dir(path);
    }
    if meta.permissions().readonly() {
        let mut writable = meta.permissions();
        #[allow(clippy::permissions_set_readonly_false)]
        writable.set_readonly(false);
        std::fs::set_permissions(path, writable)?;
    }
    std::fs::remove_file(path)?;
    deleted.files += 1;
    deleted.bytes += meta.len();
    Ok(())
}

#[cfg(windows)]
fn remove_link(path: &Path, kind: &std::fs::FileType) -> io::Result<()> {
    use std::os::windows::fs::FileTypeExt;
    if kind.is_symlink_dir() {
        // A directory symlink or a junction: removes the link, never what it points to.
        std::fs::remove_dir(path)
    } else {
        std::fs::remove_file(path)
    }
}

#[cfg(not(windows))]
fn remove_link(path: &Path, _: &std::fs::FileType) -> io::Result<()> {
    std::fs::remove_file(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{remove_all, temp_dir};

    fn data(name: &str) -> (std::path::PathBuf, std::path::PathBuf) {
        let base = temp_dir("wipe", name);
        let data = base.join(DATA_DIR);
        std::fs::create_dir_all(data.join("library").join("gb-1").join("1")).expect("library");
        std::fs::create_dir_all(data.join("profiles")).expect("profiles");
        std::fs::create_dir_all(data.join("logs")).expect("logs");
        std::fs::create_dir_all(data.join("cache").join("thumbs")).expect("cache");
        std::fs::create_dir_all(data.join("webview")).expect("webview");
        let vpk = data.join("library").join("gb-1").join("1").join("1.vpk");
        std::fs::write(&vpk, vec![1u8; 1000]).expect("vpk");
        let mut ro = std::fs::metadata(&vpk).expect("meta").permissions();
        ro.set_readonly(true);
        std::fs::set_permissions(&vpk, ro).expect("read-only");
        std::fs::write(data.join("manifest.json"), vec![b' '; 100]).expect("manifest");
        std::fs::write(data.join("profiles").join("Default.json"), vec![b' '; 10])
            .expect("profile");
        std::fs::write(
            data.join("logs").join("app-2026-09-16.jsonl"),
            vec![b' '; 20],
        )
        .expect("log");
        std::fs::write(
            data.join("logs").join("files-2026-10-15.jsonl"),
            vec![b' '; 30],
        )
        .expect("log");
        std::fs::write(
            data.join("cache").join("thumbs").join("x.jpg"),
            vec![0u8; 400],
        )
        .expect("thumb");
        std::fs::write(data.join("settings.json"), vec![b' '; 50]).expect("settings");
        std::fs::write(data.join("settings.json.bak-20261015-1131"), vec![b' '; 5])
            .expect("backup");
        std::fs::write(data.join("webview").join("Local State"), vec![0u8; 7]).expect("webview");
        (base, data)
    }

    #[test]
    fn the_summary_says_what_is_deleted_and_how_big_it_is() {
        let (base, data) = data("summary");
        let manifest = Manifest {
            mods: vec![crate::library::Mod {
                id: "gb-1".into(),
                name: "Haze".into(),
                author: None,
                source: crate::library::Source::Local {
                    imported_from: None,
                },
                files: Vec::new(),
            }],
            ..Manifest::default()
        };
        let s = summary(&data, &manifest, &["Default".to_owned()]);
        assert_eq!(s.library.mods, 1);
        assert_eq!(s.library.bytes, 1100);
        assert_eq!(s.profiles.bytes, 10);
        assert_eq!(s.logs.files, 2);
        assert_eq!(s.logs.oldest.as_deref(), Some("2026-09-16"));
        assert_eq!(s.logs.bytes, 50);
        assert_eq!(s.cache_bytes, 400);
        assert_eq!(s.settings.backups, 1);
        assert_eq!(s.settings.bytes, 55);
        assert_eq!(s.other_bytes, 7);
        assert_eq!(s.total_bytes, 1622);
        remove_all(&base);
    }

    #[test]
    fn deleting_removes_the_whole_folder_and_nothing_next_to_it() {
        let (base, data) = data("all");
        let neighbour = base.join("deadloader.exe");
        std::fs::write(&neighbour, b"exe").expect("exe");
        mark_pending(&data).expect("marked");
        assert!(is_pending(&data));
        let deleted = delete_all(&data).expect("deleted");
        assert!(!data.exists());
        assert!(neighbour.exists(), "the exe stays");
        assert!(deleted.files >= 8);
        remove_all(&base);
    }

    #[test]
    fn a_pending_deletion_is_finished_at_the_next_start() {
        let (base, data) = data("pending");
        assert_eq!(finish_pending(&data).expect("nothing"), None);
        assert!(data.exists(), "not marked: kept");
        mark_pending(&data).expect("marked");
        assert!(finish_pending(&data).expect("finished").is_some());
        assert!(!data.exists());
        remove_all(&base);
    }

    #[test]
    fn a_folder_that_is_not_the_data_folder_is_refused() {
        let base = temp_dir("wipe", "refused");
        std::fs::write(base.join("keep.txt"), b"x").expect("file");
        assert_eq!(
            delete_all(&base).expect_err("refused").kind(),
            io::ErrorKind::InvalidInput
        );
        assert!(mark_pending(&base).is_err());
        assert!(base.join("keep.txt").exists());
        remove_all(&base);
    }

    #[cfg(windows)]
    #[test]
    fn a_junction_inside_is_removed_without_touching_its_target() {
        let (base, data) = data("junction");
        let outside = base.join("outside");
        std::fs::create_dir_all(&outside).expect("outside");
        std::fs::write(outside.join("precious.txt"), b"x").expect("file");
        let link = data.join("link");
        // A directory symlink needs a privilege; a junction doesn't. Skip quietly when neither can be made.
        let made = std::process::Command::new("cmd")
            .args(["/C", "mklink", "/J"])
            .arg(&link)
            .arg(&outside)
            .output()
            .is_ok_and(|o| o.status.success());
        if made {
            delete_all(&data).expect("deleted");
            assert!(!data.exists());
            assert!(outside.join("precious.txt").exists(), "never followed");
        }
        remove_all(&base);
    }
}
