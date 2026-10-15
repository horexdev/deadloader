//! The catalog in the app: opened off the UI thread after the window, its events pushed to the UI, its "always allow
//! this host" choices saved through the backend, and the commands the UI calls.

use crate::backend::Backend;
use crate::events::{Outbox, UiEvent};
use deadloader_catalog::{
    Catalog, CatalogError, Categories, ConsentError, ConsentPrompt, Event, Events, Granted,
    ImageId, Initiator, Listing, ModPage, Parts, Query, Scope, Section, SettingsSink, Status,
    Threads,
};
use deadloader_core::cache::{self, Cache, DiskCacheFs};
use deadloader_core::log::SystemClock;
use deadloader_core::message::{MessageId, UserMessage};
use deadloader_core::session::SharedLog;
use deadloader_core::settings::Settings;
use deadloader_net::Client;
use serde::Serialize;
use std::path::Path;
use std::sync::{Arc, Condvar, Mutex, MutexGuard, PoisonError, Weak};

pub type AppCatalog = Catalog<DiskCacheFs, SystemClock, SharedLog>;

/// The catalog once it is open. Commands wait for it; it opens within moments of the start.
#[derive(Default)]
pub struct CatalogSlot {
    state: Mutex<Option<Result<AppCatalog, UserMessage>>>,
    ready: Condvar,
}

impl CatalogSlot {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&self, catalog: Result<AppCatalog, UserMessage>) {
        *self.lock() = Some(catalog);
        self.ready.notify_all();
    }

    /// The catalog, or why it could not be opened; waits until it has been tried.
    pub fn wait(&self) -> Result<AppCatalog, UserMessage> {
        let mut state = self.lock();
        loop {
            if let Some(result) = state.as_ref() {
                return result.clone();
            }
            state = self
                .ready
                .wait(state)
                .unwrap_or_else(PoisonError::into_inner);
        }
    }

    /// The catalog if it is open already.
    #[cfg(test)]
    pub fn get(&self) -> Option<AppCatalog> {
        self.lock().as_ref().and_then(|r| r.as_ref().ok().cloned())
    }

    fn lock(&self) -> MutexGuard<'_, Option<Result<AppCatalog, UserMessage>>> {
        self.state.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

/// Catalog events go to the UI through the push channel.
struct UiEvents(Arc<Outbox>);

impl Events for UiEvents {
    fn notify(&self, event: Event) {
        self.0.send(UiEvent::Catalog(event));
    }
}

/// "Always allow images from this host" is saved like any other settings change, so every listener (the catalog
/// itself, the network client) hears about it.
struct SaveThroughBackend(Weak<Backend>);

impl SettingsSink for SaveThroughBackend {
    fn allow_image_host(&self, host: &str) -> Result<(), UserMessage> {
        let backend = self.0.upgrade().ok_or_else(|| {
            UserMessage::new(MessageId::Internal).arg("detail", "the app is closing")
        })?;
        let mut settings = backend.settings()?;
        let hosts = &mut settings.catalog.allowed_image_hosts;
        if !hosts.iter().any(|h| h.eq_ignore_ascii_case(host)) {
            hosts.push(host.to_owned());
        }
        backend.update_settings(settings).map(drop)
    }
}

/// Opens the cache (it lists every image folder, so never on the UI thread) and the catalog on it.
pub fn open(
    cache_dir: &Path,
    settings: &Settings,
    net: Arc<Client>,
    files: SharedLog,
    log: Arc<dyn deadloader_catalog::AppLog>,
    backend: &Arc<Backend>,
    outbox: Arc<Outbox>,
) -> Result<AppCatalog, UserMessage> {
    let cache = Cache::open(
        cache_dir,
        DiskCacheFs,
        SystemClock,
        files,
        cache::Settings::default(),
    )
    .map_err(|e| deadloader_core::message::from_io(&e, deadloader_core::message::Area::Data))?;
    Ok(Catalog::new(Parts {
        net,
        cache,
        clock: SystemClock,
        settings,
        settings_sink: Arc::new(SaveThroughBackend(Arc::downgrade(backend))),
        spawn: Arc::new(Threads),
        events: Arc::new(UiEvents(outbox)),
        log,
    }))
}

/// A catalog call's failure: the catalog's own answer (a state the UI shows in place, or a message), or a message when
/// the catalog itself is unavailable. The two are told apart by `kind` vs `id`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum Failure<E> {
    Catalog(E),
    Message(UserMessage),
}

impl<E> From<UserMessage> for Failure<E> {
    fn from(message: UserMessage) -> Self {
        Self::Message(message)
    }
}

type Reply<T, E> = Result<T, Failure<E>>;

fn parse_id<E>(id: &str) -> Reply<ImageId, E> {
    ImageId::parse(id).ok_or_else(|| {
        Failure::Message(UserMessage::new(MessageId::Internal).arg("detail", "not an image id"))
    })
}

fn initiator(by_user: bool) -> Initiator {
    if by_user {
        Initiator::User
    } else {
        Initiator::Auto
    }
}

/// The calls behind the catalog commands, on a blocking thread each.
pub fn list(slot: &CatalogSlot, query: &Query, by_user: bool) -> Reply<Listing, CatalogError> {
    slot.wait()?
        .list(query, initiator(by_user))
        .map_err(Failure::Catalog)
}

pub fn refresh(slot: &CatalogSlot, query: &Query) -> Reply<Listing, CatalogError> {
    slot.wait()?
        .refresh(query, Initiator::User)
        .map_err(Failure::Catalog)
}

pub fn prefetch(slot: &CatalogSlot, query: &Query) -> Result<bool, UserMessage> {
    Ok(slot.wait()?.prefetch(query))
}

pub fn categories(
    slot: &CatalogSlot,
    section: Section,
    parent: Option<u64>,
) -> Reply<Categories, CatalogError> {
    slot.wait()?
        .categories(section, parent)
        .map_err(Failure::Catalog)
}

pub fn details(
    slot: &CatalogSlot,
    section: Section,
    id: u64,
    listed_updated: Option<i64>,
) -> Reply<ModPage, CatalogError> {
    slot.wait()?
        .details(section, id, listed_updated, Initiator::User)
        .map_err(Failure::Catalog)
}

pub fn status(slot: &CatalogSlot) -> Result<Status, UserMessage> {
    Ok(slot.wait()?.status())
}

pub fn retry_now(slot: &CatalogSlot) -> Result<(), UserMessage> {
    slot.wait()?.retry_now();
    Ok(())
}

pub fn activate(slot: &CatalogSlot, id: &str) -> Reply<(), ConsentError> {
    let id = parse_id(id)?;
    slot.wait()?.activate(&id).map_err(Failure::Catalog)
}

pub fn consent_prompt(slot: &CatalogSlot, id: &str) -> Reply<ConsentPrompt, ConsentError> {
    let id = parse_id(id)?;
    slot.wait()?.consent_prompt(&id).map_err(Failure::Catalog)
}

pub fn grant(slot: &CatalogSlot, id: &str, scope: Scope) -> Reply<Granted, ConsentError> {
    let id = parse_id(id)?;
    slot.wait()?.grant(&id, scope).map_err(Failure::Catalog)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::tests::{FakeGame, Fixture};
    use crate::events::tests::Recorder;
    use deadloader_core::log::MemFileLog;
    use deadloader_net::Allowlist;
    use deadloader_net::transport::{HopRequest, HopResponse, Transport, TransportError};
    use std::time::Duration;

    /// Fails every request: nothing in these tests may reach the internet.
    struct NoNetwork;

    impl Transport for NoNetwork {
        fn send(&self, _: &HopRequest<'_>) -> Result<HopResponse, TransportError> {
            Err(TransportError::new(
                deadloader_net::transport::TransportErrorKind::Connect,
                "tests make no requests",
            ))
        }
    }

    struct NoLog;

    impl deadloader_net::log::NetLog for NoLog {
        fn record(&self, _: &deadloader_net::log::HopRecord) {}
    }

    impl deadloader_catalog::AppLog for NoLog {
        fn write(
            &self,
            _: deadloader_core::log::Level,
            _: &str,
            _: serde_json::Map<String, serde_json::Value>,
        ) -> Option<String> {
            None
        }
    }

    fn client() -> Arc<Client> {
        Arc::new(Client::new(
            NoNetwork,
            NoLog,
            deadloader_net::clock::SystemClock,
            Allowlist::new(),
        ))
    }

    fn opened(f: &Fixture, backend: &Arc<Backend>, outbox: Arc<Outbox>) -> AppCatalog {
        open(
            &f.dir.join("deadloader-data").join("cache"),
            &backend.settings().expect("settings"),
            client(),
            Arc::new(MemFileLog::default()),
            Arc::new(NoLog),
            backend,
            outbox,
        )
        .expect("opened")
    }

    #[test]
    fn calls_wait_for_the_catalog_to_open() {
        let f = Fixture::new("slot");
        let backend = Arc::new(f.backend(&FakeGame::default()));
        let slot = Arc::new(CatalogSlot::new());
        assert!(slot.get().is_none());
        let waiting = {
            let slot = Arc::clone(&slot);
            std::thread::spawn(move || status(&slot))
        };
        std::thread::sleep(Duration::from_millis(50));
        slot.set(Ok(opened(&f, &backend, Arc::new(Outbox::new()))));
        assert_eq!(waiting.join().expect("joined"), Ok(Status::LocalMode));
        assert!(slot.get().is_some());
    }

    #[test]
    fn a_catalog_that_could_not_open_answers_with_the_reason() {
        let slot = CatalogSlot::new();
        slot.set(Err(UserMessage::new(MessageId::E1)));
        let err = list(
            &slot,
            &Query::browse(Section::Mod, deadloader_catalog::Sort::Newest, None, 1),
            true,
        )
        .expect_err("unavailable");
        assert_eq!(err, Failure::Message(UserMessage::new(MessageId::E1)));
        assert_eq!(
            serde_json::to_value(&err).expect("json"),
            serde_json::json!({"id": "E1", "args": {}})
        );
    }

    #[test]
    fn in_local_mode_listings_say_so_and_nothing_is_requested() {
        let f = Fixture::new("catalogoff");
        let backend = Arc::new(f.backend(&FakeGame::default()));
        let slot = CatalogSlot::new();
        slot.set(Ok(opened(&f, &backend, Arc::new(Outbox::new()))));
        let query = Query::browse(Section::Mod, deadloader_catalog::Sort::Newest, None, 1);
        let err = list(&slot, &query, true).expect_err("local mode");
        assert_eq!(err, Failure::Catalog(CatalogError::LocalMode));
        assert_eq!(
            serde_json::to_value(&err).expect("json"),
            serde_json::json!({"kind": "local_mode"})
        );
    }

    #[test]
    fn catalog_events_reach_the_ui() {
        let f = Fixture::new("catalogevents");
        let backend = Arc::new(f.backend(&FakeGame::default()));
        let outbox = Arc::new(Outbox::new());
        let ui = Recorder::default();
        outbox.connect(Box::new(ui.clone()));
        let catalog = opened(&f, &backend, Arc::clone(&outbox));
        let mut settings = backend.settings().expect("settings");
        settings.catalog.enabled = true;
        catalog.apply_settings(&settings);
        assert_eq!(ui.take(), [UiEvent::Catalog(Event::Status(Status::Online))]);
    }

    #[test]
    fn an_always_allowed_host_is_saved_in_the_settings() {
        let f = Fixture::new("allowhost");
        let backend = Arc::new(f.backend(&FakeGame::default()));
        let sink = SaveThroughBackend(Arc::downgrade(&backend));
        sink.allow_image_host("i.imgur.com").expect("saved");
        sink.allow_image_host("i.imgur.com").expect("again");
        assert_eq!(
            backend
                .settings()
                .expect("settings")
                .catalog
                .allowed_image_hosts,
            ["i.imgur.com"]
        );
    }

    #[test]
    fn image_ids_from_the_ui_are_checked() {
        let slot = CatalogSlot::new();
        assert!(matches!(
            activate(&slot, "../settings.json"),
            Err(Failure::Message(m)) if m.id == MessageId::Internal
        ));
    }

    #[test]
    fn queries_come_from_the_ui_as_json() {
        let query: Query = serde_json::from_value(serde_json::json!({
            "section": "mod",
            "filter": {"kind": "search", "text": "haze"},
            "page": 2
        }))
        .expect("query");
        assert_eq!(query, Query::search(Section::Mod, "haze", 2));
    }
}
