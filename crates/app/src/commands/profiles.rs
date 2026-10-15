//! Profiles (S5): selecting, managing, sharing them as files.

use super::*;

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
