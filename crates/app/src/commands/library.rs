//! The library screen (S4): mods, their files and variants, applying, removing, importing and linking.

use super::*;

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

/// F10 + 171: the GameBanana page address pasted for a mod imported from disk.
#[tauri::command]
pub async fn mod_link(
    slot: Slot<'_>,
    backend: Shared<'_>,
    mod_id: String,
    url: String,
) -> Result<deadloader_core::session::LinkReport, crate::updates::LinkFailure> {
    let backend = Arc::clone(backend.inner());
    on_catalog(slot, move |c| {
        let catalog = c.wait()?;
        crate::updates::link_to_page(&backend, &catalog, &mod_id, &url)
    })
    .await
}

/// "Link" by hand for an archive nothing matched: its files came from GameBanana file `remote_file_id`.
#[tauri::command]
pub async fn mod_link_files(
    backend: Shared<'_>,
    mod_id: String,
    files: Vec<String>,
    remote_file_id: Option<u64>,
) -> Reply<()> {
    blocking(backend, move |b| {
        b.link_files(&mod_id, &files, remote_file_id)
    })
    .await
}

/// F10: rename a mod.
#[tauri::command]
pub async fn mod_rename(backend: Shared<'_>, mod_id: String, name: String) -> Reply<LibraryView> {
    blocking(backend, move |b| b.rename_mod(&mod_id, &name)).await
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
