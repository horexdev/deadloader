//! What the core tells the UI by itself: the game starting and stopping (with decision 143's apply after it closes),
//! files dropped on the window, and the start notices. The Tauri glue (threads, window events) lives in `startup.rs`;
//! everything here runs synchronously so it can be tested.

use crate::backend::Backend;
use crate::events::{Outbox, UiEvent};
use std::path::PathBuf;
use std::time::Duration;

/// How often the watcher looks for the game. One process-list snapshot costs about a millisecond, so this stays far
/// inside the idle CPU budget (120).
pub const GAME_POLL: Duration = Duration::from_secs(2);

/// Remembers whether the game ran at the last look.
#[derive(Debug, Default)]
pub struct GameWatch {
    running: Option<bool>,
}

impl GameWatch {
    pub fn new() -> Self {
        Self::default()
    }

    /// One look at the game: tells the UI when it started or stopped (the first look always tells), and applies what
    /// waited for it once it is closed (143). The first look counts as a close too, so changes left waiting when
    /// deadloader was last closed with the game running are applied now. Returns the new state when it changed.
    pub fn tick(&mut self, backend: &Backend, outbox: &Outbox) -> Option<bool> {
        let running = backend.game_running();
        if self.running == Some(running) {
            return None;
        }
        self.running = Some(running);
        outbox.send(UiEvent::Game { running });
        if running {
            return Some(true);
        }
        match backend.game_closed() {
            Ok(Some(after)) => outbox.send(UiEvent::AppliedAfterGame {
                changes: after.changes,
                library: after.library,
            }),
            Ok(None) => {}
            Err(message) => outbox.send(UiEvent::ApplyAfterGameFailed { message }),
        }
        Some(false)
    }
}

/// Files dropped on the window: imported like "Add from disk", the outcome pushed to the UI.
pub fn import_dropped(backend: &Backend, outbox: &Outbox, paths: &[PathBuf]) {
    outbox.send(UiEvent::DragHover { over: false });
    if paths.is_empty() {
        return;
    }
    match backend.import_paths(paths) {
        Ok(outcomes) => outbox.send(UiEvent::Dropped { outcomes }),
        Err(message) => outbox.send(UiEvent::DropFailed { message }),
    }
}

/// The start notices, once.
pub fn start_notices(backend: &Backend, outbox: &Outbox) {
    for notice in backend.take_notices() {
        outbox.send(UiEvent::Notice(notice));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::tests::{FakeGame, Fixture};
    use crate::events::Notice;
    use crate::events::tests::Recorder;
    use deadloader_core::message::MessageId;
    use std::sync::atomic::Ordering;

    fn listening() -> (Outbox, Recorder) {
        let outbox = Outbox::new();
        let ui = Recorder::default();
        outbox.connect(Box::new(ui.clone()));
        (outbox, ui)
    }

    #[test]
    fn the_ui_hears_when_the_game_starts_and_stops_and_only_then() {
        let f = Fixture::new("watch");
        let game = FakeGame::default();
        let backend = f.backend(&game);
        let (outbox, ui) = listening();
        let mut watch = GameWatch::new();

        watch.tick(&backend, &outbox);
        assert_eq!(ui.take(), [UiEvent::Game { running: false }]);
        watch.tick(&backend, &outbox);
        assert!(ui.take().is_empty());

        game.running.store(true, Ordering::Relaxed);
        watch.tick(&backend, &outbox);
        watch.tick(&backend, &outbox);
        assert_eq!(ui.take(), [UiEvent::Game { running: true }]);

        game.running.store(false, Ordering::Relaxed);
        watch.tick(&backend, &outbox);
        assert_eq!(ui.take(), [UiEvent::Game { running: false }]);
    }

    #[test]
    fn closing_the_game_applies_the_waiting_changes_and_says_so() {
        let f = Fixture::new("watchapply");
        let game = FakeGame {
            installs: vec![f.game()],
            ..FakeGame::default()
        };
        let backend = f.backend(&game);
        backend
            .import_paths(&[f.mod_file("haze.vpk", "models/haze/body.vmdl_c")])
            .expect("imported");
        let id = backend.apply().expect("applied").library.mods[0].id.clone();
        let (outbox, ui) = listening();
        let mut watch = GameWatch::new();
        game.running.store(true, Ordering::Relaxed);
        watch.tick(&backend, &outbox);
        backend.set_enabled(&id, false).expect("off");
        ui.take();

        game.running.store(false, Ordering::Relaxed);
        watch.tick(&backend, &outbox);
        let events = ui.take();
        assert_eq!(events[0], UiEvent::Game { running: false });
        let UiEvent::AppliedAfterGame { changes, library } = &events[1] else {
            panic!("{events:?}");
        };
        assert_eq!(*changes, 1);
        assert!(!library.pending);
        assert!(!f.addons().join("pak01_dir.vpk").exists());
    }

    #[test]
    fn a_failed_apply_after_the_game_is_reported() {
        let f = Fixture::new("watchfail");
        let game = FakeGame {
            installs: vec![f.game()],
            ..FakeGame::default()
        };
        let backend = f.backend(&game);
        backend
            .import_paths(&[f.mod_file("haze.vpk", "models/haze/body.vmdl_c")])
            .expect("imported");
        let id = backend.library().expect("library").mods[0].id.clone();
        backend.set_enabled(&id, false).expect("off");
        backend.apply().expect("applied");
        let (outbox, ui) = listening();
        let mut watch = GameWatch::new();
        game.running.store(true, Ordering::Relaxed);
        watch.tick(&backend, &outbox);
        backend.set_enabled(&id, true).expect("on");
        // The game folder disappears while the game runs (e.g. Steam moved it).
        std::fs::remove_dir_all(f.dir.join("Deadlock")).expect("moved away");
        ui.take();

        game.running.store(false, Ordering::Relaxed);
        watch.tick(&backend, &outbox);
        let events = ui.take();
        assert!(
            matches!(&events[1], UiEvent::ApplyAfterGameFailed { .. }),
            "{events:?}"
        );
    }

    #[test]
    fn dropped_files_are_imported_and_reported() {
        let f = Fixture::new("drop");
        let backend = f.backend(&FakeGame::default());
        let (outbox, ui) = listening();
        let text = f.dir.join("notes.txt");
        std::fs::write(&text, "not a mod").expect("txt");
        import_dropped(
            &backend,
            &outbox,
            &[f.mod_file("haze.vpk", "models/haze/body.vmdl_c"), text],
        );
        let events = ui.take();
        assert_eq!(events[0], UiEvent::DragHover { over: false });
        let UiEvent::Dropped { outcomes } = &events[1] else {
            panic!("{events:?}");
        };
        assert!(outcomes[0].is_ok());
        assert_eq!(outcomes[1].as_ref().expect_err("refused").id, MessageId::E8);
        assert_eq!(backend.library().expect("library").mods.len(), 1);
    }

    #[test]
    fn a_drop_on_a_library_that_failed_to_open_says_why() {
        let f = Fixture::new("dropfail");
        let data = f.dir.join("deadloader-data");
        std::fs::create_dir_all(&data).expect("data");
        std::fs::write(data.join("manifest.json"), "{ broken").expect("manifest");
        let backend = f.backend(&FakeGame::default());
        let (outbox, ui) = listening();
        import_dropped(
            &backend,
            &outbox,
            &[f.mod_file("haze.vpk", "models/haze/body.vmdl_c")],
        );
        assert!(matches!(
            &ui.take()[1],
            UiEvent::DropFailed { message } if message.id == MessageId::E23
        ));
    }

    #[test]
    fn the_start_notices_are_sent_once() {
        let f = Fixture::new("startnotices");
        let data = f.dir.join("deadloader-data");
        std::fs::create_dir_all(&data).expect("data");
        std::fs::write(data.join("settings.json"), r#"{"schema": 1, "zzz": 1}"#).expect("settings");
        let backend = f.backend(&FakeGame::default());
        let (outbox, ui) = listening();
        start_notices(&backend, &outbox);
        start_notices(&backend, &outbox);
        assert_eq!(
            ui.take(),
            [UiEvent::Notice(Notice::SettingsCorrected {
                fields: vec!["zzz".to_owned()]
            })]
        );
    }
}
