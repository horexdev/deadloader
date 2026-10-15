//! What the UI's commands call: the open session and the game on this PC, with no domain logic of its own beyond
//! decision 138 (edits to the applied profile take effect at once while the game is closed).

use deadloader_core::message::{MessageId, ToUserMessage, UserMessage};
use deadloader_core::session::{ImportOutcome, LibraryView, Session, SharedLog};
use serde::Serialize;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

/// The game as seen from this PC: whether it runs, and where Steam has it installed.
pub trait Game: Send + Sync {
    fn running(&self) -> bool;
    fn installations(&self) -> Vec<PathBuf>;
}

pub struct Backend {
    session: Mutex<Option<Session>>,
    /// Why the session could not be opened (the UI shows it instead of the library).
    start_error: Option<UserMessage>,
    game: Box<dyn Game>,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ApplyResult {
    pub library: LibraryView,
    pub changed: bool,
    /// `gameinfo.gi` was changed by another program, so mods won't load until it is fixed (E3, D10).
    pub gameinfo_foreign: bool,
}

impl Backend {
    pub fn start(data_dir: &Path, game: Box<dyn Game>, log: SharedLog) -> Self {
        match Session::open(data_dir, log) {
            Ok(opened) => Self {
                session: Mutex::new(Some(opened.session)),
                start_error: None,
                game,
            },
            Err(e) => Self {
                session: Mutex::new(None),
                start_error: Some(e.user_message()),
                game,
            },
        }
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
            self.follow_up()?;
        }
        Ok(outcomes)
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
        self.follow_up()
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
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};

    const VANILLA: &str = "\"GameInfo\"\r\n{\r\n\tFileSystem\r\n\t{\r\n\t\tSearchPaths\r\n\t\t{\r\n\t\t\tGame\t\t\t\tcitadel\r\n\t\t\tGame\t\t\t\tcore\r\n\t\t}\r\n\t}\r\n}\r\n";

    #[derive(Clone, Default)]
    struct FakeGame {
        running: Arc<AtomicBool>,
        installs: Vec<PathBuf>,
    }

    impl Game for FakeGame {
        fn running(&self) -> bool {
            self.running.load(Ordering::Relaxed)
        }
        fn installations(&self) -> Vec<PathBuf> {
            self.installs.clone()
        }
    }

    struct Fixture {
        dir: PathBuf,
    }

    impl Fixture {
        fn new(name: &str) -> Self {
            let dir = std::env::temp_dir()
                .join(format!("deadloader-backend-{name}-{}", std::process::id()));
            clear(&dir);
            std::fs::create_dir_all(&dir).expect("dir");
            Self { dir }
        }
        fn game(&self) -> PathBuf {
            let root = self.dir.join("Deadlock");
            let citadel = root.join("game").join("citadel");
            std::fs::create_dir_all(&citadel).expect("game");
            std::fs::write(citadel.join("gameinfo.gi"), VANILLA).expect("gameinfo");
            root
        }
        fn addons(&self) -> PathBuf {
            self.dir
                .join("Deadlock")
                .join("game")
                .join("citadel")
                .join("addons")
        }
        fn mod_file(&self, name: &str, path: &str) -> PathBuf {
            let file = self.dir.join(name);
            std::fs::write(&file, vpk(path)).expect("vpk");
            file
        }
        fn backend(&self, game: &FakeGame) -> Backend {
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
