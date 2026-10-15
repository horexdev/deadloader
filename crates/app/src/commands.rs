//! The commands the UI calls (`invoke`). Each one hands over to [`Backend`] on a worker thread, so the window stays
//! responsive while files are copied; errors reach the UI as `{ id, args }` messages it translates.

use crate::backend::{ApplyResult, Backend, Changed, Game};
use crate::events::{Outbox, Sink, UiEvent};
use deadloader_core::fs::Disk;
use deadloader_core::message::MessageId;
use deadloader_core::message::UserMessage;
use deadloader_core::remove::{RemovalPlan, Removed};
use deadloader_core::session::{ImportOutcome, LibraryView, ProfileImported};
use deadloader_core::settings::Settings;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::State;
use tauri::ipc::Channel;

type Reply<T> = Result<T, UserMessage>;
type Shared<'a> = State<'a, Arc<Backend>>;

/// Runs `work` on a blocking thread: file copies and hashing must not hold up the async runtime or the window.
async fn blocking<T: Send + 'static>(
    backend: Shared<'_>,
    work: impl FnOnce(&Backend) -> Reply<T> + Send + 'static,
) -> Reply<T> {
    let backend = Arc::clone(backend.inner());
    tauri::async_runtime::spawn_blocking(move || work(&backend))
        .await
        .map_err(|e| UserMessage::new(MessageId::Internal).arg("detail", e))?
}

/// The UI's channel for pushes (download progress, the game starting and closing, dropped files, start notices).
/// Called once after the page loads, and again after a reload; the newest channel wins.
#[tauri::command]
pub fn subscribe(outbox: State<'_, Arc<Outbox>>, events: Channel<UiEvent>) {
    outbox.connect(Box::new(ChannelSink(events)));
}

struct ChannelSink(Channel<UiEvent>);

impl Sink for ChannelSink {
    fn deliver(&self, event: &UiEvent) -> bool {
        self.0.send(event.clone()).is_ok()
    }
}

#[tauri::command]
pub async fn library(backend: Shared<'_>) -> Reply<LibraryView> {
    blocking(backend, move |b| b.library()).await
}

#[tauri::command]
pub async fn set_enabled(backend: Shared<'_>, mod_id: String, enabled: bool) -> Reply<Changed> {
    blocking(backend, move |b| b.set_enabled(&mod_id, enabled)).await
}

#[tauri::command]
pub async fn move_mod(backend: Shared<'_>, mod_id: String, index: usize) -> Reply<Changed> {
    blocking(backend, move |b| b.move_mod(&mod_id, index)).await
}

#[tauri::command]
pub async fn switch_variant(backend: Shared<'_>, mod_id: String, file: String) -> Reply<Changed> {
    blocking(backend, move |b| b.switch_variant(&mod_id, &file)).await
}

#[tauri::command]
pub async fn choose(backend: Shared<'_>, mod_id: String, files: Vec<String>) -> Reply<Changed> {
    blocking(backend, move |b| b.choose(&mod_id, &files)).await
}

#[tauri::command]
pub async fn select_profile(backend: Shared<'_>, name: String) -> Reply<LibraryView> {
    blocking(backend, move |b| b.select_profile(&name)).await
}

#[tauri::command]
pub async fn create_profile(backend: Shared<'_>, name: String) -> Reply<Vec<String>> {
    blocking(backend, move |b| b.create_profile(&name)).await
}

#[tauri::command]
pub async fn profile_names(backend: Shared<'_>) -> Reply<Vec<String>> {
    blocking(backend, move |b| b.profile_names()).await
}

#[tauri::command]
pub async fn import_paths(
    backend: Shared<'_>,
    paths: Vec<PathBuf>,
) -> Reply<Vec<Result<ImportOutcome, UserMessage>>> {
    blocking(backend, move |b| b.import_paths(&paths)).await
}

#[tauri::command]
pub async fn apply(backend: Shared<'_>) -> Reply<ApplyResult> {
    blocking(backend, move |b| b.apply()).await
}

#[tauri::command]
pub async fn removal_plan(
    backend: Shared<'_>,
    mod_id: String,
    files: Option<Vec<String>>,
) -> Reply<RemovalPlan> {
    blocking(backend, move |b| b.removal_plan(&mod_id, files)).await
}

#[tauri::command]
pub async fn remove(
    backend: Shared<'_>,
    mod_id: String,
    files: Option<Vec<String>>,
) -> Reply<Removed> {
    blocking(backend, move |b| b.remove(&mod_id, files)).await
}

#[tauri::command]
pub async fn rename_profile(backend: Shared<'_>, old: String, new: String) -> Reply<Vec<String>> {
    blocking(backend, move |b| b.rename_profile(&old, &new)).await
}

#[tauri::command]
pub async fn duplicate_profile(
    backend: Shared<'_>,
    name: String,
    new_name: String,
) -> Reply<Vec<String>> {
    blocking(backend, move |b| b.duplicate_profile(&name, &new_name)).await
}

#[tauri::command]
pub async fn delete_profile(backend: Shared<'_>, name: String) -> Reply<Vec<String>> {
    blocking(backend, move |b| b.delete_profile(&name)).await
}

#[tauri::command]
pub async fn export_profile(backend: Shared<'_>, name: String) -> Reply<String> {
    blocking(backend, move |b| b.export_profile(&name)).await
}

#[tauri::command]
pub async fn import_profile(backend: Shared<'_>, text: String) -> Reply<ProfileImported> {
    blocking(backend, move |b| b.import_profile(&text)).await
}

#[tauri::command]
pub async fn settings(backend: Shared<'_>) -> Reply<Settings> {
    blocking(backend, move |b| b.settings()).await
}

#[tauri::command]
pub async fn update_settings(backend: Shared<'_>, settings: Settings) -> Reply<Settings> {
    blocking(backend, move |b| b.update_settings(settings)).await
}

/// The texts of a system file dialog, translated by the UI (no user-visible English in Rust).
#[derive(Debug, Clone, serde::Deserialize)]
pub struct DialogText {
    pub title: String,
    /// The name of the file-type filter, e.g. "Mod files".
    pub filter: String,
}

/// Add from disk (134): the system dialog for `.vpk` / archives, then the import of each chosen file.
#[tauri::command]
pub async fn add_from_disk(
    window: tauri::WebviewWindow,
    backend: Shared<'_>,
    text: DialogText,
) -> Reply<Vec<Result<ImportOutcome, UserMessage>>> {
    blocking(backend, move |b| {
        let picked = rfd::FileDialog::new()
            .set_parent(&window)
            .set_title(&text.title)
            .add_filter(&text.filter, &["vpk", "zip", "7z", "rar"])
            .pick_files();
        match picked {
            Some(paths) => b.import_paths(&paths),
            None => Ok(Vec::new()),
        }
    })
    .await
}

/// Profiles → Export: the system save dialog, proposing `<name>.json`. `false` if the user cancelled.
#[tauri::command]
pub async fn export_profile_dialog(
    window: tauri::WebviewWindow,
    backend: Shared<'_>,
    name: String,
    text: DialogText,
) -> Reply<bool> {
    blocking(backend, move |b| {
        let chosen = rfd::FileDialog::new()
            .set_parent(&window)
            .set_title(&text.title)
            .set_file_name(format!("{name}.json"))
            .add_filter(&text.filter, &["json"])
            .save_file();
        match chosen {
            Some(path) => b.export_profile_to(&name, &path).map(|()| true),
            None => Ok(false),
        }
    })
    .await
}

/// Profiles → Import: the system open dialog for a `.json`. `None` if the user cancelled.
#[tauri::command]
pub async fn import_profile_dialog(
    window: tauri::WebviewWindow,
    backend: Shared<'_>,
    text: DialogText,
) -> Reply<Option<ProfileImported>> {
    blocking(backend, move |b| {
        let chosen = rfd::FileDialog::new()
            .set_parent(&window)
            .set_title(&text.title)
            .add_filter(&text.filter, &["json"])
            .pick_file();
        match chosen {
            Some(path) => b.import_profile_from(&path).map(Some),
            None => Ok(None),
        }
    })
    .await
}

/// Deadlock on this PC, found through Steam.
pub struct SteamGame;

impl Game for SteamGame {
    fn running(&self) -> bool {
        // If the process list can't be read, assume the game runs: waiting is safe, writing into a running game's
        // folder is not.
        deadloader_platform_windows::process::game_running().unwrap_or(true)
    }

    fn move_to_recycle_bin(&self, path: &std::path::Path) -> std::io::Result<()> {
        deadloader_platform_windows::recycle::move_to_recycle_bin(path)
    }

    fn installations(&self) -> Vec<PathBuf> {
        match deadloader_platform_windows::steam::steam_root() {
            Ok(Some(root)) => deadloader_core::steam::find_installations(&root, &Disk)
                .installations
                .into_iter()
                .map(|i| i.game_root)
                .collect(),
            _ => Vec::new(),
        }
    }
}
