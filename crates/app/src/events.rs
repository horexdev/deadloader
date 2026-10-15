//! Pushes from the core to the UI: what the UI should hear about without asking (the game started or closed, files
//! dropped on the window, notices from the start).
//!
//! The UI subscribes once with a `tauri::ipc::Channel` (no `core:event` permission is granted, see "Implementation
//! defaults"). Events sent while nobody listens (before the page has loaded, or during a reload) are kept, a bounded
//! number of them, and delivered first when the UI subscribes.

use deadloader_core::journal::Recovery;
use deadloader_core::message::UserMessage;
use deadloader_core::session::{self, ImportOutcome, LibraryView};
use serde::Serialize;
use std::collections::VecDeque;
use std::sync::{Mutex, MutexGuard, PoisonError};

/// Events kept while the UI isn't listening; the oldest go first when there are more.
const KEPT: usize = 64;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "event", content = "data", rename_all = "snake_case")]
pub enum UiEvent {
    /// Whether Deadlock runs: the N4 banner while it does.
    Game { running: bool },
    /// Deadlock was closed and the changes made while it ran went into its folder (N12, decision 143). `changes` counts
    /// the edits made while it ran; 0 when they were made before deadloader started. `items` names what went in, one
    /// per mod and aspect, as S9 lists them (the toast words them).
    AppliedAfterGame {
        changes: usize,
        items: Vec<deadloader_core::pending::PendingChange>,
        library: LibraryView,
    },
    /// Deadlock was closed, but its changes could not be applied.
    ApplyAfterGameFailed { message: UserMessage },
    /// Files are being dragged over the window (`true`) or left it (`false`).
    DragHover { over: bool },
    /// Files dropped on the window were imported, each with its own outcome.
    Dropped {
        outcomes: Vec<Result<ImportOutcome, UserMessage>>,
    },
    /// Dropped files could not be imported at all (the library is unavailable).
    DropFailed { message: UserMessage },
    /// Something found while starting.
    Notice(Notice),
    /// The catalog's network status changed, or a background refresh saved a newer copy of something on screen.
    Catalog(deadloader_catalog::Event),
    /// A download was added, moved on, made progress or left the queue.
    Download(deadloader_downloads::Event),
    /// An update the user picked in D5 arrived: applied, or waiting for the user to map files. `task` is the
    /// download it came with; `None` when its files were in the library already.
    UpdateStep {
        task: Option<deadloader_downloads::TaskId>,
        step: crate::backend::UpdateStep,
    },
    /// The result of a mod update check (N3 and the badges); after the automatic check at start too.
    Updates(crate::updates::UpdateReport),
    /// A finished download is in the library (F2); with `changed` it is enabled, otherwise D4 asks for variants.
    Installed {
        task: deadloader_downloads::TaskId,
        installed: crate::backend::Installed,
    },
    /// How many network requests are running now (the sidebar's live count). Sent only while the UI listens.
    Network { running: usize },
    /// New lines in these logs since the last such event (Activity's live view). Sent only while the UI listens.
    Log {
        kinds: Vec<deadloader_core::log::Kind>,
    },
    /// How far a running apply is (the Library's applying banner), and a last report with `finished` when it ends.
    /// Sent only while the UI listens.
    Applying(crate::progress::Applying),
}

/// What the start found, for a toast or a banner.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Notice {
    /// Settings that could not be used as written were set back to their defaults; the dotted names.
    SettingsCorrected { fields: Vec<String> },
    /// `settings.json` could not be read; defaults are used and the file is left alone (E23).
    SettingsUnreadable { message: UserMessage },
    /// An interrupted change to the game folder was finished or undone (N5).
    Recovered { undone: bool },
    /// Library files nothing referred to were deleted.
    Swept { files: usize },
    /// Profile files that could not be read; they are left as they are.
    BrokenProfiles { names: Vec<String> },
    /// Library files that are missing or no longer readable (E14).
    DamagedFiles { files: Vec<String> },
    /// Items in the game's mod folder deadloader didn't put there and the user hasn't left as they are (N6, F9).
    Foreign { files: usize },
}

impl From<&session::Notice> for Notice {
    fn from(notice: &session::Notice) -> Self {
        match notice {
            session::Notice::SettingsCorrected(warnings) => Self::SettingsCorrected {
                fields: warnings.iter().map(|w| w.path.clone()).collect(),
            },
            session::Notice::SettingsUnreadable(message) => Self::SettingsUnreadable {
                message: message.clone(),
            },
            session::Notice::Recovered(recovery) => Self::Recovered {
                undone: *recovery == Recovery::RolledBack,
            },
            session::Notice::Swept(files) => Self::Swept { files: *files },
            session::Notice::BrokenProfiles(names) => Self::BrokenProfiles {
                names: names.clone(),
            },
            session::Notice::DamagedFiles(files) => Self::DamagedFiles {
                files: files.clone(),
            },
        }
    }
}

/// Where events go: the UI's channel, or a recorder in tests.
pub trait Sink: Send + Sync {
    /// `false` when the listener is gone; the event is then kept for the next one.
    fn deliver(&self, event: &UiEvent) -> bool;
}

#[derive(Default)]
pub struct Outbox {
    state: Mutex<State>,
}

#[derive(Default)]
struct State {
    sink: Option<Box<dyn Sink>>,
    kept: VecDeque<UiEvent>,
}

impl Outbox {
    pub fn new() -> Self {
        Self::default()
    }

    /// The UI subscribed (again after a reload): it replaces the previous listener and first gets what was kept.
    pub fn connect(&self, sink: Box<dyn Sink>) {
        let mut state = self.lock();
        state.sink = Some(sink);
        while let Some(event) = state.kept.pop_front() {
            if !deliver(&mut state, &event) {
                state.kept.push_front(event);
                break;
            }
        }
    }

    pub fn send(&self, event: UiEvent) {
        let mut state = self.lock();
        if state.kept.is_empty() && deliver(&mut state, &event) {
            return;
        }
        if state.kept.len() == KEPT {
            state.kept.pop_front();
        }
        state.kept.push_back(event);
    }

    /// Delivers `event` only if the UI listens now; never kept for later. For news that is stale once missed, so it
    /// can't push out kept events that matter.
    pub fn send_live(&self, event: UiEvent) {
        let mut state = self.lock();
        if state.kept.is_empty() {
            deliver(&mut state, &event);
        }
    }

    fn lock(&self) -> MutexGuard<'_, State> {
        self.state.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

/// Hands one event to the listener; forgets a listener that is gone.
fn deliver(state: &mut State, event: &UiEvent) -> bool {
    let delivered = state.sink.as_ref().is_some_and(|sink| sink.deliver(event));
    if !delivered {
        state.sink = None;
    }
    delivered
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};

    /// Records what it is given; can pretend the listener is gone.
    #[derive(Clone, Default)]
    pub(crate) struct Recorder {
        pub events: Arc<Mutex<Vec<UiEvent>>>,
        pub gone: Arc<AtomicBool>,
    }

    impl Recorder {
        pub(crate) fn take(&self) -> Vec<UiEvent> {
            std::mem::take(&mut *self.events.lock().expect("lock"))
        }
    }

    impl Sink for Recorder {
        fn deliver(&self, event: &UiEvent) -> bool {
            if self.gone.load(Ordering::Relaxed) {
                return false;
            }
            self.events.lock().expect("lock").push(event.clone());
            true
        }
    }

    fn game(running: bool) -> UiEvent {
        UiEvent::Game { running }
    }

    #[test]
    fn events_reach_the_listener_in_order() {
        let outbox = Outbox::new();
        let ui = Recorder::default();
        outbox.connect(Box::new(ui.clone()));
        outbox.send(game(true));
        outbox.send(game(false));
        assert_eq!(ui.take(), [game(true), game(false)]);
    }

    #[test]
    fn events_before_the_ui_listens_are_delivered_when_it_does() {
        let outbox = Outbox::new();
        outbox.send(UiEvent::Notice(Notice::Swept { files: 2 }));
        outbox.send(game(false));
        let ui = Recorder::default();
        outbox.connect(Box::new(ui.clone()));
        assert_eq!(
            ui.take(),
            [UiEvent::Notice(Notice::Swept { files: 2 }), game(false)]
        );
        outbox.send(game(true));
        assert_eq!(ui.take(), [game(true)]);
    }

    #[test]
    fn a_gone_listener_is_dropped_and_the_next_one_gets_what_it_missed() {
        let outbox = Outbox::new();
        let old = Recorder::default();
        outbox.connect(Box::new(old.clone()));
        old.gone.store(true, Ordering::Relaxed);
        outbox.send(game(true));
        assert!(old.take().is_empty());

        let new = Recorder::default();
        outbox.connect(Box::new(new.clone()));
        assert_eq!(new.take(), [game(true)]);
    }

    #[test]
    fn only_the_newest_events_are_kept_while_nobody_listens() {
        let outbox = Outbox::new();
        for files in 0..KEPT + 3 {
            outbox.send(UiEvent::Notice(Notice::Swept { files }));
        }
        let ui = Recorder::default();
        outbox.connect(Box::new(ui.clone()));
        let got = ui.take();
        assert_eq!(got.len(), KEPT);
        assert_eq!(got[0], UiEvent::Notice(Notice::Swept { files: 3 }));
    }

    #[test]
    fn live_events_reach_a_listener_and_are_never_kept() {
        let outbox = Outbox::new();
        outbox.send_live(UiEvent::Log {
            kinds: vec![deadloader_core::log::Kind::App],
        });
        let ui = Recorder::default();
        outbox.connect(Box::new(ui.clone()));
        assert!(ui.take().is_empty());
        outbox.send_live(UiEvent::Log {
            kinds: vec![deadloader_core::log::Kind::Files],
        });
        assert_eq!(
            serde_json::to_value(ui.take()).expect("json"),
            serde_json::json!([{"event": "log", "data": {"kinds": ["files"]}}])
        );
    }

    #[test]
    fn apply_progress_is_tagged_for_the_ui() {
        let event = UiEvent::Applying(crate::progress::Applying {
            progress: deadloader_core::engine::ApplyProgress {
                done: 3,
                total: 4,
                ..Default::default()
            },
            left_ms: Some(5000),
            finished: false,
        });
        let json = serde_json::to_value(&event).expect("json");
        assert_eq!(json["event"], "applying");
        assert_eq!(json["data"]["done"], 3);
        assert_eq!(json["data"]["left_ms"], 5000);
    }

    /// The shape `ui/src/lib/api.ts` relies on.
    #[test]
    fn events_are_tagged_for_the_ui() {
        let json = |e: &UiEvent| serde_json::to_value(e).expect("json");
        assert_eq!(
            json(&game(true)),
            serde_json::json!({"event": "game", "data": {"running": true}})
        );
        assert_eq!(
            json(&UiEvent::Network { running: 3 }),
            serde_json::json!({"event": "network", "data": {"running": 3}})
        );
        assert_eq!(
            json(&UiEvent::DragHover { over: false }),
            serde_json::json!({"event": "drag_hover", "data": {"over": false}})
        );
        assert_eq!(
            json(&UiEvent::Download(deadloader_downloads::Event::State {
                id: deadloader_downloads::TaskId(17),
                state: deadloader_downloads::TaskState::Waiting {
                    message: UserMessage::new(deadloader_core::message::MessageId::E6),
                    retry_in_s: Some(5),
                },
            })),
            serde_json::json!({"event": "download", "data": {"event": "state", "id": 17, "state": {
                "state": "waiting", "message": {"id": "E6", "args": {}}, "retry_in_s": 5
            }}})
        );
        assert_eq!(
            json(&UiEvent::Notice(Notice::Recovered { undone: true })),
            serde_json::json!({"event": "notice", "data": {"kind": "recovered", "undone": true}})
        );
    }
}
