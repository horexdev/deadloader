//! The on-disk cache (decision 58, ADR 0003): catalog pages and mod cards in `cache\catalog\`, thumbnails in
//! `cache\thumbs\`, description images in `cache\images\`. Everything in it can be deleted at any time; losing it only
//! costs requests. No database: every entry is one file.
//!
//! **Images** ([`BlobCache`]) are stored as plain image files named after the SHA-256 of their URL, in folders named
//! after its first two hex digits: `thumbs\3f\3f9c…e1.webp`. At the 100 MB limit a folder would otherwise hold
//! 2 000–7 000 thumbnails; split 256 ways each holds a few dozen, which keeps Explorer, on-access antivirus scans and
//! 8.3 short-name generation (still on for system drives on many machines, and slow with many similar long names in
//! one folder) cheap. The cost is one folder creation per new shard and up to 257 listings when the cache opens.
//!
//! There is no index file: the folder is the index. Opening lists it once (on Windows a listing carries each file's
//! size and modification time, so no file is opened) and keeps the index in memory; callers open the cache off the
//! UI thread. The last access of an entry is its file's modification time, set explicitly on every hit. Not the
//! last-access time: NTFS updates that lazily, up to an hour late, and it is switched off on many systems. Not an
//! index file: it would have to be rewritten or journaled on every store and hit and could drift from the files after
//! a crash; the modification time cannot drift, survives copying the portable folder (Explorer and robocopy keep
//! it), and a hit costs one `SetFileTime` through a handle opened only to write attributes, so the content is never
//! reopened for writing and scanners have nothing to rescan.
//!
//! Writes go to a temporary file in the same folder (`<name>.<n>.tmp`), are flushed to the disk, and are then renamed
//! over the target, so a crash leaves the old entry or the new one, never a torn file under a real name. Leftover
//! temporary files are deleted when the cache opens and by [`Cache::clear`]. Every read checks that the size matches
//! the index and that the bytes start with the signature of the format in the file name; anything else is deleted and
//! reported as a miss. Only JPEG, PNG, GIF and WebP are stored (no SVG: it can carry script). The declared content
//! type must be one of them, and the type stored is the one detected from the bytes, so a WebP mislabelled as JPEG is
//! still served correctly and an HTML error page labelled as PNG is refused.
//!
//! When an area is over its limit, the least recently used entries go first (oldest access time; within one run,
//! equal times fall back to the order of access), but never the entry just stored. An entry larger than the per-entry
//! cap is not stored at all, so one huge image cannot flush the cache.
//!
//! **Catalog** ([`CatalogCache`]) entries are small JSON files that name what they hold: `page-<hash>.json` for a
//! catalog page (the hash of its request parameters, which are also stored inside) and `card-mod-655209.json` for a
//! mod's card. A page is [`Lookup::Fresh`] for the TTL (10 minutes by default) after it was fetched and
//! [`Lookup::Stale`] afterwards; stale pages are still returned, with their fetch time and age, to show while
//! revalidating and when offline (decision 98). A fetch time in the future (the clock was set back) counts as stale.
//! A card stays current while the listing shows no newer update date than the one stored with it. The catalog area
//! has its own size limit; the oldest written files go first.
//!
//! Every file written or deleted is reported to a [`FileLog`] (the files log in the app). The time stamp a hit sets is
//! not: it would be one line per thumbnail shown.
//!
//! Concurrency: deadloader runs as a single instance (decision 92), so there is no locking between processes. Within
//! the process every area is `Sync`. Reading and writing file contents happen outside the area's lock; renames, deletes
//! and index changes happen under it, so the index and the folder agree.

use crate::fs::sha256;
use crate::library::Section;
pub use crate::log::FileLog;
use crate::log::{Clock, FileEvent, FileOp, Outcome};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, MutexGuard, PoisonError};
use std::time::{Duration, SystemTime};
use time::OffsetDateTime;

/// Version of the catalog file format. Files of a newer version are ignored (and replaced on the next fetch).
pub const SCHEMA: u32 = 1;

/// The SHA-256 of an entry's URL.
type Key = [u8; 32];

// ---- filesystem and log

/// One entry of a folder listing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub name: String,
    pub is_dir: bool,
    pub len: u64,
    pub modified: SystemTime,
}

/// The file access the cache needs.
pub trait CacheFs {
    fn read(&self, path: &Path) -> io::Result<Vec<u8>>;
    /// Creates or truncates `path`, writes `bytes`, sets the modification time and flushes it all to the disk.
    fn write_synced(&self, path: &Path, bytes: &[u8], modified: SystemTime) -> io::Result<()>;
    /// Renames `from` to `to`, replacing `to` if it exists.
    fn replace(&self, from: &Path, to: &Path) -> io::Result<()>;
    fn set_modified(&self, path: &Path, modified: SystemTime) -> io::Result<()>;
    fn remove_file(&self, path: &Path) -> io::Result<()>;
    fn create_dir_all(&self, path: &Path) -> io::Result<()>;
    /// Removes an empty folder; fails if anything is in it.
    fn remove_dir(&self, path: &Path) -> io::Result<()>;
    /// The entries directly inside `dir`; `NotFound` if it doesn't exist.
    fn list(&self, dir: &Path) -> io::Result<Vec<Entry>>;
}

impl<T: CacheFs + ?Sized> CacheFs for &T {
    fn read(&self, path: &Path) -> io::Result<Vec<u8>> {
        (**self).read(path)
    }

    fn write_synced(&self, path: &Path, bytes: &[u8], modified: SystemTime) -> io::Result<()> {
        (**self).write_synced(path, bytes, modified)
    }

    fn replace(&self, from: &Path, to: &Path) -> io::Result<()> {
        (**self).replace(from, to)
    }

    fn set_modified(&self, path: &Path, modified: SystemTime) -> io::Result<()> {
        (**self).set_modified(path, modified)
    }

    fn remove_file(&self, path: &Path) -> io::Result<()> {
        (**self).remove_file(path)
    }

    fn create_dir_all(&self, path: &Path) -> io::Result<()> {
        (**self).create_dir_all(path)
    }

    fn remove_dir(&self, path: &Path) -> io::Result<()> {
        (**self).remove_dir(path)
    }

    fn list(&self, dir: &Path) -> io::Result<Vec<Entry>> {
        (**self).list(dir)
    }
}

/// The real filesystem.
#[derive(Debug, Default, Clone, Copy)]
pub struct DiskCacheFs;

impl CacheFs for DiskCacheFs {
    fn read(&self, path: &Path) -> io::Result<Vec<u8>> {
        std::fs::read(path)
    }

    fn write_synced(&self, path: &Path, bytes: &[u8], modified: SystemTime) -> io::Result<()> {
        let mut file = std::fs::File::create(path)?;
        file.write_all(bytes)?;
        file.set_modified(modified)?;
        file.sync_all()
    }

    fn replace(&self, from: &Path, to: &Path) -> io::Result<()> {
        // Replaces an existing target on Windows too (MoveFileEx with MOVEFILE_REPLACE_EXISTING).
        std::fs::rename(from, to)
    }

    fn set_modified(&self, path: &Path, modified: SystemTime) -> io::Result<()> {
        open_for_attributes(path)?.set_modified(modified)
    }

    fn remove_file(&self, path: &Path) -> io::Result<()> {
        std::fs::remove_file(path)
    }

    fn create_dir_all(&self, path: &Path) -> io::Result<()> {
        std::fs::create_dir_all(path)
    }

    fn remove_dir(&self, path: &Path) -> io::Result<()> {
        std::fs::remove_dir(path)
    }

    fn list(&self, dir: &Path) -> io::Result<Vec<Entry>> {
        let mut out = Vec::new();
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let meta = entry.metadata()?;
            out.push(Entry {
                name: entry.file_name().to_string_lossy().into_owned(),
                is_dir: meta.is_dir(),
                len: meta.len(),
                modified: meta.modified()?,
            });
        }
        Ok(out)
    }
}

/// A handle that may change the file's time stamps but not read or write its content.
#[cfg(windows)]
fn open_for_attributes(path: &Path) -> io::Result<std::fs::File> {
    use std::os::windows::fs::OpenOptionsExt;
    /// `FILE_WRITE_ATTRIBUTES` from the Windows SDK.
    const FILE_WRITE_ATTRIBUTES: u32 = 0x0100;
    std::fs::OpenOptions::new()
        .access_mode(FILE_WRITE_ATTRIBUTES)
        .open(path)
}

#[cfg(not(windows))]
fn open_for_attributes(path: &Path) -> io::Result<std::fs::File> {
    std::fs::OpenOptions::new().write(true).open(path)
}

fn file_event(op: FileOp, path: &Path, len: Option<u64>, error: Option<&io::Error>) -> FileEvent {
    let shown = Some(path.display().to_string());
    let (src, dst) = match op {
        FileOp::Write => (None, shown),
        _ => (shown, None),
    };
    FileEvent {
        src,
        dst,
        bytes: len,
        result: if error.is_some() {
            Outcome::Failed
        } else {
            Outcome::Ok
        },
        error: error.map(ToString::to_string),
        ..FileEvent::new(op)
    }
}

/// Deletes a file and logs it; a file that is already gone counts as deleted and is not logged.
fn remove_logged(
    fs: &impl CacheFs,
    log: &impl FileLog,
    path: &Path,
    len: u64,
    msg: &str,
) -> io::Result<()> {
    match fs.remove_file(path) {
        Ok(()) => {
            log.record(msg, &file_event(FileOp::Delete, path, Some(len), None));
            Ok(())
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(e) => {
            log.record(
                "cache: could not delete a file",
                &file_event(FileOp::Delete, path, Some(len), Some(&e)),
            );
            Err(e)
        }
    }
}

/// Writes the temporary file of an entry; on failure it is removed and the failure logged.
fn write_temp(
    fs: &impl CacheFs,
    log: &impl FileLog,
    temp: &Path,
    bytes: &[u8],
    modified: SystemTime,
) -> io::Result<()> {
    fs.write_synced(temp, bytes, modified).inspect_err(|e| {
        let _ = fs.remove_file(temp);
        log.record(
            "cache: could not write an entry",
            &file_event(FileOp::Write, temp, Some(bytes.len() as u64), Some(e)),
        );
    })
}

/// Renames a written temporary file into place and logs the stored entry.
fn commit(
    fs: &impl CacheFs,
    log: &impl FileLog,
    temp: &Path,
    path: &Path,
    len: u64,
) -> io::Result<()> {
    match fs.replace(temp, path) {
        Ok(()) => {
            log.record(
                "cache: stored",
                &file_event(FileOp::Write, path, Some(len), None),
            );
            Ok(())
        }
        Err(e) => {
            let _ = fs.remove_file(temp);
            log.record(
                "cache: could not store an entry",
                &FileEvent {
                    dst: Some(path.display().to_string()),
                    ..file_event(FileOp::Rename, temp, Some(len), Some(&e))
                },
            );
            Err(e)
        }
    }
}

fn list_or_empty(fs: &impl CacheFs, dir: &Path) -> io::Result<Vec<Entry>> {
    match fs.list(dir) {
        Ok(entries) => Ok(entries),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(Vec::new()),
        Err(e) => Err(e),
    }
}

/// `<name>.<n>.tmp` next to `path`, unique within the process.
fn temp_path(path: &Path, counter: &AtomicU64) -> PathBuf {
    let n = counter.fetch_add(1, Ordering::Relaxed);
    let mut name = path.file_name().unwrap_or_default().to_os_string();
    name.push(format!(".{n}.tmp"));
    path.with_file_name(name)
}

/// The name a temporary file will be renamed to, if `name` is one of our temporary files.
fn temp_target(name: &str) -> Option<&str> {
    let (target, n) = name.strip_suffix(".tmp")?.rsplit_once('.')?;
    (!n.is_empty() && n.bytes().all(|b| b.is_ascii_digit())).then_some(target)
}

fn hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        out.push(char::from(DIGITS[usize::from(b >> 4)]));
        out.push(char::from(DIGITS[usize::from(b & 0x0f)]));
    }
    out
}

/// 64 lowercase hex digits.
fn parse_key(text: &str) -> Option<Key> {
    fn nibble(c: u8) -> Option<u8> {
        match c {
            b'0'..=b'9' => Some(c - b'0'),
            b'a'..=b'f' => Some(c - b'a' + 10),
            _ => None,
        }
    }
    let bytes = text.as_bytes();
    if bytes.len() != 64 {
        return None;
    }
    let mut out = [0u8; 32];
    for (byte, pair) in out.iter_mut().zip(bytes.chunks_exact(2)) {
        *byte = (nibble(pair[0])? << 4) | nibble(pair[1])?;
    }
    Some(out)
}

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}

fn unix_ms(t: OffsetDateTime) -> i64 {
    i64::try_from(t.unix_timestamp_nanos() / 1_000_000).unwrap_or(i64::MAX)
}

fn from_unix_ms(ms: i64) -> Option<OffsetDateTime> {
    OffsetDateTime::from_unix_timestamp_nanos(i128::from(ms) * 1_000_000).ok()
}

/// What a clear removed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Cleared {
    pub bytes: u64,
    pub files: usize,
    /// Files that could not be deleted (logged; still counted in the size).
    pub failed: usize,
}

impl std::ops::Add for Cleared {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            bytes: self.bytes + other.bytes,
            files: self.files + other.files,
            failed: self.failed + other.failed,
        }
    }
}

// ---- images

/// The image formats the cache stores.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImageType {
    Jpeg,
    Png,
    Gif,
    Webp,
}

impl ImageType {
    pub const ALL: [ImageType; 4] = [Self::Jpeg, Self::Png, Self::Gif, Self::Webp];

    /// The content type to serve it with.
    pub fn mime(self) -> &'static str {
        match self {
            Self::Jpeg => "image/jpeg",
            Self::Png => "image/png",
            Self::Gif => "image/gif",
            Self::Webp => "image/webp",
        }
    }

    fn ext(self) -> &'static str {
        match self {
            Self::Jpeg => "jpg",
            Self::Png => "png",
            Self::Gif => "gif",
            Self::Webp => "webp",
        }
    }

    fn from_ext(ext: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|kind| kind.ext() == ext)
    }

    /// A `Content-Type` header value, ignoring parameters and case; `image/jpg` is taken as JPEG.
    pub fn from_mime(content_type: &str) -> Option<Self> {
        let essence = content_type
            .split_once(';')
            .map_or(content_type, |(essence, _)| essence)
            .trim()
            .to_ascii_lowercase();
        match essence.as_str() {
            "image/jpeg" | "image/jpg" => Some(Self::Jpeg),
            "image/png" => Some(Self::Png),
            "image/gif" => Some(Self::Gif),
            "image/webp" => Some(Self::Webp),
            _ => None,
        }
    }

    /// The format the bytes are in, from their signature.
    pub fn sniff(bytes: &[u8]) -> Option<Self> {
        if bytes.starts_with(&[0xFF, 0xD8, 0xFF]) {
            Some(Self::Jpeg)
        } else if bytes.starts_with(b"\x89PNG\r\n\x1a\n") {
            Some(Self::Png)
        } else if bytes.starts_with(b"GIF87a") || bytes.starts_with(b"GIF89a") {
            Some(Self::Gif)
        } else if bytes.starts_with(b"RIFF") && bytes.get(8..12) == Some(b"WEBP".as_slice()) {
            Some(Self::Webp)
        } else {
            None
        }
    }
}

/// How much an image area may hold (decision 58).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobLimits {
    /// Least recently used entries are evicted until the area is under this.
    pub max_total_bytes: u64,
    /// Larger images are not stored. Never more than `max_total_bytes`.
    pub max_entry_bytes: u64,
}

impl Default for BlobLimits {
    fn default() -> Self {
        Self {
            max_total_bytes: 100 * 1024 * 1024,
            max_entry_bytes: 5 * 1024 * 1024,
        }
    }
}

impl BlobLimits {
    fn entry_cap(self) -> u64 {
        self.max_entry_bytes.min(self.max_total_bytes)
    }
}

/// A cached image.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Blob {
    pub bytes: Vec<u8>,
    pub content_type: ImageType,
}

/// Why an image was not stored.
#[derive(Debug)]
pub enum PutError {
    /// The declared content type is not one of the image types we store.
    UnsupportedType(String),
    /// The bytes are not a JPEG, PNG, GIF or WebP image.
    NotAnImage,
    TooLarge {
        len: u64,
        max: u64,
    },
    Io(io::Error),
}

impl fmt::Display for PutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedType(t) => write!(
                f,
                "the server sent \"{t}\", which is not a JPEG, PNG, GIF or WebP image"
            ),
            Self::NotAnImage => write!(f, "the data is not a JPEG, PNG, GIF or WebP image"),
            Self::TooLarge { len, max } => write!(
                f,
                "the image is {len} bytes; images over {max} bytes are not cached"
            ),
            Self::Io(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for PutError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for PutError {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

#[derive(Debug, Clone, Copy)]
struct Item {
    kind: ImageType,
    len: u64,
    access: SystemTime,
    /// Increases with every store and hit: orders equal access times and tells whether the item changed meanwhile.
    seq: u64,
}

#[derive(Debug)]
struct Index {
    limits: BlobLimits,
    items: HashMap<Key, Item>,
    /// Least recently used first.
    order: BTreeMap<(SystemTime, u64), Key>,
    total: u64,
    next_seq: u64,
}

impl Index {
    fn new(limits: BlobLimits) -> Self {
        Self {
            limits,
            items: HashMap::new(),
            order: BTreeMap::new(),
            total: 0,
            next_seq: 0,
        }
    }

    fn bump(&mut self) -> u64 {
        let seq = self.next_seq;
        self.next_seq += 1;
        seq
    }

    /// Adds or replaces an item; returns the replaced one.
    fn insert(&mut self, key: Key, kind: ImageType, len: u64, access: SystemTime) -> Option<Item> {
        let old = self.remove(&key);
        let seq = self.bump();
        self.items.insert(
            key,
            Item {
                kind,
                len,
                access,
                seq,
            },
        );
        self.order.insert((access, seq), key);
        self.total += len;
        old
    }

    fn remove(&mut self, key: &Key) -> Option<Item> {
        let item = self.items.remove(key)?;
        self.order.remove(&(item.access, item.seq));
        self.total -= item.len;
        Some(item)
    }

    /// Removes the item only if nothing changed it since it was looked up as `seq`.
    fn remove_if(&mut self, key: &Key, seq: u64) -> Option<Item> {
        if self.items.get(key).is_some_and(|item| item.seq == seq) {
            self.remove(key)
        } else {
            None
        }
    }

    /// Marks a hit; false if the item changed since it was looked up as `seq`.
    fn touch(&mut self, key: &Key, seq: u64, now: SystemTime) -> bool {
        match self.items.get(key).copied() {
            Some(item) if item.seq == seq => {
                self.order.remove(&(item.access, item.seq));
                let seq = self.bump();
                self.items.insert(
                    *key,
                    Item {
                        access: now,
                        seq,
                        ..item
                    },
                );
                self.order.insert((now, seq), *key);
                true
            }
            _ => false,
        }
    }

    fn oldest_except(&self, keep: Option<&Key>) -> Option<Key> {
        self.order.values().find(|key| Some(*key) != keep).copied()
    }

    fn clear(&mut self) {
        self.items.clear();
        self.order.clear();
        self.total = 0;
    }
}

fn is_shard_name(name: &str) -> bool {
    name.len() == 2 && name.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
}

/// The key and format of one of our image files in the shard folder `shard`.
fn blob_name(shard: &str, name: &str) -> Option<(Key, ImageType)> {
    let (stem, ext) = name.split_once('.')?;
    let kind = ImageType::from_ext(ext)?;
    let key = parse_key(stem)?;
    (stem.get(..2) == Some(shard)).then_some((key, kind))
}

fn is_blob_file(shard: &str, name: &str) -> bool {
    blob_name(shard, name).is_some()
        || temp_target(name).is_some_and(|t| blob_name(shard, t).is_some())
}

fn url_key(url: &str) -> Key {
    sha256(url.as_bytes())
}

/// Thumbnails or description images: plain image files keyed by URL, least recently used evicted first.
#[derive(Debug)]
pub struct BlobCache<F, C, L> {
    root: PathBuf,
    fs: F,
    clock: C,
    log: L,
    index: Mutex<Index>,
    temp: AtomicU64,
}

impl<F: CacheFs, C: Clock, L: FileLog> BlobCache<F, C, L> {
    /// Reads the folder (which need not exist yet), deletes leftover temporary files and evicts down to the limit.
    pub fn open(root: PathBuf, fs: F, clock: C, log: L, limits: BlobLimits) -> io::Result<Self> {
        let cache = Self {
            root,
            fs,
            clock,
            log,
            index: Mutex::new(Index::new(limits)),
            temp: AtomicU64::new(0),
        };
        let mut found = Vec::new();
        for shard in list_or_empty(&cache.fs, &cache.root)? {
            if !shard.is_dir || !is_shard_name(&shard.name) {
                continue;
            }
            let dir = cache.root.join(&shard.name);
            for file in list_or_empty(&cache.fs, &dir)? {
                if file.is_dir {
                    continue;
                }
                if let Some((key, kind)) = blob_name(&shard.name, &file.name) {
                    found.push((file.modified, file.name, key, kind, file.len));
                } else if is_blob_file(&shard.name, &file.name) {
                    let _ = remove_logged(
                        &cache.fs,
                        &cache.log,
                        &dir.join(&file.name),
                        file.len,
                        "cache: removed a leftover temporary file",
                    );
                }
            }
        }
        found.sort_by(|a, b| (a.0, &a.1).cmp(&(b.0, &b.1)));
        {
            let mut index = cache.lock();
            for (modified, _, key, kind, len) in found {
                index.insert(key, kind, len, modified);
            }
            cache.evict(&mut index, None);
        }
        Ok(cache)
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    /// The cached image for `url`, checked against the index; a damaged entry is deleted and reported as a miss.
    pub fn get(&self, url: &str) -> io::Result<Option<Blob>> {
        let key = url_key(url);
        let Some(item) = self.lock().items.get(&key).copied() else {
            return Ok(None);
        };
        let path = self.path_of(&key, item.kind);
        let bytes = match self.fs.read(&path) {
            Ok(bytes) => bytes,
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                self.lock().remove_if(&key, item.seq);
                return Ok(None);
            }
            Err(e) => return Err(e),
        };
        if bytes.len() as u64 != item.len || ImageType::sniff(&bytes) != Some(item.kind) {
            let mut index = self.lock();
            if index.remove_if(&key, item.seq).is_some() {
                let _ = remove_logged(
                    &self.fs,
                    &self.log,
                    &path,
                    bytes.len() as u64,
                    "cache: dropped a damaged entry",
                );
            }
            return Ok(None);
        }
        let now = SystemTime::from(self.clock.now());
        let touched = self.lock().touch(&key, item.seq, now);
        if touched {
            // Keeps the access order for the next start; if it fails, the entry only looks older then.
            let _ = self.fs.set_modified(&path, now);
        }
        Ok(Some(Blob {
            bytes,
            content_type: item.kind,
        }))
    }

    /// Whether `url` is cached, without reading it (for "Not saved" on cards while offline).
    pub fn contains(&self, url: &str) -> bool {
        self.lock().items.contains_key(&url_key(url))
    }

    /// Stores the image for `url`, replacing an older one, then evicts down to the limit. Returns the stored type,
    /// detected from the bytes.
    pub fn put(&self, url: &str, bytes: &[u8], content_type: &str) -> Result<ImageType, PutError> {
        if ImageType::from_mime(content_type).is_none() {
            return Err(PutError::UnsupportedType(content_type.to_owned()));
        }
        let kind = ImageType::sniff(bytes).ok_or(PutError::NotAnImage)?;
        let len = bytes.len() as u64;
        let max = self.lock().limits.entry_cap();
        if len > max {
            return Err(PutError::TooLarge { len, max });
        }
        let key = url_key(url);
        let path = self.path_of(&key, kind);
        if let Some(dir) = path.parent() {
            self.fs.create_dir_all(dir)?;
        }
        let temp = temp_path(&path, &self.temp);
        let now = SystemTime::from(self.clock.now());
        write_temp(&self.fs, &self.log, &temp, bytes, now)?;
        let mut index = self.lock();
        commit(&self.fs, &self.log, &temp, &path, len)?;
        if let Some(old) = index.insert(key, kind, len, now)
            && old.kind != kind
        {
            let _ = remove_logged(
                &self.fs,
                &self.log,
                &self.path_of(&key, old.kind),
                old.len,
                "cache: removed the previous version of an entry",
            );
        }
        self.evict(&mut index, Some(&key));
        Ok(kind)
    }

    /// Bytes held by the entries.
    pub fn size(&self) -> u64 {
        self.lock().total
    }

    /// How many images are stored.
    pub fn count(&self) -> usize {
        self.lock().items.len()
    }

    pub fn limits(&self) -> BlobLimits {
        self.lock().limits
    }

    /// Applies new limits and evicts down to them right away.
    pub fn set_limits(&self, limits: BlobLimits) {
        let mut index = self.lock();
        index.limits = limits;
        self.evict(&mut index, None);
    }

    /// Deletes every entry and leftover temporary file, and shard folders left empty; nothing else in the folder.
    pub fn clear(&self) -> io::Result<Cleared> {
        let mut index = self.lock();
        index.clear();
        let mut cleared = Cleared::default();
        for shard in list_or_empty(&self.fs, &self.root)? {
            if !shard.is_dir || !is_shard_name(&shard.name) {
                continue;
            }
            let dir = self.root.join(&shard.name);
            let mut removed = false;
            for file in list_or_empty(&self.fs, &dir)? {
                if file.is_dir || !is_blob_file(&shard.name, &file.name) {
                    continue;
                }
                match remove_logged(
                    &self.fs,
                    &self.log,
                    &dir.join(&file.name),
                    file.len,
                    "cache: cleared",
                ) {
                    Ok(()) => {
                        cleared.bytes += file.len;
                        cleared.files += 1;
                        removed = true;
                    }
                    Err(_) => {
                        cleared.failed += 1;
                        if let Some((key, kind)) = blob_name(&shard.name, &file.name) {
                            index.insert(key, kind, file.len, file.modified);
                        }
                    }
                }
            }
            // Fails, and keeps the folder, when anything else is still in it.
            if removed && self.fs.remove_dir(&dir).is_ok() {
                self.log.record(
                    "cache: removed an empty folder",
                    &file_event(FileOp::Delete, &dir, None, None),
                );
            }
        }
        Ok(cleared)
    }

    fn lock(&self) -> MutexGuard<'_, Index> {
        lock(&self.index)
    }

    fn path_of(&self, key: &Key, kind: ImageType) -> PathBuf {
        let name = hex(key);
        self.root
            .join(&name[..2])
            .join(format!("{name}.{}", kind.ext()))
    }

    fn evict(&self, index: &mut Index, keep: Option<&Key>) {
        while index.total > index.limits.max_total_bytes {
            let Some(key) = index.oldest_except(keep) else {
                break;
            };
            if let Some(item) = index.remove(&key) {
                let _ = remove_logged(
                    &self.fs,
                    &self.log,
                    &self.path_of(&key, item.kind),
                    item.len,
                    "cache: evicted the least recently used entry",
                );
            }
        }
    }
}

// ---- catalog

/// The request a catalog page answers; every field is part of the cache key.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PageKey {
    pub section: Section,
    pub sort: String,
    pub category: Option<u64>,
    pub page: u32,
    pub search: Option<String>,
}

/// How long catalog pages stay fresh and how much the catalog area may hold (decision 58).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CatalogLimits {
    pub ttl: Duration,
    /// The oldest written files are deleted until the area is under this.
    pub max_total_bytes: u64,
}

impl Default for CatalogLimits {
    fn default() -> Self {
        Self {
            ttl: Duration::from_secs(10 * 60),
            max_total_bytes: 20 * 1024 * 1024,
        }
    }
}

/// A saved value with the time it was fetched (in the clock's offset) and its age.
#[derive(Debug, Clone, PartialEq)]
pub struct Saved<T> {
    pub value: T,
    pub fetched_at: OffsetDateTime,
    pub age: Duration,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Lookup<T> {
    /// Younger than the TTL: show it without a request.
    Fresh(Saved<T>),
    /// Older than the TTL: show it and fetch again (stale-while-revalidate), or show it as saved while offline.
    Stale(Saved<T>),
    Missing,
}

impl<T> Lookup<T> {
    pub fn saved(self) -> Option<Saved<T>> {
        match self {
            Self::Fresh(saved) | Self::Stale(saved) => Some(saved),
            Self::Missing => None,
        }
    }

    pub fn is_stale(&self) -> bool {
        matches!(self, Self::Stale(_))
    }
}

/// A mod's card, kept by mod ID and update date (decision 58).
#[derive(Debug, Clone, PartialEq)]
pub struct Card<T> {
    pub value: T,
    /// The mod's update date when the card was fetched.
    pub updated: OffsetDateTime,
    pub fetched_at: OffsetDateTime,
}

impl<T> Card<T> {
    /// Whether the card still describes the mod, given the update date from a fresh listing.
    pub fn is_current(&self, listed_updated: OffsetDateTime) -> bool {
        self.updated >= listed_updated
    }
}

#[derive(Serialize)]
struct PageOut<'a, T> {
    schema: u32,
    key: &'a PageKey,
    fetched_at_unix_ms: i64,
    value: &'a T,
}

#[derive(Deserialize)]
struct PageIn<T> {
    key: PageKey,
    fetched_at_unix_ms: i64,
    value: T,
}

#[derive(Serialize)]
struct CardOut<'a, T> {
    schema: u32,
    section: Section,
    mod_id: u64,
    updated_unix_ms: i64,
    fetched_at_unix_ms: i64,
    value: &'a T,
}

#[derive(Deserialize)]
struct CardIn<T> {
    section: Section,
    mod_id: u64,
    updated_unix_ms: i64,
    fetched_at_unix_ms: i64,
    value: T,
}

#[derive(Deserialize)]
struct Head {
    schema: u32,
}

enum Decoded<T> {
    Ok(T),
    /// Written by a newer deadloader: left alone.
    Newer,
    Damaged,
}

fn decode<T: DeserializeOwned>(bytes: &[u8]) -> Decoded<T> {
    match serde_json::from_slice::<Head>(bytes) {
        Err(_) => Decoded::Damaged,
        Ok(head) if head.schema > SCHEMA => Decoded::Newer,
        Ok(_) => serde_json::from_slice(bytes).map_or(Decoded::Damaged, Decoded::Ok),
    }
}

fn section_name(section: Section) -> &'static str {
    match section {
        Section::Mod => "mod",
        Section::Sound => "sound",
    }
}

fn page_name(key: &PageKey) -> io::Result<String> {
    let canonical = serde_json::to_vec(key).map_err(io::Error::other)?;
    Ok(format!("page-{}.json", hex(&sha256(&canonical))))
}

fn card_name(section: Section, mod_id: u64) -> String {
    format!("card-{}-{mod_id}.json", section_name(section))
}

/// `page-<64 hex>.json` or `card-<section>-<id>.json`.
fn is_catalog_name(name: &str) -> bool {
    let Some(stem) = name.strip_suffix(".json") else {
        return false;
    };
    if let Some(hash) = stem.strip_prefix("page-") {
        return parse_key(hash).is_some();
    }
    if let Some((section, id)) = stem.strip_prefix("card-").and_then(|r| r.split_once('-')) {
        return matches!(section, "mod" | "sound")
            && !id.is_empty()
            && id.bytes().all(|b| b.is_ascii_digit());
    }
    false
}

fn is_catalog_file(name: &str) -> bool {
    is_catalog_name(name) || temp_target(name).is_some_and(is_catalog_name)
}

/// Catalog pages and mod cards as JSON files.
#[derive(Debug)]
pub struct CatalogCache<F, C, L> {
    root: PathBuf,
    fs: F,
    clock: C,
    log: L,
    /// Also serialises renames and deletes in the folder.
    limits: Mutex<CatalogLimits>,
    temp: AtomicU64,
}

impl<F: CacheFs, C: Clock, L: FileLog> CatalogCache<F, C, L> {
    /// Deletes leftover temporary files and brings the folder (which need not exist yet) under the size limit.
    pub fn open(root: PathBuf, fs: F, clock: C, log: L, limits: CatalogLimits) -> io::Result<Self> {
        let cache = Self {
            root,
            fs,
            clock,
            log,
            limits: Mutex::new(limits),
            temp: AtomicU64::new(0),
        };
        for file in list_or_empty(&cache.fs, &cache.root)? {
            if !file.is_dir && !is_catalog_name(&file.name) && is_catalog_file(&file.name) {
                let _ = remove_logged(
                    &cache.fs,
                    &cache.log,
                    &cache.root.join(&file.name),
                    file.len,
                    "cache: removed a leftover temporary file",
                );
            }
        }
        cache.shrink(limits.max_total_bytes, None)?;
        Ok(cache)
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn limits(&self) -> CatalogLimits {
        *lock(&self.limits)
    }

    /// Applies a new TTL and size limit; the size limit is enforced right away.
    pub fn set_limits(&self, limits: CatalogLimits) {
        let mut current = lock(&self.limits);
        *current = limits;
        let _ = self.shrink(limits.max_total_bytes, None);
    }

    /// Saves a page fetched now.
    pub fn put_page<T: Serialize>(&self, key: &PageKey, value: &T) -> io::Result<()> {
        let file = PageOut {
            schema: SCHEMA,
            key,
            fetched_at_unix_ms: unix_ms(self.clock.now()),
            value,
        };
        let bytes = serde_json::to_vec(&file).map_err(io::Error::other)?;
        self.write(&page_name(key)?, &bytes)
    }

    /// The saved page for `key`, fresh or stale by its age; a damaged file is deleted and reported as missing.
    pub fn get_page<T: DeserializeOwned>(&self, key: &PageKey) -> io::Result<Lookup<T>> {
        let name = page_name(key)?;
        let Some(bytes) = self.read(&name)? else {
            return Ok(Lookup::Missing);
        };
        let lookup = match decode::<PageIn<T>>(&bytes) {
            Decoded::Newer => return Ok(Lookup::Missing),
            Decoded::Ok(file) if file.key == *key => {
                self.classify(file.value, file.fetched_at_unix_ms)
            }
            _ => None,
        };
        Ok(lookup.unwrap_or_else(|| {
            self.drop_damaged(&name, bytes.len());
            Lookup::Missing
        }))
    }

    /// Saves a mod's card fetched now, with the mod's update date at that time.
    pub fn put_card<T: Serialize>(
        &self,
        section: Section,
        mod_id: u64,
        updated: OffsetDateTime,
        value: &T,
    ) -> io::Result<()> {
        let file = CardOut {
            schema: SCHEMA,
            section,
            mod_id,
            updated_unix_ms: unix_ms(updated),
            fetched_at_unix_ms: unix_ms(self.clock.now()),
            value,
        };
        let bytes = serde_json::to_vec(&file).map_err(io::Error::other)?;
        self.write(&card_name(section, mod_id), &bytes)
    }

    /// The saved card of a mod; check it with [`Card::is_current`] against a fresh listing.
    pub fn get_card<T: DeserializeOwned>(
        &self,
        section: Section,
        mod_id: u64,
    ) -> io::Result<Option<Card<T>>> {
        let name = card_name(section, mod_id);
        let Some(bytes) = self.read(&name)? else {
            return Ok(None);
        };
        let offset = self.clock.now().offset();
        let card = match decode::<CardIn<T>>(&bytes) {
            Decoded::Newer => return Ok(None),
            Decoded::Ok(file) if file.section == section && file.mod_id == mod_id => {
                match (
                    from_unix_ms(file.updated_unix_ms),
                    from_unix_ms(file.fetched_at_unix_ms),
                ) {
                    (Some(updated), Some(fetched_at)) => Some(Card {
                        value: file.value,
                        updated: updated.to_offset(offset),
                        fetched_at: fetched_at.to_offset(offset),
                    }),
                    _ => None,
                }
            }
            _ => None,
        };
        if card.is_none() {
            self.drop_damaged(&name, bytes.len());
        }
        Ok(card)
    }

    /// How many pages and mod cards are stored.
    pub fn count(&self) -> io::Result<usize> {
        Ok(list_or_empty(&self.fs, &self.root)?
            .iter()
            .filter(|f| !f.is_dir && is_catalog_name(&f.name))
            .count())
    }

    /// Bytes held by our files in the folder.
    pub fn size(&self) -> io::Result<u64> {
        Ok(list_or_empty(&self.fs, &self.root)?
            .iter()
            .filter(|f| !f.is_dir && is_catalog_file(&f.name))
            .map(|f| f.len)
            .sum())
    }

    /// Deletes every page, card and leftover temporary file; nothing else in the folder.
    pub fn clear(&self) -> io::Result<Cleared> {
        let _guard = lock(&self.limits);
        let mut cleared = Cleared::default();
        for file in list_or_empty(&self.fs, &self.root)? {
            if file.is_dir || !is_catalog_file(&file.name) {
                continue;
            }
            match remove_logged(
                &self.fs,
                &self.log,
                &self.root.join(&file.name),
                file.len,
                "cache: cleared",
            ) {
                Ok(()) => {
                    cleared.bytes += file.len;
                    cleared.files += 1;
                }
                Err(_) => cleared.failed += 1,
            }
        }
        Ok(cleared)
    }

    fn classify<T>(&self, value: T, fetched_ms: i64) -> Option<Lookup<T>> {
        let now = self.clock.now();
        let fetched_at = from_unix_ms(fetched_ms)?.to_offset(now.offset());
        let ttl = self.limits().ttl;
        Some(
            match u64::try_from(unix_ms(now).saturating_sub(fetched_ms)) {
                // Fetched "in the future": the clock was set back, so the age is unknown.
                Err(_) => Lookup::Stale(Saved {
                    value,
                    fetched_at,
                    age: Duration::ZERO,
                }),
                Ok(ms) => {
                    let age = Duration::from_millis(ms);
                    let saved = Saved {
                        value,
                        fetched_at,
                        age,
                    };
                    if age < ttl {
                        Lookup::Fresh(saved)
                    } else {
                        Lookup::Stale(saved)
                    }
                }
            },
        )
    }

    fn read(&self, name: &str) -> io::Result<Option<Vec<u8>>> {
        match self.fs.read(&self.root.join(name)) {
            Ok(bytes) => Ok(Some(bytes)),
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e),
        }
    }

    fn drop_damaged(&self, name: &str, len: usize) {
        let _guard = lock(&self.limits);
        let _ = remove_logged(
            &self.fs,
            &self.log,
            &self.root.join(name),
            len as u64,
            "cache: dropped a damaged entry",
        );
    }

    fn write(&self, name: &str, bytes: &[u8]) -> io::Result<()> {
        let path = self.root.join(name);
        self.fs.create_dir_all(&self.root)?;
        let temp = temp_path(&path, &self.temp);
        let now = SystemTime::from(self.clock.now());
        write_temp(&self.fs, &self.log, &temp, bytes, now)?;
        let limits = lock(&self.limits);
        commit(&self.fs, &self.log, &temp, &path, bytes.len() as u64)?;
        self.shrink(limits.max_total_bytes, Some(name))
    }

    /// Deletes the oldest written files until ours take at most `max` bytes; never `keep`. Call under the lock.
    fn shrink(&self, max: u64, keep: Option<&str>) -> io::Result<()> {
        let mut files: Vec<Entry> = list_or_empty(&self.fs, &self.root)?
            .into_iter()
            .filter(|f| !f.is_dir && is_catalog_name(&f.name))
            .collect();
        let mut total: u64 = files.iter().map(|f| f.len).sum();
        if total <= max {
            return Ok(());
        }
        files.sort_by(|a, b| (a.modified, &a.name).cmp(&(b.modified, &b.name)));
        for file in files {
            if total <= max {
                break;
            }
            if Some(file.name.as_str()) == keep {
                continue;
            }
            if remove_logged(
                &self.fs,
                &self.log,
                &self.root.join(&file.name),
                file.len,
                "cache: evicted the oldest catalog entry",
            )
            .is_ok()
            {
                total -= file.len;
            }
        }
        Ok(())
    }
}

// ---- the whole cache

/// The limits of every area, from the settings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Settings {
    pub catalog: CatalogLimits,
    pub thumbs: BlobLimits,
    pub images: BlobLimits,
}

/// Bytes per area, for the "Clear cache (N MB)" button.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Sizes {
    pub catalog: u64,
    pub thumbs: u64,
    pub images: u64,
}

impl Sizes {
    pub fn total(&self) -> u64 {
        self.catalog + self.thumbs + self.images
    }
}

/// What Settings → Cache shows: bytes per area and how many entries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Usage {
    pub sizes: Sizes,
    pub thumbnails: usize,
    pub images: usize,
    /// Catalog pages and mod cards.
    pub pages: usize,
}

/// `deadloader-data\cache\`: the catalog, thumbnails and description images.
#[derive(Debug)]
pub struct Cache<F, C, L> {
    pub catalog: CatalogCache<F, C, L>,
    pub thumbs: BlobCache<F, C, L>,
    pub images: BlobCache<F, C, L>,
}

impl<F: CacheFs + Clone, C: Clock + Clone, L: FileLog + Clone> Cache<F, C, L> {
    /// Opens `catalog\`, `thumbs\` and `images\` inside `cache_dir`. Lists every image folder: run it off the UI thread.
    pub fn open(cache_dir: &Path, fs: F, clock: C, log: L, settings: Settings) -> io::Result<Self> {
        Ok(Self {
            catalog: CatalogCache::open(
                cache_dir.join("catalog"),
                fs.clone(),
                clock.clone(),
                log.clone(),
                settings.catalog,
            )?,
            thumbs: BlobCache::open(
                cache_dir.join("thumbs"),
                fs.clone(),
                clock.clone(),
                log.clone(),
                settings.thumbs,
            )?,
            images: BlobCache::open(cache_dir.join("images"), fs, clock, log, settings.images)?,
        })
    }
}

impl<F: CacheFs, C: Clock, L: FileLog> Cache<F, C, L> {
    pub fn sizes(&self) -> io::Result<Sizes> {
        Ok(Sizes {
            catalog: self.catalog.size()?,
            thumbs: self.thumbs.size(),
            images: self.images.size(),
        })
    }

    pub fn usage(&self) -> io::Result<Usage> {
        Ok(Usage {
            sizes: self.sizes()?,
            thumbnails: self.thumbs.count(),
            images: self.images.count(),
            pages: self.catalog.count()?,
        })
    }

    /// Empties every area, leaving files that are not ours where they are.
    pub fn clear(&self) -> io::Result<Cleared> {
        Ok(self.catalog.clear()? + self.thumbs.clear()? + self.images.clear()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    const ROOT: &str = r"D:\dl\deadloader-data\cache";

    fn t0() -> OffsetDateTime {
        OffsetDateTime::from_unix_timestamp(1_792_065_600).expect("time")
    }

    fn st(t: OffsetDateTime) -> SystemTime {
        SystemTime::from(t)
    }

    #[derive(Debug)]
    struct TestClock(Mutex<OffsetDateTime>);

    impl TestClock {
        fn new() -> Self {
            Self(Mutex::new(t0()))
        }

        fn advance(&self, by: Duration) {
            *self.0.lock().expect("clock") += by;
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

    /// An in-memory folder tree; paths compare case-insensitively with either separator, like on Windows.
    #[derive(Debug, Default)]
    struct MemFs(Mutex<MemState>);

    #[derive(Debug, Default)]
    struct MemState {
        files: BTreeMap<String, (Vec<u8>, SystemTime)>,
        dirs: BTreeSet<String>,
        fail_writes: bool,
        fail_replace: bool,
    }

    fn norm(path: impl AsRef<Path>) -> String {
        path.as_ref()
            .to_string_lossy()
            .replace('/', "\\")
            .to_ascii_lowercase()
    }

    fn parent_of(key: &str) -> Option<&str> {
        key.rsplit_once('\\').map(|(p, _)| p)
    }

    impl MemState {
        fn add_dirs(&mut self, dir: &str) {
            let mut k = dir.to_owned();
            loop {
                self.dirs.insert(k.clone());
                match parent_of(&k) {
                    Some(p) => k = p.to_owned(),
                    None => break,
                }
            }
        }

        fn parent_exists(&self, key: &str) -> bool {
            parent_of(key).is_some_and(|p| self.dirs.contains(p))
        }
    }

    impl MemFs {
        fn state(&self) -> MutexGuard<'_, MemState> {
            self.0.lock().expect("fs")
        }

        fn add(&self, path: &str, bytes: &[u8], modified: SystemTime) {
            let k = norm(path);
            let mut s = self.state();
            if let Some(p) = parent_of(&k) {
                let p = p.to_owned();
                s.add_dirs(&p);
            }
            s.files.insert(k, (bytes.to_vec(), modified));
        }

        fn file(&self, path: &str) -> Option<Vec<u8>> {
            self.state().files.get(&norm(path)).map(|(b, _)| b.clone())
        }

        fn mtime(&self, path: &str) -> Option<SystemTime> {
            self.state().files.get(&norm(path)).map(|(_, t)| *t)
        }

        fn set_bytes(&self, path: &str, bytes: &[u8]) {
            let mut s = self.state();
            let entry = s.files.get_mut(&norm(path)).expect("file to damage");
            entry.0 = bytes.to_vec();
        }

        fn delete(&self, path: &str) {
            self.state().files.remove(&norm(path));
        }

        fn names(&self) -> Vec<String> {
            self.state().files.keys().cloned().collect()
        }

        fn has_dir(&self, path: &str) -> bool {
            self.state().dirs.contains(&norm(path))
        }

        fn fail_writes(&self, on: bool) {
            self.state().fail_writes = on;
        }

        fn fail_replace(&self, on: bool) {
            self.state().fail_replace = on;
        }
    }

    impl CacheFs for MemFs {
        fn read(&self, path: &Path) -> io::Result<Vec<u8>> {
            self.state()
                .files
                .get(&norm(path))
                .map(|(b, _)| b.clone())
                .ok_or_else(|| io::Error::from(io::ErrorKind::NotFound))
        }

        fn write_synced(&self, path: &Path, bytes: &[u8], modified: SystemTime) -> io::Result<()> {
            let mut s = self.state();
            if s.fail_writes {
                return Err(io::Error::other("disk full"));
            }
            let k = norm(path);
            if !s.parent_exists(&k) {
                return Err(io::Error::from(io::ErrorKind::NotFound));
            }
            s.files.insert(k, (bytes.to_vec(), modified));
            Ok(())
        }

        fn replace(&self, from: &Path, to: &Path) -> io::Result<()> {
            let mut s = self.state();
            if s.fail_replace {
                return Err(io::Error::from(io::ErrorKind::PermissionDenied));
            }
            let to = norm(to);
            if !s.parent_exists(&to) {
                return Err(io::Error::from(io::ErrorKind::NotFound));
            }
            let data = s
                .files
                .remove(&norm(from))
                .ok_or_else(|| io::Error::from(io::ErrorKind::NotFound))?;
            s.files.insert(to, data);
            Ok(())
        }

        fn set_modified(&self, path: &Path, modified: SystemTime) -> io::Result<()> {
            let mut s = self.state();
            let entry = s
                .files
                .get_mut(&norm(path))
                .ok_or_else(|| io::Error::from(io::ErrorKind::NotFound))?;
            entry.1 = modified;
            Ok(())
        }

        fn remove_file(&self, path: &Path) -> io::Result<()> {
            self.state()
                .files
                .remove(&norm(path))
                .map(|_| ())
                .ok_or_else(|| io::Error::from(io::ErrorKind::NotFound))
        }

        fn create_dir_all(&self, path: &Path) -> io::Result<()> {
            self.state().add_dirs(&norm(path));
            Ok(())
        }

        fn remove_dir(&self, path: &Path) -> io::Result<()> {
            let mut s = self.state();
            let k = norm(path);
            if !s.dirs.contains(&k) {
                return Err(io::Error::from(io::ErrorKind::NotFound));
            }
            let prefix = format!("{k}\\");
            if s.files.keys().any(|f| f.starts_with(&prefix))
                || s.dirs.iter().any(|d| d.starts_with(&prefix))
            {
                return Err(io::Error::from(io::ErrorKind::DirectoryNotEmpty));
            }
            s.dirs.remove(&k);
            Ok(())
        }

        fn list(&self, dir: &Path) -> io::Result<Vec<Entry>> {
            let s = self.state();
            let k = norm(dir);
            if !s.dirs.contains(&k) {
                return Err(io::Error::from(io::ErrorKind::NotFound));
            }
            let prefix = format!("{k}\\");
            let mut out = Vec::new();
            for (name, (bytes, modified)) in &s.files {
                if let Some(rest) = name.strip_prefix(&prefix)
                    && !rest.contains('\\')
                {
                    out.push(Entry {
                        name: rest.to_owned(),
                        is_dir: false,
                        len: bytes.len() as u64,
                        modified: *modified,
                    });
                }
            }
            for name in &s.dirs {
                if let Some(rest) = name.strip_prefix(&prefix)
                    && !rest.contains('\\')
                {
                    out.push(Entry {
                        name: rest.to_owned(),
                        is_dir: true,
                        len: 0,
                        modified: SystemTime::UNIX_EPOCH,
                    });
                }
            }
            Ok(out)
        }
    }

    #[derive(Debug, Default)]
    struct Events(Mutex<Vec<(String, FileEvent)>>);

    impl FileLog for Events {
        fn record(&self, msg: &str, event: &FileEvent) {
            self.0
                .lock()
                .expect("events")
                .push((msg.to_owned(), event.clone()));
        }
    }

    impl Events {
        fn all(&self) -> Vec<FileEvent> {
            self.0
                .lock()
                .expect("events")
                .iter()
                .map(|(_, e)| e.clone())
                .collect()
        }

        fn deleted(&self) -> Vec<String> {
            self.all()
                .into_iter()
                .filter(|e| e.op == FileOp::Delete && e.result == Outcome::Ok)
                .filter_map(|e| e.src.map(norm))
                .collect()
        }
    }

    fn image(kind: ImageType, len: usize) -> Vec<u8> {
        let mut bytes = match kind {
            ImageType::Jpeg => vec![0xFF, 0xD8, 0xFF, 0xE0],
            ImageType::Png => b"\x89PNG\r\n\x1a\n".to_vec(),
            ImageType::Gif => b"GIF89a".to_vec(),
            ImageType::Webp => b"RIFF\0\0\0\0WEBPVP8 ".to_vec(),
        };
        bytes.resize(len, 0x5A);
        bytes
    }

    fn png(len: usize) -> Vec<u8> {
        image(ImageType::Png, len)
    }

    fn url(n: u32) -> String {
        format!("https://images.gamebanana.com/img/ss/mods/220-90_{n}.jpg")
    }

    fn hex_of(url: &str) -> String {
        hex(&sha256(url.as_bytes()))
    }

    /// Where the thumbnail for `url` lives, as the fake stores it.
    fn thumb_path(url: &str, ext: &str) -> String {
        let h = hex_of(url);
        norm(format!(r"{ROOT}\thumbs\{}\{h}.{ext}", &h[..2]))
    }

    fn small() -> BlobLimits {
        BlobLimits {
            max_total_bytes: 300,
            max_entry_bytes: 150,
        }
    }

    #[derive(Debug, Default)]
    struct World {
        fs: MemFs,
        clock: TestClock,
        log: Events,
    }

    impl Default for TestClock {
        fn default() -> Self {
            Self::new()
        }
    }

    impl World {
        fn thumbs(&self, limits: BlobLimits) -> BlobCache<&MemFs, &TestClock, &Events> {
            BlobCache::open(
                Path::new(ROOT).join("thumbs"),
                &self.fs,
                &self.clock,
                &self.log,
                limits,
            )
            .expect("open thumbs")
        }

        fn catalog(&self, limits: CatalogLimits) -> CatalogCache<&MemFs, &TestClock, &Events> {
            CatalogCache::open(
                Path::new(ROOT).join("catalog"),
                &self.fs,
                &self.clock,
                &self.log,
                limits,
            )
            .expect("open catalog")
        }

        fn cache(&self) -> Cache<&MemFs, &TestClock, &Events> {
            Cache::open(
                Path::new(ROOT),
                &self.fs,
                &self.clock,
                &self.log,
                Settings::default(),
            )
            .expect("open cache")
        }

        fn our_files(&self) -> Vec<String> {
            self.fs
                .names()
                .into_iter()
                .filter(|n| n.starts_with(&norm(ROOT)))
                .collect()
        }
    }

    // ---- images

    #[test]
    fn stores_and_returns_an_image_with_its_type() {
        let w = World::default();
        let thumbs = w.thumbs(small());
        assert_eq!(
            thumbs.put(&url(1), &png(100), "image/png").ok(),
            Some(ImageType::Png)
        );
        let blob = thumbs.get(&url(1)).expect("read").expect("hit");
        assert_eq!(blob.bytes, png(100));
        assert_eq!(blob.content_type, ImageType::Png);
        assert_eq!(blob.content_type.mime(), "image/png");
    }

    #[test]
    fn an_unknown_url_is_a_miss() {
        let w = World::default();
        let thumbs = w.thumbs(small());
        assert!(thumbs.get(&url(2)).expect("read").is_none());
        assert!(!thumbs.contains(&url(2)));
        thumbs.put(&url(2), &png(10), "image/png").expect("put");
        assert!(thumbs.contains(&url(2)));
        assert!(!thumbs.contains(&url(3)));
    }

    #[test]
    fn files_are_named_after_the_url_hash_in_two_letter_folders() {
        let w = World::default();
        let thumbs = w.thumbs(small());
        let bytes = image(ImageType::Webp, 64);
        thumbs.put(&url(1), &bytes, "image/webp").expect("put");
        assert_eq!(w.fs.file(&thumb_path(&url(1), "webp")), Some(bytes));
        assert_eq!(
            w.our_files(),
            vec![thumb_path(&url(1), "webp")],
            "no temp file left"
        );
        assert_eq!(w.fs.mtime(&thumb_path(&url(1), "webp")), Some(st(t0())));
    }

    #[test]
    fn only_image_content_types_are_accepted() {
        let w = World::default();
        let thumbs = w.thumbs(small());
        for ct in [
            "text/html",
            "image/svg+xml",
            "image/bmp",
            "application/octet-stream",
            "",
        ] {
            assert!(
                matches!(thumbs.put(&url(1), &png(20), ct), Err(PutError::UnsupportedType(t)) if t == ct),
                "{ct:?}"
            );
        }
        assert!(w.our_files().is_empty());
        for (ct, kind) in [
            ("image/jpeg", ImageType::Jpeg),
            ("image/jpg", ImageType::Jpeg),
            ("IMAGE/JPEG; charset=binary", ImageType::Jpeg),
            (" image/png ", ImageType::Png),
            ("image/gif", ImageType::Gif),
            ("image/webp", ImageType::Webp),
        ] {
            assert_eq!(ImageType::from_mime(ct), Some(kind), "{ct:?}");
        }
    }

    #[test]
    fn bytes_that_are_not_an_image_are_refused() {
        let w = World::default();
        let thumbs = w.thumbs(small());
        let page = b"<!doctype html><title>Error 503</title>".to_vec();
        assert!(matches!(
            thumbs.put(&url(1), &page, "image/png"),
            Err(PutError::NotAnImage)
        ));
        assert!(matches!(
            thumbs.put(&url(1), &[], "image/png"),
            Err(PutError::NotAnImage)
        ));
        assert!(w.our_files().is_empty());
    }

    #[test]
    fn the_stored_type_comes_from_the_bytes() {
        let w = World::default();
        let thumbs = w.thumbs(small());
        let webp = image(ImageType::Webp, 40);
        assert_eq!(
            thumbs.put(&url(1), &webp, "image/jpeg").ok(),
            Some(ImageType::Webp)
        );
        let blob = thumbs.get(&url(1)).expect("read").expect("hit");
        assert_eq!(blob.content_type, ImageType::Webp);
        for kind in ImageType::ALL {
            assert_eq!(ImageType::sniff(&image(kind, 32)), Some(kind));
        }
        assert_eq!(ImageType::sniff(b"RIFF\0\0\0\0WAVEfmt "), None);
    }

    #[test]
    fn an_entry_over_the_per_entry_cap_is_not_stored() {
        let w = World::default();
        let thumbs = w.thumbs(small());
        thumbs.put(&url(1), &png(100), "image/png").expect("put");
        thumbs.put(&url(2), &png(100), "image/png").expect("put");
        assert!(matches!(
            thumbs.put(&url(3), &png(151), "image/png"),
            Err(PutError::TooLarge { len: 151, max: 150 })
        ));
        assert!(thumbs.contains(&url(1)) && thumbs.contains(&url(2)));
        assert!(!thumbs.contains(&url(3)));
        assert_eq!(thumbs.size(), 200);
        assert_eq!(w.our_files().len(), 2);
    }

    #[test]
    fn the_per_entry_cap_never_exceeds_the_total_limit() {
        let w = World::default();
        let thumbs = w.thumbs(BlobLimits {
            max_total_bytes: 100,
            max_entry_bytes: 5 * 1024 * 1024,
        });
        assert!(matches!(
            thumbs.put(&url(1), &png(101), "image/png"),
            Err(PutError::TooLarge { len: 101, max: 100 })
        ));
    }

    #[test]
    fn the_least_recently_used_entry_is_evicted_first() {
        let w = World::default();
        let thumbs = w.thumbs(small());
        for n in 1..=3 {
            thumbs.put(&url(n), &png(100), "image/png").expect("put");
            w.clock.advance(Duration::from_secs(1));
        }
        assert!(thumbs.get(&url(1)).expect("read").is_some());
        w.clock.advance(Duration::from_secs(1));
        thumbs.put(&url(4), &png(100), "image/png").expect("put");
        assert!(thumbs.contains(&url(1)), "recently read");
        assert!(!thumbs.contains(&url(2)), "least recently used");
        assert!(thumbs.contains(&url(3)) && thumbs.contains(&url(4)));
        assert_eq!(thumbs.size(), 300);
        assert_eq!(w.fs.file(&thumb_path(&url(2), "png")), None);
        assert_eq!(w.log.deleted(), vec![thumb_path(&url(2), "png")]);
    }

    #[test]
    fn equal_times_are_ordered_by_access() {
        let w = World::default();
        let thumbs = w.thumbs(small());
        for n in 1..=3 {
            thumbs.put(&url(n), &png(100), "image/png").expect("put");
        }
        thumbs.get(&url(1)).expect("read");
        thumbs.put(&url(4), &png(100), "image/png").expect("put");
        assert!(thumbs.contains(&url(1)));
        assert!(!thumbs.contains(&url(2)));
    }

    #[test]
    fn many_puts_stay_under_the_limit() {
        let w = World::default();
        let thumbs = w.thumbs(small());
        for n in 0..200u32 {
            let len = 20 + (n as usize * 37) % 130;
            thumbs.put(&url(n), &png(len), "image/png").expect("put");
            assert!(thumbs.size() <= 300, "after put {n}: {}", thumbs.size());
            assert!(thumbs.contains(&url(n)), "the newest entry stays");
            w.clock.advance(Duration::from_millis(10));
        }
        let on_disk: u64 = w
            .our_files()
            .iter()
            .map(|p| w.fs.file(p).map_or(0, |b| b.len() as u64))
            .sum();
        assert_eq!(on_disk, thumbs.size(), "index and folder agree");
    }

    #[test]
    fn replacing_an_entry_keeps_the_size_right() {
        let w = World::default();
        let thumbs = w.thumbs(small());
        thumbs.put(&url(1), &png(100), "image/png").expect("put");
        thumbs.put(&url(1), &png(50), "image/png").expect("put");
        assert_eq!(thumbs.size(), 50);
        thumbs
            .put(&url(1), &image(ImageType::Webp, 80), "image/webp")
            .expect("put");
        assert_eq!(thumbs.size(), 80);
        assert_eq!(w.our_files(), vec![thumb_path(&url(1), "webp")]);
        assert_eq!(
            thumbs.get(&url(1)).expect("read").map(|b| b.content_type),
            Some(ImageType::Webp)
        );
    }

    #[test]
    fn a_damaged_entry_is_dropped() {
        let w = World::default();
        let thumbs = w.thumbs(small());
        thumbs.put(&url(1), &png(100), "image/png").expect("put");
        let path = thumb_path(&url(1), "png");
        w.fs.set_bytes(&path, &[0u8; 100]);
        assert!(thumbs.get(&url(1)).expect("read").is_none());
        assert_eq!(w.fs.file(&path), None);
        assert!(!thumbs.contains(&url(1)));
        assert_eq!(thumbs.size(), 0);
        assert_eq!(w.log.deleted(), vec![path]);
    }

    #[test]
    fn a_truncated_entry_is_dropped() {
        let w = World::default();
        let thumbs = w.thumbs(small());
        thumbs.put(&url(1), &png(100), "image/png").expect("put");
        let path = thumb_path(&url(1), "png");
        w.fs.set_bytes(&path, &png(60));
        assert!(thumbs.get(&url(1)).expect("read").is_none());
        assert_eq!(w.fs.file(&path), None);
        assert_eq!(thumbs.size(), 0);
    }

    #[test]
    fn an_entry_deleted_behind_our_back_is_a_miss() {
        let w = World::default();
        let thumbs = w.thumbs(small());
        thumbs.put(&url(1), &png(100), "image/png").expect("put");
        w.fs.delete(&thumb_path(&url(1), "png"));
        assert!(thumbs.get(&url(1)).expect("read").is_none());
        assert_eq!(thumbs.size(), 0);
    }

    #[test]
    fn a_failed_write_stores_nothing() {
        let w = World::default();
        let thumbs = w.thumbs(small());
        w.fs.fail_writes(true);
        assert!(matches!(
            thumbs.put(&url(1), &png(100), "image/png"),
            Err(PutError::Io(_))
        ));
        w.fs.fail_writes(false);
        w.fs.fail_replace(true);
        assert!(matches!(
            thumbs.put(&url(1), &png(100), "image/png"),
            Err(PutError::Io(_))
        ));
        assert!(w.our_files().is_empty(), "{:?}", w.our_files());
        assert!(thumbs.get(&url(1)).expect("read").is_none());
        assert_eq!(thumbs.size(), 0);
        assert!(
            w.log.all().iter().any(|e| e.result == Outcome::Failed),
            "failures are logged"
        );
    }

    #[test]
    fn leftover_temp_files_are_removed_and_never_served() {
        let w = World::default();
        let final_path = thumb_path(&url(1), "png");
        let tmp = format!("{final_path}.7.tmp");
        w.fs.add(&tmp, &png(100), st(t0()));
        let thumbs = w.thumbs(small());
        assert_eq!(w.fs.file(&tmp), None);
        assert!(thumbs.get(&url(1)).expect("read").is_none());
        assert_eq!(thumbs.size(), 0);
        assert_eq!(w.log.deleted(), vec![norm(&tmp)]);
    }

    #[test]
    fn reopening_rebuilds_the_index_and_the_access_order() {
        let w = World::default();
        {
            let thumbs = w.thumbs(small());
            thumbs.put(&url(1), &png(100), "image/png").expect("put");
            w.clock.advance(Duration::from_secs(1));
            thumbs.put(&url(2), &png(100), "image/png").expect("put");
            w.clock.advance(Duration::from_secs(1));
            thumbs.get(&url(1)).expect("read").expect("hit");
        }
        let touched = t0() + Duration::from_secs(2);
        assert_eq!(w.fs.mtime(&thumb_path(&url(1), "png")), Some(st(touched)));
        let thumbs = w.thumbs(small());
        assert_eq!(thumbs.size(), 200);
        w.clock.advance(Duration::from_secs(1));
        thumbs.put(&url(3), &png(100), "image/png").expect("put");
        thumbs.put(&url(4), &png(100), "image/png").expect("put");
        assert!(thumbs.contains(&url(1)), "read last before the restart");
        assert!(!thumbs.contains(&url(2)), "oldest access");
        assert_eq!(
            thumbs.get(&url(1)).expect("read").map(|b| b.bytes),
            Some(png(100))
        );
    }

    #[test]
    fn opening_over_the_limit_evicts_the_oldest() {
        let w = World::default();
        {
            let thumbs = w.thumbs(small());
            for n in 1..=3 {
                thumbs.put(&url(n), &png(100), "image/png").expect("put");
                w.clock.advance(Duration::from_secs(1));
            }
        }
        let thumbs = w.thumbs(BlobLimits {
            max_total_bytes: 200,
            max_entry_bytes: 150,
        });
        assert_eq!(thumbs.size(), 200);
        assert!(!thumbs.contains(&url(1)));
        assert!(thumbs.contains(&url(2)) && thumbs.contains(&url(3)));
    }

    #[test]
    fn lowering_the_limit_evicts_right_away() {
        let w = World::default();
        let thumbs = w.thumbs(small());
        for n in 1..=3 {
            thumbs.put(&url(n), &png(100), "image/png").expect("put");
            w.clock.advance(Duration::from_secs(1));
        }
        let lower = BlobLimits {
            max_total_bytes: 150,
            max_entry_bytes: 150,
        };
        thumbs.set_limits(lower);
        assert_eq!(thumbs.limits(), lower);
        assert_eq!(thumbs.size(), 100);
        assert!(thumbs.contains(&url(3)));
        assert_eq!(w.our_files(), vec![thumb_path(&url(3), "png")]);
    }

    #[test]
    fn foreign_files_are_neither_counted_nor_touched() {
        let w = World::default();
        let h = hex_of(&url(9));
        let wrong_shard = if h.starts_with("00") { "01" } else { "00" };
        let foreign = [
            format!(r"{ROOT}\thumbs\readme.txt"),
            format!(r"{ROOT}\thumbs\{}\notes.txt", &h[..2]),
            format!(r"{ROOT}\thumbs\zz\{h}.png"),
            format!(r"{ROOT}\thumbs\{wrong_shard}\{h}.png"),
            format!(r"{ROOT}\thumbs\{}\{h}.bmp", &h[..2]),
            format!(r"{ROOT}\thumbs\{}\{}.png", &h[..2], &h[..63]),
        ];
        for f in &foreign {
            w.fs.add(f, &png(10), st(t0()));
        }
        let thumbs = w.thumbs(small());
        assert_eq!(thumbs.size(), 0);
        assert!(!thumbs.contains(&url(9)));
        thumbs.put(&url(9), &png(20), "image/png").expect("put");
        let cleared = thumbs.clear().expect("clear");
        assert_eq!(
            cleared,
            Cleared {
                bytes: 20,
                files: 1,
                failed: 0
            }
        );
        for f in &foreign {
            assert!(w.fs.file(f).is_some(), "{f} kept");
        }
    }

    #[test]
    fn clear_removes_our_files_and_empty_shard_folders() {
        let w = World::default();
        let thumbs = w.thumbs(small());
        thumbs.put(&url(1), &png(100), "image/png").expect("put");
        thumbs.put(&url(2), &png(50), "image/png").expect("put");
        let tmp = format!("{}.3.tmp", thumb_path(&url(1), "png"));
        w.fs.add(&tmp, &png(30), st(t0()));
        let cleared = thumbs.clear().expect("clear");
        assert_eq!(
            cleared,
            Cleared {
                bytes: 180,
                files: 3,
                failed: 0
            }
        );
        assert!(w.our_files().is_empty());
        assert_eq!(thumbs.size(), 0);
        assert!(thumbs.get(&url(1)).expect("read").is_none());
        let h = hex_of(&url(1));
        assert!(!w.fs.has_dir(&format!(r"{ROOT}\thumbs\{}", &h[..2])));
        assert!(w.fs.has_dir(&format!(r"{ROOT}\thumbs")));
        thumbs
            .put(&url(1), &png(100), "image/png")
            .expect("works after clear");
        assert!(thumbs.contains(&url(1)));
    }

    #[test]
    fn writes_and_deletes_are_logged() {
        let w = World::default();
        let thumbs = w.thumbs(BlobLimits {
            max_total_bytes: 100,
            max_entry_bytes: 100,
        });
        thumbs.put(&url(1), &png(100), "image/png").expect("put");
        thumbs.put(&url(2), &png(100), "image/png").expect("put");
        let events = w.log.all();
        assert_eq!(events.len(), 3, "{events:?}");
        assert_eq!(events[0].op, FileOp::Write);
        assert_eq!(
            events[0].dst.as_deref().map(norm),
            Some(thumb_path(&url(1), "png"))
        );
        assert_eq!(events[0].bytes, Some(100));
        assert_eq!(events[0].result, Outcome::Ok);
        assert_eq!(events[1].op, FileOp::Write);
        assert_eq!(events[2].op, FileOp::Delete);
        assert_eq!(
            events[2].src.as_deref().map(norm),
            Some(thumb_path(&url(1), "png"))
        );
        assert_eq!(events[2].bytes, Some(100));
    }

    #[test]
    fn threads_can_share_one_area() {
        let w = World::default();
        let thumbs = w.thumbs(small());
        std::thread::scope(|s| {
            for t in 0..4u32 {
                let thumbs = &thumbs;
                s.spawn(move || {
                    for n in 0..50u32 {
                        let u = url(t * 1000 + n % 7);
                        thumbs
                            .put(&u, &png(30 + n as usize), "image/png")
                            .expect("put");
                        let _ = thumbs.get(&url(n % 7)).expect("read");
                    }
                });
            }
        });
        assert!(thumbs.size() <= 300);
        let on_disk: u64 = w
            .our_files()
            .iter()
            .map(|p| w.fs.file(p).map_or(0, |b| b.len() as u64))
            .sum();
        assert_eq!(on_disk, thumbs.size());
    }

    // ---- catalog

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct Page {
        ids: Vec<u64>,
    }

    fn page_value() -> Page {
        Page {
            ids: vec![655_209, 655_210],
        }
    }

    fn key(page: u32) -> PageKey {
        PageKey {
            section: Section::Mod,
            sort: "new".to_owned(),
            category: Some(2_000),
            page,
            search: None,
        }
    }

    fn ten_minutes() -> CatalogLimits {
        CatalogLimits {
            ttl: Duration::from_secs(600),
            max_total_bytes: 1_000_000,
        }
    }

    fn page_file(w: &World) -> String {
        w.our_files()
            .into_iter()
            .find(|n| n.contains(r"\catalog\page-"))
            .expect("page file")
    }

    #[test]
    fn a_page_just_saved_is_fresh() {
        let w = World::default();
        let catalog = w.catalog(ten_minutes());
        catalog.put_page(&key(1), &page_value()).expect("put");
        match catalog.get_page::<Page>(&key(1)).expect("get") {
            Lookup::Fresh(saved) => {
                assert_eq!(saved.value, page_value());
                assert_eq!(saved.fetched_at, t0());
                assert_eq!(saved.age, Duration::ZERO);
            }
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn a_page_turns_stale_at_the_ttl() {
        let w = World::default();
        let catalog = w.catalog(ten_minutes());
        catalog.put_page(&key(1), &page_value()).expect("put");
        w.clock.advance(Duration::from_millis(599_999));
        assert!(matches!(
            catalog.get_page::<Page>(&key(1)).expect("get"),
            Lookup::Fresh(Saved { age, .. }) if age == Duration::from_millis(599_999)
        ));
        w.clock.advance(Duration::from_millis(1));
        assert!(matches!(
            catalog.get_page::<Page>(&key(1)).expect("get"),
            Lookup::Stale(Saved { age, .. }) if age == Duration::from_secs(600)
        ));
        w.clock.advance(Duration::from_secs(86_400));
        let lookup = catalog.get_page::<Page>(&key(1)).expect("get");
        assert!(lookup.is_stale());
        let saved = lookup.saved().expect("still returned");
        assert_eq!(saved.value, page_value());
        assert_eq!(
            saved.fetched_at,
            t0(),
            "for \"showing saved catalog from …\""
        );
        assert_eq!(saved.age, Duration::from_secs(86_400 + 600));
    }

    #[test]
    fn a_page_from_the_future_is_stale() {
        let w = World::default();
        let catalog = w.catalog(ten_minutes());
        catalog.put_page(&key(1), &page_value()).expect("put");
        w.clock.set(t0() - Duration::from_secs(3_600));
        assert!(matches!(
            catalog.get_page::<Page>(&key(1)).expect("get"),
            Lookup::Stale(Saved { age, .. }) if age == Duration::ZERO
        ));
    }

    #[test]
    fn fetch_times_are_shown_in_the_clock_offset() {
        let w = World::default();
        let local = time::UtcOffset::from_hms(3, 0, 0).expect("offset");
        w.clock.set(t0().to_offset(local));
        let catalog = w.catalog(ten_minutes());
        catalog.put_page(&key(1), &page_value()).expect("put");
        let saved = catalog
            .get_page::<Page>(&key(1))
            .expect("get")
            .saved()
            .expect("hit");
        assert_eq!(saved.fetched_at, t0());
        assert_eq!(saved.fetched_at.offset(), local);
    }

    #[test]
    fn every_request_parameter_is_part_of_the_key() {
        let w = World::default();
        let catalog = w.catalog(ten_minutes());
        catalog.put_page(&key(1), &page_value()).expect("put");
        let variants = [
            PageKey {
                section: Section::Sound,
                ..key(1)
            },
            PageKey {
                sort: "popular".to_owned(),
                ..key(1)
            },
            PageKey {
                category: None,
                ..key(1)
            },
            key(2),
            PageKey {
                search: Some("faun".to_owned()),
                ..key(1)
            },
            PageKey {
                search: Some(String::new()),
                ..key(1)
            },
        ];
        for k in &variants {
            assert!(
                matches!(catalog.get_page::<Page>(k).expect("get"), Lookup::Missing),
                "{k:?}"
            );
        }
        assert!(
            catalog
                .get_page::<Page>(&key(1))
                .expect("get")
                .saved()
                .is_some()
        );
    }

    #[test]
    fn the_ttl_can_be_changed() {
        let w = World::default();
        let catalog = w.catalog(ten_minutes());
        catalog.put_page(&key(1), &page_value()).expect("put");
        w.clock.advance(Duration::from_secs(300));
        catalog.set_limits(CatalogLimits {
            ttl: Duration::from_secs(60),
            ..ten_minutes()
        });
        assert!(catalog.get_page::<Page>(&key(1)).expect("get").is_stale());
        catalog.set_limits(CatalogLimits {
            ttl: Duration::from_secs(3_600),
            ..ten_minutes()
        });
        assert!(matches!(
            catalog.get_page::<Page>(&key(1)).expect("get"),
            Lookup::Fresh(_)
        ));
        assert_eq!(catalog.limits().ttl, Duration::from_secs(3_600));
    }

    #[test]
    fn a_damaged_page_is_dropped() {
        let w = World::default();
        let catalog = w.catalog(ten_minutes());
        catalog.put_page(&key(1), &page_value()).expect("put");
        let path = page_file(&w);
        w.fs.set_bytes(&path, b"{\"schema\":1,\"key\":");
        assert!(matches!(
            catalog.get_page::<Page>(&key(1)).expect("get"),
            Lookup::Missing
        ));
        assert_eq!(w.fs.file(&path), None);
        assert_eq!(w.log.deleted(), vec![path]);
    }

    #[test]
    fn a_page_of_another_shape_is_missing() {
        let w = World::default();
        let catalog = w.catalog(ten_minutes());
        catalog.put_page(&key(1), &page_value()).expect("put");
        assert!(matches!(
            catalog.get_page::<Vec<String>>(&key(1)).expect("get"),
            Lookup::Missing
        ));
    }

    #[test]
    fn a_page_from_a_newer_version_is_left_alone() {
        let w = World::default();
        let catalog = w.catalog(ten_minutes());
        catalog.put_page(&key(1), &page_value()).expect("put");
        let path = page_file(&w);
        w.fs.set_bytes(&path, br#"{"schema":2,"whatever":true}"#);
        assert!(matches!(
            catalog.get_page::<Page>(&key(1)).expect("get"),
            Lookup::Missing
        ));
        assert!(w.fs.file(&path).is_some());
    }

    #[test]
    fn pages_are_json_that_names_their_request() {
        let w = World::default();
        let catalog = w.catalog(ten_minutes());
        catalog.put_page(&key(1), &page_value()).expect("put");
        let path = page_file(&w);
        let text = String::from_utf8(w.fs.file(&path).expect("file")).expect("utf-8");
        assert!(text.starts_with(r#"{"schema":1,"#), "{text}");
        assert!(text.contains(r#""sort":"new""#), "{text}");
        assert!(text.contains(r#""section":"mod""#), "{text}");
        assert!(
            text.contains(&format!(
                r#""fetched_at_unix_ms":{}"#,
                t0().unix_timestamp() * 1000
            )),
            "{text}"
        );
        assert_eq!(w.our_files().len(), 1, "no temp file left");
    }

    #[test]
    fn a_card_is_current_until_the_listing_shows_a_newer_update() {
        let w = World::default();
        let catalog = w.catalog(ten_minutes());
        let updated = t0() - Duration::from_secs(86_400);
        catalog
            .put_card(Section::Mod, 655_209, updated, &page_value())
            .expect("put");
        assert!(
            w.fs.file(&format!(r"{ROOT}\catalog\card-mod-655209.json"))
                .is_some()
        );
        w.clock.advance(Duration::from_secs(3_600));
        let card = catalog
            .get_card::<Page>(Section::Mod, 655_209)
            .expect("get")
            .expect("hit");
        assert_eq!(card.value, page_value());
        assert_eq!(card.updated, updated);
        assert_eq!(card.fetched_at, t0());
        assert!(card.is_current(updated));
        assert!(card.is_current(updated - Duration::from_secs(60)));
        assert!(!card.is_current(updated + Duration::from_secs(1)));
        assert!(
            catalog
                .get_card::<Page>(Section::Sound, 655_209)
                .expect("get")
                .is_none()
        );
        assert!(
            catalog
                .get_card::<Page>(Section::Mod, 1)
                .expect("get")
                .is_none()
        );
    }

    #[test]
    fn the_catalog_stays_under_its_limit_by_dropping_the_oldest() {
        let w = World::default();
        let probe = w.catalog(ten_minutes());
        probe.put_page(&key(0), &page_value()).expect("put");
        let one = probe.size().expect("size");
        probe.clear().expect("clear");
        let catalog = w.catalog(CatalogLimits {
            max_total_bytes: one * 3,
            ..ten_minutes()
        });
        for p in 1..=5 {
            catalog.put_page(&key(p), &page_value()).expect("put");
            w.clock.advance(Duration::from_secs(1));
        }
        assert!(catalog.size().expect("size") <= one * 3);
        for p in 1..=2 {
            assert!(matches!(
                catalog.get_page::<Page>(&key(p)).expect("get"),
                Lookup::Missing
            ));
        }
        for p in 3..=5 {
            assert!(
                catalog
                    .get_page::<Page>(&key(p))
                    .expect("get")
                    .saved()
                    .is_some()
            );
        }
    }

    // ---- the whole cache

    #[test]
    fn sizes_are_reported_per_area() {
        let w = World::default();
        let cache = w.cache();
        assert_eq!(cache.sizes().expect("sizes"), Sizes::default());
        cache
            .thumbs
            .put(&url(1), &png(100), "image/png")
            .expect("put");
        cache
            .images
            .put(&url(1), &png(50), "image/png")
            .expect("put");
        cache.catalog.put_page(&key(1), &page_value()).expect("put");
        let page_len = w.fs.file(&page_file(&w)).expect("page").len() as u64;
        let sizes = cache.sizes().expect("sizes");
        assert_eq!(
            sizes,
            Sizes {
                catalog: page_len,
                thumbs: 100,
                images: 50
            }
        );
        assert_eq!(sizes.total(), page_len + 150);
    }

    #[test]
    fn clear_empties_every_area_and_keeps_foreign_files() {
        let w = World::default();
        let foreign = [
            format!(r"{ROOT}\catalog\notes.txt"),
            format!(r"{ROOT}\catalog\page-abc.json"),
            format!(r"{ROOT}\thumbs\keep.me"),
            format!(r"{ROOT}\images\ab\x.png"),
            format!(r"{ROOT}\other\file.bin"),
            format!(r"{ROOT}\manifest.json"),
        ];
        for f in &foreign {
            w.fs.add(f, b"mine", st(t0()));
        }
        let cache = w.cache();
        cache
            .thumbs
            .put(&url(1), &png(100), "image/png")
            .expect("put");
        cache
            .images
            .put(&url(2), &png(50), "image/png")
            .expect("put");
        cache.catalog.put_page(&key(1), &page_value()).expect("put");
        cache
            .catalog
            .put_card(Section::Sound, 7, t0(), &page_value())
            .expect("put");
        let before = cache.sizes().expect("sizes").total();
        let usage = cache.usage().expect("usage");
        assert_eq!(
            (usage.thumbnails, usage.images, usage.pages),
            (1, 1, 2),
            "foreign files aren't counted"
        );
        assert_eq!(usage.sizes.total(), before);
        let cleared = cache.clear().expect("clear");
        assert_eq!(cleared.bytes, before);
        assert_eq!(cleared.files, 4);
        assert_eq!(cleared.failed, 0);
        assert_eq!(cache.sizes().expect("sizes"), Sizes::default());
        assert_eq!(cache.usage().expect("usage"), Usage::default());
        let mut left = w.our_files();
        left.sort();
        let mut expected: Vec<String> = foreign.iter().map(norm).collect();
        expected.sort();
        assert_eq!(left, expected);
    }

    #[test]
    fn works_on_a_real_disk() {
        let dir = std::env::temp_dir().join(format!("deadloader-cache-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("thumbs")).expect("temp dir");
        std::fs::write(dir.join("thumbs").join("keep.txt"), b"foreign").expect("foreign");
        let clock = TestClock::new();
        let log = Events::default();
        {
            let cache =
                Cache::open(&dir, DiskCacheFs, &clock, &log, Settings::default()).expect("open");
            cache
                .thumbs
                .put(&url(1), &png(1000), "image/png")
                .expect("put");
            cache
                .thumbs
                .put(&url(2), &image(ImageType::Jpeg, 2000), "image/jpeg")
                .expect("put");
            clock.advance(Duration::from_secs(5));
            assert_eq!(
                cache.thumbs.get(&url(1)).expect("read").map(|b| b.bytes),
                Some(png(1000))
            );
            cache.catalog.put_page(&key(1), &page_value()).expect("put");
            assert!(matches!(
                cache.catalog.get_page::<Page>(&key(1)).expect("get"),
                Lookup::Fresh(_)
            ));
        }
        let h = hex_of(&url(1));
        let path = dir.join("thumbs").join(&h[..2]).join(format!("{h}.png"));
        let modified = std::fs::metadata(&path)
            .and_then(|m| m.modified())
            .expect("mtime");
        assert_eq!(
            modified,
            st(t0() + Duration::from_secs(5)),
            "touched on read"
        );
        let cache =
            Cache::open(&dir, DiskCacheFs, &clock, &log, Settings::default()).expect("reopen");
        let sizes = cache.sizes().expect("sizes");
        assert_eq!(sizes.thumbs, 3000);
        assert!(sizes.catalog > 0);
        assert_eq!(
            cache
                .thumbs
                .get(&url(2))
                .expect("read")
                .map(|b| b.content_type),
            Some(ImageType::Jpeg)
        );
        let cleared = cache.clear().expect("clear");
        assert_eq!(cleared.files, 3);
        assert_eq!(cache.sizes().expect("sizes"), Sizes::default());
        assert!(dir.join("thumbs").join("keep.txt").exists());
        assert!(!dir.join("thumbs").join(&h[..2]).exists());
        std::fs::remove_dir_all(&dir).expect("cleanup");
    }

    #[test]
    fn the_cache_can_be_shared_between_threads() {
        fn shareable<T: Send + Sync>() {}
        shareable::<
            Cache<
                DiskCacheFs,
                crate::log::SystemClock,
                std::sync::Arc<crate::log::Logger<crate::log::SystemClock, crate::log::DiskLogDir>>,
            >,
        >();
    }
}
