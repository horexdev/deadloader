//! What Settings → About, Settings → Cache, the setup step (S1 step 6) and "Delete all deadloader data" (D8) need from
//! this PC, put together from the backend, the catalog and Windows.

use crate::backend::Backend;
use deadloader_core::message::{Area, UserMessage, from_io};
use deadloader_core::session::ForeignFile;
use deadloader_core::setup::{ExeLocation, GameinfoState, exe_location, same_volume, volume};
use serde::Serialize;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};

/// The places the app works with: the exe's folder and the data folder next to it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Places {
    pub exe: PathBuf,
    pub data: PathBuf,
}

impl Places {
    pub fn exe_dir(&self) -> &Path {
        self.exe.parent().unwrap_or(&self.exe)
    }
}

/// Settings → About.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct About {
    /// From `Cargo.toml` (158).
    pub version: String,
    /// The commit CI built this exe from; `None` for a local build.
    pub commit: Option<String>,
    /// The WebView2 runtime that draws the window.
    pub webview2: Option<String>,
    pub data_folder: String,
    pub data_bytes: u64,
    /// SHA-256 of this exe, to compare with the release page.
    pub exe_sha256: Option<String>,
}

pub fn about(places: &Places, webview2: Option<String>) -> About {
    let hex = |bytes: [u8; 32]| bytes.iter().map(|b| format!("{b:02x}")).collect::<String>();
    About {
        version: env!("CARGO_PKG_VERSION").to_owned(),
        commit: build_commit(option_env!("GITHUB_SHA")),
        webview2,
        data_folder: places.data.display().to_string(),
        data_bytes: deadloader_core::wipe::size_of(&places.data),
        exe_sha256: std::fs::read(&places.exe)
            .ok()
            .map(|b| hex(deadloader_core::fs::sha256(&b))),
    }
}

/// A full commit id set by CI; anything else means a local build.
fn build_commit(sha: Option<&str>) -> Option<String> {
    sha.filter(|s| s.len() == 40 && s.bytes().all(|b| b.is_ascii_hexdigit()))
        .map(str::to_ascii_lowercase)
}

/// Settings → Cache.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize)]
pub struct CacheUsage {
    pub bytes: u64,
    pub thumbnails: usize,
    pub images: usize,
    pub pages: usize,
}

impl From<deadloader_core::cache::Usage> for CacheUsage {
    fn from(u: deadloader_core::cache::Usage) -> Self {
        Self {
            bytes: u.sizes.total(),
            thumbnails: u.thumbnails,
            images: u.images,
            pages: u.pages,
        }
    }
}

/// What "Clear cache" freed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct CacheCleared {
    pub bytes: u64,
    pub files: usize,
    /// Files that could not be deleted (still in use); they stay counted.
    pub failed: usize,
}

impl From<deadloader_core::cache::Cleared> for CacheCleared {
    fn from(c: deadloader_core::cache::Cleared) -> Self {
        Self {
            bytes: c.bytes,
            files: c.files,
            failed: c.failed,
        }
    }
}

/// The setup step's checks (S1 step 6): nothing here changes anything.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SetupCheck {
    /// The game folder the selected profile will use; `None`: Deadlock wasn't found.
    pub game_root: Option<String>,
    /// How many installations Steam knows.
    pub installations: usize,
    pub gameinfo: GameinfoState,
    /// Files in the game's mod folder that deadloader didn't create.
    pub foreign: Vec<ForeignFile>,
    pub exe_folder: String,
    pub exe_location: ExeLocation,
    /// `C:` or `\\server\share`.
    pub data_volume: Option<String>,
    pub game_volume: Option<String>,
    /// Library and game on one drive: hard links, no extra space (124). `None` without a game.
    pub same_volume: Option<bool>,
}

pub fn setup_check(
    backend: &Backend,
    places: &Places,
    downloads: Option<&Path>,
    desktop: Option<&Path>,
) -> Result<SetupCheck, UserMessage> {
    let status = backend.game_status()?;
    let preview = match status.root {
        Some(_) => Some(backend.rollback_preview()?),
        None => None,
    };
    let game_root = preview.as_ref().map(|p| p.game_root.clone());
    let game_volume = game_root.as_deref().and_then(|r| volume(Path::new(r)));
    Ok(SetupCheck {
        installations: status.installations,
        gameinfo: preview
            .as_ref()
            .map_or(GameinfoState::Missing, |p| p.gameinfo),
        foreign: preview.map(|p| p.foreign).unwrap_or_default(),
        exe_folder: places.exe_dir().display().to_string(),
        exe_location: exe_location(places.exe_dir(), downloads, desktop),
        data_volume: volume(&places.data),
        same_volume: game_root
            .as_deref()
            .map(|r| same_volume(&places.data, Path::new(r))),
        game_volume,
        game_root,
    })
}

/// Which folder "Open folder" shows. The UI names a place, never a path, except a game installation from the list.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[serde(tag = "place", rename_all = "snake_case")]
pub enum Place {
    Data,
    Logs,
    Cache,
    /// The folder of deadloader.exe (the setup step's location warning).
    Exe,
    Game {
        root: String,
    },
}

/// The folder to show, checked: a game folder must be one the installations list shows.
pub fn folder_of(
    place: &Place,
    places: &Places,
    known_games: &[String],
) -> Result<PathBuf, UserMessage> {
    Ok(match place {
        Place::Data => places.data.clone(),
        Place::Logs => places.data.join("logs"),
        Place::Cache => places.data.join("cache"),
        Place::Exe => places.exe_dir().to_path_buf(),
        Place::Game { root } => known_games
            .iter()
            .find(|g| g.eq_ignore_ascii_case(root))
            .map(PathBuf::from)
            .ok_or_else(|| {
                UserMessage::new(deadloader_core::message::MessageId::Internal)
                    .arg("detail", format!("{root} is not a known game folder"))
            })?,
    })
}

/// Shows `folder` in Explorer; a missing folder is reported with its path.
pub fn open(folder: &Path) -> Result<(), UserMessage> {
    deadloader_platform_windows::shell::open_folder(folder)
        .map_err(|e| from_io(&e, Area::Data).arg("path", folder.display()))
}

/// Set when the user confirmed D8: the data folder goes when the app exits.
#[derive(Debug, Default)]
pub struct WipeOnExit(AtomicBool);

impl WipeOnExit {
    pub fn arm(&self) {
        self.0.store(true, Ordering::SeqCst);
    }

    pub fn armed(&self) -> bool {
        self.0.load(Ordering::SeqCst)
    }
}

/// At exit, after everything else shut down: deletes the data folder, waiting a little for the window engine to let
/// go of its files. What stays locked is finished at the next start (the marker stays with it).
pub fn wipe_at_exit(data: &Path, attempts: u32, pause: std::time::Duration) -> bool {
    for attempt in 0..attempts {
        if deadloader_core::wipe::delete_all(data).is_ok() || !data.exists() {
            return true;
        }
        if attempt + 1 < attempts {
            std::thread::sleep(pause);
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn places() -> Places {
        Places {
            exe: PathBuf::from(r"D:\Games\deadloader\deadloader.exe"),
            data: PathBuf::from(r"D:\Games\deadloader\deadloader-data"),
        }
    }

    #[test]
    fn only_a_full_commit_id_from_ci_counts() {
        let sha = "3F9C2AB0123456789abcdef0123456789abcdef0";
        assert_eq!(
            build_commit(Some(sha)).as_deref(),
            Some("3f9c2ab0123456789abcdef0123456789abcdef0")
        );
        assert_eq!(build_commit(Some("local")), None);
        assert_eq!(build_commit(None), None);
    }

    #[test]
    fn open_folder_names_places_and_only_known_game_folders() {
        let p = places();
        assert_eq!(
            folder_of(&Place::Logs, &p, &[]).expect("logs"),
            PathBuf::from(r"D:\Games\deadloader\deadloader-data\logs")
        );
        assert_eq!(
            folder_of(&Place::Exe, &p, &[]).expect("exe"),
            PathBuf::from(r"D:\Games\deadloader")
        );
        let known = [r"E:\SteamLibrary\steamapps\common\Deadlock".to_owned()];
        assert!(
            folder_of(
                &Place::Game {
                    root: r"e:\steamlibrary\steamapps\common\deadlock".into()
                },
                &p,
                &known
            )
            .is_ok()
        );
        assert!(
            folder_of(
                &Place::Game {
                    root: r"C:\Windows".into()
                },
                &p,
                &known
            )
            .is_err()
        );
        let place: Place =
            serde_json::from_value(serde_json::json!({"place": "game", "root": "X"}))
                .expect("json");
        assert_eq!(place, Place::Game { root: "X".into() });
    }

    #[test]
    fn about_says_which_version_and_how_big_the_data_is() {
        let dir = std::env::temp_dir().join(format!("deadloader-about-{}", std::process::id()));
        let data = dir.join("deadloader-data");
        std::fs::create_dir_all(&data).expect("data");
        std::fs::write(data.join("settings.json"), vec![b' '; 120]).expect("settings");
        let exe = dir.join("deadloader.exe");
        std::fs::write(&exe, b"exe").expect("exe");
        let a = about(&Places { exe, data }, Some("155.0.4283.45".into()));
        assert_eq!(a.version, env!("CARGO_PKG_VERSION"));
        assert_eq!(a.data_bytes, 120);
        assert_eq!(a.exe_sha256.as_ref().map(String::len), Some(64));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_data_folder_goes_at_exit_once_nothing_holds_it() {
        let dir = std::env::temp_dir().join(format!("deadloader-exitwipe-{}", std::process::id()));
        let data = dir.join("deadloader-data");
        std::fs::create_dir_all(data.join("logs")).expect("data");
        std::fs::write(data.join("logs").join("app-2026-10-15.jsonl"), b"{}").expect("log");
        deadloader_core::wipe::mark_pending(&data).expect("marked");
        let flag = WipeOnExit::default();
        assert!(!flag.armed());
        flag.arm();
        assert!(flag.armed());
        assert!(wipe_at_exit(&data, 3, std::time::Duration::from_millis(1)));
        assert!(!data.exists());
        assert!(dir.exists());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
