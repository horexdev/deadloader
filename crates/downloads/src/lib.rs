//! The download engine: a persistent FIFO queue with a parallel limit, resume over HTTP Range, a global speed limit,
//! md5 and size checks, a free-space check, throttled progress events and a hand-off of verified archives.
//!
//! Blocking threads, no async runtime (decisions 82, 125): one scheduler thread that sleeps until something changes,
//! and per running task a worker thread (owns the `.part` file) and a fetch thread (owns the HTTP body). The worker
//! never blocks on the network, so pause, cancel and the stall timeout take effect at once even when the connection
//! hangs. Nothing runs while the queue is idle.
//!
//! Every request goes through [`deadloader_net::Client`] (allowlist on every hop, network log); every file
//! operation goes to the files log; GameBanana URLs are the caller's business.

mod error;
mod http;
mod limiter;
mod log;
mod queue;
mod speed;
mod store;
mod task;
#[cfg(test)]
mod test_support;
#[cfg(test)]
mod tests;
mod worker;

pub use error::DownloadError;
pub use log::DownloadLog;
pub use queue::Downloads;
pub use store::QUEUE_FILE;
pub use task::{Event, Initiator, Md5, NewTask, TaskId, TaskInfo, TaskSnapshot, TaskState};

use deadloader_core::message::UserMessage;
use deadloader_net::Client;
use deadloader_net::clock::Clock;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

/// Free bytes on the volume of a folder; `None` when it can't be told (the check is then skipped). The app wires it
/// to `deadloader_platform_windows::disk::free_space`.
pub trait FreeSpace: Send + Sync {
    fn available_bytes(&self, dir: &Path) -> Option<u64>;
}

impl<F: Fn(&Path) -> Option<u64> + Send + Sync> FreeSpace for F {
    fn available_bytes(&self, dir: &Path) -> Option<u64> {
        self(dir)
    }
}

/// Called with every verified download, on the task's worker thread (it counts against the parallel limit while it
/// runs). The app unpacks and imports the archive here. `Ok` means the archive is no longer needed: it is deleted
/// unless `keep_archives` is on. `Err` fails the task with that message; the archive stays and Retry calls the hook
/// again without downloading.
pub trait CompletionHook: Send + Sync {
    fn downloaded(&self, archive: &Path, task: &TaskInfo) -> Result<(), UserMessage>;
}

impl<F: Fn(&Path, &TaskInfo) -> Result<(), UserMessage> + Send + Sync> CompletionHook for F {
    fn downloaded(&self, archive: &Path, task: &TaskInfo) -> Result<(), UserMessage> {
        self(archive, task)
    }
}

/// Receives every [`Event`], in order per task. Called with the queue locked: it must be quick and must not call back
/// into [`Downloads`] (forward to a channel or the UI's event emitter).
pub type EventSink = Arc<dyn Fn(&Event) + Send + Sync>;

#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    /// `deadloader-data\downloads\`.
    pub dir: PathBuf,
    /// Downloads at the same time (decision 44).
    pub parallel: u32,
    /// KiB/s across all downloads; `None` = unlimited.
    pub speed_limit_kib_per_s: Option<u32>,
    /// Keep the archive after a successful hand-off (decision 45; default off).
    pub keep_archives: bool,
    pub timing: Timing,
}

impl Config {
    pub fn new(dir: PathBuf, settings: &deadloader_core::settings::Downloads) -> Self {
        Self {
            dir,
            parallel: settings.parallel,
            speed_limit_kib_per_s: settings.speed_limit_kib_per_s,
            keep_archives: settings.keep_archives,
            timing: Timing::default(),
        }
    }
}

/// Our own numbers; none of them is a user setting.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Timing {
    /// No byte for this long while a body is being received = stalled (E6, retried).
    pub idle_timeout: Duration,
    /// How often a running download flushes its `.part` and saves its position.
    pub checkpoint_every: Duration,
    /// Progress events per task are at least this far apart.
    pub progress_every: Duration,
    /// Time constant of the speed smoothing.
    pub speed_smoothing: Duration,
    /// First automatic retry after a temporary failure; doubled each time up to `retry_max`.
    pub retry_base: Duration,
    pub retry_max: Duration,
}

impl Default for Timing {
    fn default() -> Self {
        Self {
            idle_timeout: Duration::from_secs(30),
            checkpoint_every: Duration::from_secs(10),
            progress_every: Duration::from_millis(100),
            speed_smoothing: Duration::from_secs(2),
            retry_base: Duration::from_secs(5),
            retry_max: Duration::from_secs(300),
        }
    }
}

/// What the engine talks to.
#[derive(Clone)]
pub struct Deps {
    pub client: Arc<Client>,
    pub clock: Arc<dyn Clock>,
    pub log: Arc<dyn DownloadLog>,
    pub space: Arc<dyn FreeSpace>,
    pub hook: Arc<dyn CompletionHook>,
    pub events: EventSink,
}

impl std::fmt::Debug for Deps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Deps")
            .field("client", &self.client)
            .finish_non_exhaustive()
    }
}
