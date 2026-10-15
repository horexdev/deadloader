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

/// The Profiles screen (S5): every profile with its game folder, enabled mods and when it was applied.
#[tauri::command]
pub async fn profiles_overview(backend: Shared<'_>) -> Reply<crate::backend::ProfilesView> {
    blocking(backend, move |b| b.profiles_view()).await
}

/// "Change game installation" (153): the known game folder Apply and Play use for the profile.
#[tauri::command]
pub async fn set_profile_installation(
    backend: Shared<'_>,
    name: String,
    root: String,
) -> Reply<crate::backend::ProfilesView> {
    blocking(backend, move |b| b.set_profile_installation(&name, &root)).await
}

/// What "Download missing" in D9 started, and the mods that couldn't be queued.
#[derive(Debug, Clone, serde::Serialize)]
pub struct MissingStarted {
    pub tasks: Vec<deadloader_downloads::TaskId>,
    pub failed: Vec<crate::downloads::NotStarted>,
}

/// "Download missing" in D9 (37): the GameBanana files an imported profile names; the mod page is fetched first when
/// it isn't saved. Each download joins the selected profile (the imported one) when it is installed (136).
#[tauri::command]
pub async fn profile_download_missing(
    slot: Slot<'_>,
    downloads: State<'_, Arc<deadloader_downloads::Downloads>>,
    mods: Vec<crate::downloads::MissingMod>,
) -> Reply<MissingStarted> {
    let downloads = Arc::clone(downloads.inner());
    on_catalog(slot, move |c| {
        let catalog = c.wait()?;
        let (tasks, mut failed) = crate::downloads::tasks_for_missing(
            &mods,
            |section, mod_id, file_id| {
                let section = super::updates::catalog_section(section);
                if catalog.remote_file(section, mod_id, file_id).is_none() {
                    catalog
                        .details(section, mod_id, None, deadloader_catalog::Initiator::User)
                        .map_err(|e| super::updates::catalog_message(&e))?;
                }
                Ok(catalog.remote_file(section, mod_id, file_id))
            },
            |section, mod_id| {
                catalog
                    .download_thumbnail(super::updates::catalog_section(section), mod_id)
                    .map(|id| id.to_string())
            },
        );
        let mut started = Vec::new();
        for task in tasks {
            let name = task.meta.get("mod_name").cloned().unwrap_or_default();
            match downloads.add(task) {
                Ok(id) => started.push(id),
                Err(e) => failed.push(crate::downloads::NotStarted {
                    name,
                    message: deadloader_core::message::ToUserMessage::user_message(&e),
                }),
            }
        }
        Ok(MissingStarted {
            tasks: started,
            failed,
        })
    })
    .await
}
