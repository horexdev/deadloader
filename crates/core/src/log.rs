//! Logs (ADR 0009): what the program did, on disk next to the exe, as JSON Lines.
//!
//! Three logs, one file per local day: `app-YYYY-MM-DD.jsonl` (start, settings, errors, external programs),
//! `network-…` (one line per request and per redirect hop) and `files-…` (one line per file operation). Every line
//! starts with `ts`, `id`, `lvl` and `msg`; the id is returned to the caller so an error shown to the user can point
//! at the line with the details (decision 98). Every string goes through [`Masker`] before it is written, so the
//! Windows user name does not leak into bug reports (decision 94).
//!
//! A line is written with a single write call and not fsynced: losing the last lines in a power cut is acceptable
//! for logs (the intent journal, which must survive, is separate). A crash mid-write leaves a damaged last line,
//! which [`parse`] skips.
//!
//! Rotation runs at start and on the first write of a new day: files older than the retention go first — judged by
//! the date in the name, which survives copying and unpacking, unlike the modification time — then the oldest until
//! the folder is under the size cap. Only files named like ours are touched, and the current day's files are never
//! deleted, so a single very busy day can exceed the cap until the next day.

use crate::fs::DirEntry;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard, PoisonError};
use time::{Date, Month, OffsetDateTime};

/// Which of the three logs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    App,
    Network,
    Files,
}

impl Kind {
    pub const ALL: [Kind; 3] = [Kind::App, Kind::Network, Kind::Files];

    pub fn prefix(self) -> &'static str {
        match self {
            Kind::App => "app",
            Kind::Network => "network",
            Kind::Files => "files",
        }
    }

    /// `app-2026-10-15.jsonl`.
    pub fn file_name(self, date: Date) -> String {
        format!(
            "{}-{:04}-{:02}-{:02}.jsonl",
            self.prefix(),
            date.year(),
            u8::from(date.month()),
            date.day()
        )
    }

    /// The kind and date of one of our file names; `None` for every other name.
    pub fn parse_file_name(name: &str) -> Option<(Kind, Date)> {
        let stem = name.strip_suffix(".jsonl")?;
        let (prefix, date) = stem.split_once('-')?;
        let kind = Kind::ALL.into_iter().find(|k| k.prefix() == prefix)?;
        let bytes = date.as_bytes();
        if bytes.len() != 10 || bytes.get(4) != Some(&b'-') || bytes.get(7) != Some(&b'-') {
            return None;
        }
        let number = |from: usize, to: usize| -> Option<u16> {
            let digits = date.get(from..to)?;
            if !digits.bytes().all(|b| b.is_ascii_digit()) {
                return None;
            }
            digits.parse().ok()
        };
        let month = Month::try_from(u8::try_from(number(5, 7)?).ok()?).ok()?;
        let day = u8::try_from(number(8, 10)?).ok()?;
        let date = Date::from_calendar_date(i32::from(number(0, 4)?), month, day).ok()?;
        Some((kind, date))
    }
}

/// Identifies one log line: a run prefix (the start time in milliseconds, base 36) and a counter, e.g.
/// `mv9fkgpf-000042`. Unique per run and increasing within it; runs started later sort after earlier ones.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LogId(pub(crate) String);

impl LogId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for LogId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Level {
    Debug,
    Info,
    Warn,
    Error,
}

/// One line of any log: the common fields, then the fields of the log's own event type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Record<T> {
    /// RFC 3339 with the local offset and milliseconds.
    pub ts: String,
    pub id: LogId,
    pub lvl: Level,
    /// Stable English text; the UI translates its own messages, not log lines.
    pub msg: String,
    #[serde(flatten)]
    pub body: T,
}

/// An `app` line: optional structured details.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AppEvent {
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub fields: Map<String, Value>,
}

/// A `network` line: one request or one redirect hop of it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkEvent {
    /// Shared by all hops of one request; from [`Logger::request_id`].
    pub req: LogId,
    /// 0 for the request as made, 1 for the first redirect, and so on.
    pub hop: u32,
    /// 1 for the first try of this hop, 2.. for retries after 429/5xx; written only for retries.
    #[serde(default = "first_attempt", skip_serializing_if = "is_first_attempt")]
    pub attempt: u32,
    pub method: String,
    pub url: String,
    /// Which allowlist entry let the hop through; `None` when it was refused (see `blocked`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_source: Option<HostSource>,
    pub initiator: Initiator,
    pub purpose: Purpose,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<u16>,
    /// The body size the server announced (`Content-Length`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bytes: Option<u64>,
    /// Duration in milliseconds.
    pub ms: u64,
    /// The `Range` header sent when resuming a download.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Refused before any byte left the machine, and why (`host-not-listed`, `local-mode`, `not-https`…).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocked: Option<String>,
}

fn first_attempt() -> u32 {
    1
}

#[allow(clippy::trivially_copy_pass_by_ref)] // serde passes a reference
fn is_first_attempt(attempt: &u32) -> bool {
    *attempt == 1
}

/// Why the host was allowed (decision 53).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HostSource {
    Builtin,
    /// GitHub's API, listed only while deadloader's own update check is on (54).
    Github,
    User,
    UserImage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Initiator {
    /// The program decided to make the request (e.g. loading the catalog page being viewed).
    Auto,
    /// A direct user action (e.g. pressing Download).
    User,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Purpose {
    Catalog,
    Thumbnail,
    DescriptionImage,
    Download,
    UpdateCheck,
}

/// A `files` line: one file operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileEvent {
    pub op: FileOp,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub src: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dst: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bytes: Option<u64>,
    /// Written as 64 lowercase hex digits.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "opt_hex")]
    pub sha256: Option<[u8; 32]>,
    /// The intent-journal transaction the operation belongs to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub txn: Option<u64>,
    pub result: Outcome,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl FileEvent {
    /// A successful `op` with every optional field empty, to fill in with struct update syntax.
    pub fn new(op: FileOp) -> Self {
        Self {
            op,
            src: None,
            dst: None,
            bytes: None,
            sha256: None,
            txn: None,
            result: Outcome::Ok,
            error: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FileOp {
    Hardlink,
    Copy,
    Rename,
    Delete,
    Write,
    Backup,
    Restore,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Outcome {
    Ok,
    Failed,
}

/// [`crate::hex`] for an optional digest.
mod opt_hex {
    use serde::{Deserialize, Deserializer, Serializer};

    #[derive(Deserialize)]
    struct Digest(#[serde(with = "crate::hex")] [u8; 32]);

    pub fn serialize<S: Serializer>(value: &Option<[u8; 32]>, s: S) -> Result<S::Ok, S::Error> {
        match value {
            Some(bytes) => crate::hex::serialize(bytes, s),
            None => s.serialize_none(),
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Option<[u8; 32]>, D::Error> {
        Ok(Option::<Digest>::deserialize(d)?.map(|Digest(bytes)| bytes))
    }
}

/// The current local time; behind a trait so tests choose the day.
pub trait Clock {
    fn now(&self) -> OffsetDateTime;
}

impl<T: Clock + ?Sized> Clock for &T {
    fn now(&self) -> OffsetDateTime {
        (**self).now()
    }
}

/// The system clock with the system's current UTC offset (UTC if the offset cannot be determined).
#[derive(Debug, Default, Clone, Copy)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> OffsetDateTime {
        OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc())
    }
}

/// The folder the logs live in; names are bare file names inside it.
pub trait LogDir {
    /// Appends `line` and a newline to the file, creating the folder and the file as needed, in a single write so
    /// lines never interleave. Flushed to the operating system, not synced to the disk.
    fn append_line(&self, name: &str, line: &str) -> io::Result<()>;
    /// The entries in the folder; `NotFound` if it doesn't exist.
    fn list(&self) -> io::Result<Vec<DirEntry>>;
    fn remove(&self, name: &str) -> io::Result<()>;
    fn read(&self, name: &str) -> io::Result<Vec<u8>>;

    /// The file's current size; `NotFound` if it doesn't exist.
    fn file_len(&self, name: &str) -> io::Result<u64> {
        self.read(name).map(|bytes| bytes.len() as u64)
    }

    /// Up to `len` bytes from `start`; fewer at the end of the file. The Activity screen reads logs of up to the size
    /// cap this way, a part at a time, instead of the whole file.
    fn read_range(&self, name: &str, start: u64, len: u64) -> io::Result<Vec<u8>> {
        let bytes = self.read(name)?;
        let from = usize::try_from(start)
            .unwrap_or(usize::MAX)
            .min(bytes.len());
        let to = from
            .saturating_add(usize::try_from(len).unwrap_or(usize::MAX))
            .min(bytes.len());
        Ok(bytes[from..to].to_vec())
    }
}

impl<T: LogDir + ?Sized> LogDir for &T {
    fn append_line(&self, name: &str, line: &str) -> io::Result<()> {
        (**self).append_line(name, line)
    }

    fn list(&self) -> io::Result<Vec<DirEntry>> {
        (**self).list()
    }

    fn remove(&self, name: &str) -> io::Result<()> {
        (**self).remove(name)
    }

    fn read(&self, name: &str) -> io::Result<Vec<u8>> {
        (**self).read(name)
    }

    fn file_len(&self, name: &str) -> io::Result<u64> {
        (**self).file_len(name)
    }

    fn read_range(&self, name: &str, start: u64, len: u64) -> io::Result<Vec<u8>> {
        (**self).read_range(name, start, len)
    }
}

/// `deadloader-data\logs\` on disk. Keeps one file open per log so a line costs one write call, not an open and a
/// close (which antivirus scanners hook).
#[derive(Debug)]
pub struct DiskLogDir {
    dir: PathBuf,
    /// Log prefix → the open file's name and handle.
    open: Mutex<HashMap<String, (String, File)>>,
}

impl DiskLogDir {
    pub fn new(dir: PathBuf) -> Self {
        Self {
            dir,
            open: Mutex::new(HashMap::new()),
        }
    }

    pub fn path(&self) -> &Path {
        &self.dir
    }
}

impl DiskLogDir {
    fn handles(&self) -> MutexGuard<'_, HashMap<String, (String, File)>> {
        self.open.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// Opens for appending. If the file ends in the middle of a line (a crash mid-write), the line is ended first so
    /// the damage stays confined to it and the next record starts on a line of its own.
    fn open_for_append(&self, name: &str) -> io::Result<File> {
        std::fs::create_dir_all(&self.dir)?;
        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(self.dir.join(name))?;
        if file.metadata()?.len() > 0 {
            file.seek(SeekFrom::End(-1))?;
            let mut last = [0u8; 1];
            file.read_exact(&mut last)?;
            if last != *b"\n" {
                file.write_all(b"\n")?;
            }
        }
        Ok(file)
    }
}

impl LogDir for DiskLogDir {
    fn append_line(&self, name: &str, line: &str) -> io::Result<()> {
        let slot = name.split_once('-').map_or(name, |(prefix, _)| prefix);
        let mut open = self.handles();
        if !open
            .get(slot)
            .is_some_and(|(open_name, _)| open_name == name)
        {
            // A new day: the previous day's file of this log is closed.
            open.remove(slot);
            let file = self.open_for_append(name)?;
            open.insert(slot.to_owned(), (name.to_owned(), file));
        }
        let mut buf = Vec::with_capacity(line.len() + 1);
        buf.extend_from_slice(line.as_bytes());
        buf.push(b'\n');
        let written = match open.get_mut(slot) {
            Some((_, file)) => file.write_all(&buf),
            None => Err(io::Error::other("log file is not open")),
        };
        if written.is_err() {
            // Reopen on the next line; the handle may be stale.
            open.remove(slot);
        }
        written
    }

    fn list(&self) -> io::Result<Vec<DirEntry>> {
        let mut out = Vec::new();
        for entry in std::fs::read_dir(&self.dir)? {
            let entry = entry?;
            let meta = entry.metadata()?;
            // On Windows the directory listing shows the size as of the last close, so files we still have open
            // look empty; asking the file itself gives the current size.
            let len = match std::fs::metadata(entry.path()) {
                Ok(current) => current.len(),
                Err(_) => meta.len(),
            };
            out.push(DirEntry {
                name: entry.file_name().to_string_lossy().into_owned(),
                is_dir: meta.is_dir(),
                len,
            });
        }
        Ok(out)
    }

    fn remove(&self, name: &str) -> io::Result<()> {
        self.handles().retain(|_, (open_name, _)| open_name != name);
        std::fs::remove_file(self.dir.join(name))
    }

    fn read(&self, name: &str) -> io::Result<Vec<u8>> {
        std::fs::read(self.dir.join(name))
    }

    fn file_len(&self, name: &str) -> io::Result<u64> {
        // The file's own size, not the directory's, which lags behind for files we keep open.
        std::fs::metadata(self.dir.join(name)).map(|m| m.len())
    }

    fn read_range(&self, name: &str, start: u64, len: u64) -> io::Result<Vec<u8>> {
        // Opened for reading only; the writer's handle shares reading and writing, so lines keep being appended.
        let mut file = File::open(self.dir.join(name))?;
        file.seek(SeekFrom::Start(start))?;
        let mut out = Vec::with_capacity(usize::try_from(len.min(1 << 20)).unwrap_or(0));
        file.take(len).read_to_end(&mut out)?;
        Ok(out)
    }
}

/// How much history to keep (decision 51).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Limits {
    /// Files dated this many days before today or earlier are deleted; today is always kept.
    pub retention_days: u32,
    /// The total size of our files, in bytes, that rotation brings the folder under.
    pub max_bytes: u64,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            retention_days: 30,
            max_bytes: 100 * 1024 * 1024,
        }
    }
}

#[derive(Debug)]
pub enum LogError {
    /// The record could not be turned into JSON.
    Encode { id: LogId, error: serde_json::Error },
    /// The line could not be written.
    Write { id: LogId, error: io::Error },
}

impl LogError {
    /// The id the line would have had.
    pub fn id(&self) -> &LogId {
        match self {
            Self::Encode { id, .. } | Self::Write { id, .. } => id,
        }
    }
}

impl fmt::Display for LogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Encode { id, error } => write!(f, "could not encode log entry {id}: {error}"),
            Self::Write { id, error } => write!(f, "could not write log entry {id}: {error}"),
        }
    }
}

impl std::error::Error for LogError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Encode { error, .. } => Some(error),
            Self::Write { error, .. } => Some(error),
        }
    }
}

/// Where file operations are reported (the files log). Writing a log line never makes the operation fail.
pub trait FileLog {
    fn record(&self, msg: &str, event: &FileEvent);
}

impl<T: FileLog + ?Sized> FileLog for &T {
    fn record(&self, msg: &str, event: &FileEvent) {
        (**self).record(msg, event);
    }
}

impl<T: FileLog + ?Sized> FileLog for std::sync::Arc<T> {
    fn record(&self, msg: &str, event: &FileEvent) {
        (**self).record(msg, event);
    }
}

impl<T: FileLog + ?Sized> FileLog for Box<T> {
    fn record(&self, msg: &str, event: &FileEvent) {
        (**self).record(msg, event);
    }
}

impl<C: Clock, D: LogDir> FileLog for Logger<C, D> {
    fn record(&self, msg: &str, event: &FileEvent) {
        // An operation must not fail because its log line could not be written.
        let _ = self.file(msg, event);
    }
}

/// The file operations of one change to the game folder: every event carries the change's intent-journal transaction,
/// so the Activity screen can show which change an operation belonged to.
#[derive(Debug)]
pub struct InChange<L> {
    pub log: L,
    pub txn: u64,
}

impl<L: FileLog> FileLog for InChange<L> {
    fn record(&self, msg: &str, event: &FileEvent) {
        let event = FileEvent {
            txn: Some(self.txn),
            ..event.clone()
        };
        self.log.record(msg, &event);
    }
}

/// Keeps the events in memory, for tests.
#[derive(Debug, Default)]
pub struct MemFileLog(pub Mutex<Vec<(String, FileEvent)>>);

impl MemFileLog {
    pub fn events(&self) -> Vec<(String, FileEvent)> {
        self.0
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }
}

impl FileLog for MemFileLog {
    fn record(&self, msg: &str, event: &FileEvent) {
        self.0
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .push((msg.to_owned(), event.clone()));
    }
}

/// `msg` of a network line whose response will be retried (429/5xx): not the request's last word.
pub const RETRYING_MSG: &str = "response; retrying";

/// Told the log of every line written, for the Activity screen's live view. Called while the logger is locked, so it
/// must return at once and must not log.
pub type Watcher = Box<dyn Fn(Kind) + Send + Sync>;

/// Writes the three logs. Safe to share between threads; lines are written in id order.
#[derive(Debug)]
pub struct Logger<C, D> {
    clock: C,
    dir: D,
    run: String,
    state: Mutex<State>,
    watcher: Watching,
}

#[derive(Default)]
struct Watching(Mutex<Option<Watcher>>);

impl fmt::Debug for Watching {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Watching")
    }
}

#[derive(Debug)]
struct State {
    next: u64,
    /// The day of the last rotation.
    day: Option<Date>,
    masker: Masker,
    limits: Limits,
}

impl Logger<SystemClock, DiskLogDir> {
    /// The production logger: `logs_dir` on disk, the system clock, the profile from `USERPROFILE`, default limits.
    pub fn open(logs_dir: PathBuf) -> Self {
        Self::new(
            SystemClock,
            DiskLogDir::new(logs_dir),
            Masker::from_env(),
            Limits::default(),
        )
    }
}

impl<C: Clock, D: LogDir> Logger<C, D> {
    /// Starts a run and rotates the folder.
    pub fn new(clock: C, dir: D, masker: Masker, limits: Limits) -> Self {
        let started_ms = clock.now().unix_timestamp_nanos() / 1_000_000;
        let logger = Self {
            clock,
            dir,
            run: base36(u128::try_from(started_ms).unwrap_or(0)),
            state: Mutex::new(State {
                next: 1,
                day: None,
                masker,
                limits,
            }),
            watcher: Watching::default(),
        };
        logger.rotate();
        logger
    }

    pub fn dir(&self) -> &D {
        &self.dir
    }

    /// The clock the lines are stamped with: readers of the logs use it for "today".
    pub fn clock(&self) -> &C {
        &self.clock
    }

    pub fn app(
        &self,
        lvl: Level,
        msg: &str,
        fields: Map<String, Value>,
    ) -> Result<LogId, LogError> {
        self.write(Kind::App, lvl, msg, &AppEvent { fields })
    }

    /// Level `error` when the event has an error, `info` otherwise.
    pub fn network(&self, msg: &str, event: &NetworkEvent) -> Result<LogId, LogError> {
        let lvl = if event.error.is_some() {
            Level::Error
        } else if event.blocked.is_some() {
            Level::Warn
        } else {
            Level::Info
        };
        self.write(Kind::Network, lvl, msg, event)
    }

    /// Level `error` when the operation failed, `info` otherwise.
    pub fn file(&self, msg: &str, event: &FileEvent) -> Result<LogId, LogError> {
        let lvl = match event.result {
            Outcome::Ok => Level::Info,
            Outcome::Failed => Level::Error,
        };
        self.write(Kind::Files, lvl, msg, event)
    }

    /// A fresh id for [`NetworkEvent::req`], from the same sequence as line ids.
    pub fn request_id(&self) -> LogId {
        let mut state = self.lock();
        self.next_id(&mut state)
    }

    /// `watcher` hears of every line written from now on (it replaces an earlier one).
    pub fn watch(&self, watcher: Watcher) {
        *self
            .watcher
            .0
            .lock()
            .unwrap_or_else(PoisonError::into_inner) = Some(watcher);
    }

    /// Switches user-path masking for the lines written from now on.
    pub fn set_masking(&self, on: bool) {
        self.lock().masker.set_enabled(on);
    }

    /// Applies new limits and rotates right away.
    pub fn set_limits(&self, limits: Limits) {
        let mut state = self.lock();
        state.limits = limits;
        self.rotate_locked(&mut state);
    }

    /// Deletes old files now (it also happens at start and on the first write of each day).
    pub fn rotate(&self) {
        let mut state = self.lock();
        self.rotate_locked(&mut state);
    }

    fn lock(&self) -> MutexGuard<'_, State> {
        self.state.lock().unwrap_or_else(PoisonError::into_inner)
    }

    fn next_id(&self, state: &mut State) -> LogId {
        let n = state.next;
        state.next += 1;
        LogId(format!("{}-{n:06}", self.run))
    }

    fn write<T: Serialize + DeserializeOwned>(
        &self,
        kind: Kind,
        lvl: Level,
        msg: &str,
        body: &T,
    ) -> Result<LogId, LogError> {
        let mut state = self.lock();
        self.write_locked(&mut state, kind, lvl, msg, body)
    }

    fn write_locked<T: Serialize + DeserializeOwned>(
        &self,
        state: &mut State,
        kind: Kind,
        lvl: Level,
        msg: &str,
        body: &T,
    ) -> Result<LogId, LogError> {
        let now = self.clock.now();
        if state.day != Some(now.date()) {
            self.rotate_locked(state);
        }
        let id = self.next_id(state);
        let line = match encode(&state.masker, &now, &id, lvl, msg, body) {
            Ok(line) => line,
            Err(error) => return Err(LogError::Encode { id, error }),
        };
        match self.dir.append_line(&kind.file_name(now.date()), &line) {
            Ok(()) => {
                if let Some(watcher) = self
                    .watcher
                    .0
                    .lock()
                    .unwrap_or_else(PoisonError::into_inner)
                    .as_ref()
                {
                    watcher(kind);
                }
                Ok(id)
            }
            Err(error) => Err(LogError::Write { id, error }),
        }
    }

    /// Deletes what [`plan_rotation`] picks and reports each deletion in the app log. Failures to report have
    /// nowhere else to go and are dropped.
    fn rotate_locked(&self, state: &mut State) {
        let today = self.clock.now().date();
        state.day = Some(today);
        let entries = match self.dir.list() {
            Ok(entries) => entries,
            Err(e) if e.kind() == io::ErrorKind::NotFound => return,
            Err(e) => {
                let mut fields = Map::new();
                fields.insert("error".into(), e.to_string().into());
                let _ = self.write_locked(
                    state,
                    Kind::App,
                    Level::Warn,
                    "could not list log files",
                    &AppEvent { fields },
                );
                return;
            }
        };
        for doomed in plan_rotation(&entries, today, state.limits) {
            let mut fields = Map::new();
            fields.insert("file".into(), doomed.name.clone().into());
            fields.insert("bytes".into(), doomed.bytes.into());
            fields.insert("reason".into(), doomed.reason.as_str().into());
            let (lvl, msg) = match self.dir.remove(&doomed.name) {
                Ok(()) => (Level::Info, "old log file deleted"),
                Err(e) => {
                    fields.insert("error".into(), e.to_string().into());
                    (Level::Warn, "could not delete old log file")
                }
            };
            let _ = self.write_locked(state, Kind::App, lvl, msg, &AppEvent { fields });
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Doomed {
    name: String,
    bytes: u64,
    reason: Reason,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Reason {
    Retention,
    SizeCap,
}

impl Reason {
    fn as_str(self) -> &'static str {
        match self {
            Reason::Retention => "retention",
            Reason::SizeCap => "size-cap",
        }
    }
}

/// Which of our files to delete, oldest first: those past the retention, then the oldest until the rest fit under
/// the cap. Today's (and, after a clock change, future) files are never picked.
fn plan_rotation(entries: &[DirEntry], today: Date, limits: Limits) -> Vec<Doomed> {
    let mut ours: Vec<(Date, &str, u64)> = entries
        .iter()
        .filter(|e| !e.is_dir)
        .filter_map(|e| {
            Kind::parse_file_name(&e.name).map(|(_, date)| (date, e.name.as_str(), e.len))
        })
        .collect();
    ours.sort_unstable();
    let keep_days = i64::from(limits.retention_days.max(1));
    let (expired, kept): (Vec<_>, Vec<_>) = ours
        .into_iter()
        .partition(|&(date, _, _)| date < today && (today - date).whole_days() >= keep_days);
    let doomed = |(_, name, bytes): (Date, &str, u64), reason| Doomed {
        name: name.to_owned(),
        bytes,
        reason,
    };
    let mut plan: Vec<Doomed> = expired
        .into_iter()
        .map(|file| doomed(file, Reason::Retention))
        .collect();
    let mut total: u64 = kept.iter().map(|&(_, _, bytes)| bytes).sum();
    for file in kept {
        if total <= limits.max_bytes || file.0 >= today {
            break;
        }
        total = total.saturating_sub(file.2);
        plan.push(doomed(file, Reason::SizeCap));
    }
    plan
}

/// The line for one record, every string masked.
fn encode<T: Serialize + DeserializeOwned>(
    masker: &Masker,
    now: &OffsetDateTime,
    id: &LogId,
    lvl: Level,
    msg: &str,
    body: &T,
) -> Result<String, serde_json::Error> {
    let masked;
    let body = if masker.active() {
        // Through a JSON value and back: every string of any event type is reached, and the typed value keeps the
        // documented field order when written.
        let mut value = serde_json::to_value(body)?;
        mask_value(masker, &mut value);
        masked = serde_json::from_value::<T>(value)?;
        &masked
    } else {
        body
    };
    serde_json::to_string(&Record {
        ts: rfc3339(now),
        id: id.clone(),
        lvl,
        msg: masker.mask(msg).into_owned(),
        body,
    })
}

fn mask_value(masker: &Masker, value: &mut Value) {
    match value {
        Value::String(s) => {
            if let Cow::Owned(masked) = masker.mask(s) {
                *s = masked;
            }
        }
        Value::Array(items) => items.iter_mut().for_each(|v| mask_value(masker, v)),
        Value::Object(map) => {
            for (key, mut v) in std::mem::take(map) {
                mask_value(masker, &mut v);
                map.insert(masker.mask(&key).into_owned(), v);
            }
        }
        Value::Null | Value::Bool(_) | Value::Number(_) => {}
    }
}

fn base36(mut n: u128) -> String {
    let mut digits = Vec::new();
    loop {
        let digit = u32::try_from(n % 36).unwrap_or(0);
        digits.push(char::from_digit(digit, 36).unwrap_or('0'));
        n /= 36;
        if n == 0 {
            break;
        }
    }
    digits.iter().rev().collect()
}

/// A log file as the reader sees it: records, plus the lines that could not be read.
#[derive(Debug, Clone, PartialEq)]
pub struct ReadLog<T> {
    pub records: Vec<Record<T>>,
    /// Damaged lines other than the last one: those point at a real problem.
    pub damaged: Vec<DamagedLine>,
    /// The last line was damaged and skipped — what a crash mid-write leaves behind.
    pub torn_tail: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DamagedLine {
    /// 1-based.
    pub line: usize,
    pub error: String,
}

/// `2026-10-15T14:03:27.123+03:00`.
fn rfc3339(t: &OffsetDateTime) -> String {
    let offset = t.offset().whole_minutes();
    let sign = if offset < 0 { '-' } else { '+' };
    let offset = offset.unsigned_abs();
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:03}{sign}{:02}:{:02}",
        t.year(),
        u8::from(t.month()),
        t.day(),
        t.hour(),
        t.minute(),
        t.second(),
        t.millisecond(),
        offset / 60,
        offset % 60
    )
}

/// Parses one log file's content. Works on bytes, since a crash can cut a line inside a multi-byte character.
pub fn parse<T: DeserializeOwned>(bytes: &[u8]) -> ReadLog<T> {
    let lines: Vec<&[u8]> = bytes
        .split(|&b| b == b'\n')
        .map(|line| line.strip_suffix(b"\r").unwrap_or(line))
        .collect();
    let blank = |line: &[u8]| line.iter().all(u8::is_ascii_whitespace);
    let last = lines.iter().rposition(|line| !blank(line));
    let mut read = ReadLog {
        records: Vec::new(),
        damaged: Vec::new(),
        torn_tail: false,
    };
    for (index, line) in lines.iter().enumerate() {
        if blank(line) {
            continue;
        }
        let record = std::str::from_utf8(line)
            .map_err(|e| e.to_string())
            .and_then(|text| serde_json::from_str::<Record<T>>(text).map_err(|e| e.to_string()));
        match record {
            Ok(record) => read.records.push(record),
            Err(_) if Some(index) == last => read.torn_tail = true,
            Err(error) => read.damaged.push(DamagedLine {
                line: index + 1,
                error,
            }),
        }
    }
    read
}

/// Reads and parses the `kind` log of `date`.
pub fn read_log<T: DeserializeOwned>(
    dir: &impl LogDir,
    kind: Kind,
    date: Date,
) -> io::Result<ReadLog<T>> {
    dir.read(&kind.file_name(date)).map(|bytes| parse(&bytes))
}

/// One of our files in the logs folder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogFile {
    pub kind: Kind,
    pub date: Date,
    pub name: String,
    pub bytes: u64,
}

/// Our files in the folder, oldest first; empty if the folder doesn't exist.
pub fn log_files(dir: &impl LogDir) -> io::Result<Vec<LogFile>> {
    let entries = match dir.list() {
        Ok(entries) => entries,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(e),
    };
    let mut files: Vec<LogFile> = entries
        .into_iter()
        .filter(|e| !e.is_dir)
        .filter_map(|e| {
            Kind::parse_file_name(&e.name).map(|(kind, date)| LogFile {
                kind,
                date,
                name: e.name,
                bytes: e.len,
            })
        })
        .collect();
    files.sort_by_key(|f| (f.date, f.kind));
    Ok(files)
}

/// What the user's profile folder is replaced with.
pub const MASK: &str = "%USERPROFILE%";

/// Replaces the user's profile folder in log text with [`MASK`] (decision 94), so a log attached to a bug report
/// does not reveal the Windows user name.
///
/// The profile is matched case-insensitively, with `\` and `/` interchangeable and a run of separators matching one
/// (Debug-formatted paths double every backslash), also in its percent-encoded form inside URLs. A match counts only
/// where the profile's last name ends: `C:\Users\Al` is not masked inside `C:\Users\Alice`. Only the path is masked;
/// the bare user name elsewhere in a message is not recognised.
#[derive(Debug, Clone)]
pub struct Masker {
    /// The profile path as written, then percent-encoded when that differs.
    patterns: Vec<Vec<Tok>>,
    enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tok {
    /// One or more `\` or `/`.
    Sep,
    /// A case-folded character.
    Char(char),
}

impl Masker {
    /// Masks `profile`, e.g. `C:\Users\Alice`. A profile without a folder below the drive masks nothing, since it
    /// would match every path on that drive.
    pub fn new(profile: &str) -> Self {
        let trimmed = profile.trim().trim_end_matches(is_sep);
        let mut patterns = Vec::new();
        if trimmed.contains(is_sep) {
            let literal = tokens(trimmed);
            let encoded = tokens(&percent_encode(trimmed));
            if encoded != literal {
                patterns.push(encoded);
            }
            patterns.push(literal);
        }
        Self {
            patterns,
            enabled: true,
        }
    }

    /// The profile from the `USERPROFILE` environment variable.
    pub fn from_env() -> Self {
        match std::env::var("USERPROFILE") {
            Ok(profile) => Self::new(&profile),
            Err(_) => Self::none(),
        }
    }

    /// Nothing to mask.
    pub fn none() -> Self {
        Self {
            patterns: Vec::new(),
            enabled: true,
        }
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, on: bool) {
        self.enabled = on;
    }

    fn active(&self) -> bool {
        self.enabled && !self.patterns.is_empty()
    }

    pub fn mask<'a>(&self, text: &'a str) -> Cow<'a, str> {
        if !self.active() {
            return Cow::Borrowed(text);
        }
        let chars: Vec<(usize, char)> = text.char_indices().collect();
        let mut out = String::new();
        let mut copied = 0;
        let mut i = 0;
        while i < chars.len() {
            let found = self
                .patterns
                .iter()
                .filter_map(|p| match_at(p, &chars, i))
                .find(|&end| name_ends(&chars, end));
            match found {
                Some(end) => {
                    out.push_str(&text[copied..chars[i].0]);
                    out.push_str(MASK);
                    copied = chars.get(end).map_or(text.len(), |&(at, _)| at);
                    i = end;
                }
                None => i += 1,
            }
        }
        if out.is_empty() {
            return Cow::Borrowed(text);
        }
        out.push_str(&text[copied..]);
        Cow::Owned(out)
    }
}

fn is_sep(c: char) -> bool {
    c == '\\' || c == '/'
}

/// Simple case folding; characters whose lowercase form is several characters are kept as they are.
fn fold(c: char) -> char {
    let mut lower = c.to_lowercase();
    match (lower.next(), lower.next()) {
        (Some(l), None) => l,
        _ => c,
    }
}

fn tokens(s: &str) -> Vec<Tok> {
    let mut out = Vec::new();
    for c in s.chars() {
        if is_sep(c) {
            if out.last() != Some(&Tok::Sep) {
                out.push(Tok::Sep);
            }
        } else {
            out.push(Tok::Char(fold(c)));
        }
    }
    out
}

/// How a path appears in a `file:` URL: everything but unreserved characters, `:` and separators as `%XX`.
fn percent_encode(s: &str) -> String {
    let mut out = String::new();
    for b in s.bytes() {
        if b.is_ascii_alphanumeric() || b"-._~:\\/".contains(&b) {
            out.push(char::from(b));
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}

/// Where a match of `pattern` starting at `start` ends, if it matches.
fn match_at(pattern: &[Tok], chars: &[(usize, char)], start: usize) -> Option<usize> {
    let mut i = start;
    for tok in pattern {
        match *tok {
            Tok::Sep => {
                let from = i;
                while chars.get(i).is_some_and(|&(_, c)| is_sep(c)) {
                    i += 1;
                }
                if i == from {
                    return None;
                }
            }
            Tok::Char(want) => match chars.get(i) {
                Some(&(_, c)) if fold(c) == want => i += 1,
                _ => return None,
            },
        }
    }
    Some(i)
}

/// Whether the folder name ends before `chars[at]`: a letter, digit, `_`, `-`, a `.` followed by a letter or digit,
/// or a `%XX` escape would continue it.
fn name_ends(chars: &[(usize, char)], at: usize) -> bool {
    let c = |i: usize| chars.get(i).map(|&(_, c)| c);
    match c(at) {
        None => true,
        Some(ch) if ch.is_alphanumeric() || ch == '_' || ch == '-' => false,
        Some('.') => !c(at + 1).is_some_and(char::is_alphanumeric),
        Some('%') => {
            !(c(at + 1).is_some_and(|h| h.is_ascii_hexdigit())
                && c(at + 2).is_some_and(|h| h.is_ascii_hexdigit()))
        }
        Some(_) => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::collections::BTreeMap;
    use time::{Time, UtcOffset};

    fn alice() -> Masker {
        Masker::new(r"C:\Users\Alice")
    }

    #[test]
    fn masks_the_profile_prefix_of_a_path() {
        assert_eq!(
            alice().mask(r"C:\Users\Alice\Downloads\mod.zip"),
            r"%USERPROFILE%\Downloads\mod.zip"
        );
        assert_eq!(alice().mask(r"C:\Users\Alice"), "%USERPROFILE%");
    }

    #[test]
    fn masking_ignores_case() {
        assert_eq!(
            alice().mask(r"c:\USERS\alice\x and C:\users\ALICE"),
            r"%USERPROFILE%\x and %USERPROFILE%"
        );
        let cyrillic = Masker::new(r"C:\Users\Алиса");
        assert_eq!(cyrillic.mask(r"c:\users\АЛИСА\x"), r"%USERPROFILE%\x");
    }

    #[test]
    fn masking_accepts_both_slash_styles_and_doubled_separators() {
        let m = alice();
        assert_eq!(m.mask("C:/Users/Alice/x"), "%USERPROFILE%/x");
        assert_eq!(m.mask(r"C:/Users\Alice/x"), "%USERPROFILE%/x");
        // Debug-formatted paths double every backslash.
        assert_eq!(
            m.mask(r#"Os { path: "C:\\Users\\Alice\\x" }"#),
            r#"Os { path: "%USERPROFILE%\\x" }"#
        );
    }

    #[test]
    fn masks_inside_urls_including_percent_encoding() {
        assert_eq!(
            alice().mask("file:///C:/Users/Alice/Pictures/a.png"),
            "file:///%USERPROFILE%/Pictures/a.png"
        );
        let spaced = Masker::new(r"C:\Users\Al Smith");
        assert_eq!(
            spaced.mask("file:///C:/Users/Al%20Smith/a.png and C:\\Users\\Al Smith\\b"),
            "file:///%USERPROFILE%/a.png and %USERPROFILE%\\b"
        );
    }

    #[test]
    fn masks_in_the_middle_of_messages() {
        assert_eq!(
            alice().mask(r#"cannot open "C:\Users\Alice\a.vpk": access denied (C:\Users\Alice)."#),
            r#"cannot open "%USERPROFILE%\a.vpk": access denied (%USERPROFILE%)."#
        );
    }

    #[test]
    fn a_profile_name_that_prefixes_another_name_masks_only_itself() {
        let al = Masker::new(r"C:\Users\Al");
        assert_eq!(al.mask(r"C:\Users\Alice\x"), r"C:\Users\Alice\x");
        assert_eq!(al.mask(r"C:\Users\Al_old\x"), r"C:\Users\Al_old\x");
        assert_eq!(al.mask(r"C:\Users\Al-2\x"), r"C:\Users\Al-2\x");
        assert_eq!(al.mask(r"C:\Users\Al.WORK\x"), r"C:\Users\Al.WORK\x");
        assert_eq!(al.mask("C:/Users/Al%20Smith/x"), "C:/Users/Al%20Smith/x");
        assert_eq!(al.mask(r"C:\Users\Al\x"), r"%USERPROFILE%\x");
        assert_eq!(al.mask(r"see C:\Users\Al."), "see %USERPROFILE%.");
        assert_eq!(
            al.mask(r"C:\Users\Al: not found"),
            "%USERPROFILE%: not found"
        );
        assert_eq!(al.mask(r"C:\Users\Al is gone"), "%USERPROFILE% is gone");
        assert_eq!(
            al.mask(r"C:\Users\Alice and C:\Users\Al"),
            r"C:\Users\Alice and %USERPROFILE%"
        );
    }

    #[test]
    fn a_trailing_separator_in_the_profile_does_not_matter() {
        assert_eq!(
            Masker::new("C:\\Users\\Alice\\").mask(r"C:\Users\Alice\x"),
            r"%USERPROFILE%\x"
        );
    }

    #[test]
    fn a_drive_root_or_empty_profile_masks_nothing() {
        assert_eq!(Masker::new(r"C:\").mask(r"C:\Games"), r"C:\Games");
        assert_eq!(Masker::new("").mask(r"C:\Games"), r"C:\Games");
        assert_eq!(Masker::none().mask(r"C:\Users\Alice"), r"C:\Users\Alice");
    }

    #[test]
    fn masking_can_be_switched_off() {
        let mut m = alice();
        assert!(m.enabled());
        m.set_enabled(false);
        assert!(!m.enabled());
        assert_eq!(m.mask(r"C:\Users\Alice\x"), r"C:\Users\Alice\x");
        m.set_enabled(true);
        assert_eq!(m.mask(r"C:\Users\Alice\x"), r"%USERPROFILE%\x");
    }

    #[test]
    fn text_without_the_profile_is_borrowed() {
        assert!(matches!(alice().mask(r"D:\Games\x"), Cow::Borrowed(_)));
    }

    // --- fakes -------------------------------------------------------------------------------------------------

    fn dt(date: Date, h: u8, mi: u8, s: u8, ms: u16, offset_minutes: i16) -> OffsetDateTime {
        let time = Time::from_hms_milli(h, mi, s, ms).expect("time");
        let offset = UtcOffset::from_whole_seconds(i32::from(offset_minutes) * 60).expect("offset");
        OffsetDateTime::new_in_offset(date, time, offset)
    }

    fn day(y: i32, m: u8, d: u8) -> Date {
        Date::from_calendar_date(y, Month::try_from(m).expect("month"), d).expect("date")
    }

    /// 2026-10-15 14:03:27.123 at UTC+3.
    fn start() -> OffsetDateTime {
        dt(day(2026, 10, 15), 14, 3, 27, 123, 180)
    }

    /// The run prefix of a logger started at [`start`].
    const RUN: &str = "mv9fkgpf";

    #[derive(Debug)]
    struct TestClock(Mutex<OffsetDateTime>);

    impl TestClock {
        fn at(t: OffsetDateTime) -> Self {
            Self(Mutex::new(t))
        }

        fn set(&self, t: OffsetDateTime) {
            *self.0.lock().expect("clock") = t;
        }
    }

    impl Clock for TestClock {
        fn now(&self) -> OffsetDateTime {
            *self.0.lock().expect("clock")
        }
    }

    /// `len` bytes of filler ending with a newline, like a log file.
    fn line_of(len: usize) -> Vec<u8> {
        let mut bytes = vec![b'x'; len.saturating_sub(1)];
        bytes.push(b'\n');
        bytes
    }

    #[derive(Debug, Default)]
    struct MemDir {
        files: Mutex<BTreeMap<String, Vec<u8>>>,
        dirs: Vec<String>,
        undeletable: Vec<String>,
        fail_writes: bool,
    }

    impl MemDir {
        fn with(self, name: &str, len: usize) -> Self {
            self.files
                .lock()
                .expect("files")
                .insert(name.to_owned(), line_of(len));
            self
        }

        fn with_dir(mut self, name: &str) -> Self {
            self.dirs.push(name.to_owned());
            self
        }

        fn undeletable(mut self, name: &str) -> Self {
            self.undeletable.push(name.to_owned());
            self
        }

        fn names(&self) -> Vec<String> {
            self.files.lock().expect("files").keys().cloned().collect()
        }

        fn text(&self, name: &str) -> String {
            let files = self.files.lock().expect("files");
            String::from_utf8(files.get(name).cloned().unwrap_or_default()).expect("utf-8")
        }

        fn lines(&self, name: &str) -> Vec<String> {
            self.text(name).lines().map(str::to_owned).collect()
        }
    }

    impl LogDir for MemDir {
        fn append_line(&self, name: &str, line: &str) -> io::Result<()> {
            if self.fail_writes {
                return Err(io::Error::other("disk full"));
            }
            let mut files = self.files.lock().expect("files");
            let file = files.entry(name.to_owned()).or_default();
            file.extend_from_slice(line.as_bytes());
            file.push(b'\n');
            Ok(())
        }

        fn list(&self) -> io::Result<Vec<DirEntry>> {
            let files = self.files.lock().expect("files");
            let mut out: Vec<DirEntry> = files
                .iter()
                .map(|(name, data)| DirEntry {
                    name: name.clone(),
                    is_dir: false,
                    len: data.len() as u64,
                })
                .collect();
            out.extend(self.dirs.iter().map(|name| DirEntry {
                name: name.clone(),
                is_dir: true,
                len: 0,
            }));
            Ok(out)
        }

        fn remove(&self, name: &str) -> io::Result<()> {
            if self.undeletable.iter().any(|n| n == name) {
                return Err(io::Error::from(io::ErrorKind::PermissionDenied));
            }
            self.files
                .lock()
                .expect("files")
                .remove(name)
                .map(|_| ())
                .ok_or_else(|| io::Error::from(io::ErrorKind::NotFound))
        }

        fn read(&self, name: &str) -> io::Result<Vec<u8>> {
            self.files
                .lock()
                .expect("files")
                .get(name)
                .cloned()
                .ok_or_else(|| io::Error::from(io::ErrorKind::NotFound))
        }
    }

    fn logger<'a>(clock: &'a TestClock, dir: &'a MemDir) -> Logger<&'a TestClock, &'a MemDir> {
        Logger::new(clock, dir, alice(), Limits::default())
    }

    fn fields(v: Value) -> Map<String, Value> {
        v.as_object().cloned().expect("object")
    }

    fn download_hop(req: LogId) -> NetworkEvent {
        NetworkEvent {
            req,
            hop: 1,
            attempt: 1,
            method: "GET".into(),
            url: "https://files.gamebanana.com/mods/x.zip".into(),
            host_source: Some(HostSource::Builtin),
            initiator: Initiator::User,
            purpose: Purpose::Download,
            status: Some(206),
            bytes: Some(1_048_576),
            ms: 184,
            range: Some("bytes=1000-".into()),
            error: None,
            blocked: None,
        }
    }

    // --- record formats ----------------------------------------------------------------------------------------

    #[test]
    fn app_record_format() {
        let (clock, dir) = (TestClock::at(start()), MemDir::default());
        let log = logger(&clock, &dir);
        let id = log
            .app(
                Level::Warn,
                r"settings file C:\Users\Alice\s.json was invalid",
                fields(json!({ "path": r"C:\Users\Alice\s.json", "line": 3 })),
            )
            .expect("written");
        assert_eq!(id.as_str(), format!("{RUN}-000001"));
        log.app(Level::Info, "started", Map::new())
            .expect("written");
        assert_eq!(
            dir.lines("app-2026-10-15.jsonl"),
            [
                format!(
                    r#"{{"ts":"2026-10-15T14:03:27.123+03:00","id":"{RUN}-000001","lvl":"warn","msg":"settings file %USERPROFILE%\\s.json was invalid","fields":{{"line":3,"path":"%USERPROFILE%\\s.json"}}}}"#
                ),
                format!(
                    r#"{{"ts":"2026-10-15T14:03:27.123+03:00","id":"{RUN}-000002","lvl":"info","msg":"started"}}"#
                ),
            ]
        );
    }

    #[test]
    fn network_record_format() {
        let (clock, dir) = (TestClock::at(start()), MemDir::default());
        let log = logger(&clock, &dir);
        let req = log.request_id();
        log.network("redirect", &download_hop(req.clone()))
            .expect("written");
        log.network(
            "request failed",
            &NetworkEvent {
                hop: 0,
                url: "https://images.example.org/a.png".into(),
                host_source: Some(HostSource::UserImage),
                initiator: Initiator::Auto,
                purpose: Purpose::DescriptionImage,
                status: None,
                bytes: None,
                ms: 30_000,
                range: None,
                error: Some("timed out".into()),
                ..download_hop(req)
            },
        )
        .expect("written");
        assert_eq!(
            dir.lines("network-2026-10-15.jsonl"),
            [
                format!(
                    r#"{{"ts":"2026-10-15T14:03:27.123+03:00","id":"{RUN}-000002","lvl":"info","msg":"redirect","req":"{RUN}-000001","hop":1,"method":"GET","url":"https://files.gamebanana.com/mods/x.zip","host_source":"builtin","initiator":"user","purpose":"download","status":206,"bytes":1048576,"ms":184,"range":"bytes=1000-"}}"#
                ),
                format!(
                    r#"{{"ts":"2026-10-15T14:03:27.123+03:00","id":"{RUN}-000003","lvl":"error","msg":"request failed","req":"{RUN}-000001","hop":0,"method":"GET","url":"https://images.example.org/a.png","host_source":"user-image","initiator":"auto","purpose":"description-image","ms":30000,"error":"timed out"}}"#
                ),
            ]
        );
    }

    #[test]
    fn files_record_format() {
        let (clock, dir) = (TestClock::at(start()), MemDir::default());
        let log = logger(&clock, &dir);
        log.file(
            "mod file placed",
            &FileEvent {
                src: Some(r"C:\Users\Alice\deadloader-data\library\a.vpk".into()),
                dst: Some(r"D:\Steam\citadel\addons\pak01_dir.vpk".into()),
                bytes: Some(42),
                sha256: Some([0xab; 32]),
                txn: Some(7),
                ..FileEvent::new(FileOp::Hardlink)
            },
        )
        .expect("written");
        log.file(
            "could not delete file",
            &FileEvent {
                src: Some(r"D:\Steam\citadel\addons\pak02_dir.vpk".into()),
                result: Outcome::Failed,
                error: Some("Access is denied. (os error 5)".into()),
                ..FileEvent::new(FileOp::Delete)
            },
        )
        .expect("written");
        let hash = "ab".repeat(32);
        assert_eq!(
            dir.lines("files-2026-10-15.jsonl"),
            [
                format!(
                    r#"{{"ts":"2026-10-15T14:03:27.123+03:00","id":"{RUN}-000001","lvl":"info","msg":"mod file placed","op":"hardlink","src":"%USERPROFILE%\\deadloader-data\\library\\a.vpk","dst":"D:\\Steam\\citadel\\addons\\pak01_dir.vpk","bytes":42,"sha256":"{hash}","txn":7,"result":"ok"}}"#
                ),
                format!(
                    r#"{{"ts":"2026-10-15T14:03:27.123+03:00","id":"{RUN}-000002","lvl":"error","msg":"could not delete file","op":"delete","src":"D:\\Steam\\citadel\\addons\\pak02_dir.vpk","result":"failed","error":"Access is denied. (os error 5)"}}"#
                ),
            ]
        );
    }

    #[test]
    fn enum_values_are_written_as_documented() {
        let text = |v: &dyn erased::Json| v.json();
        assert_eq!(
            [
                text(&HostSource::Builtin),
                text(&HostSource::User),
                text(&HostSource::UserImage),
                text(&Initiator::Auto),
                text(&Initiator::User),
            ],
            ["builtin", "user", "user-image", "auto", "user"]
        );
        assert_eq!(
            [
                text(&Purpose::Catalog),
                text(&Purpose::Thumbnail),
                text(&Purpose::DescriptionImage),
                text(&Purpose::Download),
                text(&Purpose::UpdateCheck),
            ],
            [
                "catalog",
                "thumbnail",
                "description-image",
                "download",
                "update-check"
            ]
        );
        assert_eq!(
            [
                text(&FileOp::Hardlink),
                text(&FileOp::Copy),
                text(&FileOp::Rename),
                text(&FileOp::Delete),
                text(&FileOp::Write),
                text(&FileOp::Backup),
                text(&FileOp::Restore),
            ],
            [
                "hardlink", "copy", "rename", "delete", "write", "backup", "restore"
            ]
        );
        assert_eq!(
            [
                text(&Level::Debug),
                text(&Level::Info),
                text(&Level::Warn),
                text(&Level::Error),
                text(&Outcome::Ok),
                text(&Outcome::Failed),
            ],
            ["debug", "info", "warn", "error", "ok", "failed"]
        );
    }

    mod erased {
        pub trait Json {
            fn json(&self) -> String;
        }

        impl<T: serde::Serialize> Json for T {
            fn json(&self) -> String {
                serde_json::to_value(self)
                    .expect("serializable")
                    .as_str()
                    .expect("a string")
                    .to_owned()
            }
        }
    }

    #[test]
    fn timestamps_carry_the_local_offset_and_milliseconds() {
        assert_eq!(
            rfc3339(&dt(day(2026, 1, 2), 3, 4, 5, 6, -330)),
            "2026-01-02T03:04:05.006-05:30"
        );
        assert_eq!(
            rfc3339(&dt(day(2026, 12, 31), 23, 59, 59, 999, 0)),
            "2026-12-31T23:59:59.999+00:00"
        );
    }

    #[test]
    fn written_records_read_back() {
        let (clock, dir) = (TestClock::at(start()), MemDir::default());
        let log = logger(&clock, &dir);
        let req = log.request_id();
        let id = log
            .network("redirect", &download_hop(req.clone()))
            .expect("written");
        let read: ReadLog<NetworkEvent> =
            read_log(&dir, Kind::Network, day(2026, 10, 15)).expect("read");
        assert_eq!(
            read.records,
            [Record {
                ts: "2026-10-15T14:03:27.123+03:00".into(),
                id,
                lvl: Level::Info,
                msg: "redirect".into(),
                body: download_hop(req),
            }]
        );
        let event = FileEvent {
            sha256: Some([7; 32]),
            ..FileEvent::new(FileOp::Copy)
        };
        log.file("copied", &event).expect("written");
        let read: ReadLog<FileEvent> =
            read_log(&dir, Kind::Files, day(2026, 10, 15)).expect("read");
        assert_eq!(read.records.first().map(|r| &r.body), Some(&event));
    }

    #[test]
    fn refused_hops_retries_and_the_update_check_are_logged() {
        let (clock, dir) = (TestClock::at(start()), MemDir::default());
        let log = logger(&clock, &dir);
        let req = log.request_id();
        // Refused before any byte left the machine: no host source, the reason instead, level warn.
        log.network(
            "request blocked",
            &NetworkEvent {
                hop: 0,
                url: "https://evil.example/x.zip".into(),
                host_source: None,
                status: None,
                bytes: None,
                ms: 0,
                range: None,
                blocked: Some("host-not-listed".into()),
                ..download_hop(req.clone())
            },
        )
        .expect("written");
        // The second try of a hop after a 503; the update check's own host source.
        log.network(
            "response",
            &NetworkEvent {
                hop: 0,
                attempt: 2,
                url: "https://api.github.com/repos/horexdev/deadloader/releases/latest".into(),
                host_source: Some(HostSource::Github),
                initiator: Initiator::Auto,
                purpose: Purpose::UpdateCheck,
                status: Some(200),
                bytes: None,
                range: None,
                ..download_hop(req)
            },
        )
        .expect("written");
        assert_eq!(
            dir.lines("network-2026-10-15.jsonl"),
            [
                format!(
                    r#"{{"ts":"2026-10-15T14:03:27.123+03:00","id":"{RUN}-000002","lvl":"warn","msg":"request blocked","req":"{RUN}-000001","hop":0,"method":"GET","url":"https://evil.example/x.zip","initiator":"user","purpose":"download","ms":0,"blocked":"host-not-listed"}}"#
                ),
                format!(
                    r#"{{"ts":"2026-10-15T14:03:27.123+03:00","id":"{RUN}-000003","lvl":"info","msg":"response","req":"{RUN}-000001","hop":0,"attempt":2,"method":"GET","url":"https://api.github.com/repos/horexdev/deadloader/releases/latest","host_source":"github","initiator":"auto","purpose":"update-check","status":200,"ms":184}}"#
                ),
            ]
        );
    }

    // --- masking in records ------------------------------------------------------------------------------------

    #[test]
    fn every_string_field_is_masked() {
        let (clock, dir) = (TestClock::at(start()), MemDir::default());
        let log = logger(&clock, &dir);
        let req = log.request_id();
        log.network(
            r"open C:\Users\Alice\x",
            &NetworkEvent {
                url: "file:///C:/Users/Alice/x.png".into(),
                range: Some(r"C:\Users\Alice".into()),
                error: Some(r"cannot read C:\Users\Alice\x.png".into()),
                ..download_hop(req)
            },
        )
        .expect("written");
        log.file(
            r"copy C:\Users\Alice\a",
            &FileEvent {
                src: Some(r"C:\Users\Alice\a".into()),
                dst: Some("C:/Users/Alice/b".into()),
                result: Outcome::Failed,
                error: Some(r#"Os { path: "C:\\Users\\Alice\\b" }"#.into()),
                ..FileEvent::new(FileOp::Copy)
            },
        )
        .expect("written");
        log.app(
            Level::Info,
            "program started",
            fields(json!({
                "exe": r"C:\Users\Alice\Desktop\deadloader.exe",
                "args": [r"--data=C:\Users\Alice\d", 3],
                "env": { "WEBVIEW2_USER_DATA_FOLDER": r"C:\Users\Alice\d\webview" },
                r"C:\Users\Alice\key": true,
            })),
        )
        .expect("written");
        for name in dir.names() {
            let text = dir.text(&name);
            assert!(!text.to_lowercase().contains("alice"), "{name}: {text}");
            assert!(text.contains(MASK), "{name}");
        }
        assert_eq!(
            dir.text("network-2026-10-15.jsonl").matches(MASK).count(),
            4
        );
        assert_eq!(dir.text("files-2026-10-15.jsonl").matches(MASK).count(), 4);
        assert_eq!(dir.text("app-2026-10-15.jsonl").matches(MASK).count(), 4);
    }

    #[test]
    fn masking_can_be_switched_off_while_running() {
        let (clock, dir) = (TestClock::at(start()), MemDir::default());
        let log = logger(&clock, &dir);
        log.app(Level::Info, r"C:\Users\Alice\a", Map::new())
            .expect("written");
        log.set_masking(false);
        log.app(Level::Info, r"C:\Users\Alice\b", Map::new())
            .expect("written");
        log.set_masking(true);
        log.app(Level::Info, r"C:\Users\Alice\c", Map::new())
            .expect("written");
        let msgs: Vec<String> = parse::<AppEvent>(dir.text("app-2026-10-15.jsonl").as_bytes())
            .records
            .into_iter()
            .map(|r| r.msg)
            .collect();
        assert_eq!(
            msgs,
            [r"%USERPROFILE%\a", r"C:\Users\Alice\b", r"%USERPROFILE%\c"]
        );
    }

    // --- ids ---------------------------------------------------------------------------------------------------

    #[test]
    fn ids_are_unique_and_increasing() {
        let (clock, dir) = (TestClock::at(start()), MemDir::default());
        let log = logger(&clock, &dir);
        let mut ids = Vec::new();
        for i in 0..5 {
            ids.push(log.app(Level::Info, "a", Map::new()).expect("written"));
            let req = log.request_id();
            ids.push(req.clone());
            ids.push(log.network("n", &download_hop(req)).expect("written"));
            ids.push(
                log.file("f", &FileEvent::new(FileOp::Write))
                    .expect("written"),
            );
            if i == 2 {
                clock.set(dt(day(2026, 10, 16), 0, 0, 1, 0, 180));
            }
        }
        let numbers: Vec<u64> = ids
            .iter()
            .map(|id| {
                let (run, n) = id.as_str().split_once('-').expect("run-counter");
                assert_eq!(run, RUN);
                n.parse().expect("number")
            })
            .collect();
        assert!(numbers.windows(2).all(|w| w[0] < w[1]), "{numbers:?}");

        let later_dir = MemDir::default();
        let later_clock = TestClock::at(dt(day(2026, 10, 15), 14, 3, 27, 124, 180));
        let later = logger(&later_clock, &later_dir);
        let first_later = later.app(Level::Info, "a", Map::new()).expect("written");
        assert!(ids.iter().all(|id| *id < first_later));
    }

    #[test]
    fn a_failed_write_reports_the_id_and_does_not_reuse_it() {
        let clock = TestClock::at(start());
        let dir = MemDir {
            fail_writes: true,
            ..MemDir::default()
        };
        let log = logger(&clock, &dir);
        let err = log
            .app(Level::Error, "x", Map::new())
            .expect_err("disk full");
        assert_eq!(err.id().as_str(), format!("{RUN}-000001"));
        assert!(err.to_string().contains(&format!("{RUN}-000001")));
        assert!(matches!(err, LogError::Write { .. }));
        assert_eq!(log.request_id().as_str(), format!("{RUN}-000002"));
    }

    #[test]
    fn the_logger_can_be_shared_between_threads() {
        fn shareable<T: Send + Sync>() {}
        shareable::<Logger<SystemClock, DiskLogDir>>();
    }

    // --- days and rotation -------------------------------------------------------------------------------------

    #[test]
    fn a_new_day_starts_a_new_file() {
        let clock = TestClock::at(dt(day(2026, 10, 15), 23, 59, 59, 999, 180));
        let dir = MemDir::default();
        let log = logger(&clock, &dir);
        log.app(Level::Info, "before midnight", Map::new())
            .expect("written");
        clock.set(dt(day(2026, 10, 16), 0, 0, 0, 0, 180));
        log.app(Level::Info, "after midnight", Map::new())
            .expect("written");
        // The local date names the file, not the UTC one (this is still 10-15 in UTC).
        clock.set(dt(day(2026, 10, 16), 1, 0, 0, 0, 180));
        log.app(Level::Info, "later", Map::new()).expect("written");
        assert_eq!(
            dir.names(),
            ["app-2026-10-15.jsonl", "app-2026-10-16.jsonl"]
        );
        assert_eq!(dir.lines("app-2026-10-15.jsonl").len(), 1);
        assert_eq!(dir.lines("app-2026-10-16.jsonl").len(), 2);
    }

    #[test]
    fn retention_goes_by_the_date_in_the_name() {
        let clock = TestClock::at(start());
        let dir = MemDir::default()
            .with("app-2026-09-15.jsonl", 10)
            .with("network-2026-09-16.jsonl", 10)
            .with("files-2025-01-01.jsonl", 10)
            .with("app-2026-10-15.jsonl", 10)
            .with("app-2026-12-01.jsonl", 10);
        let _log = logger(&clock, &dir);
        assert_eq!(
            dir.names(),
            [
                "app-2026-10-15.jsonl",
                "app-2026-12-01.jsonl",
                "network-2026-09-16.jsonl"
            ]
        );
        let lines = parse::<AppEvent>(dir.text("app-2026-10-15.jsonl").as_bytes()).records;
        let deleted: Vec<(&str, &str, &str)> = lines
            .iter()
            .map(|r| {
                (
                    r.msg.as_str(),
                    r.body.fields["file"].as_str().unwrap_or_default(),
                    r.body.fields["reason"].as_str().unwrap_or_default(),
                )
            })
            .collect();
        assert_eq!(
            deleted,
            [
                (
                    "old log file deleted",
                    "files-2025-01-01.jsonl",
                    "retention"
                ),
                ("old log file deleted", "app-2026-09-15.jsonl", "retention"),
            ]
        );
    }

    #[test]
    fn retention_also_runs_on_the_first_write_of_a_new_day() {
        let clock = TestClock::at(start());
        let dir = MemDir::default().with("network-2026-09-16.jsonl", 10);
        let log = logger(&clock, &dir);
        log.app(Level::Info, "a", Map::new()).expect("written");
        assert!(dir.names().contains(&"network-2026-09-16.jsonl".to_owned()));
        clock.set(dt(day(2026, 10, 16), 0, 0, 0, 5, 180));
        log.app(Level::Info, "b", Map::new()).expect("written");
        assert!(!dir.names().contains(&"network-2026-09-16.jsonl".to_owned()));
        let msgs: Vec<String> = parse::<AppEvent>(dir.text("app-2026-10-16.jsonl").as_bytes())
            .records
            .into_iter()
            .map(|r| r.msg)
            .collect();
        assert_eq!(msgs, ["old log file deleted", "b"]);
    }

    #[test]
    fn limits_are_configurable() {
        let clock = TestClock::at(start());
        let dir = MemDir::default()
            .with("app-2026-10-08.jsonl", 10)
            .with("app-2026-10-09.jsonl", 10)
            .with("app-2026-10-10.jsonl", 10);
        let log = Logger::new(
            &clock,
            &dir,
            alice(),
            Limits {
                retention_days: 7,
                ..Limits::default()
            },
        );
        assert!(!dir.names().contains(&"app-2026-10-08.jsonl".to_owned()));
        assert!(dir.names().contains(&"app-2026-10-09.jsonl".to_owned()));
        // Today's file counts towards the cap too.
        let today = dir.text("app-2026-10-15.jsonl").len() as u64;
        log.set_limits(Limits {
            retention_days: 30,
            max_bytes: today + 15,
        });
        assert!(!dir.names().contains(&"app-2026-10-09.jsonl".to_owned()));
        assert!(dir.names().contains(&"app-2026-10-10.jsonl".to_owned()));
    }

    #[test]
    fn the_size_cap_removes_the_oldest_files_first() {
        let clock = TestClock::at(start());
        // Name order differs from date order.
        let dir = MemDir::default()
            .with("app-2026-10-12.jsonl", 40)
            .with("files-2026-10-10.jsonl", 50)
            .with("network-2026-10-11.jsonl", 30)
            .with("app-2026-10-15.jsonl", 20);
        let _log = Logger::new(
            &clock,
            &dir,
            alice(),
            Limits {
                max_bytes: 100,
                ..Limits::default()
            },
        );
        assert_eq!(
            dir.names(),
            [
                "app-2026-10-12.jsonl",
                "app-2026-10-15.jsonl",
                "network-2026-10-11.jsonl"
            ]
        );
        let reasons: Vec<String> = parse::<AppEvent>(dir.text("app-2026-10-15.jsonl").as_bytes())
            .records
            .iter()
            .map(|r| format!("{} {}", r.body.fields["file"], r.body.fields["reason"]))
            .collect();
        assert_eq!(reasons, [r#""files-2026-10-10.jsonl" "size-cap""#]);
    }

    #[test]
    fn the_size_cap_never_deletes_todays_files() {
        let clock = TestClock::at(start());
        let dir = MemDir::default()
            .with("app-2026-10-14.jsonl", 10)
            .with("network-2026-10-15.jsonl", 500);
        let _log = Logger::new(
            &clock,
            &dir,
            alice(),
            Limits {
                max_bytes: 100,
                ..Limits::default()
            },
        );
        assert_eq!(
            dir.names(),
            ["app-2026-10-15.jsonl", "network-2026-10-15.jsonl"]
        );
    }

    #[test]
    fn foreign_files_are_never_deleted_or_counted() {
        let clock = TestClock::at(start());
        let foreign = [
            "notes.txt",
            "app-2020-01-01.jsonl.bak",
            "APP-2020-01-01.jsonl",
            "crash-2020-01-01.jsonl",
            "app-2020-13-01.jsonl",
            "app-2020-02-30.jsonl",
            "app-2020-1-01.jsonl",
            "app-20200101.jsonl",
            "app-2020-01-01.json",
            "app-+020-01-01.jsonl",
        ];
        let mut dir = MemDir::default()
            .with("big.dmp", 10_000)
            .with("app-2026-10-14.jsonl", 10);
        for name in foreign {
            dir = dir.with(name, 10);
        }
        let dir = dir.with_dir("files-2020-01-01.jsonl");
        let _log = Logger::new(
            &clock,
            &dir,
            alice(),
            Limits {
                max_bytes: 100,
                ..Limits::default()
            },
        );
        let names = dir.names();
        for name in foreign.iter().chain(&["big.dmp", "app-2026-10-14.jsonl"]) {
            assert!(names.contains(&(*name).to_owned()), "{name} was deleted");
        }
        assert!(
            !names.contains(&"app-2026-10-15.jsonl".to_owned()),
            "nothing to report"
        );
    }

    #[test]
    fn a_file_that_cannot_be_deleted_is_reported_and_the_rest_continue() {
        let clock = TestClock::at(start());
        let dir = MemDir::default()
            .with("app-2020-01-01.jsonl", 10)
            .with("network-2020-01-01.jsonl", 10)
            .undeletable("app-2020-01-01.jsonl");
        let _log = logger(&clock, &dir);
        assert!(dir.names().contains(&"app-2020-01-01.jsonl".to_owned()));
        assert!(!dir.names().contains(&"network-2020-01-01.jsonl".to_owned()));
        let records = parse::<AppEvent>(dir.text("app-2026-10-15.jsonl").as_bytes()).records;
        let summary: Vec<(Level, &str, bool)> = records
            .iter()
            .map(|r| (r.lvl, r.msg.as_str(), r.body.fields.contains_key("error")))
            .collect();
        assert_eq!(
            summary,
            [
                (Level::Warn, "could not delete old log file", true),
                (Level::Info, "old log file deleted", false),
            ]
        );
    }

    #[test]
    fn file_names_parse_back() {
        let date = day(2026, 3, 7);
        for kind in Kind::ALL {
            assert_eq!(
                Kind::parse_file_name(&kind.file_name(date)),
                Some((kind, date))
            );
        }
        assert_eq!(Kind::Network.file_name(date), "network-2026-03-07.jsonl");
    }

    #[test]
    fn log_files_lists_ours_oldest_first() {
        let dir = MemDir::default()
            .with("network-2026-10-14.jsonl", 3)
            .with("app-2026-10-15.jsonl", 1)
            .with("app-2026-10-14.jsonl", 2)
            .with("notes.txt", 4);
        let files = log_files(&dir).expect("listed");
        let names: Vec<(&str, u64)> = files.iter().map(|f| (f.name.as_str(), f.bytes)).collect();
        assert_eq!(
            names,
            [
                ("app-2026-10-14.jsonl", 2),
                ("network-2026-10-14.jsonl", 3),
                ("app-2026-10-15.jsonl", 1),
            ]
        );
        assert_eq!(
            files.first().map(|f| (f.kind, f.date)),
            Some((Kind::App, day(2026, 10, 14)))
        );
    }

    // --- reader ------------------------------------------------------------------------------------------------

    fn app_line(id: &str) -> String {
        format!(r#"{{"ts":"2026-10-15T14:03:27.123+03:00","id":"{id}","lvl":"info","msg":"m"}}"#)
    }

    fn ids(read: &ReadLog<AppEvent>) -> Vec<&str> {
        read.records.iter().map(|r| r.id.as_str()).collect()
    }

    #[test]
    fn the_reader_skips_a_damaged_last_line() {
        let text = format!("{}\n{}\n{{\"ts\":\"2026-", app_line("a"), app_line("b"));
        let read = parse::<AppEvent>(text.as_bytes());
        assert_eq!(ids(&read), ["a", "b"]);
        assert!(read.torn_tail);
        assert!(read.damaged.is_empty());

        // Cut inside a multi-byte character.
        let mut bytes = format!("{}\n{{\"msg\":\"", app_line("a")).into_bytes();
        bytes.push(0xd0);
        let read = parse::<AppEvent>(&bytes);
        assert_eq!(ids(&read), ["a"]);
        assert!(read.torn_tail);
    }

    #[test]
    fn the_reader_reports_damaged_lines_elsewhere() {
        let text = format!(
            "{}\ngarbage\n{}\n{{\"valid\":\"json, wrong shape\"}}\n{}\n",
            app_line("a"),
            app_line("b"),
            app_line("c")
        );
        let read = parse::<AppEvent>(text.as_bytes());
        assert_eq!(ids(&read), ["a", "b", "c"]);
        assert!(!read.torn_tail);
        let lines: Vec<usize> = read.damaged.iter().map(|d| d.line).collect();
        assert_eq!(lines, [2, 4]);
        assert!(read.damaged.iter().all(|d| !d.error.is_empty()));
    }

    #[test]
    fn the_reader_accepts_crlf_blank_lines_and_a_missing_final_newline() {
        let text = format!("{}\r\n\n{}", app_line("a"), app_line("b"));
        let read = parse::<AppEvent>(text.as_bytes());
        assert_eq!(ids(&read), ["a", "b"]);
        assert!(!read.torn_tail && read.damaged.is_empty());
        let empty = parse::<AppEvent>(b"");
        assert!(empty.records.is_empty() && !empty.torn_tail && empty.damaged.is_empty());
    }

    // --- disk --------------------------------------------------------------------------------------------------

    fn temp_dir(name: &str) -> PathBuf {
        let dir =
            std::env::temp_dir().join(format!("deadloader-log-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        dir
    }

    #[test]
    fn disk_dir_appends_lines_creating_the_folder() {
        let root = temp_dir("append");
        let dir = DiskLogDir::new(root.join("logs"));
        assert_eq!(
            dir.list().map_err(|e| e.kind()).err(),
            Some(io::ErrorKind::NotFound)
        );
        dir.append_line("app-2026-10-15.jsonl", "one")
            .expect("append");
        dir.append_line("app-2026-10-15.jsonl", "two")
            .expect("append");
        dir.append_line("app-2026-10-16.jsonl", "three")
            .expect("append");
        dir.append_line("network-2026-10-16.jsonl", "four")
            .expect("append");
        assert_eq!(
            dir.read("app-2026-10-15.jsonl").expect("read"),
            b"one\ntwo\n"
        );
        assert_eq!(dir.read("app-2026-10-16.jsonl").expect("read"), b"three\n");
        let mut listed: Vec<(String, u64, bool)> = dir
            .list()
            .expect("list")
            .into_iter()
            .map(|e| (e.name, e.len, e.is_dir))
            .collect();
        listed.sort();
        assert_eq!(
            listed,
            [
                ("app-2026-10-15.jsonl".to_owned(), 8, false),
                ("app-2026-10-16.jsonl".to_owned(), 6, false),
                ("network-2026-10-16.jsonl".to_owned(), 5, false),
            ]
        );
        std::fs::remove_dir_all(&root).expect("cleanup");
    }

    #[test]
    fn disk_dir_starts_a_fresh_line_after_a_torn_one() {
        let root = temp_dir("torn");
        std::fs::create_dir_all(&root).expect("dir");
        std::fs::write(root.join("app-2026-10-15.jsonl"), b"ok\n{\"ts\":").expect("seed");
        let dir = DiskLogDir::new(root.clone());
        dir.append_line("app-2026-10-15.jsonl", "next")
            .expect("append");
        assert_eq!(
            dir.read("app-2026-10-15.jsonl").expect("read"),
            b"ok\n{\"ts\":\nnext\n"
        );
        std::fs::remove_dir_all(&root).expect("cleanup");
    }

    #[test]
    fn disk_dir_removes_an_open_file_and_recreates_it() {
        let root = temp_dir("remove");
        let dir = DiskLogDir::new(root.clone());
        dir.append_line("app-2026-10-15.jsonl", "one")
            .expect("append");
        dir.remove("app-2026-10-15.jsonl").expect("remove");
        assert_eq!(
            dir.read("app-2026-10-15.jsonl").map_err(|e| e.kind()).err(),
            Some(io::ErrorKind::NotFound)
        );
        dir.append_line("app-2026-10-15.jsonl", "two")
            .expect("append");
        assert_eq!(dir.read("app-2026-10-15.jsonl").expect("read"), b"two\n");
        std::fs::remove_dir_all(&root).expect("cleanup");
    }

    #[test]
    fn disk_dir_reads_part_of_a_file_and_its_length() {
        let root = temp_dir("range");
        let dir = DiskLogDir::new(root.clone());
        dir.append_line("app-2026-10-15.jsonl", "one")
            .expect("append");
        dir.append_line("app-2026-10-15.jsonl", "two")
            .expect("append");
        assert_eq!(dir.file_len("app-2026-10-15.jsonl").expect("len"), 8);
        assert_eq!(
            dir.read_range("app-2026-10-15.jsonl", 4, 3).expect("range"),
            b"two"
        );
        // Past the end: what is there.
        assert_eq!(
            dir.read_range("app-2026-10-15.jsonl", 6, 100)
                .expect("range"),
            b"o\n"
        );
        assert_eq!(
            dir.file_len("files-2026-10-15.jsonl")
                .map_err(|e| e.kind())
                .err(),
            Some(io::ErrorKind::NotFound)
        );
        std::fs::remove_dir_all(&root).expect("cleanup");
    }

    #[test]
    fn any_log_dir_can_read_ranges_through_the_whole_file() {
        let dir = MemDir::default();
        dir.append_line("app-2026-10-15.jsonl", "abcdef")
            .expect("append");
        assert_eq!(dir.file_len("app-2026-10-15.jsonl").expect("len"), 7);
        assert_eq!(
            dir.read_range("app-2026-10-15.jsonl", 2, 3).expect("range"),
            b"cde"
        );
        assert_eq!(
            dir.read_range("app-2026-10-15.jsonl", 9, 3).expect("range"),
            b""
        );
    }

    #[test]
    fn a_watcher_hears_every_written_line() {
        let (clock, dir) = (TestClock::at(start()), MemDir::default());
        let log = logger(&clock, &dir);
        let heard = std::sync::Arc::new(Mutex::new(Vec::new()));
        let sink = std::sync::Arc::clone(&heard);
        log.watch(Box::new(move |kind| sink.lock().expect("heard").push(kind)));
        log.app(Level::Info, "a", Map::new()).expect("written");
        let req = log.request_id();
        log.network("n", &download_hop(req)).expect("written");
        log.file("f", &FileEvent::new(FileOp::Write))
            .expect("written");
        assert_eq!(
            *heard.lock().expect("heard"),
            [Kind::App, Kind::Network, Kind::Files]
        );
    }

    #[test]
    fn a_failed_write_is_not_reported_to_the_watcher() {
        let clock = TestClock::at(start());
        let dir = MemDir {
            fail_writes: true,
            ..MemDir::default()
        };
        let log = logger(&clock, &dir);
        let heard = std::sync::Arc::new(Mutex::new(0));
        let sink = std::sync::Arc::clone(&heard);
        log.watch(Box::new(move |_| *sink.lock().expect("heard") += 1));
        let _ = log.app(Level::Info, "a", Map::new());
        assert_eq!(*heard.lock().expect("heard"), 0);
    }

    #[test]
    fn file_operations_of_one_change_carry_its_transaction() {
        let mem = MemFileLog::default();
        let change = InChange { log: &mem, txn: 42 };
        change.record("placed", &FileEvent::new(FileOp::Hardlink));
        assert_eq!(mem.events()[0].1.txn, Some(42));
        assert_eq!(mem.events()[0].0, "placed");
    }

    #[test]
    fn kinds_are_named_like_their_files() {
        assert_eq!(
            serde_json::to_value(Kind::ALL).expect("json"),
            json!(["app", "network", "files"])
        );
    }

    #[test]
    fn logger_on_disk_with_the_system_clock() {
        let root = temp_dir("e2e");
        let log = Logger::new(
            SystemClock,
            DiskLogDir::new(root.clone()),
            Masker::new(r"C:\Users\Alice"),
            Limits::default(),
        );
        let id = log
            .app(
                Level::Info,
                r"started from C:\Users\Alice\Desktop",
                Map::new(),
            )
            .expect("written");
        let files = log_files(log.dir()).expect("listed");
        assert_eq!(files.len(), 1);
        let read: ReadLog<AppEvent> = read_log(log.dir(), Kind::App, files[0].date).expect("read");
        assert_eq!(read.records.len(), 1);
        assert_eq!(read.records[0].id, id);
        assert_eq!(read.records[0].msg, r"started from %USERPROFILE%\Desktop");
        drop(log);
        std::fs::remove_dir_all(&root).expect("cleanup");
    }
}
