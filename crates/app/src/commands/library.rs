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

/// S9: the pending changes and the last automatic apply while it can be undone.
#[tauri::command]
pub async fn pending(backend: Shared<'_>) -> Reply<crate::backend::Pending> {
    blocking(backend, move |b| b.pending()).await
}

/// S9 "Undo this change": that part of the mod goes back to what the game folder has.
#[tauri::command]
pub async fn pending_revert(
    backend: Shared<'_>,
    mod_id: String,
    aspect: deadloader_core::pending::Aspect,
) -> Reply<Changed> {
    blocking(backend, move |b| b.revert_pending(&mod_id, aspect)).await
}

/// S9 "Discard all", or "Keep <the applied profile>" for a profile switch.
#[tauri::command]
pub async fn pending_discard(backend: Shared<'_>) -> Reply<Changed> {
    blocking(backend, move |b| b.discard_pending()).await
}

/// Undo of what was applied after the game closed (N12, 143, 147).
#[tauri::command]
pub async fn pending_undo_applied(backend: Shared<'_>) -> Reply<Changed> {
    blocking(backend, move |b| b.undo_applied()).await
}

/// F9: the foreign items of the game's mod folder.
#[tauri::command]
pub async fn foreign_files(backend: Shared<'_>) -> Reply<crate::backend::ForeignReport> {
    blocking(backend, move |b| b.foreign_files()).await
}

/// F9 "Leave as is".
#[tauri::command]
pub async fn foreign_leave(
    backend: Shared<'_>,
    name: String,
) -> Reply<crate::backend::ForeignReport> {
    blocking(backend, move |b| b.leave_foreign(&name)).await
}

/// D11 "Take under management".
#[tauri::command]
pub async fn foreign_take(
    backend: Shared<'_>,
    name: String,
    mod_name: String,
) -> Reply<crate::backend::Taken> {
    blocking(backend, move |b| b.take_foreign(&name, &mod_name)).await
}

/// D10: the state of `gameinfo.gi` and, when another program changed it, the differences. `fix` ("Check again")
/// adds our line to the original file.
#[tauri::command]
pub async fn gameinfo_status(
    backend: Shared<'_>,
    fix: bool,
) -> Reply<crate::backend::GameinfoStatus> {
    blocking(backend, move |b| b.gameinfo_status(fix)).await
}

type AppLog = Arc<
    deadloader_core::log::Logger<
        deadloader_core::log::SystemClock,
        deadloader_core::log::DiskLogDir,
    >,
>;

/// Logs an external program deadloader started (ADR 0009).
fn log_shell(log: &AppLog, result: &Reply<crate::backend::ShellRequest>) {
    use deadloader_core::log::Level;
    if let Ok(request) = result {
        let (program, arguments) = request.describe();
        let fields = serde_json::Map::from_iter([
            ("program".to_owned(), program.into()),
            ("arguments".to_owned(), arguments.into()),
        ]);
        let _ = log.app(Level::Info, "external program started", fields);
    }
}

/// D10 "Verify game files in Steam".
#[tauri::command]
pub async fn verify_game_files(backend: Shared<'_>, log: State<'_, AppLog>) -> Reply<()> {
    let log = Arc::clone(log.inner());
    blocking(backend, move |b| {
        let result = b.verify_game_files();
        log_shell(&log, &result);
        result.map(|_| ())
    })
    .await
}

/// "Open file" / "Open folder" for a place the core knows.
#[tauri::command]
pub async fn show_place(
    backend: Shared<'_>,
    log: State<'_, AppLog>,
    place: crate::backend::Place,
    as_file: bool,
) -> Reply<()> {
    let log = Arc::clone(log.inner());
    blocking(backend, move |b| {
        let result = b.show(&place, as_file);
        log_shell(&log, &result);
        result.map(|_| ())
    })
    .await
}
