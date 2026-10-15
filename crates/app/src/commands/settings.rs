//! Settings (S8), the first-run wizard (S1) and the maintenance dialogs (D7, D8).

use super::*;
use crate::setup::{About, CacheCleared, CacheUsage, Place, Places, SetupCheck, WipeOnExit};
use deadloader_core::log::{DiskLogDir, Level, Logger, SystemClock};
use deadloader_core::session::{
    EditRefused, EditSaved, InstallationView, RollbackPreview, RolledBack,
};
use deadloader_core::settings::{CatalogChoice, Lang};

type Log<'a> = State<'a, Arc<Logger<SystemClock, DiskLogDir>>>;

#[tauri::command]
pub async fn settings(backend: Shared<'_>) -> Reply<Settings> {
    blocking(backend, move |b| b.settings()).await
}

#[tauri::command]
pub async fn update_settings(backend: Shared<'_>, settings: Settings) -> Reply<Settings> {
    blocking(backend, move |b| b.update_settings(settings)).await
}

fn app_log(
    log: &Logger<SystemClock, DiskLogDir>,
    lvl: Level,
    msg: &str,
    fields: serde_json::Value,
) {
    let fields = match fields {
        serde_json::Value::Object(map) => map,
        _ => serde_json::Map::new(),
    };
    let _ = log.app(lvl, msg, fields);
}

/// "Reset all settings" (100, 156).
#[derive(Debug, Clone, serde::Serialize)]
pub struct ResetDone {
    pub backup: Option<String>,
    pub settings: Settings,
}

#[tauri::command]
pub async fn settings_reset(backend: Shared<'_>, log: Log<'_>) -> Reply<ResetDone> {
    let log = Arc::clone(log.inner());
    blocking(backend, move |b| {
        let (saved, settings) = b.reset_settings()?;
        app_log(
            &log,
            Level::Info,
            "settings reset to their defaults",
            serde_json::json!({ "backup": saved.backup }),
        );
        Ok(ResetDone {
            backup: saved.backup,
            settings,
        })
    })
    .await
}

/// Which file the Advanced editor shows.
#[derive(Debug, Clone, Copy, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EditedFile {
    Settings,
    Manifest,
}

#[tauri::command]
pub async fn settings_file_text(backend: Shared<'_>, file: EditedFile) -> Reply<String> {
    blocking(backend, move |b| match file {
        EditedFile::Settings => b.settings_text(),
        EditedFile::Manifest => b.manifest_text(),
    })
    .await
}

/// The Advanced editor's "Check and save" (156): `Ok` with the backup's name, or `Err` with why nothing was saved.
#[tauri::command]
pub async fn settings_file_save(
    backend: Shared<'_>,
    file: EditedFile,
    text: String,
) -> Reply<Result<EditSaved, EditRefused>> {
    blocking(backend, move |b| match file {
        EditedFile::Settings => b.save_settings_text(&text),
        EditedFile::Manifest => b.save_manifest_text(&text),
    })
    .await
}

/// Settings → Cache: what the picture and page cache holds.
#[tauri::command]
pub async fn cache_usage(slot: Slot<'_>) -> Reply<CacheUsage> {
    on_catalog(slot, move |c| {
        let catalog = c.wait()?;
        catalog
            .cache_usage()
            .map(CacheUsage::from)
            .map_err(|e| from_io(&e))
    })
    .await
}

/// "Clear cache" (58); every deletion is in the files log.
#[tauri::command]
pub async fn cache_clear(slot: Slot<'_>) -> Reply<CacheCleared> {
    on_catalog(slot, move |c| {
        let catalog = c.wait()?;
        catalog
            .clear_cache()
            .map(CacheCleared::from)
            .map_err(|e| from_io(&e))
    })
    .await
}

fn from_io(e: &std::io::Error) -> UserMessage {
    deadloader_core::message::from_io(e, deadloader_core::message::Area::Data)
}

#[tauri::command]
pub async fn installations(backend: Shared<'_>) -> Reply<Vec<InstallationView>> {
    blocking(backend, move |b| b.installations()).await
}

/// "Add manually…" (Settings → Game) and "Change…" / "Browse…" (setup step): the system folder dialog; the chosen
/// Deadlock installation joins the list, and with `select` the selected profile uses it. `None` when cancelled; E2
/// when the folder holds no Deadlock installation.
#[tauri::command]
pub async fn game_folder_dialog(
    window: tauri::WebviewWindow,
    backend: Shared<'_>,
    text: DialogText,
    select: bool,
) -> Reply<Option<String>> {
    blocking(backend, move |b| {
        let Some(chosen) = rfd::FileDialog::new()
            .set_parent(&window)
            .set_title(&text.title)
            .pick_folder()
        else {
            return Ok(None);
        };
        let root = b.add_installation(&chosen)?;
        if select {
            b.set_game_root(Some(&root))?;
        }
        Ok(Some(root.display().to_string()))
    })
    .await
}

#[tauri::command]
pub async fn installation_remove(backend: Shared<'_>, game_root: String) -> Reply<Settings> {
    blocking(backend, move |b| b.remove_installation(&game_root)).await
}

/// "Open folder": a place by name, or a game folder from the installations list.
#[tauri::command]
pub async fn open_folder(
    backend: Shared<'_>,
    places: State<'_, Places>,
    place: Place,
) -> Reply<()> {
    let places = places.inner().clone();
    blocking(backend, move |b| {
        let known: Vec<String> = match &place {
            Place::Game { .. } => b
                .installations()?
                .into_iter()
                .map(|i| i.game_root)
                .collect(),
            _ => Vec::new(),
        };
        crate::setup::open(&crate::setup::folder_of(&place, &places, &known)?)
    })
    .await
}

#[tauri::command]
pub async fn about(backend: Shared<'_>, places: State<'_, Places>) -> Reply<About> {
    let places = places.inner().clone();
    blocking(backend, move |_| {
        Ok(crate::setup::about(&places, tauri::webview_version().ok()))
    })
    .await
}

/// D7: what Rollback would remove from the selected profile's game folder.
#[tauri::command]
pub async fn rollback_preview(backend: Shared<'_>) -> Reply<RollbackPreview> {
    blocking(backend, move |b| b.rollback_preview()).await
}

/// D7 / F11.
#[tauri::command]
pub async fn rollback(
    backend: Shared<'_>,
    log: Log<'_>,
    remove_foreign: bool,
) -> Reply<RolledBack> {
    let log = Arc::clone(log.inner());
    blocking(backend, move |b| {
        let done = b.rollback(remove_foreign);
        match &done {
            Ok(d) => app_log(
                &log,
                Level::Info,
                "game folder rolled back",
                serde_json::json!({
                    "game_root": d.game_root,
                    "removed": d.removed,
                    "foreign_removed": d.foreign_removed,
                    "gameinfo_restored": d.gameinfo_restored,
                }),
            ),
            Err(m) => app_log(
                &log,
                Level::Warn,
                "rollback stopped",
                serde_json::json!({ "message": m }),
            ),
        }
        done
    })
    .await
}

/// D8: what deleting all data removes.
#[tauri::command]
pub async fn wipe_summary(
    backend: Shared<'_>,
    places: State<'_, Places>,
) -> Reply<deadloader_core::wipe::Summary> {
    let data = places.data.clone();
    blocking(backend, move |b| b.wipe_summary(&data)).await
}

/// D8 confirmed: rolls every game folder back first when asked, marks the data folder for deletion and closes the
/// window. The folder is deleted when the app has exited (the window engine holds files in it until then); whatever
/// is still locked is finished at the next start.
#[tauri::command]
pub async fn delete_all_data(
    window: tauri::WebviewWindow,
    backend: Shared<'_>,
    log: Log<'_>,
    places: State<'_, Places>,
    wipe: State<'_, Arc<WipeOnExit>>,
    rollback: bool,
) -> Reply<Vec<RolledBack>> {
    let (log, data, wipe) = (
        Arc::clone(log.inner()),
        places.data.clone(),
        Arc::clone(wipe.inner()),
    );
    let done = blocking(backend, move |b| {
        let rolled = if rollback {
            b.rollback_all()?
        } else {
            Vec::new()
        };
        deadloader_core::wipe::mark_pending(&data).map_err(|e| from_io(&e))?;
        wipe.arm();
        app_log(
            &log,
            Level::Info,
            "all deadloader data will be deleted when the app exits",
            serde_json::json!({
                "data_folder": data.display().to_string(),
                "rolled_back": rolled.iter().map(|r| r.game_root.clone()).collect::<Vec<_>>(),
            }),
        );
        Ok(rolled)
    })
    .await?;
    let _ = window.close();
    Ok(done)
}

/// The setup step's checks (S1 step 6).
#[tauri::command]
pub async fn setup_check(backend: Shared<'_>, places: State<'_, Places>) -> Reply<SetupCheck> {
    let places = places.inner().clone();
    blocking(backend, move |b| {
        crate::setup::setup_check(
            b,
            &places,
            deadloader_platform_windows::folders::downloads().as_deref(),
            deadloader_platform_windows::folders::desktop().as_deref(),
        )
    })
    .await
}

/// The first-run notice acknowledged (107–111): stored and logged.
#[tauri::command]
pub async fn acknowledge_notice(
    backend: Shared<'_>,
    log: Log<'_>,
    version: u32,
    language: String,
    catalog: CatalogChoice,
    check_app_updates: bool,
) -> Reply<Settings> {
    let log = Arc::clone(log.inner());
    blocking(backend, move |b| {
        let lang = Lang::from_code(&language).unwrap_or(Lang::En);
        let saved = b.acknowledge_notice(version, lang, catalog, check_app_updates)?;
        app_log(
            &log,
            Level::Info,
            "first-run notice acknowledged",
            serde_json::json!({
                "version": version,
                "language": lang.code(),
                "catalog": catalog,
                "check_app_updates": saved.general.check_app_updates,
            }),
        );
        Ok(saved)
    })
    .await
}

/// F13: the catalog on (after its network step) or off (local mode); logged.
#[tauri::command]
pub async fn choose_catalog(backend: Shared<'_>, log: Log<'_>, enabled: bool) -> Reply<Settings> {
    let log = Arc::clone(log.inner());
    blocking(backend, move |b| {
        let saved = b.choose_catalog(enabled)?;
        let msg = if enabled {
            "catalog turned on"
        } else {
            "catalog turned off: local mode"
        };
        app_log(&log, Level::Info, msg, serde_json::json!({}));
        Ok(saved)
    })
    .await
}

/// How the app started: the UI shows E1 instead of everything else when the data folder can't be written.
#[tauri::command]
pub fn startup_state(state: State<'_, crate::setup::Startup>) -> crate::setup::Startup {
    state.inner().clone()
}

/// E1's "Open folder": the folder of deadloader.exe, the only thing that can be opened while blocked.
#[tauri::command]
pub fn blocked_open_folder(state: State<'_, crate::setup::Startup>) -> Reply<()> {
    match state.inner() {
        crate::setup::Startup::NoWrite { folder, .. } => {
            crate::setup::open(std::path::Path::new(folder))
        }
        crate::setup::Startup::Ready => Ok(()),
    }
}
