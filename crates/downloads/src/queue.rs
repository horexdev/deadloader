//! The queue: tasks in FIFO order, a scheduler thread that starts workers up to the parallel limit and wakes up for
//! automatic retries, and the public [`Downloads`] handle.

use crate::error::{DownloadError, Recovery};
use crate::limiter::RateLimiter;
use crate::log::{fields, file_op};
use crate::store::{QueueFile, Saved, Store, TaskRecord};
use crate::task::{Event, Md5, NewTask, TaskId, TaskSnapshot, TaskState};
use crate::worker::{self, Msg, Outcome};
use crate::{Config, Deps, Timing};
use deadloader_core::log::{FileOp, Level};
use deadloader_core::message::{MessageId, ToUserMessage, UserMessage};
use std::collections::BTreeSet;
use std::fmt;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::mpsc::SyncSender;
use std::sync::{Arc, Condvar, Mutex, MutexGuard, PoisonError};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

/// What the queue asks a running worker to do. Checked after every message the worker receives; asking also sends a
/// wake-up message so a worker waiting on a silent connection reacts at once.
pub(crate) struct Control {
    request: AtomicU8,
    wake: Mutex<Option<SyncSender<Msg>>>,
}

/// The worker is acting on a request.
const TAKEN: u8 = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum Ask {
    Nothing = 0,
    Pause = 1,
    Cancel = 2,
    /// The app is closing: save the position and leave the task queued.
    Stop = 3,
}

impl Control {
    fn new() -> Self {
        Self {
            request: AtomicU8::new(Ask::Nothing as u8),
            wake: Mutex::new(None),
        }
    }

    /// The pending request, taken over by the worker: after this it can no longer be withdrawn.
    pub(crate) fn take(&self) -> Ask {
        loop {
            let current = self.request.load(Ordering::SeqCst);
            let ask = match current {
                1 => Ask::Pause,
                2 => Ask::Cancel,
                3 => Ask::Stop,
                _ => return Ask::Nothing,
            };
            if self
                .request
                .compare_exchange(current, TAKEN, Ordering::SeqCst, Ordering::SeqCst)
                .is_ok()
            {
                return ask;
            }
        }
    }

    fn ask(&self, what: Ask) {
        self.request.store(what as u8, Ordering::SeqCst);
        if let Some(tx) = self
            .wake
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .as_ref()
        {
            // A full channel means the worker is busy with data and will look at the request right after.
            let _ = tx.try_send(Msg::Wake);
        }
    }

    /// Withdraws a request that the worker has not acted on yet (resume right after pause).
    fn withdraw(&self, what: Ask) -> bool {
        self.request
            .compare_exchange(
                what as u8,
                Ask::Nothing as u8,
                Ordering::SeqCst,
                Ordering::SeqCst,
            )
            .is_ok()
    }

    pub(crate) fn attach(&self, tx: Option<SyncSender<Msg>>) {
        *self.wake.lock().unwrap_or_else(PoisonError::into_inner) = tx;
    }
}

pub(crate) struct Task {
    pub record: TaskRecord,
    pub state: TaskState,
    pub bytes: u64,
    pub total: Option<u64>,
    pub speed: u64,
    retry_at: Option<Instant>,
    failures: u32,
    /// Set while a worker runs the task.
    control: Option<Arc<Control>>,
    /// Resume was asked while the worker was already pausing: queue it again when it has.
    resume_after_pause: bool,
}

impl Task {
    fn snapshot(&self, now: Instant) -> TaskSnapshot {
        let state = match &self.state {
            TaskState::Waiting { message, .. } => TaskState::Waiting {
                message: message.clone(),
                retry_in_s: self
                    .retry_at
                    .map(|at| at.saturating_duration_since(now).as_secs()),
            },
            other => other.clone(),
        };
        TaskSnapshot {
            info: self.record.info(),
            state,
            bytes: self.bytes,
            total: self.total,
            speed: self.speed,
        }
    }

    fn saved(&self) -> TaskRecord {
        let (state, message) = match &self.state {
            TaskState::Paused => (Saved::Paused, None),
            TaskState::Waiting { message, .. } => (Saved::Waiting, Some(message.clone())),
            TaskState::Failed { message } => (Saved::Failed, Some(message.clone())),
            _ => (Saved::Queued, None),
        };
        TaskRecord {
            state,
            message,
            ..self.record.clone()
        }
    }
}

pub(crate) struct State {
    pub tasks: Vec<Task>,
    next_id: u64,
    parallel: usize,
    pub keep_archives: bool,
    stopping: bool,
    /// Worker threads that have not finished yet (including the final queue save).
    workers: usize,
}

impl State {
    pub(crate) fn task_mut(&mut self, id: TaskId) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|t| t.record.id == id)
    }

    fn running(&self) -> usize {
        self.tasks.iter().filter(|t| t.control.is_some()).count()
    }

    fn queue_file(&self) -> QueueFile {
        QueueFile {
            next_id: self.next_id,
            tasks: self
                .tasks
                .iter()
                .filter(|t| t.state != TaskState::Done)
                .map(Task::saved)
                .collect(),
            ..QueueFile::default()
        }
    }
}

pub(crate) struct Shared {
    pub dir: PathBuf,
    pub deps: Deps,
    pub limiter: RateLimiter,
    pub timing: Timing,
    state: Mutex<State>,
    changed: Condvar,
    /// Serialises queue saves so an older snapshot never overwrites a newer one.
    persisting: Mutex<()>,
    /// Tests: bytes the disk accepts before it is "full".
    #[cfg(test)]
    pub write_budget: Mutex<Option<u64>>,
}

impl Shared {
    pub(crate) fn lock(&self) -> MutexGuard<'_, State> {
        self.state.lock().unwrap_or_else(PoisonError::into_inner)
    }

    pub(crate) fn emit(&self, event: &Event) {
        (self.deps.events)(event);
    }

    pub(crate) fn store(&self) -> Store<'_> {
        Store {
            dir: &self.dir,
            log: self.deps.log.as_ref(),
        }
    }

    pub(crate) fn now(&self) -> Instant {
        self.deps.clock.now()
    }

    /// Saves the queue. Must be called without the state lock held. A failure is in the files log; the queue keeps
    /// running from memory and the next save tries again.
    pub(crate) fn persist(&self) -> std::io::Result<()> {
        let _order = self
            .persisting
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        let queue = self.lock().queue_file();
        self.store().save(&queue)
    }

    pub(crate) fn set_state(&self, state: &mut State, id: TaskId, new: TaskState) {
        if let Some(task) = state.task_mut(id) {
            task.state = new.clone();
            if !matches!(new, TaskState::Waiting { .. }) {
                task.retry_at = None;
            }
            self.emit(&Event::State { id, state: new });
        }
    }

    pub(crate) fn progress(&self, id: TaskId, bytes: u64, total: Option<u64>, speed: u64) {
        let mut state = self.lock();
        if let Some(task) = state.task_mut(id) {
            if bytes > task.bytes {
                task.failures = 0;
            }
            task.bytes = bytes;
            task.total = total;
            task.speed = speed;
            self.emit(&Event::Progress {
                id,
                bytes,
                total,
                speed,
            });
        }
    }

    pub(crate) fn notify(&self) {
        self.changed.notify_all();
    }

    /// A worker is done with its task.
    pub(crate) fn finish(&self, id: TaskId, outcome: Outcome) {
        let mut state = self.lock();
        let now = self.now();
        let Some(task) = state.task_mut(id) else {
            state.workers -= 1;
            drop(state);
            self.notify();
            return;
        };
        task.control = None;
        task.speed = 0;
        match outcome {
            Outcome::Done => {
                self.set_state(&mut state, id, TaskState::Done);
                state.tasks.retain(|t| t.record.id != id);
                self.emit(&Event::Removed { id });
            }
            Outcome::Cancelled => {
                state.tasks.retain(|t| t.record.id != id);
                self.emit(&Event::Removed { id });
            }
            Outcome::Paused => {
                let next = if std::mem::take(&mut task.resume_after_pause) {
                    TaskState::Queued
                } else {
                    TaskState::Paused
                };
                self.set_state(&mut state, id, next);
            }
            Outcome::Stopped => self.set_state(&mut state, id, TaskState::Queued),
            Outcome::Failed(error) => {
                let mut message = error.user_message();
                let line = self.deps.log.app(
                    Level::Error,
                    "download failed",
                    fields([
                        ("task", id.0.into()),
                        ("message", format!("{:?}", message.id).into()),
                        ("error", error.to_string().into()),
                    ]),
                );
                if let Some(line) = line {
                    message = message.arg("log_id", line);
                }
                let failures = task.failures + 1;
                task.failures = failures;
                let new = match error.recovery() {
                    Recovery::Automatic { min_wait } => {
                        let wait =
                            backoff(&self.timing, failures).max(min_wait.unwrap_or_default());
                        task.retry_at = Some(now + wait);
                        TaskState::Waiting {
                            message,
                            retry_in_s: Some(wait.as_secs()),
                        }
                    }
                    Recovery::WhenOnline => TaskState::Waiting {
                        message,
                        retry_in_s: None,
                    },
                    Recovery::User => TaskState::Failed { message },
                };
                self.set_state(&mut state, id, new);
            }
        }
        drop(state);
        let _ = self.persist();
        self.lock().workers -= 1;
        self.notify();
    }

    /// Tests: until every worker has finished, its last queue save included.
    #[cfg(test)]
    pub(crate) fn wait_idle(&self) {
        let mut state = self.lock();
        while state.workers > 0 {
            state = self
                .changed
                .wait(state)
                .unwrap_or_else(PoisonError::into_inner);
        }
    }
}

/// `retry_base × 2^(failures − 1)`, capped at `retry_max`.
fn backoff(timing: &Timing, failures: u32) -> Duration {
    let factor = 1u32
        .checked_shl(failures.saturating_sub(1))
        .unwrap_or(u32::MAX);
    timing
        .retry_base
        .checked_mul(factor)
        .map_or(timing.retry_max, |d| d.min(timing.retry_max))
}

fn scheduler(shared: Arc<Shared>) {
    let mut state = shared.lock();
    loop {
        if state.stopping {
            return;
        }
        let now = shared.now();
        let due: Vec<TaskId> = state
            .tasks
            .iter()
            .filter(|t| {
                matches!(t.state, TaskState::Waiting { .. })
                    && t.retry_at.is_some_and(|at| at <= now)
            })
            .map(|t| t.record.id)
            .collect();
        for id in due {
            shared.set_state(&mut state, id, TaskState::Queued);
        }
        let mut running = state.running();
        let mut index = 0;
        while running < state.parallel && index < state.tasks.len() {
            let task = &mut state.tasks[index];
            index += 1;
            if task.state != TaskState::Queued || task.control.is_some() {
                continue;
            }
            let id = task.record.id;
            let control = Arc::new(Control::new());
            task.control = Some(Arc::clone(&control));
            running += 1;
            state.workers += 1;
            shared.set_state(&mut state, id, TaskState::Downloading);
            let worker_shared = Arc::clone(&shared);
            let spawned = std::thread::Builder::new()
                .name(format!("download-{id}"))
                .spawn(move || worker::run(worker_shared, id, control));
            if let Err(e) = spawned {
                if let Some(task) = state.task_mut(id) {
                    task.control = None;
                }
                running -= 1;
                state.workers -= 1;
                let message = UserMessage::new(MessageId::Internal).arg("detail", e);
                shared.set_state(&mut state, id, TaskState::Failed { message });
            }
        }
        let next = state
            .tasks
            .iter()
            .filter(|t| matches!(t.state, TaskState::Waiting { .. }))
            .filter_map(|t| t.retry_at)
            .min();
        state = match next {
            Some(at) => {
                let wait = at
                    .saturating_duration_since(now)
                    .max(Duration::from_millis(1));
                shared
                    .changed
                    .wait_timeout(state, wait)
                    .unwrap_or_else(PoisonError::into_inner)
                    .0
            }
            None => shared
                .changed
                .wait(state)
                .unwrap_or_else(PoisonError::into_inner),
        };
    }
}

/// The download queue. Every method is quick and thread-safe; the work happens on the queue's own threads.
/// Dropping it (or [`Downloads::shutdown`]) stops running downloads at a resumable point.
pub struct Downloads {
    shared: Arc<Shared>,
    scheduler: Mutex<Option<JoinHandle<()>>>,
}

impl fmt::Debug for Downloads {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Downloads")
            .field("dir", &self.shared.dir)
            .finish_non_exhaustive()
    }
}

impl Downloads {
    /// Loads `queue.json` from `config.dir` and starts: queued tasks and tasks that were running when deadloader
    /// closed or crashed continue; paused and failed tasks stay as they were.
    pub fn open(config: Config, deps: Deps) -> Result<Self, DownloadError> {
        std::fs::create_dir_all(&config.dir).map_err(DownloadError::Disk)?;
        let store = Store {
            dir: &config.dir,
            log: deps.log.as_ref(),
        };
        let loaded = store.load();
        let next_id = loaded
            .queue
            .next_id
            .max(store.highest_id_on_disk().saturating_add(1));
        if loaded.trusted {
            let owned: BTreeSet<TaskId> = loaded.queue.tasks.iter().map(|t| t.id).collect();
            store.sweep_orphans(&owned);
        }
        let now = deps.clock.now();
        let tasks = loaded
            .queue
            .tasks
            .into_iter()
            .map(|record| restored(record, now))
            .collect();
        let shared = Arc::new(Shared {
            limiter: RateLimiter::new(Arc::clone(&deps.clock), config.speed_limit_kib_per_s),
            dir: config.dir,
            deps,
            timing: config.timing,
            state: Mutex::new(State {
                tasks,
                next_id,
                parallel: config.parallel.max(1) as usize,
                keep_archives: config.keep_archives,
                stopping: false,
                workers: 0,
            }),
            changed: Condvar::new(),
            persisting: Mutex::new(()),
            #[cfg(test)]
            write_budget: Mutex::new(None),
        });
        let for_scheduler = Arc::clone(&shared);
        let scheduler = std::thread::Builder::new()
            .name("download-queue".into())
            .spawn(move || scheduler(for_scheduler))
            .map_err(DownloadError::Disk)?;
        Ok(Self {
            shared,
            scheduler: Mutex::new(Some(scheduler)),
        })
    }

    /// Queues a download at the end. Adding a URL that is already queued returns the existing task.
    pub fn add(&self, new: NewTask) -> Result<TaskId, DownloadError> {
        let md5 = match new.md5 {
            Some(text) => Some(Md5::parse(&text).ok_or(DownloadError::InvalidMd5(text))?),
            None => None,
        };
        let id = {
            let mut state = self.shared.lock();
            if let Some(existing) = state.tasks.iter().find(|t| t.record.url == new.url) {
                return Ok(existing.record.id);
            }
            let id = TaskId(state.next_id);
            state.next_id += 1;
            let task = Task {
                record: TaskRecord {
                    id,
                    url: new.url,
                    file_name: new.file_name,
                    md5,
                    size: new.size,
                    initiator: new.initiator,
                    meta: new.meta,
                    keep_as: new.keep_as,
                    state: Saved::Queued,
                    message: None,
                    checkpoint: 0,
                    validator: None,
                },
                state: TaskState::Queued,
                bytes: 0,
                total: new.size,
                speed: 0,
                retry_at: None,
                failures: 0,
                control: None,
                resume_after_pause: false,
            };
            let snapshot = task.snapshot(self.shared.now());
            state.tasks.push(task);
            self.shared.emit(&Event::Added { task: snapshot });
            id
        };
        if let Err(e) = self.shared.persist() {
            let mut state = self.shared.lock();
            state.tasks.retain(|t| t.record.id != id);
            self.shared.emit(&Event::Removed { id });
            return Err(DownloadError::Disk(e));
        }
        self.shared.notify();
        Ok(id)
    }

    /// Queued or waiting tasks pause at once; a running one stops at the next chunk and keeps its `.part`.
    pub fn pause(&self, id: TaskId) -> Result<(), DownloadError> {
        let mut state = self.shared.lock();
        let task = state.task_mut(id).ok_or(DownloadError::UnknownTask(id))?;
        match (&task.state, &task.control) {
            (TaskState::Paused, _) => Ok(()),
            (TaskState::Downloading, Some(control)) => {
                control.ask(Ask::Pause);
                Ok(())
            }
            (TaskState::Queued | TaskState::Waiting { .. }, None) => {
                self.shared.set_state(&mut state, id, TaskState::Paused);
                drop(state);
                self.saved()
            }
            (other, _) => Err(invalid(id, other, "paused")),
        }
    }

    pub fn resume(&self, id: TaskId) -> Result<(), DownloadError> {
        let mut state = self.shared.lock();
        let task = state.task_mut(id).ok_or(DownloadError::UnknownTask(id))?;
        match (&task.state, &task.control) {
            (TaskState::Paused, None) => {
                self.shared.set_state(&mut state, id, TaskState::Queued);
                drop(state);
                self.saved()?;
                self.shared.notify();
                Ok(())
            }
            (TaskState::Downloading, Some(control)) => {
                if !control.withdraw(Ask::Pause) {
                    task.resume_after_pause = true;
                }
                Ok(())
            }
            (TaskState::Queued, _) => Ok(()),
            (other, _) => Err(invalid(id, other, "resumed")),
        }
    }

    /// Starts a failed or waiting task again now. A task that failed its md5 check downloads from the start; one
    /// whose hook failed only runs the hook again.
    pub fn retry(&self, id: TaskId) -> Result<(), DownloadError> {
        let mut state = self.shared.lock();
        let task = state.task_mut(id).ok_or(DownloadError::UnknownTask(id))?;
        match &task.state {
            TaskState::Failed { .. } | TaskState::Waiting { .. } => {
                task.failures = 0;
                self.shared.set_state(&mut state, id, TaskState::Queued);
                drop(state);
                self.saved()?;
                self.shared.notify();
                Ok(())
            }
            other => Err(invalid(id, other, "retried")),
        }
    }

    /// Removes the task and deletes its files (a running one stops first). Not possible while it is being verified
    /// or installed.
    pub fn cancel(&self, id: TaskId) -> Result<(), DownloadError> {
        let mut state = self.shared.lock();
        let task = state.task_mut(id).ok_or(DownloadError::UnknownTask(id))?;
        if let Some(control) = &task.control {
            return match task.state {
                TaskState::Verifying | TaskState::Installing => {
                    Err(invalid(id, &task.state, "cancelled"))
                }
                _ => {
                    control.ask(Ask::Cancel);
                    Ok(())
                }
            };
        }
        let record = task.record.clone();
        state.tasks.retain(|t| t.record.id != id);
        self.shared.emit(&Event::Removed { id });
        drop(state);
        let store = self.shared.store();
        for path in [
            store.part_path(id),
            store.archive_path(id, &record.file_name),
        ] {
            if let Ok(meta) = std::fs::metadata(&path) {
                let _ = file_op(
                    self.shared.deps.log.as_ref(),
                    FileOp::Delete,
                    "deleted the files of a cancelled download",
                    Some(&path),
                    None,
                    Some(meta.len()),
                    std::fs::remove_file(&path),
                );
            }
        }
        self.saved()
    }

    /// Every task, in queue order.
    pub fn tasks(&self) -> Vec<TaskSnapshot> {
        let now = self.shared.now();
        self.shared
            .lock()
            .tasks
            .iter()
            .map(|t| t.snapshot(now))
            .collect()
    }

    pub fn task(&self, id: TaskId) -> Option<TaskSnapshot> {
        let now = self.shared.now();
        self.shared
            .lock()
            .tasks
            .iter()
            .find(|t| t.record.id == id)
            .map(|t| t.snapshot(now))
    }

    /// Running downloads above a lowered limit finish; no new one starts until there is room.
    pub fn set_parallel(&self, parallel: u32) {
        self.shared.lock().parallel = parallel.max(1) as usize;
        self.shared.notify();
    }

    pub fn set_speed_limit(&self, kib_per_s: Option<u32>) {
        self.shared.limiter.set_limit(kib_per_s);
    }

    pub fn set_keep_archives(&self, keep: bool) {
        self.shared.lock().keep_archives = keep;
    }

    /// The network is usable again (the app saw a request succeed, or local mode was turned off): every waiting task
    /// is retried now instead of at its scheduled time.
    pub fn retry_waiting(&self) {
        let mut state = self.shared.lock();
        let waiting: Vec<TaskId> = state
            .tasks
            .iter()
            .filter(|t| matches!(t.state, TaskState::Waiting { .. }))
            .map(|t| t.record.id)
            .collect();
        for id in waiting {
            self.shared.set_state(&mut state, id, TaskState::Queued);
        }
        drop(state);
        self.shared.notify();
    }

    /// Stops running downloads at a resumable point (their position is saved, they stay queued) and waits for them.
    /// A task in its completion hook finishes it first.
    pub fn shutdown(&self) {
        {
            let mut state = self.shared.lock();
            if !state.stopping {
                state.stopping = true;
                for task in &state.tasks {
                    if let Some(control) = &task.control {
                        control.ask(Ask::Stop);
                    }
                }
            }
        }
        self.shared.notify();
        let handle = self
            .scheduler
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .take();
        if let Some(handle) = handle {
            let _ = handle.join();
        }
        let mut state = self.shared.lock();
        while state.workers > 0 {
            state = self
                .shared
                .changed
                .wait(state)
                .unwrap_or_else(PoisonError::into_inner);
        }
    }

    fn saved(&self) -> Result<(), DownloadError> {
        self.shared.persist().map_err(DownloadError::Disk)
    }

    #[cfg(test)]
    pub(crate) fn shared(&self) -> &Arc<Shared> {
        &self.shared
    }
}

impl Drop for Downloads {
    fn drop(&mut self) {
        self.shutdown();
    }
}

fn invalid(id: TaskId, state: &TaskState, action: &'static str) -> DownloadError {
    DownloadError::InvalidState {
        id,
        state: state.name(),
        action,
    }
}

fn restored(record: TaskRecord, now: Instant) -> Task {
    let message = || {
        record
            .message
            .clone()
            .unwrap_or_else(|| UserMessage::new(MessageId::Internal))
    };
    let (state, retry_at) = match record.state {
        Saved::Queued => (TaskState::Queued, None),
        Saved::Paused => (TaskState::Paused, None),
        // Retried right away: the problem may be gone after a restart.
        Saved::Waiting => (
            TaskState::Waiting {
                message: message(),
                retry_in_s: Some(0),
            },
            Some(now),
        ),
        Saved::Failed => (TaskState::Failed { message: message() }, None),
    };
    Task {
        bytes: record.checkpoint,
        total: record.size,
        state,
        speed: 0,
        retry_at,
        failures: 0,
        control: None,
        resume_after_pause: false,
        record,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backoff_doubles_up_to_the_cap() {
        let timing = Timing::default();
        let secs: Vec<u64> = (1..=9).map(|n| backoff(&timing, n).as_secs()).collect();
        assert_eq!(secs, [5, 10, 20, 40, 80, 160, 300, 300, 300]);
        assert_eq!(backoff(&timing, 200), timing.retry_max);
    }
}
