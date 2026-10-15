//! The commands the UI calls (`invoke`). Each one hands over to [`Backend`] on a worker thread, so the window stays
//! responsive while files are copied; errors reach the UI as `{ id, args }` messages it translates.

use crate::backend::{ApplyResult, Backend, Changed, Game};
use crate::catalog::{self, CatalogSlot, Failure};
use crate::events::{Outbox, Sink, UiEvent};
use deadloader_catalog::{
    CatalogError, Categories, ConsentError, ConsentPrompt, Granted, Listing, ModPage, Query, Scope,
    Section, Status,
};
use deadloader_core::fs::Disk;
use deadloader_core::message::MessageId;
use deadloader_core::message::UserMessage;
use deadloader_core::remove::{RemovalPlan, Removed};
use deadloader_core::session::{ImportOutcome, LibraryView, ProfileImported};
use deadloader_core::settings::Settings;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::State;
use tauri::ipc::Channel;

type Reply<T> = Result<T, UserMessage>;
type Shared<'a> = State<'a, Arc<Backend>>;

/// Runs `work` on a blocking thread: file copies and hashing must not hold up the async runtime or the window.
async fn blocking<T: Send + 'static>(
    backend: Shared<'_>,
    work: impl FnOnce(&Backend) -> Reply<T> + Send + 'static,
) -> Reply<T> {
    let backend = Arc::clone(backend.inner());
    tauri::async_runtime::spawn_blocking(move || work(&backend))
        .await
        .map_err(|e| UserMessage::new(MessageId::Internal).arg("detail", e))?
}

/// The UI's channel for pushes (download progress, the game starting and closing, dropped files, start notices).
/// Called once after the page loads, and again after a reload; the newest channel wins.
#[tauri::command]
pub fn subscribe(outbox: State<'_, Arc<Outbox>>, events: Channel<UiEvent>) {
    outbox.connect(Box::new(ChannelSink(events)));
}

struct ChannelSink(Channel<UiEvent>);

impl Sink for ChannelSink {
    fn deliver(&self, event: &UiEvent) -> bool {
        self.0.send(event.clone()).is_ok()
    }
}

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

#[tauri::command]
pub async fn settings(backend: Shared<'_>) -> Reply<Settings> {
    blocking(backend, move |b| b.settings()).await
}

#[tauri::command]
pub async fn update_settings(backend: Shared<'_>, settings: Settings) -> Reply<Settings> {
    blocking(backend, move |b| b.update_settings(settings)).await
}

type Slot<'a> = State<'a, Arc<CatalogSlot>>;

/// Runs a catalog call on a blocking thread: it may wait for the catalog to open and for GameBanana to answer.
async fn on_catalog<T, E>(
    slot: Slot<'_>,
    work: impl FnOnce(&CatalogSlot) -> Result<T, E> + Send + 'static,
) -> Result<T, E>
where
    T: Send + 'static,
    E: From<UserMessage> + Send + 'static,
{
    let slot = Arc::clone(slot.inner());
    tauri::async_runtime::spawn_blocking(move || work(&slot))
        .await
        .map_err(|e| E::from(UserMessage::new(MessageId::Internal).arg("detail", e)))?
}

/// A catalog page; `by_user` when the user asked for it (scrolling to it, searching), not a background load.
#[tauri::command]
pub async fn catalog_list(
    slot: Slot<'_>,
    query: Query,
    by_user: bool,
) -> Result<Listing, Failure<CatalogError>> {
    on_catalog(slot, move |c| catalog::list(c, &query, by_user)).await
}

/// Pull to refresh / Retry: the page from GameBanana now.
#[tauri::command]
pub async fn catalog_refresh(
    slot: Slot<'_>,
    query: Query,
) -> Result<Listing, Failure<CatalogError>> {
    on_catalog(slot, move |c| catalog::refresh(c, &query)).await
}

/// The next page at ~70 % scroll (28); `false` when nothing was fetched.
#[tauri::command]
pub async fn catalog_prefetch(slot: Slot<'_>, query: Query) -> Reply<bool> {
    on_catalog(slot, move |c| catalog::prefetch(c, &query)).await
}

#[tauri::command]
pub async fn catalog_categories(
    slot: Slot<'_>,
    section: Section,
    parent: Option<u64>,
) -> Result<Categories, Failure<CatalogError>> {
    on_catalog(slot, move |c| catalog::categories(c, section, parent)).await
}

/// A mod page; `listed_updated` is the update date the card showed (Unix seconds), to tell a saved page is current.
#[tauri::command]
pub async fn catalog_details(
    slot: Slot<'_>,
    section: Section,
    id: u64,
    listed_updated: Option<i64>,
) -> Result<ModPage, Failure<CatalogError>> {
    on_catalog(slot, move |c| {
        catalog::details(c, section, id, listed_updated)
    })
    .await
}

/// The global network indicator's state; changes arrive as pushes afterwards.
#[tauri::command]
pub async fn catalog_status(slot: Slot<'_>) -> Reply<Status> {
    on_catalog(slot, catalog::status).await
}

/// "Retry now" in E5.
#[tauri::command]
pub async fn catalog_retry_now(slot: Slot<'_>) -> Reply<()> {
    on_catalog(slot, catalog::retry_now).await
}

/// A click on a description image (29).
#[tauri::command]
pub async fn catalog_activate(slot: Slot<'_>, id: String) -> Result<(), Failure<ConsentError>> {
    on_catalog(slot, move |c| catalog::activate(c, &id)).await
}

/// What dialog D2 shows for an image from a host not allowed yet.
#[tauri::command]
pub async fn catalog_consent_prompt(
    slot: Slot<'_>,
    id: String,
) -> Result<ConsentPrompt, Failure<ConsentError>> {
    on_catalog(slot, move |c| catalog::consent_prompt(c, &id)).await
}

/// The user's answer in D2 (122).
#[tauri::command]
pub async fn catalog_grant(
    slot: Slot<'_>,
    id: String,
    scope: Scope,
) -> Result<Granted, Failure<ConsentError>> {
    on_catalog(slot, move |c| catalog::grant(c, &id, scope)).await
}

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
        let file = c
            .wait()?
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

/// The user's Windows languages, best first (the "system" UI language, 179), and the regional format of times and
/// numbers (178). Empty or `None` when Windows can't tell; the UI then uses English and its own defaults.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SystemLocale {
    pub languages: Vec<String>,
    pub region: Option<String>,
}

#[tauri::command]
pub fn system_locale() -> SystemLocale {
    SystemLocale {
        languages: deadloader_platform_windows::locale::preferred_ui_languages()
            .unwrap_or_default(),
        region: deadloader_platform_windows::locale::regional_format().ok(),
    }
}

/// The texts of a system file dialog, translated by the UI (no user-visible English in Rust).
#[derive(Debug, Clone, serde::Deserialize)]
pub struct DialogText {
    pub title: String,
    /// The name of the file-type filter, e.g. "Mod files".
    pub filter: String,
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

/// Deadlock on this PC, found through Steam.
pub struct SteamGame;

impl Game for SteamGame {
    fn running(&self) -> bool {
        // If the process list can't be read, assume the game runs: waiting is safe, writing into a running game's
        // folder is not.
        deadloader_platform_windows::process::game_running().unwrap_or(true)
    }

    fn move_to_recycle_bin(&self, path: &std::path::Path) -> std::io::Result<()> {
        deadloader_platform_windows::recycle::move_to_recycle_bin(path)
    }

    fn installations(&self) -> Vec<PathBuf> {
        match deadloader_platform_windows::steam::steam_root() {
            Ok(Some(root)) => deadloader_core::steam::find_installations(&root, &Disk)
                .installations
                .into_iter()
                .map(|i| i.game_root)
                .collect(),
            _ => Vec::new(),
        }
    }
}
