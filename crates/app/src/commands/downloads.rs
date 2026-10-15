//! The download list (S6) and Install (F2).

use super::*;

/// Install (F2): the file of a mod page the UI showed becomes a download. `name` and `author` are what the page showed.
#[tauri::command]
pub async fn download_install(
    slot: Slot<'_>,
    downloads: State<'_, Arc<deadloader_downloads::Downloads>>,
    section: Section,
    mod_id: u64,
    file_id: u64,
    name: String,
    author: Option<String>,
) -> Reply<deadloader_downloads::TaskId> {
    let downloads = Arc::clone(downloads.inner());
    on_catalog(slot, move |c| {
        let catalog = c.wait()?;
        let file = catalog
            .remote_file(section, mod_id, file_id)
            .ok_or_else(|| {
                UserMessage::new(MessageId::Internal).arg(
                    "detail",
                    format!("file {file_id} of mod {mod_id} is not on a saved mod page"),
                )
            })?;
        let of = crate::downloads::ModRef {
            section: match section {
                Section::Mod => deadloader_core::library::Section::Mod,
                Section::Sound => deadloader_core::library::Section::Sound,
            },
            mod_id,
            name,
            author,
            thumbnail: catalog
                .download_thumbnail(section, mod_id)
                .map(|id| id.to_string()),
        };
        crate::downloads::install(&downloads, &file, &of)
    })
    .await
}

/// The download list (S6); changes arrive as pushes afterwards.
#[tauri::command]
pub fn downloads(
    downloads: State<'_, Arc<deadloader_downloads::Downloads>>,
) -> Vec<deadloader_downloads::TaskSnapshot> {
    downloads.tasks()
}

/// Pause, Resume, Retry or Cancel on one download.
#[tauri::command]
pub fn download_control(
    downloads: State<'_, Arc<deadloader_downloads::Downloads>>,
    id: deadloader_downloads::TaskId,
    action: DownloadAction,
) -> Reply<()> {
    let result = match action {
        DownloadAction::Pause => downloads.pause(id),
        DownloadAction::Resume => downloads.resume(id),
        DownloadAction::Retry => downloads.retry(id),
        DownloadAction::Cancel => downloads.cancel(id),
    };
    result.map_err(|e| deadloader_core::message::ToUserMessage::user_message(&e))
}

#[derive(Debug, Clone, Copy, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DownloadAction {
    Pause,
    Resume,
    Retry,
    Cancel,
}

/// E11's "Open folder": File Explorer with the download's kept archive selected. The core finds the file by the task;
/// the UI never names a path. Logged as an external program (ADR 0009).
#[tauri::command]
pub async fn download_show_archive(
    backend: Shared<'_>,
    log: State<'_, super::library::AppLog>,
    downloads: State<'_, Arc<deadloader_downloads::Downloads>>,
    id: deadloader_downloads::TaskId,
) -> Reply<()> {
    let archive = downloads.archive(id).ok_or_else(|| {
        UserMessage::new(MessageId::Internal)
            .arg("detail", format!("download {id} keeps no archive"))
    })?;
    let log = Arc::clone(log.inner());
    blocking(backend, move |b| {
        let result = b.show_file(archive);
        super::library::log_shell(&log, &result);
        result.map(|_| ())
    })
    .await
}
