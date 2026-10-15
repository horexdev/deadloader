//! The window frame and the sidebar: the custom title bar's buttons and drag area, and Play (F4).

use super::*;
use deadloader_core::log::{DiskLogDir, Level, Logger, SystemClock};

/// Play: applies the selected profile, then Steam starts Deadlock. Logged as an external program (ADR 0009).
#[tauri::command]
pub async fn play(
    backend: Shared<'_>,
    log: State<'_, Arc<Logger<SystemClock, DiskLogDir>>>,
) -> Reply<ApplyResult> {
    let log = Arc::clone(log.inner());
    blocking(backend, move |b| {
        let result = b.play();
        let fields = serde_json::Map::from_iter([(
            "url".to_owned(),
            deadloader_platform_windows::shell::DEADLOCK_RUN_URL.into(),
        )]);
        let (lvl, msg) = if result.is_ok() {
            (Level::Info, "Deadlock started through Steam")
        } else {
            (Level::Warn, "Play stopped before Deadlock was started")
        };
        let _ = log.app(lvl, msg, fields);
        result
    })
    .await
}

/// D13: the mods Apply or Play would take out of the game (by name).
#[tauri::command]
pub async fn turned_off_by_apply(backend: Shared<'_>) -> Reply<Vec<String>> {
    blocking(backend, move |b| b.turned_off_by_apply()).await
}

/// The title bar's buttons: our window has no system frame (the design's title bar).
#[tauri::command]
pub fn window_minimize(window: tauri::WebviewWindow) {
    let _ = window.minimize();
}

#[tauri::command]
pub fn window_toggle_maximize(window: tauri::WebviewWindow) {
    let maximized = window.is_maximized().unwrap_or(false);
    let _ = if maximized {
        window.unmaximize()
    } else {
        window.maximize()
    };
}

#[tauri::command]
pub fn window_close(window: tauri::WebviewWindow) {
    let _ = window.close();
}

/// A press on the title bar's empty area moves the window (no `core:window` permission is granted, so Tauri's own
/// drag region can't be used).
#[tauri::command]
pub fn window_start_drag(window: tauri::WebviewWindow) {
    let _ = window.start_dragging();
}
