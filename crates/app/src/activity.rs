//! Activity (S7) in the app: the log pages, totals and summary for the UI, "open logs folder", the live view's
//! pushes, and the Logs settings (masking, retention, size cap) applied to the logger.

use crate::events::{Outbox, UiEvent};
use deadloader_core::activity::{Counts, Entry, Page, Query, Range, Reader, Summary, Tallies};
use deadloader_core::log::{Clock, Kind, Level, Limits, LogDir, Logger, Masker};
use deadloader_core::message::{MessageId, UserMessage};
use deadloader_core::settings::Settings;
use serde_json::Map;
use std::collections::BTreeSet;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{Receiver, SyncSender, TrySendError};
use std::sync::{Arc, Mutex, PoisonError};
use std::time::Duration;

/// New lines are gathered this long before one push tells the UI: a burst of thumbnails is one refresh, not hundreds.
pub const PUSH_DELAY: Duration = Duration::from_millis(500);

/// What the Activity commands read.
pub struct Activity<C, D> {
    log: Arc<Logger<C, D>>,
    /// The logs folder, for "open logs folder".
    folder: PathBuf,
    /// The user's profile folder, masked in what is returned while the setting is on (94).
    masker: Masker,
    tallies: Mutex<Tallies>,
}

impl<C: Clock, D: LogDir> Activity<C, D> {
    pub fn new(log: Arc<Logger<C, D>>, folder: PathBuf, masker: Masker) -> Self {
        Self {
            log,
            folder,
            masker,
            tallies: Mutex::new(Tallies::default()),
        }
    }

    fn masker(&self, mask: bool) -> Masker {
        let mut masker = self.masker.clone();
        masker.set_enabled(mask);
        masker
    }

    /// A page of the list; `mask` is the "hide the Windows user name" setting.
    pub fn page(&self, query: &Query, mask: bool) -> Result<Page, UserMessage> {
        let masker = self.masker(mask);
        let reader = Reader {
            dir: self.log.dir(),
            now: self.log.clock().now(),
            masker: &masker,
        };
        reader.page(query).map_err(|e| unreadable(&e))
    }

    /// One entry in full (a request with all its hops); `None` when it is no longer in the logs.
    pub fn entry(&self, id: &str, mask: bool) -> Result<Option<Entry>, UserMessage> {
        let masker = self.masker(mask);
        let reader = Reader {
            dir: self.log.dir(),
            now: self.log.clock().now(),
            masker: &masker,
        };
        reader.entry(id).map_err(|e| unreadable(&e))
    }

    pub fn counts(&self, range: Range) -> Result<Counts, UserMessage> {
        self.lock()
            .counts(self.log.dir(), self.log.clock().now(), range)
            .map_err(|e| unreadable(&e))
    }

    pub fn summary(&self, mask: bool) -> Result<Summary, UserMessage> {
        let masker = self.masker(mask);
        self.lock()
            .summary(self.log.dir(), self.log.clock().now(), &masker)
            .map_err(|e| unreadable(&e))
    }

    /// Opens deadloader's own logs folder in File Explorer, and nothing else; logged as an external program. Returns
    /// the folder as the UI may show it.
    pub fn open_folder(
        &self,
        mask: bool,
        open: impl FnOnce(&Path) -> io::Result<()>,
    ) -> Result<String, UserMessage> {
        let shown = self.folder_shown(mask);
        let mut fields = Map::new();
        fields.insert("program".into(), "explorer".into());
        fields.insert("folder".into(), self.folder.display().to_string().into());
        // A fresh install may not have written a log yet; the folder then still has to exist to be opened.
        let opened = std::fs::create_dir_all(&self.folder).and_then(|()| open(&self.folder));
        match opened {
            Ok(()) => {
                let _ = self
                    .log
                    .app(Level::Info, "logs folder opened in File Explorer", fields);
                Ok(shown)
            }
            Err(e) => {
                fields.insert("error".into(), e.to_string().into());
                let id = self
                    .log
                    .app(Level::Error, "could not open the logs folder", fields)
                    .map(|id| id.to_string())
                    .unwrap_or_default();
                Err(UserMessage::new(MessageId::Internal)
                    .arg(
                        "detail",
                        format!("File Explorer could not open {shown}: {e}"),
                    )
                    .arg("log_id", id))
            }
        }
    }

    /// The logs folder as the UI may show it.
    pub fn folder_shown(&self, mask: bool) -> String {
        self.masker(mask)
            .mask(&self.folder.display().to_string())
            .into_owned()
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, Tallies> {
        self.tallies.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

fn unreadable(e: &io::Error) -> UserMessage {
    UserMessage::new(MessageId::Internal).arg("detail", format!("the logs could not be read: {e}"))
}

/// The Logs settings, applied to the logger now and after every saved change: masking and the rotation limits (51, 94).
pub fn apply_settings<C: Clock, D: LogDir>(log: &Logger<C, D>, settings: &Settings) {
    log.set_masking(settings.logs.mask_user_path);
    log.set_limits(Limits {
        retention_days: settings.logs.retention_days,
        max_bytes: u64::from(settings.logs.size_limit_mib) * 1024 * 1024,
    });
}

/// Tells the UI about new log lines: the logger reports each line on a channel; a thread gathers them for
/// [`PUSH_DELAY`] and sends one push. While nothing is logged the thread sleeps on the channel (no timer).
pub fn watch<C: Clock, D: LogDir>(log: &Logger<C, D>, outbox: Arc<Outbox>) -> io::Result<()> {
    let (tx, rx) = std::sync::mpsc::sync_channel::<Kind>(1024);
    std::thread::Builder::new()
        .name("log-push".to_owned())
        .spawn(move || forward_lines(&rx, &outbox, PUSH_DELAY))?;
    log.watch(Box::new(move |kind| notify(&tx, kind)));
    Ok(())
}

fn notify(tx: &SyncSender<Kind>, kind: Kind) {
    // A full channel already says "something changed"; the line itself is never held up.
    match tx.try_send(kind) {
        Ok(()) | Err(TrySendError::Full(_) | TrySendError::Disconnected(_)) => {}
    }
}

/// Until the logger is gone: waits for a line, gathers what follows within `delay`, pushes the kinds once.
fn forward_lines(rx: &Receiver<Kind>, outbox: &Outbox, delay: Duration) {
    while let Ok(first) = rx.recv() {
        std::thread::sleep(delay);
        let mut kinds = BTreeSet::from([first]);
        kinds.extend(rx.try_iter());
        outbox.send_live(UiEvent::Log {
            kinds: kinds.into_iter().collect(),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::tests::Recorder;
    use deadloader_core::activity::Results;
    use deadloader_core::log::{DiskLogDir, FileEvent, FileOp, SystemClock};

    struct Fixture {
        dir: PathBuf,
        activity: Activity<SystemClock, DiskLogDir>,
        log: Arc<Logger<SystemClock, DiskLogDir>>,
    }

    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.dir);
        }
    }

    fn fixture(name: &str) -> Fixture {
        let dir =
            std::env::temp_dir().join(format!("deadloader-activity-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let logs = dir.join("logs");
        let log = Arc::new(Logger::new(
            SystemClock,
            DiskLogDir::new(logs.clone()),
            Masker::new(r"C:\Users\Alice"),
            Limits::default(),
        ));
        let activity = Activity::new(Arc::clone(&log), logs, Masker::new(r"C:\Users\Alice"));
        Fixture { dir, activity, log }
    }

    fn query(kind: Kind) -> Query {
        serde_json::from_value(serde_json::json!({ "kind": kind })).expect("query")
    }

    #[test]
    fn written_lines_are_read_back_as_pages_counts_and_a_summary() {
        let f = fixture("pages");
        f.log.set_masking(false);
        f.log
            .app(
                Level::Info,
                r"started from C:\Users\Alice\Desktop",
                Map::new(),
            )
            .expect("written");
        f.log
            .file(
                "file operation",
                &FileEvent {
                    dst: Some(r"D:\Deadlock\game\citadel\addons\pak01_dir.vpk".into()),
                    ..FileEvent::new(FileOp::Hardlink)
                },
            )
            .expect("written");
        let page = f.activity.page(&query(Kind::App), true).expect("page");
        let json = serde_json::to_value(&page).expect("json");
        assert_eq!(
            json["entries"][0]["record"]["msg"],
            r"started from %USERPROFILE%\Desktop"
        );
        let page = f.activity.page(&query(Kind::App), false).expect("page");
        let json = serde_json::to_value(&page).expect("json");
        assert_eq!(
            json["entries"][0]["record"]["msg"],
            r"started from C:\Users\Alice\Desktop"
        );

        let mut files = query(Kind::Files);
        files.result = Results::Error;
        assert!(
            f.activity
                .page(&files, true)
                .expect("page")
                .entries
                .is_empty()
        );
        let counts = f.activity.counts(Range::Today).expect("counts");
        assert_eq!((counts.files.operations, counts.app.events), (1, 1));
        let summary = f.activity.summary(true).expect("summary");
        assert_eq!(summary.game.changes, 1);
        assert!(!summary.empty);
    }

    #[test]
    fn one_entry_is_found_by_its_id() {
        let f = fixture("entry");
        let id = f.log.app(Level::Warn, "x", Map::new()).expect("written");
        assert!(matches!(
            f.activity.entry(id.as_str(), true).expect("entry"),
            Some(Entry::App { .. })
        ));
        assert_eq!(f.activity.entry("nothing-1", true).expect("entry"), None);
    }

    #[test]
    fn opening_the_logs_folder_opens_only_that_folder_and_logs_it() {
        let f = fixture("open");
        let mut opened = Vec::new();
        let shown = f
            .activity
            .open_folder(true, |path| {
                opened.push(path.to_path_buf());
                Ok(())
            })
            .expect("opened");
        assert_eq!(opened, [f.dir.join("logs")]);
        assert_eq!(shown, f.dir.join("logs").display().to_string());
        let page = f.activity.page(&query(Kind::App), true).expect("page");
        let json = serde_json::to_value(&page).expect("json");
        assert_eq!(
            json["entries"][0]["record"]["msg"],
            "logs folder opened in File Explorer"
        );

        let err = f
            .activity
            .open_folder(true, |_| Err(io::Error::other("no explorer")))
            .expect_err("failed");
        assert_eq!(err.id, MessageId::Internal);
        assert!(
            err.args.get("log_id").is_some_and(|id| !id.is_empty()),
            "{err:?}"
        );
    }

    #[test]
    fn the_folder_shown_hides_the_user_name() {
        let f = fixture("mask");
        let activity = Activity::new(
            Arc::clone(&f.log),
            PathBuf::from(r"C:\Users\Alice\deadloader\deadloader-data\logs"),
            Masker::new(r"C:\Users\Alice"),
        );
        assert_eq!(
            activity.folder_shown(true),
            r"%USERPROFILE%\deadloader\deadloader-data\logs"
        );
        assert_eq!(
            activity.folder_shown(false),
            r"C:\Users\Alice\deadloader\deadloader-data\logs"
        );
    }

    #[test]
    fn the_logs_settings_reach_the_logger() {
        let f = fixture("settings");
        let mut settings = Settings::default();
        settings.logs.mask_user_path = false;
        apply_settings(&f.log, &settings);
        f.log
            .app(Level::Info, r"C:\Users\Alice\x", Map::new())
            .expect("written");
        settings.logs.mask_user_path = true;
        apply_settings(&f.log, &settings);
        f.log
            .app(Level::Info, r"C:\Users\Alice\y", Map::new())
            .expect("written");
        let page = f.activity.page(&query(Kind::App), false).expect("page");
        let msgs: Vec<String> = serde_json::to_value(&page).expect("json")["entries"]
            .as_array()
            .expect("entries")
            .iter()
            .map(|e| e["record"]["msg"].as_str().unwrap_or_default().to_owned())
            .collect();
        assert_eq!(msgs, [r"%USERPROFILE%\y", r"C:\Users\Alice\x"]);
    }

    #[test]
    fn new_lines_reach_the_ui_as_one_push_per_burst() {
        let outbox = Arc::new(Outbox::new());
        let ui = Recorder::default();
        outbox.connect(Box::new(ui.clone()));
        let (tx, rx) = std::sync::mpsc::sync_channel::<Kind>(4);
        for kind in [
            Kind::Network,
            Kind::Network,
            Kind::Files,
            Kind::Network,
            Kind::Network,
            Kind::App,
        ] {
            // More than the channel holds: the rest is dropped, not waited for.
            notify(&tx, kind);
        }
        drop(tx);
        let thread_outbox = Arc::clone(&outbox);
        std::thread::spawn(move || forward_lines(&rx, &thread_outbox, Duration::from_millis(10)))
            .join()
            .expect("thread");
        let got = ui.take();
        assert_eq!(got.len(), 1, "{got:?}");
        assert_eq!(
            got[0],
            UiEvent::Log {
                kinds: vec![Kind::Network, Kind::Files]
            }
        );
    }
}
