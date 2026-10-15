//! The commands the UI calls (`invoke`). Each one hands over to [`Backend`] on a worker thread, so the window stays
//! responsive while files are copied; errors reach the UI as `{ id, args }` messages it translates.

use crate::backend::{ApplyResult, Backend, Changed, Game};
use deadloader_core::fs::Disk;
use deadloader_core::message::MessageId;
use deadloader_core::message::UserMessage;
use deadloader_core::session::{ImportOutcome, LibraryView};
use std::path::PathBuf;
use std::sync::Arc;
use tauri::State;

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

/// Deadlock on this PC, found through Steam.
pub struct SteamGame;

impl Game for SteamGame {
    fn running(&self) -> bool {
        // If the process list can't be read, assume the game runs: waiting is safe, writing into a running game's
        // folder is not.
        deadloader_platform_windows::process::game_running().unwrap_or(true)
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
