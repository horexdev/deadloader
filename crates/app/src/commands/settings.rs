//! Settings (S8).

use super::*;

#[tauri::command]
pub async fn settings(backend: Shared<'_>) -> Reply<Settings> {
    blocking(backend, move |b| b.settings()).await
}

#[tauri::command]
pub async fn update_settings(backend: Shared<'_>, settings: Settings) -> Reply<Settings> {
    blocking(backend, move |b| b.update_settings(settings)).await
}
