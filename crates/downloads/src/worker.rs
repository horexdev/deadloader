//! One run of one task: resume the `.part`, receive the body, verify, hand off.
//!
//! The worker owns the file; a fetch thread owns the HTTP body and passes chunks over a small bounded channel (back
//! pressure reaches the socket). The worker only waits on that channel, so a pause, a cancel or the stall timeout is
//! handled at once even when the connection hangs; the fetch thread is then abandoned and ends at its next read.

use crate::Cancel;
use crate::error::DownloadError;
use crate::http::{Validator, parse_content_range};
use crate::limiter::CHUNK;
use crate::log::{fields, file_op};
use crate::queue::{Ask, Control, Shared};
use crate::speed::{SpeedMeter, Throttle};
use crate::store::TaskRecord;
use crate::task::{Md5, TaskId, TaskState};
use deadloader_core::log::{FileOp, Level};
use deadloader_net::allowlist::Purpose;
use deadloader_net::{ByteRange, NetError, Request};
use md5::{Digest, Md5 as Md5Hasher};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::{Component, Path, PathBuf, Prefix};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, RecvTimeoutError, sync_channel};
use std::thread::JoinHandle;
use std::time::Instant;

/// Chunks in flight between the fetch thread and the worker.
const CHANNEL: usize = 8;

pub(crate) enum Msg {
    Head(Result<Head, NetError>),
    /// Bytes and when they arrived: the worker may get to them later, and the clock may have moved on by then.
    Data(Vec<u8>, Instant),
    End,
    BodyError(io::Error),
    /// Look at the control request.
    Wake,
}

pub(crate) struct Head {
    status: u16,
    headers: Vec<(String, String)>,
}

impl Head {
    fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(name))
            .map(|(_, v)| v.as_str())
    }
}

#[derive(Debug)]
pub(crate) enum Outcome {
    Done,
    Paused,
    Cancelled,
    Stopped,
    Failed(DownloadError),
}

pub(crate) fn run(shared: Arc<Shared>, id: TaskId, control: Arc<Control>) {
    let record = shared
        .lock()
        .tasks
        .iter()
        .find(|t| t.record.id == id)
        .map(|t| t.record.clone());
    let outcome = match record {
        Some(record) => Worker::new(&shared, &control, record).go(),
        // Never happens: a running task can't be removed. `finish` still counts the worker off.
        None => Outcome::Cancelled,
    };
    control.attach(None);
    shared.finish(id, outcome);
}

struct Worker<'a> {
    shared: &'a Arc<Shared>,
    control: &'a Control,
    record: TaskRecord,
    part_path: PathBuf,
    archive_path: PathBuf,
    meter: SpeedMeter,
    throttle: Throttle,
    last_checkpoint: Instant,
    total: Option<u64>,
}

/// The `.part` file, open for appending at `len`.
struct Part {
    file: Option<File>,
    len: u64,
}

impl Part {
    fn file(&mut self) -> io::Result<&mut File> {
        self.file
            .as_mut()
            .ok_or_else(|| io::Error::other("the download file is closed"))
    }
}

/// What a response means for the bytes we already have.
enum Decision {
    Continue,
    /// A full body (200) while resuming: keep reading it, from byte 0.
    RestartInPlace(&'static str),
    /// Unusable for resuming: ask again for the whole file.
    AskAgain(String),
    Fail(DownloadError),
}

/// A fetch thread and its channel.
struct Fetch {
    rx: Receiver<Msg>,
    abandon: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

impl Fetch {
    fn stop(self, control: &Control) {
        self.abandon.store(true, Ordering::SeqCst);
        control.attach(None);
        // Dropping `rx` makes the fetch thread's next send fail.
    }

    fn finished(&self) -> bool {
        self.thread.as_ref().is_none_or(JoinHandle::is_finished)
    }
}

impl<'a> Worker<'a> {
    fn new(shared: &'a Arc<Shared>, control: &'a Control, record: TaskRecord) -> Self {
        let store = shared.store();
        let part_path = store.part_path(record.id);
        let archive_path = store.archive_path(record.id, &record.file_name);
        let now = shared.now();
        Self {
            meter: SpeedMeter::new(shared.timing.speed_smoothing),
            throttle: Throttle::new(shared.timing.progress_every),
            last_checkpoint: now,
            total: record.size,
            shared,
            control,
            record,
            part_path,
            archive_path,
        }
    }

    fn id(&self) -> TaskId {
        self.record.id
    }

    fn go(&mut self) -> Outcome {
        // A verified archive from an earlier run: its hook failed, or deadloader closed right after the rename.
        if self.archive_path.exists() {
            self.set_state(TaskState::Verifying);
            let control = self.control;
            match verify_file(&self.archive_path, &self.record, &|| {
                control.cancel_requested()
            }) {
                Ok(true) => return self.hand_off(),
                Ok(false) => return self.cancelled_install(),
                Err(DownloadError::Disk(e)) => return Outcome::Failed(DownloadError::Disk(e)),
                Err(mismatch) => {
                    self.app_log(
                        Level::Warn,
                        "the downloaded archive no longer matches; downloading it again",
                        &mismatch.to_string(),
                    );
                    let path = self.archive_path.clone();
                    if let Err(e) = self.delete(&path, "deleted a damaged download") {
                        return Outcome::Failed(DownloadError::Disk(e));
                    }
                    self.set_state(TaskState::Downloading);
                }
            }
        }
        let mut part = match self.open_part() {
            Ok(part) => part,
            Err(e) => return Outcome::Failed(DownloadError::Disk(e)),
        };
        let mut hasher = Md5Hasher::new();
        if part.len > 0 {
            let control = self.control;
            match hash_prefix(&self.part_path, part.len, &mut hasher, &|| {
                control.cancel_requested()
            }) {
                Ok(true) => {}
                // Cancelled while reading what was already downloaded (the hash is incomplete: never go on).
                Ok(false) => return self.requested(&mut part).unwrap_or(Outcome::Stopped),
                Err(e) => return Outcome::Failed(DownloadError::Disk(e)),
            }
        }
        self.report(part.len, true);
        if part.len > 0 && self.record.size == Some(part.len) {
            // Everything arrived in an earlier run (paused or closed right at the end).
            return self.verify_and_hand_off(part, hasher);
        }
        let mut asked_again = false;
        loop {
            if let Some(size) = self.record.size
                && let Err(e) = self.check_space(size.saturating_sub(part.len))
            {
                return Outcome::Failed(e);
            }
            if let Some(outcome) = self.requested(&mut part) {
                return outcome;
            }
            let fetch = match self.start_fetch(part.len) {
                Ok(fetch) => fetch,
                Err(e) => return Outcome::Failed(DownloadError::Disk(e)),
            };
            let head = match self.wait_for_head(&fetch, &mut part) {
                Ok(head) => head,
                Err(outcome) => {
                    fetch.stop(self.control);
                    return outcome;
                }
            };
            match self.decide(&head, part.len) {
                Decision::Continue => {}
                Decision::RestartInPlace(reason) => {
                    if let Err(e) = self.restart(&mut part, &mut hasher, reason) {
                        fetch.stop(self.control);
                        return Outcome::Failed(DownloadError::Disk(e));
                    }
                }
                Decision::AskAgain(reason) => {
                    fetch.stop(self.control);
                    if asked_again {
                        return self.fail_keeping(
                            &mut part,
                            DownloadError::Inconsistent { detail: reason },
                        );
                    }
                    asked_again = true;
                    if let Err(e) = self.restart(&mut part, &mut hasher, &reason) {
                        return Outcome::Failed(DownloadError::Disk(e));
                    }
                    continue;
                }
                Decision::Fail(e) => {
                    fetch.stop(self.control);
                    return self.fail_keeping(&mut part, e);
                }
            }
            if let (Some(expected), Some(actual)) = (self.record.size, self.total)
                && expected != actual
            {
                fetch.stop(self.control);
                self.discard(part);
                return Outcome::Failed(DownloadError::Size { expected, actual });
            }
            if self.record.size.is_none()
                && let Some(total) = self.total
                && let Err(e) = self.check_space(total.saturating_sub(part.len))
            {
                fetch.stop(self.control);
                return self.fail_keeping(&mut part, e);
            }
            let validator = Validator::from_headers(|n| head.header(n).map(str::to_owned));
            self.remember(validator);
            match self.receive(&fetch, &mut part, &mut hasher) {
                Ok(()) => {
                    fetch.stop(self.control);
                    break;
                }
                Err(outcome) => {
                    fetch.stop(self.control);
                    return outcome;
                }
            }
        }
        self.verify_and_hand_off(part, hasher)
    }

    fn verify_and_hand_off(&mut self, mut part: Part, hasher: Md5Hasher) -> Outcome {
        // A cancel or pause that arrived with the last bytes still counts.
        if let Some(outcome) = self.requested(&mut part) {
            return outcome;
        }
        self.set_state(TaskState::Verifying);
        if let Err(e) = part.file().and_then(|f| f.sync_all()) {
            return Outcome::Failed(DownloadError::Disk(e));
        }
        let len = part.len;
        part.file = None;
        if let Some(expected) = self.record.size
            && expected != len
        {
            self.discard(part);
            return Outcome::Failed(DownloadError::Size {
                expected,
                actual: len,
            });
        }
        let actual = Md5(hasher.finalize().into());
        if let Some(expected) = self.record.md5
            && expected != actual
        {
            self.discard(part);
            return Outcome::Failed(DownloadError::Checksum { expected, actual });
        }
        if self.control.cancel_requested() {
            self.control.take();
            self.discard(part);
            return Outcome::Cancelled;
        }
        let renamed = file_op(
            self.shared.deps.log.as_ref(),
            FileOp::Rename,
            "verified a download",
            Some(&self.part_path),
            Some(&self.archive_path),
            Some(len),
            std::fs::rename(&self.part_path, &self.archive_path),
        );
        if let Err(e) = renamed {
            return Outcome::Failed(DownloadError::Disk(e));
        }
        self.hand_off()
    }

    fn hand_off(&mut self) -> Outcome {
        self.set_state(TaskState::Installing);
        if self.control.cancel_requested() {
            return self.cancelled_install();
        }
        let info = self.record.info();
        let control = self.control;
        let asked = || control.cancel_requested();
        let result =
            self.shared
                .deps
                .hook
                .downloaded(&self.archive_path, &info, &Cancel::new(&asked));
        match result {
            Err(message) if self.control.cancel_requested() => {
                self.app_log(
                    Level::Info,
                    "install cancelled",
                    &format!("{:?}", message.id),
                );
                self.cancelled_install()
            }
            Ok(()) => {
                if self.control.cancel_requested() {
                    self.app_log(
                        Level::Info,
                        "a cancel came after the mod was in the library; it stays installed",
                        "",
                    );
                }
                if !self.shared.lock().keep_archives {
                    let path = self.archive_path.clone();
                    // The install worked; a leftover archive is swept by nothing but harms nothing either.
                    let _ = self.delete(&path, "deleted an installed download");
                } else if let Some(wanted) = self.record.keep_as.clone() {
                    let kept = self
                        .shared
                        .store()
                        .kept_path(&wanted, &self.record.file_name);
                    let bytes = std::fs::metadata(&self.archive_path).ok().map(|m| m.len());
                    // A failed rename leaves the archive under its queue name: still kept, just less readable.
                    let _ = file_op(
                        self.shared.deps.log.as_ref(),
                        FileOp::Rename,
                        "kept an installed download",
                        Some(&self.archive_path),
                        Some(&kept),
                        bytes,
                        std::fs::rename(&self.archive_path, &kept),
                    );
                }
                Outcome::Done
            }
            Err(message) => Outcome::Failed(DownloadError::Hook(message)),
        }
    }

    /// The user cancelled while the archive was checked or installed: it goes, kept archives or not (nothing of it
    /// is in the library).
    fn cancelled_install(&mut self) -> Outcome {
        self.control.take();
        let path = self.archive_path.clone();
        let _ = self.delete(&path, "deleted the archive of a cancelled install");
        Outcome::Cancelled
    }

    /// Opens `<id>.part` (creating it if needed) and cuts it to the saved checkpoint: bytes after it were never
    /// confirmed on disk.
    fn open_part(&mut self) -> io::Result<Part> {
        let log = self.shared.deps.log.as_ref();
        let existing = match std::fs::metadata(&self.part_path) {
            Ok(meta) => Some(meta.len()),
            Err(e) if e.kind() == io::ErrorKind::NotFound => None,
            Err(e) => return Err(e),
        };
        let mut file = match existing {
            Some(_) => OpenOptions::new()
                .read(true)
                .write(true)
                .open(&self.part_path)?,
            None => file_op(
                log,
                FileOp::Write,
                "created a download part",
                None,
                Some(&self.part_path),
                Some(0),
                OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create_new(true)
                    .open(&self.part_path),
            )?,
        };
        let on_disk = existing.unwrap_or(0);
        let keep = on_disk.min(self.record.checkpoint);
        if keep < on_disk {
            file_op(
                log,
                FileOp::Write,
                "cut a download part back to its last saved position",
                None,
                Some(&self.part_path),
                Some(keep),
                file.set_len(keep),
            )?;
        }
        file.seek(SeekFrom::Start(keep))?;
        Ok(Part {
            file: Some(file),
            len: keep,
        })
    }

    fn restart(&mut self, part: &mut Part, hasher: &mut Md5Hasher, reason: &str) -> io::Result<()> {
        let had = part.len;
        self.app_log(
            Level::Warn,
            "download restarted from the beginning",
            &format!("{reason}; {had} bytes discarded"),
        );
        let file = part.file()?;
        file_op(
            self.shared.deps.log.as_ref(),
            FileOp::Write,
            "emptied a download part to start over",
            None,
            Some(&self.part_path),
            Some(0),
            file.set_len(0),
        )?;
        file.seek(SeekFrom::Start(0))?;
        part.len = 0;
        *hasher = Md5Hasher::new();
        self.update(|t| {
            t.record.checkpoint = 0;
            t.record.validator = None;
        });
        self.meter.reset();
        self.report(0, true);
        Ok(())
    }

    fn start_fetch(&self, offset: u64) -> io::Result<Fetch> {
        let (tx, rx) = sync_channel(CHANNEL);
        self.control.attach(Some(tx.clone()));
        let abandon = Arc::new(AtomicBool::new(false));
        let mut request = Request::new(
            Purpose::Download,
            self.record.initiator.into(),
            self.record.url.clone(),
        );
        if offset > 0 {
            request = request.range(ByteRange {
                start: offset,
                end: None,
            });
            if let Some(value) = self.record.validator.as_ref().and_then(Validator::if_range) {
                request = request.header("If-Range", value);
            }
        }
        let shared = Arc::clone(self.shared);
        let flag = Arc::clone(&abandon);
        let thread = std::thread::Builder::new()
            .name(format!("download-{}-fetch", self.id()))
            .spawn(move || fetch(&shared, &request, &tx, &flag))?;
        Ok(Fetch {
            rx,
            abandon,
            thread: Some(thread),
        })
    }

    fn wait_for_head(&mut self, fetch: &Fetch, part: &mut Part) -> Result<Head, Outcome> {
        loop {
            // No stall timeout here: the client's own connect/response timeouts and retry waits bound this.
            match fetch.rx.recv_timeout(self.shared.timing.idle_timeout) {
                Ok(Msg::Head(Ok(head))) => return Ok(head),
                Ok(Msg::Head(Err(e))) => return Err(self.fail_keeping(part, e.into())),
                Ok(_) | Err(RecvTimeoutError::Timeout) => {
                    if fetch.finished() && fetch.rx.try_recv().is_err() {
                        return Err(
                            self.fail_keeping(part, lost("the request ended without an answer"))
                        );
                    }
                }
                Err(RecvTimeoutError::Disconnected) => {
                    return Err(
                        self.fail_keeping(part, lost("the request ended without an answer"))
                    );
                }
            }
            if let Some(outcome) = self.requested(part) {
                return Err(outcome);
            }
        }
    }

    fn decide(&mut self, head: &Head, offset: u64) -> Decision {
        let fresh = Validator::from_headers(|n| head.header(n).map(str::to_owned));
        let changed = match (&self.record.validator, &fresh) {
            (Some(old), Some(new)) => old.conflicts_with(new),
            _ => false,
        };
        match head.status {
            200 => {
                self.total = head
                    .header("content-length")
                    .and_then(|v| v.trim().parse().ok())
                    .or(self.record.size);
                if offset == 0 {
                    Decision::Continue
                } else if changed {
                    Decision::RestartInPlace("the file changed on the server")
                } else {
                    Decision::RestartInPlace("the server sent the whole file instead of the rest")
                }
            }
            206 => {
                let Some(range) = head.header("content-range").and_then(parse_content_range) else {
                    return Decision::AskAgain(
                        "a partial answer without a usable Content-Range".into(),
                    );
                };
                if range.first != offset {
                    return Decision::AskAgain(format!(
                        "the server resumed at byte {} instead of {offset}",
                        range.first
                    ));
                }
                if changed {
                    return Decision::AskAgain("the file changed on the server".into());
                }
                self.total = range.complete.or(self.record.size);
                Decision::Continue
            }
            416 if offset > 0 => {
                Decision::AskAgain("the server refused the requested range".into())
            }
            status => Decision::Fail(DownloadError::Status { status }),
        }
    }

    fn receive(
        &mut self,
        fetch: &Fetch,
        part: &mut Part,
        hasher: &mut Md5Hasher,
    ) -> Result<(), Outcome> {
        loop {
            let msg = match fetch.rx.recv_timeout(self.shared.timing.idle_timeout) {
                Ok(msg) => msg,
                Err(RecvTimeoutError::Timeout) => {
                    return Err(self.fail_keeping(
                        part,
                        DownloadError::Stalled {
                            after: self.shared.timing.idle_timeout,
                        },
                    ));
                }
                Err(RecvTimeoutError::Disconnected) => {
                    return Err(self.fail_keeping(part, lost("the transfer ended unexpectedly")));
                }
            };
            match msg {
                Msg::Data(bytes, at) => {
                    let after = part.len + bytes.len() as u64;
                    if let Some(total) = self.total
                        && after > total
                    {
                        let take = Part {
                            file: part.file.take(),
                            len: part.len,
                        };
                        self.discard(take);
                        return Err(Outcome::Failed(DownloadError::Size {
                            expected: total,
                            actual: after,
                        }));
                    }
                    if let Err(e) = self.write(part, &bytes) {
                        return Err(self.fail_keeping(part, DownloadError::Disk(e)));
                    }
                    hasher.update(&bytes);
                    self.report_at(at, part.len, false);
                    if self
                        .shared
                        .now()
                        .saturating_duration_since(self.last_checkpoint)
                        >= self.shared.timing.checkpoint_every
                        && let Err(e) = self.checkpoint(part)
                    {
                        return Err(Outcome::Failed(DownloadError::Disk(e)));
                    }
                }
                Msg::End => {
                    if let Some(total) = self.total
                        && part.len < total
                    {
                        return Err(self.fail_keeping(
                            part,
                            lost(&format!(
                                "the server closed the connection after {} of {total} bytes",
                                part.len
                            )),
                        ));
                    }
                    self.report(part.len, true);
                    return Ok(());
                }
                Msg::BodyError(e) => {
                    return Err(self.fail_keeping(part, lost(&e.to_string())));
                }
                Msg::Wake | Msg::Head(_) => {}
            }
            if let Some(outcome) = self.requested(part) {
                return Err(outcome);
            }
        }
    }

    fn write(&self, part: &mut Part, bytes: &[u8]) -> io::Result<()> {
        #[cfg(test)]
        {
            let mut budget = self
                .shared
                .write_budget
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            if let Some(left) = budget.as_mut() {
                if (bytes.len() as u64) > *left {
                    return Err(io::Error::from(io::ErrorKind::StorageFull));
                }
                *left -= bytes.len() as u64;
            }
        }
        part.file()?.write_all(bytes)?;
        part.len += bytes.len() as u64;
        Ok(())
    }

    /// Acts on a pause, cancel or stop request, if there is one.
    fn requested(&mut self, part: &mut Part) -> Option<Outcome> {
        match self.control.take() {
            Ask::Nothing => None,
            Ask::Pause => Some(match self.checkpoint(part) {
                Ok(()) => Outcome::Paused,
                Err(e) => Outcome::Failed(DownloadError::Disk(e)),
            }),
            Ask::Stop => Some(match self.checkpoint(part) {
                Ok(()) => Outcome::Stopped,
                Err(e) => Outcome::Failed(DownloadError::Disk(e)),
            }),
            Ask::Cancel => {
                let take = Part {
                    file: part.file.take(),
                    len: part.len,
                };
                self.discard(take);
                Some(Outcome::Cancelled)
            }
        }
    }

    /// Flushes the part to disk, then saves its length as the position to resume from.
    fn checkpoint(&mut self, part: &mut Part) -> io::Result<()> {
        part.file()?.sync_data()?;
        let len = part.len;
        self.last_checkpoint = self.shared.now();
        self.update(|t| {
            t.record.checkpoint = len;
            t.bytes = len;
        });
        self.record.checkpoint = len;
        let _ = self.shared.persist();
        Ok(())
    }

    /// Fails the attempt but keeps what was received, so the retry resumes.
    fn fail_keeping(&mut self, part: &mut Part, error: DownloadError) -> Outcome {
        self.report(part.len, true);
        // On a full disk the flush may fail too; the last saved position then stays.
        let _ = self.checkpoint(part);
        Outcome::Failed(error)
    }

    /// Deletes the part: it can't be used (checksum, size, cancel).
    fn discard(&mut self, mut part: Part) {
        part.file = None;
        let path = self.part_path.clone();
        let _ = self.delete(&path, "deleted a download part");
        self.update(|t| {
            t.record.checkpoint = 0;
            t.record.validator = None;
            t.bytes = 0;
        });
        self.record.checkpoint = 0;
        self.record.validator = None;
    }

    fn delete(&self, path: &Path, msg: &str) -> io::Result<()> {
        let bytes = std::fs::metadata(path).ok().map(|m| m.len());
        file_op(
            self.shared.deps.log.as_ref(),
            FileOp::Delete,
            msg,
            Some(path),
            None,
            bytes,
            std::fs::remove_file(path),
        )
    }

    fn check_space(&self, remaining: u64) -> Result<(), DownloadError> {
        let Some(available) = self.shared.deps.space.available_bytes(&self.shared.dir) else {
            return Ok(());
        };
        // Room for the rest of the file plus a margin, so the disk isn't left completely full.
        let needed = remaining.saturating_add((remaining / 10).max(1024 * 1024));
        if needed > available {
            return Err(DownloadError::InsufficientSpace {
                needed,
                available,
                drive: drive_of(&self.shared.dir),
            });
        }
        Ok(())
    }

    fn remember(&mut self, validator: Option<Validator>) {
        let validator = validator.or_else(|| self.record.validator.clone());
        self.record.validator.clone_from(&validator);
        let total = self.total;
        self.update(|t| {
            t.record.validator = validator;
            t.total = total;
        });
    }

    fn report(&mut self, bytes: u64, always: bool) {
        self.report_at(self.shared.now(), bytes, always);
    }

    fn report_at(&mut self, at: Instant, bytes: u64, always: bool) {
        if self.throttle.ready(at) || always {
            let speed = self.meter.sample(at, bytes);
            self.shared.progress(self.id(), bytes, self.total, speed);
        }
    }

    fn set_state(&self, state: TaskState) {
        let mut locked = self.shared.lock();
        self.shared.set_state(&mut locked, self.id(), state);
    }

    fn update(&self, change: impl FnOnce(&mut crate::queue::Task)) {
        if let Some(task) = self.shared.lock().task_mut(self.id()) {
            change(task);
        }
    }

    fn app_log(&self, lvl: Level, msg: &str, detail: &str) {
        self.shared.deps.log.app(
            lvl,
            msg,
            fields([("task", self.id().0.into()), ("detail", detail.into())]),
        );
    }
}

fn lost(detail: &str) -> DownloadError {
    DownloadError::ConnectionLost {
        detail: detail.to_owned(),
    }
}

/// The fetch thread: the request, then the body in rate-limited chunks. When the response ends or fails, the
/// connection is closed before the worker hears of it, so the next download never runs alongside it.
fn fetch(
    shared: &Shared,
    request: &Request,
    tx: &std::sync::mpsc::SyncSender<Msg>,
    abandon: &AtomicBool,
) {
    let response = match shared.deps.client.get(request) {
        Ok(response) => response,
        Err(e) => {
            let _ = tx.send(Msg::Head(Err(e)));
            return;
        }
    };
    let status = response.status;
    let head = Head {
        status,
        headers: response.headers.clone(),
    };
    if !matches!(status, 200 | 206) {
        drop(response);
        let _ = tx.send(Msg::Head(Ok(head)));
        return;
    }
    if tx.send(Msg::Head(Ok(head))).is_err() {
        return;
    }
    let mut body = response.into_body();
    loop {
        if abandon.load(Ordering::SeqCst) {
            return;
        }
        let grant = shared.limiter.acquire(CHUNK);
        let mut buf = vec![0u8; grant];
        match body.read(&mut buf) {
            Ok(0) => {
                shared.limiter.refund(grant);
                drop(body);
                let _ = tx.send(Msg::End);
                return;
            }
            Ok(n) => {
                let at = shared.now();
                shared.limiter.refund(grant - n);
                buf.truncate(n);
                if tx.send(Msg::Data(buf, at)).is_err() {
                    return;
                }
            }
            Err(e) if e.kind() == io::ErrorKind::Interrupted => shared.limiter.refund(grant),
            Err(e) => {
                drop(body);
                let _ = tx.send(Msg::BodyError(e));
                return;
            }
        }
    }
}

/// Hashes the first `len` bytes of `path`; `false` when `stop` said so between chunks (a cancel).
fn hash_prefix(
    path: &Path,
    len: u64,
    hasher: &mut Md5Hasher,
    stop: &dyn Fn() -> bool,
) -> io::Result<bool> {
    let mut file = File::open(path)?.take(len);
    let mut buf = vec![0u8; 1 << 20];
    loop {
        if stop() {
            return Ok(false);
        }
        let n = file.read(&mut buf)?;
        if n == 0 {
            return Ok(true);
        }
        hasher.update(&buf[..n]);
    }
}

/// Size and md5 of a finished file against the listing; `false` when `stop` cut the check short.
fn verify_file(
    path: &Path,
    record: &TaskRecord,
    stop: &dyn Fn() -> bool,
) -> Result<bool, DownloadError> {
    let len = std::fs::metadata(path).map_err(DownloadError::Disk)?.len();
    if let Some(expected) = record.size
        && expected != len
    {
        return Err(DownloadError::Size {
            expected,
            actual: len,
        });
    }
    if let Some(expected) = record.md5 {
        let mut hasher = Md5Hasher::new();
        if !hash_prefix(path, len, &mut hasher, stop).map_err(DownloadError::Disk)? {
            return Ok(false);
        }
        let actual = Md5(hasher.finalize().into());
        if actual != expected {
            return Err(DownloadError::Checksum { expected, actual });
        }
    }
    Ok(true)
}

/// `D:` for a folder on drive D, for the "not enough space on drive D:" message.
fn drive_of(dir: &Path) -> Option<String> {
    match dir.components().next() {
        Some(Component::Prefix(prefix)) => match prefix.kind() {
            Prefix::Disk(letter) | Prefix::VerbatimDisk(letter) => {
                Some(format!("{}:", char::from(letter).to_ascii_uppercase()))
            }
            _ => None,
        },
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[test]
    fn hashing_a_file_stops_between_chunks_when_asked() {
        let dir = std::env::temp_dir().join(format!("deadloader-hash-stop-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("dir");
        let file = dir.join("a.bin");
        let len = 3u64 << 20;
        std::fs::write(&file, vec![7u8; 3 << 20]).expect("file");
        let asked = Cell::new(0);
        let stop = || {
            asked.set(asked.get() + 1);
            asked.get() > 1
        };
        let mut hasher = Md5Hasher::new();
        let done = hash_prefix(&file, len, &mut hasher, &stop).expect("read");
        assert!(!done, "stopped");
        assert_eq!(asked.get(), 2, "after the first chunk");
        let mut full = Md5Hasher::new();
        assert!(hash_prefix(&file, len, &mut full, &|| false).expect("read"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn drives_are_named_like_explorer_does() {
        assert_eq!(
            drive_of(Path::new("d:\\games\\deadloader-data\\downloads")).as_deref(),
            Some("D:")
        );
        assert_eq!(drive_of(Path::new("\\\\?\\C:\\x")).as_deref(), Some("C:"));
        assert_eq!(drive_of(Path::new("\\\\server\\share\\x")), None);
        assert_eq!(drive_of(Path::new("relative")), None);
    }
}
