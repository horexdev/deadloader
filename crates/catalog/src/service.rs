//! The catalog service itself; see the crate documentation for what it promises.

use crate::error::{CatalogError, ConsentError, ImageError, Status};
use crate::fetch::{Failure, NetFetch};
use crate::gate::Gate;
use crate::id::ImageId;
use crate::registry::{self, Entry, Kind, Registry};
use crate::view::{
    Access, Card, Categories, ConsentPrompt, Description, DescriptionImage, FileView, Filter,
    GalleryImage, Granted, LinkView, Listing, ModPage, Origin, PendingHost, Query, Scope,
    Thumbnail,
};
use deadloader_core::cache::{
    BlobCache, BlobLimits, Cache, CacheFs, CatalogLimits, FileLog, ImageType, Lookup, PageKey,
    PutError, Saved,
};
use deadloader_core::library;
use deadloader_core::log::{Clock, Level, LogDir, Logger};
use deadloader_core::message::{MessageId, UserMessage};
use deadloader_core::settings::{DescriptionImages, NsfwMode, Settings};
use deadloader_core::updates::{self, RemoteMod, RemovedReason};
use deadloader_gamebanana::{
    self as gamebanana, CatalogItem, CatalogPage, Category, FetchError, Image, ListQuery,
    ModDetails, RemoteFile, SearchQuery, Section, Sort, Visibility,
};
use deadloader_net::client::BodyError;
use deadloader_net::{Client, Host, HttpsUrl, Initiator, NetError, Purpose, Request};
use serde::Serialize;
use serde_json::{Map, Value};
use std::collections::{BTreeMap, HashSet};
use std::fmt;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError, RwLock};
use std::time::Duration;
use time::OffsetDateTime;

/// How long to wait after GameBanana said "busy" without saying how long.
const DEFAULT_BACK_OFF: Duration = Duration::from_secs(30);
/// The longest wait a `Retry-After` can impose; the user can always press "Retry now".
const MAX_BACK_OFF: Duration = Duration::from_secs(3600);
/// Parallel picture requests. Not the "parallel API requests" setting: pictures are not API calls, a page of cards
/// needs up to 50 of them, and they come from a CDN.
const IMAGE_REQUESTS: usize = 4;
const MIB: u64 = 1024 * 1024;
/// The mod page's gallery is at most 530 CSS pixels wide; `GalleryImage::preview` is sharp at that width and scale 1.
const GALLERY_WIDTH: u32 = 530;
/// Device pixels `GalleryImage::large` is sharp up to (530 px at 125–150 %).
const GALLERY_LARGE: u32 = 800;

/// Runs background work (refreshes of stale pages, prefetching) off the caller's thread.
pub trait Spawn: Send + Sync {
    fn spawn(&self, job: Box<dyn FnOnce() + Send>);
}

/// A new thread per job. Jobs are rare (one per stale page, at most one per page at a time) and short.
#[derive(Debug, Default, Clone, Copy)]
pub struct Threads;

impl Spawn for Threads {
    fn spawn(&self, job: Box<dyn FnOnce() + Send>) {
        // If no thread can be created the refresh is skipped; the stale copy stays on screen and the next call tries
        // again.
        let _ = std::thread::Builder::new()
            .name("catalog-refresh".to_owned())
            .spawn(job);
    }
}

/// What the UI should hear about without asking.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "event", content = "data", rename_all = "snake_case")]
pub enum Event {
    /// The network status changed (the global indicator, banners N7/N10, E4/E5).
    Status(Status),
    /// A background refresh saved a newer copy of this listing page: ask for it again.
    PageUpdated(Query),
    CategoriesUpdated {
        section: Section,
        parent: Option<u64>,
    },
    DetailsUpdated {
        section: Section,
        id: u64,
    },
}

pub trait Events: Send + Sync {
    fn notify(&self, event: Event);
}

/// The `app` log (ADR 0009).
pub trait AppLog: Send + Sync {
    /// Writes one line; returns its id, for the "Details" of a message, when it was written.
    fn write(&self, level: Level, msg: &str, fields: Map<String, Value>) -> Option<String>;
}

impl<C, D> AppLog for Logger<C, D>
where
    C: Clock + Send + Sync,
    D: LogDir + Send + Sync,
{
    fn write(&self, level: Level, msg: &str, fields: Map<String, Value>) -> Option<String> {
        self.app(level, msg, fields)
            .ok()
            .map(|id| id.as_str().to_owned())
    }
}

/// Where "always allow images from this host" is saved.
pub trait SettingsSink: Send + Sync {
    /// Adds `host` to `catalog.allowed_image_hosts` and saves the settings. The app's own copy of the settings must
    /// contain the host afterwards: [`Catalog::apply_settings`] replaces the allowed image hosts with the settings'
    /// list. A failure is the message the user sees (usually E1 or E23).
    fn allow_image_host(&self, host: &str) -> Result<(), UserMessage>;
}

/// Everything the catalog is made of.
#[allow(missing_debug_implementations)]
pub struct Parts<'a, F, C, L> {
    /// Shared with downloads; the catalog only manages the description-image hosts of its allowlist.
    pub net: Arc<Client>,
    pub cache: Cache<F, C, L>,
    pub clock: C,
    /// The settings at start; later changes come through [`Catalog::apply_settings`].
    pub settings: &'a Settings,
    pub settings_sink: Arc<dyn SettingsSink>,
    pub spawn: Arc<dyn Spawn>,
    pub events: Arc<dyn Events>,
    pub log: Arc<dyn AppLog>,
}

/// The catalog service. Cheap to clone; every clone is the same catalog.
pub struct Catalog<F, C, L> {
    inner: Arc<Inner<F, C, L>>,
}

impl<F, C, L> Clone for Catalog<F, C, L> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<F, C, L> fmt::Debug for Catalog<F, C, L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Catalog").finish_non_exhaustive()
    }
}

struct Inner<F, C, L> {
    net: Arc<Client>,
    cache: Cache<F, C, L>,
    clock: C,
    sink: Arc<dyn SettingsSink>,
    spawn: Arc<dyn Spawn>,
    events: Arc<dyn Events>,
    log: Arc<dyn AppLog>,
    config: RwLock<Config>,
    link: Mutex<Link>,
    registry: Mutex<Registry>,
    running: Mutex<HashSet<Job>>,
    api_gate: Gate,
    image_gate: Gate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Config {
    enabled: bool,
    nsfw: NsfwMode,
    images: DescriptionImages,
    prechecked: bool,
    prefetch: bool,
    ttl: Duration,
}

impl Config {
    fn of(settings: &Settings) -> Self {
        let catalog = &settings.catalog;
        Self {
            enabled: catalog.enabled,
            nsfw: catalog.nsfw,
            images: catalog.description_images,
            prechecked: catalog.always_allow_host_prechecked,
            prefetch: catalog.prefetch_next_page,
            ttl: Duration::from_secs(u64::from(catalog.page_ttl_minutes) * 60),
        }
    }
}

/// The state of the connection to GameBanana while the catalog is on.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Link {
    Online,
    Offline {
        since: OffsetDateTime,
    },
    RateLimited {
        until: OffsetDateTime,
        message: UserMessage,
    },
    ApiChanged {
        message: UserMessage,
    },
}

/// Background work, one at a time per target.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Job {
    Page(Query),
    Categories(Section, Option<u64>),
    Details(Section, u64),
}

/// Which request failed, for what the failure means.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Asked {
    Listing,
    Details,
}

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}

fn unix_ms(t: OffsetDateTime) -> i64 {
    i64::try_from(t.unix_timestamp_nanos() / 1_000_000).unwrap_or(i64::MAX)
}

fn library_section(section: Section) -> library::Section {
    match section {
        Section::Mod => library::Section::Mod,
        Section::Sound => library::Section::Sound,
    }
}

fn sort_name(sort: Sort) -> &'static str {
    match sort {
        Sort::Newest => "newest",
        Sort::LatestUpdated => "latest_updated",
        Sort::MostDownloaded => "most_downloaded",
        Sort::MostLiked => "most_liked",
        Sort::MostViewed => "most_viewed",
        Sort::MostCommented => "most_commented",
        Sort::Oldest => "oldest",
    }
}

fn fields<const N: usize>(pairs: [(&str, Value); N]) -> Map<String, Value> {
    pairs.into_iter().map(|(k, v)| (k.to_owned(), v)).collect()
}

fn saved_origin<T>(saved: &Saved<T>, stale: bool) -> Origin {
    Origin::Cache {
        fetched_at_unix_ms: unix_ms(saved.fetched_at),
        stale,
    }
}

fn is_nsfw(rated: bool, visibility: Visibility) -> bool {
    rated || visibility != Visibility::Show
}

/// The host of an https URL, as the allowlist sees it.
fn https_host(url: &str) -> Option<String> {
    HttpsUrl::parse(url)
        .ok()
        .map(|u| u.host().as_str().to_owned())
}

impl<F, C, L> Catalog<F, C, L>
where
    F: CacheFs + Send + Sync + 'static,
    C: Clock + Send + Sync + 'static,
    L: FileLog + Send + Sync + 'static,
{
    pub fn new(parts: Parts<'_, F, C, L>) -> Self {
        let catalog = Self {
            inner: Arc::new(Inner {
                net: parts.net,
                cache: parts.cache,
                clock: parts.clock,
                sink: parts.settings_sink,
                spawn: parts.spawn,
                events: parts.events,
                log: parts.log,
                config: RwLock::new(Config::of(parts.settings)),
                link: Mutex::new(Link::Online),
                registry: Mutex::new(Registry::new(registry::CAPACITY)),
                running: Mutex::new(HashSet::new()),
                api_gate: Gate::new(1),
                image_gate: Gate::new(IMAGE_REQUESTS),
            }),
        };
        catalog.configure(parts.settings);
        catalog
    }

    /// Applies changed settings: local mode, NSFW, description images and their allowed hosts, page freshness,
    /// prefetching, parallel API requests and the picture cache limit.
    pub fn apply_settings(&self, settings: &Settings) {
        let before = self.status();
        let was_enabled = self.config().enabled;
        *self
            .inner
            .config
            .write()
            .unwrap_or_else(PoisonError::into_inner) = Config::of(settings);
        self.configure(settings);
        if was_enabled != settings.catalog.enabled {
            let msg = if settings.catalog.enabled {
                "catalog turned on"
            } else {
                "catalog turned off: local mode, no network requests"
            };
            self.write(Level::Info, msg, Map::new());
        }
        let after = self.status();
        if after != before {
            self.inner.events.notify(Event::Status(after));
        }
    }

    fn configure(&self, settings: &Settings) {
        let inner = &self.inner;
        let ttl = Duration::from_secs(u64::from(settings.catalog.page_ttl_minutes) * 60);
        let catalog_limits = inner.cache.catalog.limits();
        if catalog_limits.ttl != ttl {
            inner.cache.catalog.set_limits(CatalogLimits {
                ttl,
                ..catalog_limits
            });
        }
        let total = u64::from(settings.cache.thumbnail_limit_mib) * MIB;
        for area in [&inner.cache.thumbs, &inner.cache.images] {
            let limits = area.limits();
            if limits.max_total_bytes != total {
                area.set_limits(BlobLimits {
                    max_total_bytes: total,
                    ..limits
                });
            }
        }
        inner.api_gate.set_limit(
            usize::try_from(settings.network.parallel_api_requests).unwrap_or(usize::MAX),
        );
        let wanted: Vec<Host> = settings
            .catalog
            .allowed_image_hosts
            .iter()
            .filter_map(|h| Host::parse(h).ok())
            .collect();
        inner.net.update_allowlist(|allowlist| {
            let stale: Vec<Host> = allowlist
                .image_hosts()
                .filter(|h| !wanted.contains(h))
                .cloned()
                .collect();
            for host in &stale {
                allowlist.remove_image_host(host);
            }
            for host in &wanted {
                allowlist.add_image_host(host.clone());
            }
        });
    }

    /// The network status for the global indicator.
    pub fn status(&self) -> Status {
        if !self.config().enabled {
            return Status::LocalMode;
        }
        let now = self.now();
        match &*lock(&self.inner.link) {
            // Once the wait is over nothing is known to be wrong; the next request tells.
            Link::Online => Status::Online,
            Link::RateLimited { until, .. } if *until <= now => Status::Online,
            Link::Offline { since } => Status::Offline {
                since_unix_ms: unix_ms(*since),
            },
            Link::RateLimited { until, message } => Status::RateLimited {
                until_unix_ms: unix_ms(*until),
                message: message.clone(),
            },
            Link::ApiChanged { message } => Status::ApiChanged {
                message: message.clone(),
            },
        }
    }

    /// Settings → Cache: bytes and entries per area of the picture and page cache.
    pub fn cache_usage(&self) -> std::io::Result<deadloader_core::cache::Usage> {
        self.inner.cache.usage()
    }

    /// Settings → Cache → "Clear cache": every cached picture and page goes (each deletion is logged); they are
    /// fetched again when needed.
    pub fn clear_cache(&self) -> std::io::Result<deadloader_core::cache::Cleared> {
        self.inner.cache.clear()
    }

    /// "Retry now" (E5): ends the wait after GameBanana said it was busy.
    pub fn retry_now(&self) {
        let before = self.status();
        let now = self.now();
        if let Link::RateLimited { until, .. } = &mut *lock(&self.inner.link) {
            *until = now;
        }
        let after = self.status();
        if after != before {
            self.inner.events.notify(Event::Status(after));
        }
    }

    // ---- listings

    /// A page of cards: saved and fresh → no request; saved and stale → served, refreshed in the background;
    /// missing → fetched now.
    pub fn list(&self, query: &Query, initiator: Initiator) -> Result<Listing, CatalogError> {
        if !self.config().enabled {
            return Err(CatalogError::LocalMode);
        }
        let key = page_key(query)?;
        match self.lookup_page::<CatalogPage>(&key) {
            Lookup::Fresh(saved) => {
                let origin = saved_origin(&saved, false);
                Ok(self.listing(query, saved.value, origin))
            }
            Lookup::Stale(saved) => {
                self.revalidate(Job::Page(query.clone()));
                let origin = saved_origin(&saved, true);
                Ok(self.listing(query, saved.value, origin))
            }
            Lookup::Missing => {
                self.check_back_off()?;
                let page = self.fetch_page(query, &key, initiator)?;
                Ok(self.listing(query, page, Origin::Network))
            }
        }
    }

    /// Fetches the page now, whatever the cache holds (a pull to refresh, Retry).
    pub fn refresh(&self, query: &Query, initiator: Initiator) -> Result<Listing, CatalogError> {
        if !self.config().enabled {
            return Err(CatalogError::LocalMode);
        }
        let key = page_key(query)?;
        self.check_back_off()?;
        let page = self.fetch_page(query, &key, initiator)?;
        Ok(self.listing(query, page, Origin::Network))
    }

    /// Fetches the page in the background unless it is saved and fresh (decision 28: the next page, at ~70 %
    /// scroll). Does nothing when the setting is off. Returns whether a fetch was queued.
    pub fn prefetch(&self, query: &Query) -> bool {
        let config = self.config();
        if !config.enabled || !config.prefetch {
            return false;
        }
        let Ok(key) = page_key(query) else {
            return false;
        };
        if matches!(self.lookup_page::<CatalogPage>(&key), Lookup::Fresh(_)) {
            return false;
        }
        self.revalidate(Job::Page(query.clone()))
    }

    /// Root categories of a section, or the children of `parent` (the heroes under Skins), cached like pages.
    pub fn categories(
        &self,
        section: Section,
        parent: Option<u64>,
    ) -> Result<Categories, CatalogError> {
        if !self.config().enabled {
            return Err(CatalogError::LocalMode);
        }
        let key = categories_key(section, parent);
        match self.lookup_page::<Vec<Category>>(&key) {
            Lookup::Fresh(saved) => Ok(Categories {
                origin: saved_origin(&saved, false),
                items: saved.value,
            }),
            Lookup::Stale(saved) => {
                self.revalidate(Job::Categories(section, parent));
                Ok(Categories {
                    origin: saved_origin(&saved, true),
                    items: saved.value,
                })
            }
            Lookup::Missing => {
                self.check_back_off()?;
                Ok(Categories {
                    items: self.fetch_categories(section, parent, &key)?,
                    origin: Origin::Network,
                })
            }
        }
    }

    // ---- mod pages

    /// A mod page. A saved one is current while `listed_updated` (the update date the grid shows, Unix seconds) is
    /// not newer than its own, or, without a listing date, for the page TTL; otherwise it is served and refreshed in
    /// the background. In local mode only a saved page is shown.
    pub fn details(
        &self,
        section: Section,
        id: u64,
        listed_updated: Option<i64>,
        initiator: Initiator,
    ) -> Result<ModPage, CatalogError> {
        let config = self.config();
        let card = self.saved_details(section, id);
        if let Some(card) = card {
            let fresh =
                match listed_updated.and_then(|s| OffsetDateTime::from_unix_timestamp(s).ok()) {
                    Some(listed) => card.is_current(listed),
                    None => {
                        let age = self.now() - card.fetched_at;
                        !age.is_negative() && age < config.ttl
                    }
                };
            if !fresh && config.enabled {
                self.revalidate(Job::Details(section, id));
            }
            let origin = Origin::Cache {
                fetched_at_unix_ms: unix_ms(card.fetched_at),
                stale: !fresh,
            };
            return Ok(self.mod_page(&card.value, origin));
        }
        if !config.enabled {
            return Err(CatalogError::LocalMode);
        }
        self.check_back_off()?;
        let details = self.fetch_details(section, id, initiator)?;
        Ok(self.mod_page(&details, Origin::Network))
    }

    /// A file of a saved mod page, with its download URL, for the download queue. Nothing is requested.
    pub fn remote_file(&self, section: Section, mod_id: u64, file_id: u64) -> Option<RemoteFile> {
        let card = self.saved_details(section, mod_id)?;
        card.value
            .files
            .into_iter()
            .chain(card.value.archived_files)
            .find(|f| f.id == file_id)
    }

    /// The picture a download of this mod shows in its row: the first picture of its saved page at card size
    /// (220 px), under the same id as the card's own. Nothing is requested; `None` when the page isn't saved, has no
    /// pictures, or the mod is NSFW and NSFW isn't shown (a row has no blur).
    pub fn download_thumbnail(&self, section: Section, mod_id: u64) -> Option<ImageId> {
        let card = self.saved_details(section, mod_id)?;
        let d = &card.value;
        let nsfw = is_nsfw(!d.content_ratings.is_empty(), d.initial_visibility);
        if nsfw && self.config().nsfw != NsfwMode::Shown {
            return None;
        }
        let image = d.gallery.first()?;
        let mut registry = lock(&self.inner.registry);
        register_gamebanana(
            &mut registry,
            Kind::Thumbnail,
            image.thumbnail_url(220, 1.0),
            Initiator::Auto,
        )
    }

    // ---- update checks (decision 164)

    /// What an update check needs about one mod: its listed files, or why it is gone. Asked every time (a check is
    /// about now), through the same allowlist, log, back-off and local mode as the catalog.
    pub fn remote_mod(
        &self,
        section: Section,
        id: u64,
        initiator: Initiator,
    ) -> Result<RemoteMod, CatalogError> {
        if !self.config().enabled {
            return Err(CatalogError::LocalMode);
        }
        self.check_back_off()?;
        let _pass = self.inner.api_gate.enter();
        let fetch = NetFetch::new(&self.inner.net, initiator);
        match gamebanana::Client::new(&fetch).file_list(section, id) {
            Ok(list) => {
                self.reached();
                Ok(remote_mod_of(list))
            }
            Err(e) => match self.failed(e, fetch.take_failure(), Asked::Details) {
                CatalogError::NotFound { .. } => Ok(RemoteMod::Removed(RemovedReason::Deleted)),
                other => Err(other),
            },
        }
    }

    /// The newest update posts of a mod (one page), for the changelog in D5.
    pub fn update_posts(
        &self,
        section: Section,
        id: u64,
        initiator: Initiator,
    ) -> Result<Vec<updates::UpdatePost>, CatalogError> {
        if !self.config().enabled {
            return Err(CatalogError::LocalMode);
        }
        self.check_back_off()?;
        let _pass = self.inner.api_gate.enter();
        let fetch = NetFetch::new(&self.inner.net, initiator);
        match gamebanana::Client::new(&fetch).updates(section, id, 1) {
            Ok(page) => {
                self.reached();
                Ok(page.posts.into_iter().map(update_post).collect())
            }
            Err(e) => Err(self.failed(e, fetch.take_failure(), Asked::Details)),
        }
    }

    // ---- pictures

    /// The picture behind an id: from the cache, or fetched now (thumbnails, the gallery, description images the
    /// user clicked or consented to).
    pub fn image(&self, id: &ImageId) -> Result<deadloader_core::cache::Blob, ImageError> {
        let entry = lock(&self.inner.registry)
            .get(id)
            .cloned()
            .ok_or(ImageError::UnknownId)?;
        if entry.kind == Kind::Description
            && !(entry.activated && (entry.once || self.image_host_allowed(&entry.host)))
        {
            return Err(ImageError::NotActivated);
        }
        let area = self.area(entry.kind);
        if let Some(blob) = self.cached_image(area, &entry.url) {
            return Ok(blob);
        }
        if !self.config().enabled {
            return Err(ImageError::LocalMode);
        }
        let from_gamebanana = entry.kind != Kind::Description;
        if from_gamebanana && self.back_off_remaining().is_some() {
            return Err(ImageError::RateLimited);
        }
        if entry.initiator == Initiator::Auto
            && matches!(*lock(&self.inner.link), Link::Offline { .. })
        {
            return Err(ImageError::Offline);
        }
        let _pass = self.inner.image_gate.enter();
        // Another request for the same picture may have fetched it while this one waited.
        if let Some(blob) = self.cached_image(area, &entry.url) {
            return Ok(blob);
        }
        self.fetch_image(&entry, area)
    }

    /// A click on a description image from an allowed host: it may be loaded now.
    pub fn activate(&self, id: &ImageId) -> Result<(), ConsentError> {
        let host = self.description_host(id)?;
        let allowed = self.image_host_allowed(&host);
        let mut registry = lock(&self.inner.registry);
        let entry = registry.get_mut(id).ok_or(ConsentError::UnknownId)?;
        if !(allowed || entry.once) {
            return Err(ConsentError::NeedsConsent { host });
        }
        entry.activated = true;
        entry.initiator = Initiator::User;
        Ok(())
    }

    /// What dialog D2 shows for this image.
    pub fn consent_prompt(&self, id: &ImageId) -> Result<ConsentPrompt, ConsentError> {
        Ok(ConsentPrompt {
            host: self.description_host(id)?,
            always_allow_prechecked: self.config().prechecked,
        })
    }

    /// The user's answer in dialog D2. The image is loaded either way; `Always` also saves its host for description
    /// images (never for API calls or downloads) and, with "always load", frees the host's other images.
    pub fn grant(&self, id: &ImageId, scope: Scope) -> Result<Granted, ConsentError> {
        let host = self.description_host(id)?;
        match scope {
            Scope::Once => {
                if let Some(entry) = lock(&self.inner.registry).get_mut(id) {
                    entry.once = true;
                    entry.activated = true;
                    entry.initiator = Initiator::User;
                }
                self.write(
                    Level::Info,
                    "description image allowed once",
                    fields([("host", host.into())]),
                );
                Ok(Granted {
                    activated: vec![*id],
                })
            }
            Scope::Always => {
                if !self.image_host_allowed(&host) {
                    self.allow_host_always(&host)?;
                }
                let always_load = self.config().images == DescriptionImages::Always;
                let mut activated = vec![*id];
                let mut registry = lock(&self.inner.registry);
                if let Some(entry) = registry.get_mut(id) {
                    entry.activated = true;
                    entry.initiator = Initiator::User;
                }
                if always_load {
                    for (other, entry) in registry.descriptions_from(&host) {
                        if other != *id && !entry.activated {
                            entry.activated = true;
                            entry.initiator = Initiator::Auto;
                            activated.push(other);
                        }
                    }
                }
                Ok(Granted { activated })
            }
        }
    }

    // ---- internals: state

    fn config(&self) -> Config {
        self.inner
            .config
            .read()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }

    fn now(&self) -> OffsetDateTime {
        self.inner.clock.now()
    }

    fn write(&self, level: Level, msg: &str, fields: Map<String, Value>) -> Option<String> {
        self.inner.log.write(level, msg, fields)
    }

    /// Logs the details of a failure and points the message at that line.
    fn report(&self, level: Level, msg: &str, detail: &str, message: UserMessage) -> UserMessage {
        let mut line = fields([("detail", detail.into())]);
        for (k, v) in &message.args {
            line.insert(k.clone(), v.clone().into());
        }
        line.insert(
            "message".to_owned(),
            serde_json::to_value(message.id).unwrap_or(Value::Null),
        );
        let message = message.arg("detail", detail);
        match self.write(level, msg, line) {
            Some(id) => message.arg("log_id", id),
            None => message,
        }
    }

    fn set_link(&self, link: Link) {
        let before = self.status();
        {
            let mut current = lock(&self.inner.link);
            match (&*current, &link) {
                // Offline since the first failure, not the latest.
                (Link::Offline { .. }, Link::Offline { .. }) => {}
                _ => *current = link,
            }
        }
        let after = self.status();
        if after != before {
            if after == Status::Online {
                self.write(Level::Info, "GameBanana reachable again", Map::new());
            }
            self.inner.events.notify(Event::Status(after));
        }
    }

    fn reached(&self) {
        self.set_link(Link::Online);
    }

    fn back_off_remaining(&self) -> Option<Duration> {
        let now = self.now();
        match &*lock(&self.inner.link) {
            Link::RateLimited { until, .. } if *until > now => {
                let left = *until - now;
                Some(Duration::from_millis(
                    u64::try_from(left.whole_milliseconds()).unwrap_or(u64::MAX),
                ))
            }
            _ => None,
        }
    }

    fn check_back_off(&self) -> Result<(), CatalogError> {
        let Some(left) = self.back_off_remaining() else {
            return Ok(());
        };
        let message = match &*lock(&self.inner.link) {
            Link::RateLimited { message, .. } => message.clone(),
            _ => UserMessage::new(MessageId::E5),
        };
        Err(CatalogError::RateLimited {
            retry_in_s: left.as_secs() + u64::from(left.subsec_nanos() > 0),
            message,
        })
    }

    /// Queues a background fetch unless one for the same target is running, the catalog is off, or GameBanana asked
    /// to wait. Returns whether it was queued.
    fn revalidate(&self, job: Job) -> bool {
        if !self.config().enabled || self.back_off_remaining().is_some() {
            return false;
        }
        if !lock(&self.inner.running).insert(job.clone()) {
            return false;
        }
        let catalog = self.clone();
        self.inner.spawn.spawn(Box::new(move || catalog.run(job)));
        true
    }

    fn run(&self, job: Job) {
        let event = match &job {
            Job::Page(query) => page_key(query)
                .ok()
                .and_then(|key| self.fetch_page(query, &key, Initiator::Auto).ok())
                .map(|_| Event::PageUpdated(query.clone())),
            Job::Categories(section, parent) => self
                .fetch_categories(*section, *parent, &categories_key(*section, *parent))
                .ok()
                .map(|_| Event::CategoriesUpdated {
                    section: *section,
                    parent: *parent,
                }),
            Job::Details(section, id) => self
                .fetch_details(*section, *id, Initiator::Auto)
                .ok()
                .map(|_| Event::DetailsUpdated {
                    section: *section,
                    id: *id,
                }),
        };
        lock(&self.inner.running).remove(&job);
        if let Some(event) = event {
            self.inner.events.notify(event);
        }
    }

    // ---- internals: the cache

    fn lookup_page<T: serde::de::DeserializeOwned>(&self, key: &PageKey) -> Lookup<T> {
        match self.inner.cache.catalog.get_page(key) {
            Ok(lookup) => lookup,
            Err(e) => {
                self.write(
                    Level::Warn,
                    "catalog cache could not be read",
                    fields([("error", e.to_string().into())]),
                );
                Lookup::Missing
            }
        }
    }

    fn save_page<T: Serialize>(&self, key: &PageKey, value: &T) {
        if let Err(e) = self.inner.cache.catalog.put_page(key, value) {
            self.write(
                Level::Warn,
                "catalog page could not be saved",
                fields([("error", e.to_string().into())]),
            );
        }
    }

    fn saved_details(
        &self,
        section: Section,
        id: u64,
    ) -> Option<deadloader_core::cache::Card<ModDetails>> {
        match self
            .inner
            .cache
            .catalog
            .get_card(library_section(section), id)
        {
            Ok(card) => card,
            Err(e) => {
                self.write(
                    Level::Warn,
                    "catalog cache could not be read",
                    fields([("error", e.to_string().into())]),
                );
                None
            }
        }
    }

    fn area(&self, kind: Kind) -> &BlobCache<F, C, L> {
        match kind {
            Kind::Description => &self.inner.cache.images,
            Kind::Thumbnail | Kind::Gallery | Kind::Original => &self.inner.cache.thumbs,
        }
    }

    fn cached_image(
        &self,
        area: &BlobCache<F, C, L>,
        url: &str,
    ) -> Option<deadloader_core::cache::Blob> {
        match area.get(url) {
            Ok(blob) => blob,
            Err(e) => {
                self.write(
                    Level::Warn,
                    "image cache could not be read",
                    fields([("error", e.to_string().into())]),
                );
                None
            }
        }
    }

    // ---- internals: requests

    fn fetch_page(
        &self,
        query: &Query,
        key: &PageKey,
        initiator: Initiator,
    ) -> Result<CatalogPage, CatalogError> {
        let _pass = self.inner.api_gate.enter();
        let fetch = NetFetch::new(&self.inner.net, initiator);
        let client = gamebanana::Client::new(&fetch);
        let result = match &query.filter {
            Filter::Browse { sort, category } => client.list(&ListQuery {
                section: query.section,
                sort: *sort,
                page: query.page,
                category: *category,
            }),
            Filter::Search { text } => {
                let search = SearchQuery::new(query.section, text, query.page)
                    .map_err(|e| CatalogError::InvalidSearch { problem: e.into() })?;
                client.search(&search)
            }
        };
        match result {
            Ok(page) => {
                self.reached();
                self.save_page(key, &page);
                Ok(page)
            }
            Err(e) => Err(self.failed(e, fetch.take_failure(), Asked::Listing)),
        }
    }

    fn fetch_categories(
        &self,
        section: Section,
        parent: Option<u64>,
        key: &PageKey,
    ) -> Result<Vec<Category>, CatalogError> {
        let _pass = self.inner.api_gate.enter();
        let fetch = NetFetch::new(&self.inner.net, Initiator::Auto);
        match gamebanana::Client::new(&fetch).categories(section, parent) {
            Ok(categories) => {
                self.reached();
                self.save_page(key, &categories);
                Ok(categories)
            }
            Err(e) => Err(self.failed(e, fetch.take_failure(), Asked::Listing)),
        }
    }

    fn fetch_details(
        &self,
        section: Section,
        id: u64,
        initiator: Initiator,
    ) -> Result<ModDetails, CatalogError> {
        let _pass = self.inner.api_gate.enter();
        let fetch = NetFetch::new(&self.inner.net, initiator);
        match gamebanana::Client::new(&fetch).details(section, id) {
            Ok(details) => {
                self.reached();
                let updated = details
                    .date_updated
                    .or(details.date_modified)
                    .or(details.date_added)
                    .and_then(|s| OffsetDateTime::from_unix_timestamp(s).ok())
                    .unwrap_or(OffsetDateTime::UNIX_EPOCH);
                if let Err(e) = self.inner.cache.catalog.put_card(
                    library_section(section),
                    id,
                    updated,
                    &details,
                ) {
                    self.write(
                        Level::Warn,
                        "mod page could not be saved",
                        fields([("error", e.to_string().into())]),
                    );
                }
                Ok(details)
            }
            Err(e) => Err(self.failed(e, fetch.take_failure(), Asked::Details)),
        }
    }

    /// What a failed GameBanana request means: the status changes, the details go to the log.
    fn failed(
        &self,
        error: gamebanana::Error,
        failure: Option<Failure>,
        asked: Asked,
    ) -> CatalogError {
        let detail = error.to_string();
        match error {
            gamebanana::Error::NotFound => {
                self.reached();
                CatalogError::NotFound { other_game: None }
            }
            gamebanana::Error::WrongGame { game_id } => {
                self.reached();
                CatalogError::NotFound {
                    other_game: Some(game_id),
                }
            }
            gamebanana::Error::Fetch(FetchError::Status { status: 404, .. })
                if asked == Asked::Details =>
            {
                self.reached();
                CatalogError::NotFound { other_game: None }
            }
            // An error page, not GameBanana's error document: a refusal (often temporary), not an API change (181).
            gamebanana::Error::Fetch(FetchError::Status { status, .. })
                if (400..500).contains(&status) =>
            {
                self.reached();
                CatalogError::Refused {
                    message: self.report(
                        Level::Warn,
                        "GameBanana refused the request",
                        &detail,
                        UserMessage::new(MessageId::E25).arg("status", status),
                    ),
                }
            }
            gamebanana::Error::ApiChanged(_)
            | gamebanana::Error::Fetch(FetchError::Status { .. }) => self.api_changed(&detail),
            gamebanana::Error::Fetch(FetchError::Transport(_)) => match failure {
                Some(Failure::Net(e)) => self.net_failed(e),
                Some(Failure::Read(read)) => self.offline(&read),
                Some(Failure::TooLarge) => self.api_changed(&detail),
                None => CatalogError::Internal {
                    message: self.report(
                        Level::Error,
                        "catalog request failed",
                        &detail,
                        UserMessage::new(MessageId::Internal),
                    ),
                },
            },
        }
    }

    fn api_changed(&self, detail: &str) -> CatalogError {
        let message = self.report(
            Level::Error,
            "GameBanana's API answered in a way this version does not understand",
            detail,
            UserMessage::new(MessageId::E4),
        );
        self.set_link(Link::ApiChanged {
            message: message.clone(),
        });
        CatalogError::ApiChanged { message }
    }

    fn offline(&self, detail: &str) -> CatalogError {
        let was_offline = matches!(*lock(&self.inner.link), Link::Offline { .. });
        if !was_offline {
            self.write(
                Level::Warn,
                "GameBanana could not be reached; showing saved pages",
                fields([("detail", detail.into())]),
            );
        }
        self.set_link(Link::Offline { since: self.now() });
        let since = match *lock(&self.inner.link) {
            Link::Offline { since } => since,
            _ => self.now(),
        };
        CatalogError::Offline {
            since_unix_ms: unix_ms(since),
        }
    }

    fn net_failed(&self, error: NetError) -> CatalogError {
        let detail = error.to_string();
        match error {
            NetError::LocalMode => CatalogError::LocalMode,
            NetError::Transport { .. } => self.offline(&detail),
            NetError::Unavailable {
                status,
                retry_after,
                ..
            } => {
                let wait = retry_after
                    .unwrap_or(DEFAULT_BACK_OFF)
                    .min(MAX_BACK_OFF)
                    .max(Duration::from_secs(1));
                let message = self.report(
                    Level::Warn,
                    "GameBanana is busy; waiting before the next request",
                    &detail,
                    UserMessage::new(MessageId::E5)
                        .arg("retry_in_s", wait.as_secs())
                        .arg("status", status),
                );
                self.set_link(Link::RateLimited {
                    until: self.now() + wait,
                    message: message.clone(),
                });
                CatalogError::RateLimited {
                    retry_in_s: wait.as_secs(),
                    message,
                }
            }
            NetError::HostNotAllowed { ref host, .. } => self.blocked(host.as_str(), &detail),
            NetError::InsecureRedirect { ref location, .. } => {
                let host = HttpsUrl::parse(&location.replacen("http:", "https:", 1))
                    .map(|u| u.host().as_str().to_owned())
                    .unwrap_or_else(|_| location.clone());
                self.blocked(&host, &detail)
            }
            NetError::BadRedirect { ref from, .. } => self.blocked(from.as_str(), &detail),
            NetError::TooManyRedirects { ref last, .. } => self.blocked(last.as_str(), &detail),
            NetError::InvalidUrl { .. } => CatalogError::Internal {
                message: self.report(
                    Level::Error,
                    "catalog request has an invalid address",
                    &detail,
                    UserMessage::new(MessageId::Internal),
                ),
            },
        }
    }

    fn blocked(&self, host: &str, detail: &str) -> CatalogError {
        CatalogError::Blocked {
            message: self.report(
                Level::Warn,
                "catalog request blocked by the allowlist",
                detail,
                UserMessage::new(MessageId::E16).arg("host", host),
            ),
        }
    }

    fn fetch_image(
        &self,
        entry: &Entry,
        area: &BlobCache<F, C, L>,
    ) -> Result<deadloader_core::cache::Blob, ImageError> {
        let mut request = Request::new(entry.kind.purpose(), entry.initiator, &entry.url)
            .header("Accept", "image/webp,image/png,image/jpeg,image/gif");
        if entry.once
            && let Ok(host) = Host::parse(&entry.host)
        {
            request = request.consented_image_host(host);
        }
        let response = match self.inner.net.get(&request) {
            Ok(response) => response,
            Err(e) => return Err(self.image_failed(e, entry.kind != Kind::Description)),
        };
        if !(200..300).contains(&response.status) {
            return Err(ImageError::Upstream {
                status: response.status,
            });
        }
        let declared = response.header("content-type").map(str::to_owned);
        let limits = area.limits();
        let bytes = match response.read_body(limits.max_entry_bytes.min(limits.max_total_bytes)) {
            Ok(bytes) => bytes,
            Err(BodyError::TooLarge { .. }) => return Err(ImageError::TooLarge),
            Err(BodyError::Io(_)) => return Err(ImageError::Unreachable),
        };
        let sniffed = ImageType::sniff(&bytes).ok_or(ImageError::NotAnImage)?;
        // No type, or a generic "some bytes" one, says nothing: the bytes decide. A specific non-image type is refused.
        let content_type = declared
            .filter(|declared| !is_generic_binary(declared))
            .unwrap_or_else(|| sniffed.mime().to_owned());
        match area.put(&entry.url, &bytes, &content_type) {
            Ok(content_type) => Ok(deadloader_core::cache::Blob {
                bytes,
                content_type,
            }),
            Err(PutError::UnsupportedType(_) | PutError::NotAnImage) => Err(ImageError::NotAnImage),
            Err(PutError::TooLarge { .. }) => Err(ImageError::TooLarge),
            Err(PutError::Io(e)) => {
                // Shown anyway; it is fetched again next time.
                self.write(
                    Level::Warn,
                    "image could not be saved to the cache",
                    fields([("error", e.to_string().into())]),
                );
                Ok(deadloader_core::cache::Blob {
                    bytes,
                    content_type: sniffed,
                })
            }
        }
    }

    fn image_failed(&self, error: NetError, from_gamebanana: bool) -> ImageError {
        match error {
            NetError::LocalMode => ImageError::LocalMode,
            NetError::Unavailable { .. } if from_gamebanana => {
                // GameBanana is busy: the catalog waits too.
                let _ = self.net_failed(error);
                ImageError::RateLimited
            }
            NetError::Unavailable { status, .. } => ImageError::Upstream { status },
            NetError::Transport { .. } => ImageError::Unreachable,
            NetError::HostNotAllowed { .. }
            | NetError::InsecureRedirect { .. }
            | NetError::BadRedirect { .. }
            | NetError::TooManyRedirects { .. }
            | NetError::InvalidUrl { .. } => ImageError::Blocked,
        }
    }

    // ---- internals: consent

    fn image_host_allowed(&self, host: &str) -> bool {
        Host::parse(host).is_ok_and(|host| {
            self.inner
                .net
                .allowlist()
                .check(&host, Purpose::DescriptionImage)
                .is_ok()
        })
    }

    fn description_host(&self, id: &ImageId) -> Result<String, ConsentError> {
        match lock(&self.inner.registry).get(id) {
            None => Err(ConsentError::UnknownId),
            Some(entry) if entry.kind != Kind::Description => {
                Err(ConsentError::NotADescriptionImage)
            }
            Some(entry) => Ok(entry.host.clone()),
        }
    }

    fn allow_host_always(&self, host: &str) -> Result<(), ConsentError> {
        let parsed = Host::parse(host).map_err(|_| ConsentError::NotADescriptionImage)?;
        if let Err(e) = self.inner.sink.allow_image_host(host) {
            let detail = e.args.get("detail").cloned().unwrap_or_default();
            let message = self.report(
                Level::Error,
                "allowed image host could not be saved",
                &detail,
                e,
            );
            return Err(ConsentError::Save { message });
        }
        self.inner.net.update_allowlist(|allowlist| {
            allowlist.add_image_host(parsed);
        });
        self.write(
            Level::Info,
            "description images allowed from a host",
            fields([("host", host.into())]),
        );
        Ok(())
    }

    // ---- internals: views

    fn listing(&self, query: &Query, page: CatalogPage, origin: Origin) -> Listing {
        let nsfw_mode = self.config().nsfw;
        let mut hidden_nsfw = 0;
        let mut items = Vec::with_capacity(page.items.len());
        {
            let mut registry = lock(&self.inner.registry);
            for item in page.items {
                let nsfw = is_nsfw(item.has_content_ratings, item.initial_visibility);
                if nsfw && nsfw_mode == NsfwMode::Hidden {
                    hidden_nsfw += 1;
                    continue;
                }
                items.push(card(item, nsfw, &mut registry));
            }
        }
        Listing {
            items,
            hidden_nsfw,
            total: page.total,
            page: query.page,
            has_more: !page.is_complete,
            origin,
        }
    }

    fn mod_page(&self, d: &ModDetails, origin: Origin) -> ModPage {
        let config = self.config();
        let nsfw = is_nsfw(!d.content_ratings.is_empty(), d.initial_visibility);
        let nsfw_hidden = nsfw && config.nsfw == NsfwMode::Hidden;
        let gallery = if nsfw_hidden {
            Vec::new()
        } else {
            let mut registry = lock(&self.inner.registry);
            d.gallery
                .iter()
                .filter_map(|image| gallery_image(image, &mut registry))
                .collect()
        };
        let auto_load = config.images == DescriptionImages::Always && !nsfw_hidden;
        ModPage {
            id: d.id,
            section: d.section,
            name: d.name.clone(),
            version: d.version.clone(),
            author: d.author.clone(),
            credits: d.credits.clone(),
            category_path: d.category_path.clone(),
            gallery,
            has_audio_preview: d.audio_preview_url.is_some(),
            short_description: d.short_description.clone(),
            description: self.description(&d.description_html, auto_load),
            licence: d
                .licence_html
                .as_deref()
                .map(|html| self.description(html, auto_load)),
            likes: d.likes,
            views: d.views,
            downloads: d.downloads,
            date_added: d.date_added,
            date_modified: d.date_modified,
            date_updated: d.date_updated,
            nsfw,
            nsfw_hidden,
            content_ratings: d.content_ratings.clone(),
            obsolete: d.obsolete,
            embedded_media: d.embedded_media.clone(),
            alternate_sources: d.alternate_sources.clone(),
            files: d.files.iter().map(file_view).collect(),
            archived_files: d.archived_files.iter().map(file_view).collect(),
            origin,
        }
    }

    fn description(&self, html: &str, auto_load: bool) -> Description {
        let sanitized = deadloader_html::sanitize(html);
        let links = sanitized
            .links
            .into_iter()
            .map(|l| LinkView {
                url: to_https(l.url),
                host: l.host,
                internal_mod_id: l.internal_mod_id,
            })
            .collect();
        let allowed: Vec<(String, bool)> = {
            let mut hosts: Vec<String> = sanitized.images.iter().map(|i| i.host.clone()).collect();
            hosts.sort();
            hosts.dedup();
            hosts
                .into_iter()
                .map(|h| {
                    let ok = self.image_host_allowed(&h);
                    (h, ok)
                })
                .collect()
        };
        let is_allowed = |host: &str| allowed.iter().any(|(h, ok)| h == host && *ok);
        let mut images = Vec::with_capacity(sanitized.images.len());
        let mut consent: BTreeMap<String, Vec<ImageId>> = BTreeMap::new();
        {
            let mut registry = lock(&self.inner.registry);
            for image in sanitized.images {
                let url = to_https(image.url);
                let id = registry.register(Kind::Description, &url, &image.host, Initiator::User);
                let host_allowed = is_allowed(&image.host);
                let access = match registry.get_mut(&id) {
                    Some(entry) if entry.activated && (entry.once || host_allowed) => Access::Load,
                    Some(entry) if host_allowed && auto_load => {
                        entry.activated = true;
                        entry.initiator = Initiator::Auto;
                        Access::Load
                    }
                    _ if host_allowed => Access::Click,
                    Some(entry) if entry.once => Access::Click,
                    _ => {
                        let ids = consent.entry(image.host.clone()).or_default();
                        if !ids.contains(&id) {
                            ids.push(id);
                        }
                        Access::Consent
                    }
                };
                images.push(DescriptionImage {
                    id,
                    host: image.host,
                    alt: image.alt,
                    access,
                });
            }
        }
        Description {
            html: sanitized.html,
            links,
            images,
            consent: consent
                .into_iter()
                .map(|(host, images)| PendingHost { host, images })
                .collect(),
            truncated: sanitized.truncated,
        }
    }
}

fn page_key(query: &Query) -> Result<PageKey, CatalogError> {
    let section = library_section(query.section);
    let page = query.page.max(1);
    Ok(match &query.filter {
        Filter::Browse { sort, category } => PageKey {
            section,
            sort: sort_name(*sort).to_owned(),
            category: *category,
            page,
            search: None,
        },
        Filter::Search { text } => {
            let search = SearchQuery::new(query.section, text, page)
                .map_err(|e| CatalogError::InvalidSearch { problem: e.into() })?;
            PageKey {
                section,
                sort: "search".to_owned(),
                category: None,
                page,
                search: Some(search.text().to_owned()),
            }
        }
    })
}

fn categories_key(section: Section, parent: Option<u64>) -> PageKey {
    PageKey {
        section: library_section(section),
        sort: "categories".to_owned(),
        category: parent,
        page: 0,
        search: None,
    }
}

fn card(item: CatalogItem, nsfw: bool, registry: &mut Registry) -> Card {
    let thumbnail = item.thumbnail.as_ref().and_then(|image| {
        // Decision 152 in the UI: `small` up to 220 device pixels, `large` above.
        let small = image.thumbnail_url(220, 1.0);
        let large = image.thumbnail_url(221, 1.0);
        Some(Thumbnail {
            small: register_gamebanana(registry, Kind::Thumbnail, small, Initiator::Auto)?,
            large: register_gamebanana(registry, Kind::Thumbnail, large, Initiator::Auto)?,
        })
    });
    Card {
        id: item.id,
        section: item.section,
        name: item.name,
        author: item.author,
        root_category: item.root_category,
        subcategory: item.subcategory,
        likes: item.likes,
        views: item.views,
        date_added: item.date_added,
        date_updated: item.date_updated.or(item.date_modified),
        thumbnail,
        has_audio_preview: item.audio_preview_url.is_some(),
        nsfw,
        obsolete: item.obsolete,
        featured: item.featured,
        has_files: item.has_files,
    }
}

fn gallery_image(image: &Image, registry: &mut Registry) -> Option<GalleryImage> {
    let mut shown = |url: &str| register_gamebanana(registry, Kind::Gallery, url, Initiator::Auto);
    let preview = shown(image.sharp_url(GALLERY_WIDTH, 1.0))?;
    let large = shown(image.sharp_url(GALLERY_LARGE, 1.0))?;
    let full = shown(&image.original_url)?;
    Some(GalleryImage {
        preview,
        large,
        full,
        original: register_gamebanana(
            registry,
            Kind::Original,
            &image.original_url,
            Initiator::User,
        )?,
        caption: image.caption.clone(),
    })
}

/// Pictures named by GameBanana's API: only valid https addresses get an id.
fn register_gamebanana(
    registry: &mut Registry,
    kind: Kind,
    url: &str,
    initiator: Initiator,
) -> Option<ImageId> {
    let host = https_host(url)?;
    Some(registry.register(kind, url, &host, initiator))
}

/// `Content-Type`s that only say "bytes", as storage-backed hosts send for every file.
fn is_generic_binary(content_type: &str) -> bool {
    let essence = content_type
        .split_once(';')
        .map_or(content_type, |(essence, _)| essence)
        .trim();
    ["application/octet-stream", "binary/octet-stream"]
        .iter()
        .any(|generic| essence.eq_ignore_ascii_case(generic))
}

/// GameBanana's file list in the core's own terms; a mod GameBanana hides counts as removed.
fn remote_mod_of(list: gamebanana::ModFileList) -> RemoteMod {
    let reason = match list.availability {
        gamebanana::Availability::Public => None,
        gamebanana::Availability::Withheld => Some(RemovedReason::Withheld),
        gamebanana::Availability::Trashed => Some(RemovedReason::Trashed),
        gamebanana::Availability::Private => Some(RemovedReason::Private),
    };
    if let Some(reason) = reason {
        return RemoteMod::Removed(reason);
    }
    if list.files.is_empty() {
        return RemoteMod::Removed(RemovedReason::NoFiles);
    }
    RemoteMod::Listed(updates::ListedMod {
        version: list.version,
        date_updated: list.date_updated,
        files: list
            .files
            .into_iter()
            .map(|f| updates::RemoteFile {
                id: f.id,
                file_name: f.file_name,
                size_bytes: f.size_bytes,
                date_added: f.date_added,
                version: f.version,
                description: f.description,
                md5: f.md5.map(|m| m.to_ascii_lowercase()),
            })
            .collect(),
        posts: Vec::new(),
    })
}

fn update_post(post: gamebanana::UpdatePost) -> updates::UpdatePost {
    updates::UpdatePost {
        id: post.id,
        title: post.title,
        version: post.version,
        date_added: post.date_added,
        changelog: post
            .changelog
            .into_iter()
            .map(|c| updates::ChangeEntry {
                category: c.category,
                text: c.text,
            })
            .collect(),
        file_ids: post.file_ids,
        text_html: post.text_html,
    }
}

fn file_view(f: &RemoteFile) -> FileView {
    FileView {
        id: f.id,
        file_name: f.file_name.clone(),
        size_bytes: f.size_bytes,
        date_added: f.date_added,
        download_count: f.download_count,
        description: f.description.clone(),
        version: f.version.clone(),
        md5: f.md5.clone(),
        antivirus: f.antivirus.clone(),
    }
}

/// A description's `http://` link or picture as `https://` (175, 183): the network layer and the link opener speak
/// https only, and the hosts seen in descriptions serve it. The sanitiser has already lower-cased the scheme.
fn to_https(url: String) -> String {
    match url.strip_prefix("http://") {
        Some(rest) => format!("https://{rest}"),
        None => url,
    }
}
