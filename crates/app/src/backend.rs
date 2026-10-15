//! What the UI's commands call: the open session and the game on this PC, with no domain logic of its own beyond
//! decision 138 (edits to the applied profile take effect at once while the game is closed).

use crate::events::Notice;
use deadloader_core::log::{FileEvent, FileOp, Outcome};
use deadloader_core::message::{Area, MessageId, ToUserMessage, UserMessage, from_io};
use deadloader_core::profiles::ProfileError;
use deadloader_core::remove::{Delete, Permanently, RemovalPlan, RemoveError, Removed, Target};
use deadloader_core::session::{
    ImportOutcome, LibraryView, ProfileImported, Session, SessionError, SharedLog,
};
use deadloader_core::settings::{Deletion, Settings};
use serde::Serialize;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

/// The game as seen from this PC: whether it runs, and where Steam has it installed.
pub trait Game: Send + Sync {
    fn running(&self) -> bool;
    fn installations(&self) -> Vec<PathBuf>;
    fn move_to_recycle_bin(&self, path: &Path) -> std::io::Result<()>;
}

/// The recycle bin as a way to delete library files (decision 42).
struct RecycleBin<'a>(&'a dyn Game);

impl Delete for RecycleBin<'_> {
    fn delete(&self, path: &Path) -> std::io::Result<()> {
        self.0.move_to_recycle_bin(path)
    }
}

pub struct Backend {
    session: Mutex<Option<Session>>,
    /// Why the session could not be opened (the UI shows it instead of the library).
    start_error: Option<UserMessage>,
    game: Box<dyn Game>,
    log: SharedLog,
    /// What the start found, until the UI takes it.
    notices: Mutex<Vec<Notice>>,
    /// Edits made while the game ran since the last apply, for N12.
    edits_waiting: AtomicUsize,
}

/// The result of a change in the library.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Changed {
    pub library: LibraryView,
    /// The change was put into the game folder right away (138).
    pub applied: bool,
    /// The game is running; the change waits until it is closed (N4).
    pub waiting_for_game: bool,
}

/// The game was closed and what changed while it ran went into its folder (N12).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AfterGame {
    pub changes: usize,
    pub library: LibraryView,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ApplyResult {
    pub library: LibraryView,
    pub changed: bool,
    /// `gameinfo.gi` was changed by another program, so mods won't load until it is fixed (E3, D10).
    pub gameinfo_foreign: bool,
}

impl Backend {
    pub fn start(data_dir: &Path, game: Box<dyn Game>, log: SharedLog) -> Self {
        match Session::open(data_dir, log.clone()) {
            Ok(opened) => Self {
                session: Mutex::new(Some(opened.session)),
                start_error: None,
                game,
                log,
                notices: Mutex::new(opened.notices.iter().map(Notice::from).collect()),
                edits_waiting: AtomicUsize::new(0),
            },
            Err(e) => Self {
                session: Mutex::new(None),
                start_error: Some(e.user_message()),
                game,
                log,
                notices: Mutex::new(Vec::new()),
                edits_waiting: AtomicUsize::new(0),
            },
        }
    }

    /// What the start found; empty after the first call.
    pub fn take_notices(&self) -> Vec<Notice> {
        std::mem::take(
            &mut *self
                .notices
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner),
        )
    }

    pub fn game_running(&self) -> bool {
        self.game.running()
    }

    /// Decision 143: the game was closed, so what was changed in the applied profile while it ran goes into its folder
    /// now. `None` when there was nothing to apply, or the game runs again. A profile switch still waits for Apply.
    pub fn game_closed(&self) -> Result<Option<AfterGame>, UserMessage> {
        // Read before applying: any apply starts the count again.
        let changes = self.edits_waiting.load(Ordering::Relaxed);
        let changed = self.follow_up()?;
        Ok(changed.applied.then_some(AfterGame {
            changes,
            library: changed.library,
        }))
    }

    pub fn library(&self) -> Result<LibraryView, UserMessage> {
        self.with(|s| Ok(s.library()))
    }

    pub fn set_enabled(&self, mod_id: &str, enabled: bool) -> Result<Changed, UserMessage> {
        self.change(|s| s.set_enabled(mod_id, enabled))
    }

    pub fn move_mod(&self, mod_id: &str, index: usize) -> Result<Changed, UserMessage> {
        self.change(|s| s.move_mod(mod_id, index))
    }

    pub fn switch_variant(&self, mod_id: &str, file: &str) -> Result<Changed, UserMessage> {
        self.change(|s| s.switch_variant(mod_id, file))
    }

    pub fn choose(&self, mod_id: &str, files: &[String]) -> Result<Changed, UserMessage> {
        self.change(|s| s.choose(mod_id, files))
    }

    pub fn select_profile(&self, name: &str) -> Result<LibraryView, UserMessage> {
        self.with(|s| {
            s.select_profile(name)?;
            Ok(s.library())
        })
    }

    pub fn create_profile(&self, name: &str) -> Result<Vec<String>, UserMessage> {
        self.with(|s| {
            s.create_profile(name)?;
            Ok(s.profile_names())
        })
    }

    pub fn profile_names(&self) -> Result<Vec<String>, UserMessage> {
        self.with(|s| Ok(s.profile_names()))
    }

    /// Imports each dropped or chosen file as its own mod, named after the file. One failure doesn't stop the rest.
    pub fn import_paths(
        &self,
        paths: &[PathBuf],
    ) -> Result<Vec<Result<ImportOutcome, UserMessage>>, UserMessage> {
        let outcomes = self.with(|s| {
            Ok(paths
                .iter()
                .map(|path| {
                    let name = path
                        .file_stem()
                        .map(|n| n.to_string_lossy().into_owned())
                        .unwrap_or_default();
                    s.import_file(path, &name).map_err(|e| e.user_message())
                })
                .collect::<Vec<_>>())
        })?;
        if outcomes
            .iter()
            .any(|o| o.as_ref().is_ok_and(|i| i.choices.is_empty()))
        {
            self.follow_up_edit()?;
        }
        Ok(outcomes)
    }

    /// What deleting a mod (no `files`) or some of its files means, for D6.
    pub fn removal_plan(
        &self,
        mod_id: &str,
        files: Option<Vec<String>>,
    ) -> Result<RemovalPlan, UserMessage> {
        let target = target(mod_id, files);
        self.with(|s| Ok(s.removal_plan(&target)?))
    }

    /// Deletes a mod or some of its files the way the settings say (permanently or to the recycle bin). A whole mod
    /// that the selected, applied profile has in the game is taken out of the game first; that waits while the game
    /// runs.
    pub fn remove(&self, mod_id: &str, files: Option<Vec<String>>) -> Result<Removed, UserMessage> {
        let target = target(mod_id, files);
        let running = self.game.running();
        let game = self.game.as_ref();
        self.with(|s| {
            let recycle = RecycleBin(game);
            let deleter: &dyn Delete = match s.settings().library.deletion {
                Deletion::Permanent => &Permanently,
                Deletion::RecycleBin => &recycle,
            };
            match s.remove(&target, deleter) {
                Err(SessionError::Remove(RemoveError::StillInGame(roots))) => {
                    if running {
                        return Err(UserMessage::new(MessageId::E13).into());
                    }
                    let view = s.library();
                    let live =
                        view.applied_profile.as_deref() == Some(view.selected_profile.as_str());
                    let root = s.game_root();
                    let only_here = root
                        .as_ref()
                        .is_some_and(|r| roots.iter().all(|x| Path::new(x) == r));
                    match (&target, live, only_here, root) {
                        (Target::Mod(id), true, true, Some(root)) => {
                            s.set_enabled(id, false)?;
                            s.apply(&root, false)?;
                            Ok(s.remove(&target, deleter)?)
                        }
                        _ => Err(SessionError::Remove(RemoveError::StillInGame(roots)).into()),
                    }
                }
                other => Ok(other?),
            }
        })
    }

    pub fn rename_profile(&self, old: &str, new: &str) -> Result<Vec<String>, UserMessage> {
        self.with(|s| {
            s.rename_profile(old, new)?;
            Ok(s.profile_names())
        })
    }

    pub fn duplicate_profile(
        &self,
        name: &str,
        new_name: &str,
    ) -> Result<Vec<String>, UserMessage> {
        self.with(|s| {
            s.duplicate_profile(name, new_name)?;
            Ok(s.profile_names())
        })
    }

    pub fn delete_profile(&self, name: &str) -> Result<Vec<String>, UserMessage> {
        self.with(|s| {
            s.delete_profile(name)?;
            Ok(s.profile_names())
        })
    }

    pub fn export_profile(&self, name: &str) -> Result<String, UserMessage> {
        self.with(|s| Ok(s.export_profile(name)?))
    }

    pub fn import_profile(&self, text: &str) -> Result<ProfileImported, UserMessage> {
        self.with(|s| Ok(s.import_profile(text)?))
    }

    /// Profiles → Export: the shareable JSON written where the user chose (F14).
    pub fn export_profile_to(&self, name: &str, path: &Path) -> Result<(), UserMessage> {
        let text = self.export_profile(name)?;
        let result = std::fs::write(path, text.as_bytes());
        let mut event = FileEvent {
            dst: Some(path.display().to_string()),
            bytes: Some(text.len() as u64),
            ..FileEvent::new(FileOp::Write)
        };
        if let Err(e) = &result {
            event.result = Outcome::Failed;
            event.error = Some(e.to_string());
        }
        self.log.record("profile exported", &event);
        result.map_err(|e| from_io(&e, Area::Data))
    }

    /// Profiles → Import from a file the user chose.
    pub fn import_profile_from(&self, path: &Path) -> Result<ProfileImported, UserMessage> {
        let text = std::fs::read_to_string(path).map_err(|e| {
            UserMessage::new(MessageId::E17)
                .arg("reason", "unreadable")
                .arg("file", path.display())
                .arg("detail", e)
        })?;
        self.import_profile(&text)
    }

    pub fn settings(&self) -> Result<Settings, UserMessage> {
        self.with(|s| Ok(s.settings().clone()))
    }

    pub fn update_settings(&self, settings: Settings) -> Result<Settings, UserMessage> {
        self.with(|s| {
            s.update_settings(settings)?;
            Ok(s.settings().clone())
        })
    }

    /// Apply / Play: the selected profile goes into its game folder, or the first one Steam knows.
    pub fn apply(&self) -> Result<ApplyResult, UserMessage> {
        let running = self.game.running();
        let installations = self.game.installations();
        self.with(|s| {
            let root = s
                .game_root()
                .or_else(|| installations.first().cloned())
                .ok_or_else(|| UserMessage::new(MessageId::E2))?;
            let applied = s.apply(&root, running)?;
            self.edits_waiting.store(0, Ordering::Relaxed);
            Ok(ApplyResult {
                library: s.library(),
                changed: applied.changed,
                gameinfo_foreign: matches!(
                    applied.gameinfo,
                    deadloader_core::gamedir::GuardOutcome::Foreign(_)
                ),
            })
        })
    }

    fn change(
        &self,
        edit: impl FnOnce(&mut Session) -> Result<(), deadloader_core::session::SessionError>,
    ) -> Result<Changed, UserMessage> {
        self.with(|s| Ok(edit(s)?))?;
        self.follow_up_edit()
    }

    /// [`Self::follow_up`] after an edit: one that waits for the game counts for N12.
    fn follow_up_edit(&self) -> Result<Changed, UserMessage> {
        let changed = self.follow_up()?;
        if changed.waiting_for_game {
            self.edits_waiting.fetch_add(1, Ordering::Relaxed);
        } else if !changed.library.pending {
            // Back to what the game folder has: nothing waits any more.
            self.edits_waiting.store(0, Ordering::Relaxed);
        }
        Ok(changed)
    }

    /// Decision 138: the selected profile is the one in its game folder, so the change goes there now, unless the
    /// game runs.
    fn follow_up(&self) -> Result<Changed, UserMessage> {
        let running = self.game.running();
        self.with(|s| {
            let view = s.library();
            let live = view.pending
                && view.applied_profile.as_deref() == Some(view.selected_profile.as_str());
            match (live, s.game_root()) {
                (true, Some(_)) if running => Ok(Changed {
                    library: view,
                    applied: false,
                    waiting_for_game: true,
                }),
                (true, Some(root)) => {
                    s.apply(&root, false)?;
                    self.edits_waiting.store(0, Ordering::Relaxed);
                    Ok(Changed {
                        library: s.library(),
                        applied: true,
                        waiting_for_game: false,
                    })
                }
                _ => Ok(Changed {
                    library: view,
                    applied: false,
                    waiting_for_game: false,
                }),
            }
        })
    }

    fn with<T>(
        &self,
        f: impl FnOnce(&mut Session) -> Result<T, Failure>,
    ) -> Result<T, UserMessage> {
        if let Some(e) = &self.start_error {
            return Err(e.clone());
        }
        let mut guard = self.session.lock().map_err(|_| {
            UserMessage::new(MessageId::Internal).arg("detail", "session lock poisoned")
        })?;
        let session = guard
            .as_mut()
            .ok_or_else(|| UserMessage::new(MessageId::Internal).arg("detail", "no session"))?;
        f(session).map_err(|failure| failure.0)
    }
}

fn target(mod_id: &str, files: Option<Vec<String>>) -> Target {
    match files {
        Some(files) => Target::Files(files),
        None => Target::Mod(mod_id.to_owned()),
    }
}

impl From<ProfileError> for Failure {
    fn from(e: ProfileError) -> Self {
        Self(e.user_message())
    }
}

/// Any error on its way to the UI.
pub struct Failure(UserMessage);

impl From<UserMessage> for Failure {
    fn from(m: UserMessage) -> Self {
        Self(m)
    }
}

impl From<deadloader_core::session::SessionError> for Failure {
    fn from(e: deadloader_core::session::SessionError) -> Self {
        Self(e.user_message())
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};

    const VANILLA: &str = "\"GameInfo\"\r\n{\r\n\tFileSystem\r\n\t{\r\n\t\tSearchPaths\r\n\t\t{\r\n\t\t\tGame\t\t\t\tcitadel\r\n\t\t\tGame\t\t\t\tcore\r\n\t\t}\r\n\t}\r\n}\r\n";

    #[derive(Clone, Default)]
    pub(crate) struct FakeGame {
        pub running: Arc<AtomicBool>,
        pub installs: Vec<PathBuf>,
        pub recycled: Arc<std::sync::Mutex<Vec<PathBuf>>>,
    }

    impl Game for FakeGame {
        fn running(&self) -> bool {
            self.running.load(Ordering::Relaxed)
        }
        fn installations(&self) -> Vec<PathBuf> {
            self.installs.clone()
        }
        fn move_to_recycle_bin(&self, path: &Path) -> std::io::Result<()> {
            self.recycled.lock().expect("lock").push(path.to_path_buf());
            deadloader_core::remove::Delete::delete(&deadloader_core::remove::Permanently, path)
        }
    }

    pub(crate) struct Fixture {
        pub dir: PathBuf,
    }

    impl Fixture {
        pub(crate) fn new(name: &str) -> Self {
            let dir = std::env::temp_dir()
                .join(format!("deadloader-backend-{name}-{}", std::process::id()));
            clear(&dir);
            std::fs::create_dir_all(&dir).expect("dir");
            Self { dir }
        }
        pub(crate) fn game(&self) -> PathBuf {
            let root = self.dir.join("Deadlock");
            let citadel = root.join("game").join("citadel");
            std::fs::create_dir_all(&citadel).expect("game");
            std::fs::write(citadel.join("gameinfo.gi"), VANILLA).expect("gameinfo");
            root
        }
        pub(crate) fn addons(&self) -> PathBuf {
            self.dir
                .join("Deadlock")
                .join("game")
                .join("citadel")
                .join("addons")
        }
        pub(crate) fn mod_file(&self, name: &str, path: &str) -> PathBuf {
            let file = self.dir.join(name);
            std::fs::write(&file, vpk(path)).expect("vpk");
            file
        }
        pub(crate) fn backend(&self, game: &FakeGame) -> Backend {
            Backend::start(
                &self.dir.join("deadloader-data"),
                Box::new(game.clone()),
                Arc::new(deadloader_core::log::MemFileLog::default()),
            )
        }
    }

    impl Drop for Fixture {
        fn drop(&mut self) {
            clear(&self.dir);
        }
    }

    fn clear(dir: &Path) {
        fn walk(p: &Path) {
            if let Ok(entries) = std::fs::read_dir(p) {
                for e in entries.flatten() {
                    let path = e.path();
                    if path.is_dir() {
                        walk(&path);
                    } else if let Ok(meta) = std::fs::metadata(&path) {
                        let mut perm = meta.permissions();
                        #[allow(clippy::permissions_set_readonly_false)]
                        perm.set_readonly(false);
                        let _ = std::fs::set_permissions(&path, perm);
                    }
                }
            }
        }
        walk(dir);
        let _ = std::fs::remove_dir_all(dir);
    }

    /// A VPK v2 directory holding one file.
    fn vpk(path: &str) -> Vec<u8> {
        let (dir, rest) = path.rsplit_once('/').expect("dir");
        let (name, ext) = rest.rsplit_once('.').expect("ext");
        let mut tree = Vec::new();
        for part in [ext, dir, name] {
            tree.extend_from_slice(part.as_bytes());
            tree.push(0);
        }
        tree.extend_from_slice(&1u32.to_le_bytes());
        tree.extend_from_slice(&0u16.to_le_bytes());
        tree.extend_from_slice(&0x7FFFu16.to_le_bytes());
        tree.extend_from_slice(&0u32.to_le_bytes());
        tree.extend_from_slice(&0u32.to_le_bytes());
        tree.extend_from_slice(&0xFFFFu16.to_le_bytes());
        tree.extend_from_slice(&[0, 0, 0]);
        let mut out = Vec::new();
        out.extend_from_slice(&0x55AA_1234u32.to_le_bytes());
        out.extend_from_slice(&2u32.to_le_bytes());
        out.extend_from_slice(&(tree.len() as u32).to_le_bytes());
        out.extend_from_slice(&[0u8; 16]);
        out.extend_from_slice(&tree);
        out
    }

    #[test]
    fn a_fresh_start_shows_an_empty_library() {
        let f = Fixture::new("fresh");
        let view = f.backend(&FakeGame::default()).library().expect("library");
        assert!(view.mods.is_empty());
        assert_eq!(view.selected_profile, "Default");
    }

    #[test]
    fn a_failed_start_answers_every_call_with_the_reason() {
        let f = Fixture::new("failed");
        let data = f.dir.join("deadloader-data");
        std::fs::create_dir_all(&data).expect("data");
        std::fs::write(data.join("manifest.json"), "{ broken").expect("manifest");
        let backend = f.backend(&FakeGame::default());
        assert_eq!(backend.library().expect_err("refused").id, MessageId::E23);
        assert_eq!(backend.apply().expect_err("refused").id, MessageId::E23);
    }

    #[test]
    fn applying_without_a_game_folder_says_deadlock_was_not_found() {
        let f = Fixture::new("nogame");
        let backend = f.backend(&FakeGame::default());
        assert_eq!(backend.apply().expect_err("no game").id, MessageId::E2);
    }

    #[test]
    fn the_first_apply_uses_the_game_steam_knows_and_later_edits_follow_at_once() {
        let f = Fixture::new("live");
        let game = FakeGame {
            installs: vec![f.game()],
            ..FakeGame::default()
        };
        let backend = f.backend(&game);
        let outcomes = backend
            .import_paths(&[f.mod_file("haze.vpk", "models/haze/body.vmdl_c")])
            .expect("imported");
        assert!(outcomes[0].is_ok());
        let first = backend.apply().expect("applied");
        assert!(first.changed);
        assert!(f.addons().join("pak01_dir.vpk").exists());

        // 138: the applied profile is selected and the game is closed, so switching off takes effect now.
        let id = first.library.mods[0].id.clone();
        let changed = backend.set_enabled(&id, false).expect("off");
        assert!(changed.applied);
        assert!(!changed.library.pending);
        assert!(!f.addons().join("pak01_dir.vpk").exists());
    }

    /// The field names `ui/src/lib/api.ts` relies on.
    #[test]
    fn replies_have_the_shape_the_ui_expects() {
        let f = Fixture::new("shape");
        let backend = f.backend(&FakeGame::default());
        let outcomes = backend
            .import_paths(&[f.mod_file("haze.vpk", "models/haze/body.vmdl_c")])
            .expect("imported");
        let changed = backend
            .set_enabled(&outcomes[0].as_ref().expect("ok").mod_id, true)
            .expect("on");
        let json = serde_json::to_value(&changed).expect("json");
        let keys = |v: &serde_json::Value| -> Vec<String> {
            let mut k: Vec<String> = v.as_object().expect("object").keys().cloned().collect();
            k.sort();
            k
        };
        assert_eq!(keys(&json), ["applied", "library", "waiting_for_game"]);
        assert_eq!(
            keys(&json["library"]),
            [
                "applied_profile",
                "conflicts",
                "mods",
                "pending",
                "selected_profile"
            ]
        );
        assert_eq!(
            keys(&json["library"]["mods"][0]),
            [
                "add_ons",
                "author",
                "bytes",
                "enabled",
                "files",
                "id",
                "name",
                "source",
                "variant_groups"
            ]
        );
        assert_eq!(
            keys(&json["library"]["mods"][0]["files"][0]),
            [
                "archive_name",
                "damaged",
                "id",
                "inner_path",
                "len",
                "version"
            ]
        );
        assert_eq!(json["library"]["mods"][0]["source"]["kind"], "local");
        let import = serde_json::to_value(&outcomes).expect("json");
        assert!(import[0]["Ok"]["mod_id"].is_string());
        let failure = serde_json::to_value(UserMessage::new(MessageId::E2)).expect("json");
        assert_eq!(failure, serde_json::json!({"id": "E2", "args": {}}));
    }

    #[test]
    fn while_the_game_runs_edits_wait_for_it() {
        let f = Fixture::new("waiting");
        let game = FakeGame {
            installs: vec![f.game()],
            ..FakeGame::default()
        };
        let backend = f.backend(&game);
        backend
            .import_paths(&[f.mod_file("haze.vpk", "models/haze/body.vmdl_c")])
            .expect("imported");
        let id = backend.apply().expect("applied").library.mods[0].id.clone();

        game.running.store(true, Ordering::Relaxed);
        let changed = backend.set_enabled(&id, false).expect("off");
        assert!(!changed.applied);
        assert!(changed.waiting_for_game);
        assert!(changed.library.pending);
        assert!(
            f.addons().join("pak01_dir.vpk").exists(),
            "the game folder is untouched"
        );
    }

    #[test]
    fn closing_the_game_applies_what_was_changed_while_it_ran() {
        let f = Fixture::new("closed");
        let game = FakeGame {
            installs: vec![f.game()],
            ..FakeGame::default()
        };
        let backend = f.backend(&game);
        backend
            .import_paths(&[f.mod_file("haze.vpk", "models/haze/body.vmdl_c")])
            .expect("imported");
        let id = backend.apply().expect("applied").library.mods[0].id.clone();

        game.running.store(true, Ordering::Relaxed);
        backend.set_enabled(&id, false).expect("off");
        backend.set_enabled(&id, true).expect("on");
        backend.set_enabled(&id, false).expect("off again");
        assert!(backend.game_closed().expect("still running").is_none());
        assert!(f.addons().join("pak01_dir.vpk").exists());

        // 143: right after the game exits.
        game.running.store(false, Ordering::Relaxed);
        let after = backend.game_closed().expect("applied").expect("changes");
        // Off, on, off: one change differs from what the game has.
        assert_eq!(after.changes, 1);
        assert!(!after.library.pending);
        assert!(!f.addons().join("pak01_dir.vpk").exists());
        // Nothing left to apply.
        assert!(backend.game_closed().expect("nothing").is_none());
    }

    #[test]
    fn a_profile_switch_still_waits_for_apply_after_the_game_closes() {
        let f = Fixture::new("closedswitch");
        let game = FakeGame {
            installs: vec![f.game()],
            ..FakeGame::default()
        };
        let backend = f.backend(&game);
        backend
            .import_paths(&[f.mod_file("haze.vpk", "models/haze/body.vmdl_c")])
            .expect("imported");
        backend.apply().expect("applied");
        game.running.store(true, Ordering::Relaxed);
        backend.create_profile("Evening").expect("created");
        backend.select_profile("Evening").expect("selected");
        game.running.store(false, Ordering::Relaxed);
        assert!(backend.game_closed().expect("no apply").is_none());
        assert!(f.addons().join("pak01_dir.vpk").exists());
    }

    #[test]
    fn an_explicit_apply_starts_the_count_of_waiting_edits_again() {
        let f = Fixture::new("closedcount");
        let game = FakeGame {
            installs: vec![f.game()],
            ..FakeGame::default()
        };
        let backend = f.backend(&game);
        backend
            .import_paths(&[
                f.mod_file("haze.vpk", "models/haze/body.vmdl_c"),
                f.mod_file("wraith.vpk", "models/wraith/body.vmdl_c"),
            ])
            .expect("imported");
        let ids: Vec<String> = backend
            .apply()
            .expect("applied")
            .library
            .mods
            .iter()
            .map(|m| m.id.clone())
            .collect();
        game.running.store(true, Ordering::Relaxed);
        backend.set_enabled(&ids[0], false).expect("off");
        game.running.store(false, Ordering::Relaxed);
        backend.apply().expect("applied by hand");
        game.running.store(true, Ordering::Relaxed);
        backend.set_enabled(&ids[1], false).expect("off");
        game.running.store(false, Ordering::Relaxed);
        assert_eq!(backend.game_closed().expect("ok").expect("one").changes, 1);
    }

    #[test]
    fn start_notices_are_kept_until_taken_once() {
        let f = Fixture::new("notices");
        let data = f.dir.join("deadloader-data");
        std::fs::create_dir_all(&data).expect("data");
        std::fs::write(data.join("settings.json"), r#"{"schema": 1, "zzz": 1}"#).expect("settings");
        let backend = f.backend(&FakeGame::default());
        assert_eq!(
            backend.take_notices(),
            [crate::events::Notice::SettingsCorrected {
                fields: vec!["zzz".to_owned()]
            }]
        );
        assert!(backend.take_notices().is_empty());
    }

    #[test]
    fn edits_to_a_profile_that_was_never_applied_only_change_the_profile() {
        let f = Fixture::new("notapplied");
        let game = FakeGame {
            installs: vec![f.game()],
            ..FakeGame::default()
        };
        let backend = f.backend(&game);
        let outcomes = backend
            .import_paths(&[f.mod_file("haze.vpk", "models/haze/body.vmdl_c")])
            .expect("imported");
        let id = outcomes[0].as_ref().expect("ok").mod_id.clone();
        let changed = backend.set_enabled(&id, false).expect("off");
        assert!(!changed.applied && !changed.waiting_for_game);
        assert!(!f.addons().exists());
    }

    #[test]
    fn removing_a_mod_that_is_in_the_game_takes_it_out_first() {
        let f = Fixture::new("removelive");
        let game = FakeGame {
            installs: vec![f.game()],
            ..FakeGame::default()
        };
        let backend = f.backend(&game);
        backend
            .import_paths(&[f.mod_file("haze.vpk", "models/haze/body.vmdl_c")])
            .expect("imported");
        let id = backend.apply().expect("applied").library.mods[0].id.clone();
        assert!(f.addons().join("pak01_dir.vpk").exists());

        let removed = backend.remove(&id, None).expect("removed");
        assert_eq!(removed.files.len(), 1);
        assert!(!f.addons().join("pak01_dir.vpk").exists());
        assert!(backend.library().expect("library").mods.is_empty());
    }

    #[test]
    fn removing_while_the_game_runs_waits_for_it() {
        let f = Fixture::new("removerunning");
        let game = FakeGame {
            installs: vec![f.game()],
            ..FakeGame::default()
        };
        let backend = f.backend(&game);
        backend
            .import_paths(&[f.mod_file("haze.vpk", "models/haze/body.vmdl_c")])
            .expect("imported");
        let id = backend.apply().expect("applied").library.mods[0].id.clone();
        game.running.store(true, Ordering::Relaxed);
        assert_eq!(
            backend.remove(&id, None).expect_err("refused").id,
            MessageId::E13
        );
        assert_eq!(backend.library().expect("library").mods.len(), 1);
    }

    #[test]
    fn the_recycle_bin_setting_sends_files_there() {
        let f = Fixture::new("recycle");
        let game = FakeGame::default();
        let backend = f.backend(&game);
        let outcomes = backend
            .import_paths(&[f.mod_file("haze.vpk", "models/haze/body.vmdl_c")])
            .expect("imported");
        let id = outcomes[0].as_ref().expect("ok").mod_id.clone();
        let mut settings = backend.settings().expect("settings");
        settings.library.deletion = deadloader_core::settings::Deletion::RecycleBin;
        backend.update_settings(settings).expect("saved");
        backend.remove(&id, None).expect("removed");
        assert_eq!(game.recycled.lock().expect("lock").len(), 1);
    }

    #[test]
    fn a_profile_travels_through_a_file() {
        let f = Fixture::new("profilefile");
        let backend = f.backend(&FakeGame::default());
        let file = f.dir.join("shared").join("Default.json");
        std::fs::create_dir_all(file.parent().expect("parent")).expect("dir");
        backend
            .export_profile_to("Default", &file)
            .expect("exported");
        let text = std::fs::read_to_string(&file).expect("written");
        assert!(text.contains("deadloader_profile"));
        let imported = backend.import_profile_from(&file).expect("imported");
        assert_eq!(imported.name, "Default (2)");
    }

    #[test]
    fn a_profile_file_that_is_not_one_is_refused_plainly() {
        let f = Fixture::new("profilebad");
        let backend = f.backend(&FakeGame::default());
        let file = f.dir.join("notes.json");
        std::fs::write(&file, "[1, 2, 3]").expect("write");
        assert_eq!(
            backend.import_profile_from(&file).expect_err("refused").id,
            MessageId::E17
        );
        assert_eq!(
            backend
                .import_profile_from(&f.dir.join("missing.json"))
                .expect_err("missing")
                .id,
            MessageId::E17
        );
    }

    #[test]
    fn profiles_are_managed_through_the_backend() {
        let f = Fixture::new("profiles");
        let backend = f.backend(&FakeGame::default());
        backend.create_profile("Evening").expect("created");
        backend.rename_profile("Evening", "Night").expect("renamed");
        backend
            .duplicate_profile("Night", "Night 2")
            .expect("duplicated");
        backend.delete_profile("Night 2").expect("deleted");
        assert_eq!(
            backend.profile_names().expect("names"),
            ["Default", "Night"]
        );
        let text = backend.export_profile("Night").expect("exported");
        let imported = backend.import_profile(&text).expect("imported");
        assert_eq!(imported.name, "Night (2)");
        assert_eq!(
            backend.library().expect("library").selected_profile,
            "Night (2)"
        );
    }

    #[test]
    fn one_bad_file_does_not_stop_the_others() {
        let f = Fixture::new("batch");
        let backend = f.backend(&FakeGame::default());
        let text = f.dir.join("notes.txt");
        std::fs::write(&text, "not a mod").expect("txt");
        let outcomes = backend
            .import_paths(&[text, f.mod_file("haze.vpk", "models/haze/body.vmdl_c")])
            .expect("imported");
        assert_eq!(outcomes[0].as_ref().expect_err("refused").id, MessageId::E8);
        assert!(outcomes[1].is_ok());
        assert_eq!(backend.library().expect("library").mods.len(), 1);
    }
}
