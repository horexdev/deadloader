//! Fakes for the catalog tests: a scripted HTTP transport behind the real `net` client, an in-memory cache folder, a
//! settable clock, a queue for background jobs, and recorders for events, log lines and settings writes.

#![allow(dead_code)]

use deadloader_catalog::{AppLog, Catalog, Event, Parts, SettingsSink, Spawn};
use deadloader_core::cache::{self, Cache, CacheFs, Entry, FileLog};
use deadloader_core::log::{Clock, FileEvent, Level};
use deadloader_core::message::{ToUserMessage, UserMessage};
use deadloader_core::settings::{DescriptionImages, NsfwMode, Settings, SettingsError};
use deadloader_gamebanana::request;
use deadloader_gamebanana::{ListQuery, Section, Sort};
use deadloader_net::log::{HopRecord, NetLog};
use deadloader_net::transport::{
    HopRequest, HopResponse, Transport, TransportError, TransportErrorKind,
};
use deadloader_net::{Allowlist, Client, RetryPolicy};
use serde_json::{Map, Value};
use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};
use std::io;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{Duration, SystemTime};
use time::OffsetDateTime;

pub fn lock<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
}

macro_rules! fixture {
    ($name:literal) => {
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../gamebanana/tests/fixtures/",
            $name
        ))
        .as_slice()
    };
}
pub(crate) use fixture;

// ---- transport

#[derive(Clone)]
pub struct Reply {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl Reply {
    pub fn ok(body: &[u8]) -> Self {
        Self {
            status: 200,
            headers: Vec::new(),
            body: body.to_vec(),
        }
    }

    pub fn image(content_type: &str, body: &[u8]) -> Self {
        Self::ok(body).header("Content-Type", content_type)
    }

    pub fn status(status: u16) -> Self {
        Self {
            status,
            headers: Vec::new(),
            body: Vec::new(),
        }
    }

    pub fn header(mut self, name: &str, value: &str) -> Self {
        self.headers.push((name.to_owned(), value.to_owned()));
        self
    }
}

/// Answers by URL. A URL with several replies uses them in turn and repeats the last one; an unscripted URL fails
/// as if the host could not be reached. `offline` makes every request fail that way.
#[derive(Default)]
pub struct FakeTransport {
    replies: Mutex<HashMap<String, VecDeque<Reply>>>,
    calls: Mutex<Vec<String>>,
    pub offline: AtomicBool,
}

impl FakeTransport {
    pub fn on(&self, url: &str, reply: Reply) {
        lock(&self.replies)
            .entry(url.to_owned())
            .or_default()
            .push_back(reply);
    }

    /// Forgets the replies scripted for `url` and answers with this one from now on.
    pub fn replace(&self, url: &str, reply: Reply) {
        lock(&self.replies).insert(url.to_owned(), VecDeque::from([reply]));
    }

    pub fn calls(&self) -> Vec<String> {
        lock(&self.calls).clone()
    }

    pub fn count(&self) -> usize {
        lock(&self.calls).len()
    }

    pub fn set_offline(&self, on: bool) {
        self.offline.store(on, Ordering::SeqCst);
    }
}

pub struct SharedTransport(pub Arc<FakeTransport>);

impl Transport for SharedTransport {
    fn send(&self, request: &HopRequest<'_>) -> Result<HopResponse, TransportError> {
        let url = request.url.as_str().to_owned();
        lock(&self.0.calls).push(url.clone());
        if self.0.offline.load(Ordering::SeqCst) {
            return Err(TransportError::new(
                TransportErrorKind::Connect,
                "the network is unreachable",
            ));
        }
        let reply = {
            let mut replies = lock(&self.0.replies);
            match replies.get_mut(&url) {
                Some(queue) if queue.len() > 1 => queue.pop_front(),
                Some(queue) => queue.front().cloned(),
                None => None,
            }
        };
        match reply {
            Some(reply) => Ok(HopResponse {
                status: reply.status,
                headers: reply.headers,
                body: Box::new(io::Cursor::new(reply.body)),
            }),
            None => Err(TransportError::new(
                TransportErrorKind::Connect,
                format!("nothing scripted for {url}"),
            )),
        }
    }
}

#[derive(Default)]
pub struct NetRecords(Mutex<Vec<HopRecord>>);

impl NetRecords {
    pub fn all(&self) -> Vec<HopRecord> {
        lock(&self.0).clone()
    }
}

pub struct SharedNetLog(pub Arc<NetRecords>);

impl NetLog for SharedNetLog {
    fn record(&self, record: &HopRecord) {
        lock(&self.0.0).push(record.clone());
    }
}

/// Retries in tests never wait.
pub struct NoSleep;

impl deadloader_net::clock::Clock for NoSleep {
    fn now(&self) -> std::time::Instant {
        std::time::Instant::now()
    }

    fn sleep(&self, _: Duration) {}
}

// ---- the cache folder in memory

#[derive(Default)]
pub struct MemState {
    files: BTreeMap<String, (Vec<u8>, SystemTime)>,
    dirs: BTreeSet<String>,
}

#[derive(Clone, Default)]
pub struct MemFs(pub Arc<Mutex<MemState>>);

fn norm(path: &Path) -> String {
    path.to_string_lossy()
        .replace('/', "\\")
        .to_ascii_lowercase()
}

fn parent_of(key: &str) -> Option<&str> {
    key.rsplit_once('\\').map(|(p, _)| p)
}

impl MemFs {
    pub fn file_names(&self) -> Vec<String> {
        lock(&self.0).files.keys().cloned().collect()
    }
}

impl CacheFs for MemFs {
    fn read(&self, path: &Path) -> io::Result<Vec<u8>> {
        lock(&self.0)
            .files
            .get(&norm(path))
            .map(|(b, _)| b.clone())
            .ok_or_else(|| io::Error::from(io::ErrorKind::NotFound))
    }

    fn write_synced(&self, path: &Path, bytes: &[u8], modified: SystemTime) -> io::Result<()> {
        lock(&self.0)
            .files
            .insert(norm(path), (bytes.to_vec(), modified));
        Ok(())
    }

    fn replace(&self, from: &Path, to: &Path) -> io::Result<()> {
        let mut s = lock(&self.0);
        let data = s
            .files
            .remove(&norm(from))
            .ok_or_else(|| io::Error::from(io::ErrorKind::NotFound))?;
        s.files.insert(norm(to), data);
        Ok(())
    }

    fn set_modified(&self, path: &Path, modified: SystemTime) -> io::Result<()> {
        match lock(&self.0).files.get_mut(&norm(path)) {
            Some(entry) => {
                entry.1 = modified;
                Ok(())
            }
            None => Err(io::Error::from(io::ErrorKind::NotFound)),
        }
    }

    fn remove_file(&self, path: &Path) -> io::Result<()> {
        lock(&self.0)
            .files
            .remove(&norm(path))
            .map(|_| ())
            .ok_or_else(|| io::Error::from(io::ErrorKind::NotFound))
    }

    fn create_dir_all(&self, path: &Path) -> io::Result<()> {
        let mut s = lock(&self.0);
        let mut key = norm(path);
        loop {
            s.dirs.insert(key.clone());
            match parent_of(&key) {
                Some(p) => key = p.to_owned(),
                None => break,
            }
        }
        Ok(())
    }

    fn remove_dir(&self, path: &Path) -> io::Result<()> {
        lock(&self.0).dirs.remove(&norm(path));
        Ok(())
    }

    fn list(&self, dir: &Path) -> io::Result<Vec<Entry>> {
        let s = lock(&self.0);
        let dir = norm(dir);
        if !s.dirs.contains(&dir) {
            return Err(io::Error::from(io::ErrorKind::NotFound));
        }
        let mut out = Vec::new();
        for (key, (bytes, modified)) in &s.files {
            if parent_of(key) == Some(dir.as_str()) {
                out.push(Entry {
                    name: key.rsplit('\\').next().unwrap_or_default().to_owned(),
                    is_dir: false,
                    len: bytes.len() as u64,
                    modified: *modified,
                });
            }
        }
        for key in &s.dirs {
            if parent_of(key) == Some(dir.as_str()) {
                out.push(Entry {
                    name: key.rsplit('\\').next().unwrap_or_default().to_owned(),
                    is_dir: true,
                    len: 0,
                    modified: SystemTime::UNIX_EPOCH,
                });
            }
        }
        Ok(out)
    }
}

#[derive(Clone, Default)]
pub struct NoFileLog;

impl FileLog for NoFileLog {
    fn record(&self, _: &str, _: &FileEvent) {}
}

// ---- time

#[derive(Clone)]
pub struct TestClock(pub Arc<Mutex<OffsetDateTime>>);

impl TestClock {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(
            OffsetDateTime::from_unix_timestamp(1_800_000_000).expect("valid time"),
        )))
    }

    pub fn advance(&self, by: Duration) {
        let mut now = lock(&self.0);
        *now += by;
    }

    pub fn unix_ms(&self) -> i64 {
        i64::try_from(lock(&self.0).unix_timestamp_nanos() / 1_000_000).expect("fits")
    }
}

impl Clock for TestClock {
    fn now(&self) -> OffsetDateTime {
        *lock(&self.0)
    }
}

// ---- background jobs, events, log, settings

type Job = Box<dyn FnOnce() + Send>;

#[derive(Default)]
pub struct JobQueue(Mutex<VecDeque<Job>>);

impl JobQueue {
    pub fn len(&self) -> usize {
        lock(&self.0).len()
    }

    /// Runs queued jobs, including the ones they queue, until none is left; returns how many ran.
    pub fn run(&self) -> usize {
        let mut ran = 0;
        loop {
            let job = lock(&self.0).pop_front();
            match job {
                Some(job) => {
                    job();
                    ran += 1;
                }
                None => return ran,
            }
        }
    }
}

pub struct SharedQueue(pub Arc<JobQueue>);

impl Spawn for SharedQueue {
    fn spawn(&self, job: Job) {
        lock(&self.0.0).push_back(job);
    }
}

#[derive(Default)]
pub struct Events(Mutex<Vec<Event>>);

impl Events {
    pub fn take(&self) -> Vec<Event> {
        std::mem::take(&mut *lock(&self.0))
    }
}

pub struct SharedEvents(pub Arc<Events>);

impl deadloader_catalog::Events for SharedEvents {
    fn notify(&self, event: Event) {
        lock(&self.0.0).push(event);
    }
}

pub type Line = (Level, String, Map<String, Value>);

#[derive(Default)]
pub struct Lines(Mutex<Vec<Line>>);

impl Lines {
    pub fn all(&self) -> Vec<Line> {
        lock(&self.0).clone()
    }

    pub fn messages(&self) -> Vec<String> {
        lock(&self.0).iter().map(|(_, m, _)| m.clone()).collect()
    }
}

pub struct SharedLines(pub Arc<Lines>);

impl AppLog for SharedLines {
    fn write(&self, level: Level, msg: &str, fields: Map<String, Value>) -> Option<String> {
        let mut lines = lock(&self.0.0);
        lines.push((level, msg.to_owned(), fields));
        Some(format!("line-{}", lines.len()))
    }
}

#[derive(Default)]
pub struct Saved {
    pub hosts: Mutex<Vec<String>>,
    pub fail: AtomicBool,
}

pub struct SharedSaved(pub Arc<Saved>);

impl SettingsSink for SharedSaved {
    fn allow_image_host(&self, host: &str) -> Result<(), UserMessage> {
        if self.0.fail.load(Ordering::SeqCst) {
            return Err(SettingsError::Io("access denied".to_owned()).user_message());
        }
        lock(&self.0.hosts).push(host.to_owned());
        Ok(())
    }
}

// ---- the whole harness

pub type TestCatalog = Catalog<MemFs, TestClock, NoFileLog>;

pub struct Harness {
    pub transport: Arc<FakeTransport>,
    pub net_log: Arc<NetRecords>,
    pub net: Arc<Client>,
    pub fs: MemFs,
    pub clock: TestClock,
    pub jobs: Arc<JobQueue>,
    pub events: Arc<Events>,
    pub lines: Arc<Lines>,
    pub saved: Arc<Saved>,
    pub catalog: TestCatalog,
}

/// Settings with the catalog on (the first-run choice made) and everything else at its default.
pub fn online_settings() -> Settings {
    let mut settings = Settings::default();
    settings.set_catalog_enabled(true);
    settings
}

pub fn with_nsfw(mut settings: Settings, mode: NsfwMode) -> Settings {
    settings.confirm_age(OffsetDateTime::UNIX_EPOCH);
    settings.set_nsfw(mode).expect("age confirmed");
    settings
}

pub fn with_images(mut settings: Settings, mode: DescriptionImages) -> Settings {
    settings.catalog.description_images = mode;
    settings
}

impl Harness {
    pub fn new(settings: &Settings) -> Self {
        let transport = Arc::new(FakeTransport::default());
        let net_log = Arc::new(NetRecords::default());
        let net = Arc::new(
            Client::new(
                SharedTransport(Arc::clone(&transport)),
                SharedNetLog(Arc::clone(&net_log)),
                NoSleep,
                Allowlist::new(),
            )
            .with_retry_policy(RetryPolicy {
                max_retries: 0,
                base_delay: Duration::from_millis(1),
                max_delay: Duration::from_secs(600),
            }),
        );
        let fs = MemFs::default();
        let clock = TestClock::new();
        let cache = Cache::open(
            Path::new("C:\\data\\cache"),
            fs.clone(),
            clock.clone(),
            NoFileLog,
            cache::Settings::default(),
        )
        .expect("cache opens");
        let jobs = Arc::new(JobQueue::default());
        let events = Arc::new(Events::default());
        let lines = Arc::new(Lines::default());
        let saved = Arc::new(Saved::default());
        let catalog = Catalog::new(Parts {
            net: Arc::clone(&net),
            cache,
            clock: clock.clone(),
            settings,
            settings_sink: Arc::new(SharedSaved(Arc::clone(&saved))),
            spawn: Arc::new(SharedQueue(Arc::clone(&jobs))),
            events: Arc::new(SharedEvents(Arc::clone(&events))),
            log: Arc::new(SharedLines(Arc::clone(&lines))),
        });
        Self {
            transport,
            net_log,
            net,
            fs,
            clock,
            jobs,
            events,
            lines,
            saved,
            catalog,
        }
    }

    pub fn on(&self, url: &str, reply: Reply) {
        self.transport.on(url, reply);
    }
}

pub fn list_url(section: Section, sort: Sort, page: u32, category: Option<u64>) -> String {
    request::list_url(&ListQuery {
        section,
        sort,
        page,
        category,
    })
}

pub const THUMB_220: &str = "https://images.gamebanana.com/img/ss/mods/220-90_aaaa0001.jpg";
pub const THUMB_530: &str = "https://images.gamebanana.com/img/ss/mods/530-90_aaaa0001.jpg";

/// The smallest JPEG signature the cache accepts as an image.
pub const JPEG: &[u8] = b"\xFF\xD8\xFF\xE0fake jpeg";
pub const PNG: &[u8] = b"\x89PNG\r\n\x1a\nfake png";

/// The recorded mod page with another description and, optionally, without content ratings.
pub fn profile_with(text: &str, rated: bool) -> Vec<u8> {
    let mut doc: Value = serde_json::from_slice(fixture!("mod_profile.json")).expect("fixture");
    let object = doc.as_object_mut().expect("object");
    object.insert("_sText".to_owned(), Value::String(text.to_owned()));
    if !rated {
        object.remove("_aContentRatings");
        object.insert("_sInitialVisibility".to_owned(), "show".into());
    }
    serde_json::to_vec(&doc).expect("json")
}
