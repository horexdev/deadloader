//! Activity (S7): the logs as pages, totals and today's summary, and "open logs folder". The live view hears of new
//! lines through the push channel (`UiEvent::Log`).

use super::*;
use crate::activity::Activity;
use deadloader_core::activity::{Counts, Entry, Page, Query, Range, Summary};
use deadloader_core::log::{DiskLogDir, SystemClock};

pub type SharedActivity = Arc<Activity<SystemClock, DiskLogDir>>;
type Act<'a> = State<'a, SharedActivity>;

/// "Hide the Windows user name" (94); on when the settings can't be read.
fn masking(backend: &Backend) -> bool {
    backend.settings().map_or(true, |s| s.logs.mask_user_path)
}

/// A page of the list, newest first; pass the returned cursor back for the next one.
#[tauri::command]
pub async fn log_page(backend: Shared<'_>, activity: Act<'_>, query: Query) -> Reply<Page> {
    let activity = Arc::clone(activity.inner());
    blocking(backend, move |b| activity.page(&query, masking(b))).await
}

/// One entry in full: for a network line or request id, the whole request.
#[tauri::command]
pub async fn log_entry(backend: Shared<'_>, activity: Act<'_>, id: String) -> Reply<Option<Entry>> {
    let activity = Arc::clone(activity.inner());
    blocking(backend, move |b| activity.entry(&id, masking(b))).await
}

/// Each tab's totals over a range (tab counts and the filters' numbers).
#[tauri::command]
pub async fn log_counts(backend: Shared<'_>, activity: Act<'_>, range: Range) -> Reply<Counts> {
    let activity = Arc::clone(activity.inner());
    blocking(backend, move |_| activity.counts(range)).await
}

/// Today at a glance: hosts contacted, requests blocked, changes in the game folder.
#[tauri::command]
pub async fn log_summary(backend: Shared<'_>, activity: Act<'_>) -> Reply<Summary> {
    let activity = Arc::clone(activity.inner());
    blocking(backend, move |b| activity.summary(masking(b))).await
}

/// Opens deadloader's logs folder in File Explorer; returns the folder as the UI may show it.
#[tauri::command]
pub async fn open_logs_folder(backend: Shared<'_>, activity: Act<'_>) -> Reply<String> {
    let activity = Arc::clone(activity.inner());
    blocking(backend, move |b| {
        activity.open_folder(masking(b), deadloader_platform_windows::shell::open_folder)
    })
    .await
}

/// A mistake of the UI itself (an exception handed to the error toast or D12): written to the app log with its stack;
/// returns the entry's id for D12's Details, `None` when it couldn't be written.
#[tauri::command]
pub async fn log_ui_error(
    backend: Shared<'_>,
    error: String,
    stack: Option<String>,
) -> Reply<Option<String>> {
    blocking(backend, move |b| Ok(b.ui_error(&error, stack.as_deref()))).await
}
