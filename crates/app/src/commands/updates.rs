//! Mod update checks and installing updates (D5).

use super::*;

/// "Check for updates" (38): every GameBanana mod of the library against its file list on GameBanana.
#[tauri::command]
pub async fn updates_check(
    slot: Slot<'_>,
    backend: Shared<'_>,
) -> Reply<crate::updates::UpdateReport> {
    let backend = Arc::clone(backend.inner());
    on_catalog(slot, move |c| {
        let catalog = c.wait()?;
        let manifest = backend.manifest()?;
        Ok(crate::updates::check_all(
            &manifest,
            &catalog,
            deadloader_catalog::Initiator::User,
        ))
    })
    .await
}

/// "Hide" on "new files available" (168): these GameBanana files of the mod are not reported again.
#[tauri::command]
pub async fn updates_dismiss(backend: Shared<'_>, mod_id: String, file_ids: Vec<u64>) -> Reply<()> {
    blocking(backend, move |b| b.dismiss_new_files(&mod_id, &file_ids)).await
}

/// What "Update" in D5 started: downloads for the picked files not in the library yet, and the steps that could be
/// taken at once (kept files, replacements downloaded before). Downloads report their step as `update_step` pushes.
#[derive(Debug, Clone, serde::Serialize)]
pub struct UpdateStarted {
    pub tasks: Vec<deadloader_downloads::TaskId>,
    pub steps: Vec<crate::backend::UpdateStep>,
}

/// "Update" in D5 (39, 169): the user's pick for every replaced file of the mod.
#[tauri::command]
pub async fn update_install(
    slot: Slot<'_>,
    backend: Shared<'_>,
    downloads: State<'_, Arc<deadloader_downloads::Downloads>>,
    mod_id: String,
    picks: Vec<crate::updates::UpdatePick>,
) -> Reply<UpdateStarted> {
    let backend = Arc::clone(backend.inner());
    let downloads = Arc::clone(downloads.inner());
    on_catalog(slot, move |c| {
        let target = crate::updates::plan_picks(&backend.manifest()?, &mod_id, &picks)?;
        let mut started = UpdateStarted {
            tasks: Vec::new(),
            steps: Vec::new(),
        };
        for plan in target.plans.clone() {
            match plan {
                crate::updates::PickPlan::Keep { old_files } => {
                    let mappings = old_files
                        .into_iter()
                        .map(|old| deadloader_core::updates::Mapping { old, new: None })
                        .collect();
                    let changed = backend.confirm_update(&mod_id, mappings, None)?;
                    started.steps.push(crate::backend::UpdateStep::Applied {
                        mod_id: mod_id.clone(),
                        changed,
                    });
                }
                crate::updates::PickPlan::Present {
                    replaced,
                    new_files,
                } => started
                    .steps
                    .push(backend.update_with(&mod_id, replaced, new_files, None)?),
                crate::updates::PickPlan::Download { replaced, file } => {
                    let task = update_download(c, &target, replaced, file)?;
                    started.tasks.push(
                        downloads.add(task).map_err(|e| {
                            deadloader_core::message::ToUserMessage::user_message(&e)
                        })?,
                    );
                }
            }
        }
        Ok(started)
    })
    .await
}

/// The download task for one picked file; the mod page is fetched first when it isn't saved, for the file's URL.
fn update_download(
    c: &CatalogSlot,
    target: &crate::updates::UpdateTarget,
    replaced: u64,
    file: u64,
) -> Result<deadloader_downloads::NewTask, UserMessage> {
    let catalog = c.wait()?;
    let section = match target.section {
        deadloader_core::library::Section::Mod => Section::Mod,
        deadloader_core::library::Section::Sound => Section::Sound,
    };
    if catalog.remote_file(section, target.mod_id, file).is_none() {
        catalog
            .details(
                section,
                target.mod_id,
                None,
                deadloader_catalog::Initiator::User,
            )
            .map_err(|e| catalog_message(&e))?;
    }
    let remote = catalog
        .remote_file(section, target.mod_id, file)
        .ok_or_else(|| UserMessage::new(MessageId::E24).arg("detail", format!("file {file}")))?;
    let of = crate::downloads::ModRef {
        section: target.section,
        mod_id: target.mod_id,
        name: target.name.clone(),
        author: target.author.clone(),
    };
    Ok(crate::downloads::update_task_for(&remote, &of, replaced))
}

/// A catalog answer as the message a download-like action shows.
fn catalog_message(e: &CatalogError) -> UserMessage {
    if let Some(message) = e.message() {
        return message.clone();
    }
    match e {
        CatalogError::LocalMode => UserMessage::new(MessageId::E6).arg("reason", "local_mode"),
        CatalogError::Offline { .. } => UserMessage::new(MessageId::E6).arg("reason", "connect"),
        CatalogError::NotFound { .. } => UserMessage::new(MessageId::E24),
        _ => UserMessage::new(MessageId::Internal).arg("detail", e),
    }
}

/// One line of the user's mapping in D5: the old library file and the new one, or `None` to keep the old file.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct MappingArg {
    pub old: String,
    pub new: Option<String>,
}

/// The user's mapping in D5 after `needs_mapping`.
#[tauri::command]
pub async fn update_map(
    backend: Shared<'_>,
    mod_id: String,
    mappings: Vec<MappingArg>,
    version: Option<String>,
) -> Reply<Changed> {
    blocking(backend, move |b| {
        let mappings = mappings
            .into_iter()
            .map(|m| deadloader_core::updates::Mapping {
                old: m.old,
                new: m.new,
            })
            .collect();
        b.confirm_update(&mod_id, mappings, version)
    })
    .await
}
