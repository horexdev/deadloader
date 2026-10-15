//! The commands the UI calls (`invoke`). Each one hands over to [`Backend`] on a worker thread, so the window stays
//! responsive while files are copied; errors reach the UI as `{ id, args }` messages it translates.

use crate::backend::{ApplyResult, Backend, Changed};
use crate::catalog::{CatalogSlot, Failure};
use crate::events::{Outbox, Sink, UiEvent};
use deadloader_catalog::{
    CatalogError, Categories, ConsentError, ConsentPrompt, Granted, Listing, ModPage, Query, Scope,
    Section, Status,
};
use deadloader_core::message::MessageId;
use deadloader_core::message::UserMessage;
use deadloader_core::remove::{RemovalPlan, Removed};
use deadloader_core::session::{ImportOutcome, LibraryView, ProfileImported};
use deadloader_core::settings::Settings;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::State;
use tauri::ipc::Channel;

pub mod activity;
pub mod catalog;
pub mod downloads;
pub mod library;
pub mod profiles;
pub mod settings;
pub mod shell;
pub mod system;
pub mod updates;

type Reply<T> = Result<T, UserMessage>;
type Shared<'a> = State<'a, Arc<Backend>>;

/// Runs `work` on a blocking thread: file copies and hashing must not hold up the async runtime or the window.
async fn blocking<T: Send + 'static>(
    backend: Shared<'_>,
    work: impl FnOnce(&Backend) -> Reply<T> + Send + 'static,
) -> Reply<T> {
    let backend = Arc::clone(backend.inner());
    let reporter = Arc::clone(&backend);
    tauri::async_runtime::spawn_blocking(move || work(&backend))
        .await
        .map_err(|e| UserMessage::new(MessageId::Internal).arg("detail", e))
        .and_then(|reply| reply)
        // Every failure is in the app log, and its message names the entry (D12 links to it).
        .map_err(|message| reporter.reported(message))
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

/// The texts of a system file dialog, translated by the UI (no user-visible English in Rust).
#[derive(Debug, Clone, serde::Deserialize)]
pub struct DialogText {
    pub title: String,
    /// The name of the file-type filter, e.g. "Mod files".
    pub filter: String,
}
