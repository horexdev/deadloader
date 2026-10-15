//! What the UI's commands call: the open session and the game on this PC, with no domain logic of its own beyond
//! decision 138 (edits to the applied profile take effect at once while the game is closed).

use crate::events::Notice;
use deadloader_core::log::{Clock, FileEvent, FileOp, Outcome};
use deadloader_core::message::{Area, MessageId, ToUserMessage, UserMessage, from_io};
use deadloader_core::profiles::ProfileError;
use deadloader_core::remove::{Delete, Permanently, RemovalPlan, RemoveError, Removed, Target};
use deadloader_core::session::{
    Detected, EditRefused, EditSaved, InstallationView, RollbackPreview, RolledBack,
};
use deadloader_core::session::{
    ImportOutcome, LibraryView, ProfileImported, Session, SessionError, SharedLog,
};
use deadloader_core::settings::{CatalogChoice, Deletion, Lang, Settings};
use serde::Serialize;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

/// The game as seen from this PC: whether it runs, and where Steam has it installed.
pub trait Game: Send + Sync {
    fn running(&self) -> bool;
    fn installations(&self) -> Vec<PathBuf>;
    /// Starts Deadlock the way Steam does.
    fn launch(&self) -> std::io::Result<()>;
    fn move_to_recycle_bin(&self, path: &Path) -> std::io::Result<()>;
    /// The installations Steam knows, with their library and build (Settings → Game).
    fn detected(&self) -> Vec<Detected> {
        self.installations()
            .into_iter()
            .map(|game_root| Detected {
                game_root,
                library: None,
                build_id: None,
            })
            .collect()
    }

    /// Hands a request to Windows: Steam's check of the game files, or a file shown in Explorer or Notepad.
    fn shell(&self, request: &ShellRequest) -> std::io::Result<()>;
}

/// What deadloader asks Windows to show. Paths are always ones the core chose.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShellRequest {
    /// `steam://validate/1422450` (D10, decision 132).
    VerifyGameFiles,
    /// File Explorer with this item selected.
    Reveal(PathBuf),
    /// Notepad with this file (`gameinfo.gi` has no program of its own).
    Notepad(PathBuf),
}

impl ShellRequest {
    /// The program and its command line, for the log (ADR 0009: external programs are logged).
    pub fn describe(&self) -> (String, String) {
        match self {
            Self::VerifyGameFiles => ("steam".to_owned(), "steam://validate/1422450".to_owned()),
            Self::Reveal(p) => (
                "explorer.exe".to_owned(),
                format!("/select,\"{}\"", p.display()),
            ),
            Self::Notepad(p) => ("notepad.exe".to_owned(), format!("\"{}\"", p.display())),
        }
    }
}

/// Where "Open file" / "Open folder" point: places the core knows, never a path from the UI.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Place {
    /// The game's `gameinfo.gi` (D10).
    Gameinfo,
    /// A foreign item of the game's `addons` folder, by name (F9).
    Foreign { name: String },
    /// A library mod's files (mod details).
    ModFiles { mod_id: String },
    /// The file the last profile export wrote ("Show in folder" on its toast).
    ExportedProfile,
}

/// S9's content: the pending changes plus what the last automatic apply did, while it can be undone (143, 147).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Pending {
    #[serde(flatten)]
    pub view: deadloader_core::session::PendingView,
    /// What went into the game folder when the game was last closed; Undo is offered until the next change.
    pub applied_after_game: Option<AfterGameChanges>,
    /// The pending changes are the Undo of the apply at this time, waiting for the game to close.
    pub undoing: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AfterGameChanges {
    pub changes: Vec<deadloader_core::pending::PendingChange>,
    pub at_unix_ms: i64,
}

/// The game's `addons` folder, foreign items only (F9).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ForeignReport {
    /// The game folder looked at; `None`: Deadlock wasn't found.
    pub root: Option<String>,
    pub items: Vec<deadloader_core::session::ForeignView>,
}

/// D11's result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Taken {
    pub mod_id: String,
    pub changed: Changed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GameinfoState {
    /// Our line is in: mods load.
    Ours,
    /// The original: our line goes in at the next apply.
    Vanilla,
    /// Changed by another program (D10, E3).
    Foreign,
    /// No game folder or no file.
    Missing,
}

/// D10's content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GameinfoStatus {
    pub state: GameinfoState,
    pub path: Option<String>,
    /// Only for [`GameinfoState::Foreign`].
    pub diff: Option<deadloader_core::gameinfo::Diff>,
}

/// What Undo after an automatic apply goes back to.
struct AutoApplied {
    restore: deadloader_core::library::Profile,
    changes: Vec<deadloader_core::pending::PendingChange>,
    at_unix_ms: i64,
}

fn now_unix_ms() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .ok()
        .and_then(|d| i64::try_from(d.as_millis()).ok())
        .unwrap_or(0)
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
    /// Told about every saved settings change (the network client, the catalog).
    settings_listeners: Mutex<Vec<SettingsListener>>,
    /// The last automatic apply after the game closed, until the next change (147).
    auto: Mutex<Option<AutoApplied>>,
    /// The pending changes undo the automatic apply of this time.
    undoing: Mutex<Option<i64>>,
    /// N6 was looked for (once per start).
    foreign_told: std::sync::atomic::AtomicBool,
    /// Where the last profile export of this session went.
    last_export: Mutex<Option<PathBuf>>,
}

pub type SettingsListener = Box<dyn Fn(&Settings) + Send + Sync>;

/// The result of a change in the library.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Changed {
    pub library: LibraryView,
    /// The change was put into the game folder right away (138).
    pub applied: bool,
    /// The game is running; the change waits until it is closed (N4).
    pub waiting_for_game: bool,
}

/// The game as the sidebar shows it: running or not, and the folder the selected profile goes into (153).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GameStatus {
    pub running: bool,
    /// The profile's game folder, else the first one Steam knows; `None`: Deadlock wasn't found.
    pub root: Option<String>,
    /// How many installations Steam knows: the sidebar names the drive only when there is more than one.
    pub installations: usize,
}

/// A download put into the library (F2).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Installed {
    pub outcome: ImportOutcome,
    /// `None` while the user has to pick variants first (D4, then `choose`).
    pub changed: Option<Changed>,
}

/// One replaced GameBanana file after its update arrived (D5).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum UpdateStep {
    /// Every old file had one clear partner: the update is in the library and, when it can be, in the game.
    Applied { mod_id: String, changed: Changed },
    /// Some old file has no clear partner: the user maps them (`confirm_update`); the new files wait, not enabled.
    NeedsMapping {
        mod_id: String,
        replaced: u64,
        new_files: Vec<String>,
        proposals: Vec<deadloader_core::session::UpdateProposal>,
        version: Option<String>,
    },
}

/// The game was closed and what changed while it ran went into its folder (N12).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AfterGame {
    pub changes: usize,
    pub library: LibraryView,
}

/// The Profiles screen (S5): every profile, which one is selected, and the game folders a profile can go into.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProfilesView {
    pub selected: String,
    /// Steam's installations, then the ones added by hand in Settings.
    pub installations: Vec<String>,
    pub profiles: Vec<deadloader_core::session::ProfileOverview>,
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
                settings_listeners: Mutex::new(Vec::new()),
                auto: Mutex::new(None),
                undoing: Mutex::new(None),
                foreign_told: std::sync::atomic::AtomicBool::new(false),
                last_export: Mutex::new(None),
            },
            Err(e) => Self {
                session: Mutex::new(None),
                start_error: Some(e.user_message()),
                game,
                log,
                notices: Mutex::new(Vec::new()),
                edits_waiting: AtomicUsize::new(0),
                settings_listeners: Mutex::new(Vec::new()),
                auto: Mutex::new(None),
                undoing: Mutex::new(None),
                foreign_told: std::sync::atomic::AtomicBool::new(false),
                last_export: Mutex::new(None),
            },
        }
    }

    /// What the start found, with N6 for foreign items in the game's mod folder; empty after the first call.
    pub fn take_notices(&self) -> Vec<Notice> {
        let foreign = if self.foreign_told.swap(true, Ordering::Relaxed) {
            0
        } else {
            self.foreign_to_report()
        };
        let mut notices = std::mem::take(&mut *lock(&self.notices));
        if foreign > 0 {
            notices.push(Notice::Foreign { files: foreign });
        }
        notices
    }

    pub fn game_running(&self) -> bool {
        self.game.running()
    }

    /// Decision 143: the game was closed, so what was changed in the applied profile while it ran goes into its folder
    /// now. `None` when there was nothing to apply, or the game runs again. A profile switch still waits for Apply.
    pub fn game_closed(&self) -> Result<Option<AfterGame>, UserMessage> {
        // Read before applying: any apply starts the count again.
        let changes = self.edits_waiting.load(Ordering::Relaxed);
        let fallback = self.game.installations().into_iter().next();
        let (restore, items) = self.with(|s| {
            Ok((
                s.profile_matching_game(fallback.as_deref()),
                s.pending_view(fallback.as_deref()).changes,
            ))
        })?;
        let changed = self.follow_up()?;
        if changed.applied {
            *lock(&self.undoing) = None;
            *lock(&self.auto) = restore
                .filter(|_| !items.is_empty())
                .map(|restore| AutoApplied {
                    restore,
                    changes: items,
                    at_unix_ms: now_unix_ms(),
                });
        }
        Ok(changed.applied.then_some(AfterGame {
            changes,
            library: changed.library,
        }))
    }

    /// S9: the pending changes, and the last automatic apply while it can be undone.
    pub fn pending(&self) -> Result<Pending, UserMessage> {
        let fallback = self.game.installations().into_iter().next();
        let view = self.with(|s| Ok(s.pending_view(fallback.as_deref())))?;
        Ok(Pending {
            view,
            applied_after_game: lock(&self.auto).as_ref().map(|a| AfterGameChanges {
                changes: a.changes.clone(),
                at_unix_ms: a.at_unix_ms,
            }),
            undoing: *lock(&self.undoing),
        })
    }

    /// S9 "Undo this change".
    pub fn revert_pending(
        &self,
        mod_id: &str,
        aspect: deadloader_core::pending::Aspect,
    ) -> Result<Changed, UserMessage> {
        let fallback = self.game.installations().into_iter().next();
        self.change(|s| s.revert_pending(mod_id, aspect, fallback.as_deref()))
    }

    /// S9 "Discard all" (or "Keep <applied profile>" for a switch).
    pub fn discard_pending(&self) -> Result<Changed, UserMessage> {
        let fallback = self.game.installations().into_iter().next();
        self.change(|s| s.discard_pending(fallback.as_deref()))
    }

    /// Undo of the last automatic apply (N12, 147): an ordinary edit, so it waits while the game runs.
    pub fn undo_applied(&self) -> Result<Changed, UserMessage> {
        let auto = lock(&self.auto).take().ok_or_else(|| {
            UserMessage::new(MessageId::Internal).arg("detail", "nothing to undo")
        })?;
        let changed = self.change(|s| s.restore_profile(&auto.restore))?;
        if changed.waiting_for_game {
            *lock(&self.undoing) = Some(auto.at_unix_ms);
        }
        Ok(changed)
    }

    /// The game folder the selected profile goes to, else the first one Steam knows.
    fn root(&self) -> Result<Option<PathBuf>, UserMessage> {
        let first = self.game.installations().into_iter().next();
        Ok(self.with(|s| Ok(s.game_root()))?.or(first))
    }

    /// F9: the foreign items of the game's `addons` folder.
    pub fn foreign_files(&self) -> Result<ForeignReport, UserMessage> {
        let root = self.root()?;
        let items = match &root {
            Some(r) => self.with(|s| Ok(s.foreign(r)))?,
            None => Vec::new(),
        };
        Ok(ForeignReport {
            root: root.map(|r| r.display().to_string()),
            items,
        })
    }

    /// N6 at start: foreign items the user hasn't chosen to leave as they are.
    pub fn foreign_to_report(&self) -> usize {
        self.foreign_files()
            .map(|r| r.items.iter().filter(|i| !i.left).count())
            .unwrap_or(0)
    }

    /// F9 "Leave as is".
    pub fn leave_foreign(&self, name: &str) -> Result<ForeignReport, UserMessage> {
        let root = self
            .root()?
            .ok_or_else(|| UserMessage::new(MessageId::E2))?;
        self.with(|s| Ok(s.leave_foreign(&root, name)?))?;
        self.foreign_files()
    }

    /// D11 "Take under management"; then, like any edit, into the game at once when it can (138).
    pub fn take_foreign(&self, name: &str, mod_name: &str) -> Result<Taken, UserMessage> {
        let root = self
            .root()?
            .ok_or_else(|| UserMessage::new(MessageId::E2))?;
        let running = self.game.running();
        if running {
            return Err(UserMessage::new(MessageId::E13));
        }
        let mod_id = self.with(|s| Ok(s.take_foreign(&root, name, mod_name, running)?))?;
        let changed = self.follow_up_edit()?;
        Ok(Taken { mod_id, changed })
    }

    /// D10: the state of `gameinfo.gi`; with `fix` ("Check again") our line goes into an original file.
    pub fn gameinfo_status(&self, fix: bool) -> Result<GameinfoStatus, UserMessage> {
        use deadloader_core::gameinfo::{State, diff, inspect};
        let Some(root) = self.root()? else {
            return Ok(GameinfoStatus {
                state: GameinfoState::Missing,
                path: None,
                diff: None,
            });
        };
        let path = deadloader_core::gamedir::gameinfo_path(&root);
        let read = || std::fs::read_to_string(&path).ok();
        let mut text = read();
        if fix
            && !self.game.running()
            && text
                .as_deref()
                .is_some_and(|t| inspect(t) == State::Vanilla)
        {
            self.with(|s| Ok(s.fix_gameinfo(&root, false)?))?;
            text = read();
        }
        let (state, diff) = match text.as_deref() {
            None => (GameinfoState::Missing, None),
            Some(t) => match inspect(t) {
                State::Ours => (GameinfoState::Ours, None),
                State::Vanilla => (GameinfoState::Vanilla, None),
                State::Foreign(_) => (GameinfoState::Foreign, Some(diff(t))),
            },
        };
        Ok(GameinfoStatus {
            state,
            path: Some(path.display().to_string()),
            diff,
        })
    }

    /// D10's primary action: Steam checks Deadlock's files and restores the originals.
    pub fn verify_game_files(&self) -> Result<ShellRequest, UserMessage> {
        self.shell(ShellRequest::VerifyGameFiles)
    }

    /// "Open file" (`as_file`, Notepad) or "Open folder" (Explorer with the item selected) for a place the core knows.
    pub fn show(&self, place: &Place, as_file: bool) -> Result<ShellRequest, UserMessage> {
        let unknown = |what: &str| UserMessage::new(MessageId::Internal).arg("detail", what);
        let path = match place {
            Place::Gameinfo => {
                let root = self
                    .root()?
                    .ok_or_else(|| UserMessage::new(MessageId::E2))?;
                deadloader_core::gamedir::gameinfo_path(&root)
            }
            Place::Foreign { name } => {
                let report = self.foreign_files()?;
                let root = report.root.ok_or_else(|| UserMessage::new(MessageId::E2))?;
                let item = report
                    .items
                    .iter()
                    .find(|i| &i.name == name)
                    .ok_or_else(|| unknown("not a foreign item of the game's mod folder"))?;
                deadloader_core::gamedir::addons_path(Path::new(&root)).join(&item.name)
            }
            Place::ModFiles { mod_id } => self
                .with(|s| Ok(s.mod_file_path(mod_id)))?
                .ok_or_else(|| unknown("the mod has no files in the library"))?,
            Place::ExportedProfile => lock(&self.last_export)
                .clone()
                .ok_or_else(|| unknown("no profile was exported"))?,
        };
        let request = match (place, as_file) {
            (Place::Gameinfo, true) => ShellRequest::Notepad(path),
            _ => ShellRequest::Reveal(path),
        };
        self.shell(request)
    }

    /// File Explorer with a file selected that the core found itself (a download's kept archive), never one the UI
    /// named.
    pub fn show_file(&self, path: PathBuf) -> Result<ShellRequest, UserMessage> {
        self.shell(ShellRequest::Reveal(path))
    }

    fn shell(&self, request: ShellRequest) -> Result<ShellRequest, UserMessage> {
        self.game.shell(&request).map_err(|e| {
            UserMessage::new(MessageId::Internal).arg("detail", format!("Windows refused: {e}"))
        })?;
        Ok(request)
    }

    /// F10 + 171: link a mod imported from disk to its GameBanana page, matching its archives to the listed files.
    pub fn link_mod(
        &self,
        mod_id: &str,
        section: deadloader_core::library::Section,
        gamebanana_id: u64,
        listed: &[deadloader_core::updates::RemoteFile],
    ) -> Result<deadloader_core::session::LinkReport, UserMessage> {
        self.with(|s| Ok(s.link_mod(mod_id, section, gamebanana_id, listed)?))
    }

    /// "Link" by hand: these files of a linked mod came from GameBanana file `remote_file_id`.
    pub fn link_files(
        &self,
        mod_id: &str,
        files: &[String],
        remote_file_id: Option<u64>,
    ) -> Result<(), UserMessage> {
        self.with(|s| Ok(s.link_files(mod_id, files, remote_file_id)?))
    }

    /// F10: the name a mod is shown under.
    pub fn rename_mod(&self, mod_id: &str, name: &str) -> Result<LibraryView, UserMessage> {
        self.with(|s| {
            s.rename_mod(mod_id, name)?;
            Ok(s.library())
        })
    }

    /// "Hide" on "new files available" (168).
    pub fn dismiss_new_files(&self, mod_id: &str, file_ids: &[u64]) -> Result<(), UserMessage> {
        self.with(|s| Ok(s.dismiss_new_files(mod_id, file_ids)?))
    }

    /// A copy of the library's manifest, for the update check.
    pub fn manifest(&self) -> Result<deadloader_core::library::Manifest, UserMessage> {
        self.with(|s| Ok(s.manifest().clone()))
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
        *lock(&self.auto) = None;
        *lock(&self.undoing) = None;
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

    /// A verified download from the catalog joins the library and, like any edit, the game at once when it can (F2,
    /// 136, 138).
    /// It stops when `hooks` says the user cancelled, as long as nothing of it is in the library yet; past that point
    /// it finishes.
    pub fn install_download(
        &self,
        archive: &Path,
        download: &deadloader_core::session::Download,
        hooks: &dyn deadloader_core::import::Hooks,
    ) -> Result<Installed, UserMessage> {
        let outcome = self.with(|s| Ok(s.import_download_with(archive, download, hooks)?))?;
        let changed = if outcome.choices.is_empty() {
            Some(self.follow_up_edit()?)
        } else {
            None
        };
        Ok(Installed { outcome, changed })
    }

    /// A downloaded update of the GameBanana file `replaced` (D5, 169): imported without enabling it, then applied
    /// at once when every old file has a clear partner, otherwise handed to the user.
    /// Cancelled like [`Backend::install_download`].
    pub fn install_update_file(
        &self,
        archive: &Path,
        download: &deadloader_core::session::Download,
        replaced: u64,
        hooks: &dyn deadloader_core::import::Hooks,
    ) -> Result<UpdateStep, UserMessage> {
        // The library mod may be a local one linked to this GameBanana mod (F10), not `gb-<id>`.
        let imported = self.with(|s| Ok(s.import_update_file_with(archive, download, hooks)?))?;
        self.update_with(
            &imported.mod_id,
            replaced,
            imported.new_files,
            download.version.clone(),
        )
    }

    /// The update's files are in the library already (downloaded before): the same as after a download.
    pub fn update_with(
        &self,
        mod_id: &str,
        replaced: u64,
        new_files: Vec<String>,
        version: Option<String>,
    ) -> Result<UpdateStep, UserMessage> {
        let proposals = self.with(|s| Ok(s.propose_update(mod_id, replaced, &new_files)?))?;
        if !proposals
            .iter()
            .all(deadloader_core::session::UpdateProposal::is_settled)
        {
            return Ok(UpdateStep::NeedsMapping {
                mod_id: mod_id.to_owned(),
                replaced,
                new_files,
                proposals,
                version,
            });
        }
        let mappings = proposals
            .iter()
            .map(|p| deadloader_core::updates::Mapping {
                old: p.old.clone(),
                new: p.new_file().map(str::to_owned),
            })
            .collect();
        let changed = self.confirm_update(mod_id, mappings, version)?;
        Ok(UpdateStep::Applied {
            mod_id: mod_id.to_owned(),
            changed,
        })
    }

    /// The user's mapping in D5, or "keep the old file" (`new: None`): applied like any edit (138).
    pub fn confirm_update(
        &self,
        mod_id: &str,
        mappings: Vec<deadloader_core::updates::Mapping>,
        version: Option<String>,
    ) -> Result<Changed, UserMessage> {
        let update = deadloader_core::updates::ConfirmedUpdate {
            mod_id: mod_id.to_owned(),
            added: Vec::new(),
            mappings,
            version,
        };
        self.change(|s| s.apply_update(&update))
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

    /// Deletes a mod or some of its files the way the settings say (permanently or to the recycle bin), or the way
    /// `deletion` says for this deletion only (D6, "switchable here"). A whole mod that the selected, applied profile
    /// has in the game is taken out of the game first; that waits while the game runs.
    pub fn remove(
        &self,
        mod_id: &str,
        files: Option<Vec<String>>,
        deletion: Option<Deletion>,
    ) -> Result<Removed, UserMessage> {
        let target = target(mod_id, files);
        let running = self.game.running();
        let game = self.game.as_ref();
        self.with(|s| {
            let recycle = RecycleBin(game);
            let deleter: &dyn Delete = match deletion.unwrap_or(s.settings().library.deletion) {
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

    /// Deletes a profile. An applied one takes its mods out of the game first, as its confirmation says (S5); that
    /// waits while the game runs (E13).
    pub fn delete_profile(&self, name: &str) -> Result<Vec<String>, UserMessage> {
        let running = self.game.running();
        self.with(|s| {
            s.take_out_of_game(name, running)?;
            s.delete_profile(name)?;
            Ok(s.profile_names())
        })
    }

    /// The game folders a profile can use: Steam's, then the ones added by hand (decision 153).
    fn known_installations(&self) -> Result<Vec<String>, UserMessage> {
        let manual = self.with(|s| Ok(s.settings().game.manual_installations.clone()))?;
        let mut known: Vec<String> = Vec::new();
        for root in self
            .game
            .installations()
            .iter()
            .map(|r| r.display().to_string())
            .chain(manual)
        {
            if !known.iter().any(|k| k.eq_ignore_ascii_case(&root)) {
                known.push(root);
            }
        }
        Ok(known)
    }

    /// The Profiles screen (S5).
    pub fn profiles_view(&self) -> Result<ProfilesView, UserMessage> {
        let installations = self.known_installations()?;
        self.with(|s| {
            Ok(ProfilesView {
                selected: s.selected_profile().to_owned(),
                installations: installations.clone(),
                profiles: s.profiles_overview(),
            })
        })
    }

    /// "Change game installation" (153): one of the known game folders; nothing is applied.
    pub fn set_profile_installation(
        &self,
        name: &str,
        root: &str,
    ) -> Result<ProfilesView, UserMessage> {
        let known = self.known_installations()?;
        let root = known
            .iter()
            .find(|k| k.eq_ignore_ascii_case(root))
            .ok_or_else(|| UserMessage::new(MessageId::E2).arg("path", root))?
            .clone();
        self.with(|s| Ok(s.set_profile_game_root(name, Path::new(&root))?))?;
        self.profiles_view()
    }

    pub fn export_profile(&self, name: &str) -> Result<String, UserMessage> {
        self.with(|s| Ok(s.export_profile(name)?))
    }

    pub fn import_profile(&self, text: &str) -> Result<ProfileImported, UserMessage> {
        self.with(|s| Ok(s.import_profile(text)?))
    }

    /// Profiles → Export: the shareable JSON written where the user chose (F14). Returns that path for the toast, whose
    /// "Show in folder" is [`Place::ExportedProfile`].
    pub fn export_profile_to(&self, name: &str, path: &Path) -> Result<String, UserMessage> {
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
        result.map_err(|e| from_io(&e, Area::Data))?;
        *lock(&self.last_export) = Some(path.to_path_buf());
        Ok(path.display().to_string())
    }

    /// Profiles → Import from a file the user chose.
    /// The result and its errors name the file (its name only), as D9 and the import error show it.
    pub fn import_profile_from(&self, path: &Path) -> Result<ProfileImported, UserMessage> {
        let file = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        let text = std::fs::read_to_string(path).map_err(|e| {
            UserMessage::new(MessageId::E17)
                .arg("reason", "unreadable")
                .arg("file", &file)
                .arg("detail", e)
        })?;
        match self.import_profile(&text) {
            Ok(imported) => Ok(ProfileImported {
                file: Some(file),
                ..imported
            }),
            Err(e) if e.id == MessageId::E17 => Err(e.arg("file", &file)),
            Err(e) => Err(e),
        }
    }

    pub fn settings(&self) -> Result<Settings, UserMessage> {
        self.with(|s| Ok(s.settings().clone()))
    }

    pub fn update_settings(&self, settings: Settings) -> Result<Settings, UserMessage> {
        let saved = self.with(|s| {
            s.update_settings(settings)?;
            Ok(s.settings().clone())
        })?;
        for listener in self.lock_listeners().iter() {
            listener(&saved);
        }
        Ok(saved)
    }

    /// D3 (109): the user confirmed legal age; saved with the settings and heard by every settings listener.
    pub fn confirm_age(&self) -> Result<Settings, UserMessage> {
        let saved = self.with(|s| {
            s.confirm_age()?;
            Ok(s.settings().clone())
        })?;
        for listener in self.lock_listeners().iter() {
            listener(&saved);
        }
        Ok(saved)
    }

    /// The catalog's favourites (34), newest first.
    pub fn favourites(&self) -> Result<Vec<deadloader_core::favourites::Favourite>, UserMessage> {
        self.with(|s| Ok(s.favourites()?))
    }

    /// The heart on a card or the mod page: marks or unmarks a mod; the whole list comes back.
    pub fn set_favourite(
        &self,
        favourite: deadloader_core::favourites::Favourite,
        on: bool,
    ) -> Result<Vec<deadloader_core::favourites::Favourite>, UserMessage> {
        self.with(|s| Ok(s.set_favourite(favourite, on)?))
    }

    /// `listener` hears every saved settings change from now on.
    pub fn on_settings(&self, listener: SettingsListener) {
        self.lock_listeners().push(listener);
    }

    fn lock_listeners(&self) -> std::sync::MutexGuard<'_, Vec<SettingsListener>> {
        self.settings_listeners
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// Play (F4): the selected profile goes into the game folder, then Steam starts Deadlock. The UI asks first when
    /// [`Backend::apply_preview`] turns mods off for a profile switch (D13, 146).
    pub fn play(&self) -> Result<ApplyResult, UserMessage> {
        if self.game.running() {
            return Err(UserMessage::new(MessageId::E13));
        }
        let applied = self.apply()?;
        self.game.launch().map_err(|e| {
            UserMessage::new(MessageId::Internal)
                .arg("detail", format!("Steam could not start Deadlock: {e}"))
        })?;
        Ok(applied)
    }

    /// The game status for the sidebar (`game` pushes keep `running` current).
    pub fn game_status(&self) -> Result<GameStatus, UserMessage> {
        let installations = self.game.installations();
        let root = self
            .with(|s| Ok(s.game_root()))?
            .or_else(|| installations.first().cloned())
            .map(|r| r.display().to_string());
        Ok(GameStatus {
            running: self.game.running(),
            root,
            installations: installations.len(),
        })
    }

    /// D13: the mods Apply or Play would take out of the game and bring into it.
    pub fn apply_preview(&self) -> Result<deadloader_core::session::ApplyPreview, UserMessage> {
        let fallback = self.game.installations().into_iter().next();
        self.with(|s| Ok(s.apply_preview(fallback.as_deref())))
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
            *lock(&self.undoing) = None;
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

    /// [`Self::follow_up`] after an edit: one that waits for the game counts for N12. Any edit ends the Undo of the
    /// last automatic apply (147).
    fn follow_up_edit(&self) -> Result<Changed, UserMessage> {
        *lock(&self.auto) = None;
        *lock(&self.undoing) = None;
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

/// Settings → Game, Advanced and Reset, and the first-run notice (S8, S1, D7, D8; decisions 20, 100, 107–111, 156).
impl Backend {
    /// The folder Rollback works on: the selected profile's, else the first one Steam knows. E2 without either.
    fn rollback_root(&self) -> Result<PathBuf, UserMessage> {
        let first = self.game.installations().into_iter().next();
        self.with(|s| Ok(s.game_root()))?
            .or(first)
            .ok_or_else(|| UserMessage::new(MessageId::E2))
    }

    /// D7: what Rollback would take out of the selected profile's game folder, and what stays.
    pub fn rollback_preview(&self) -> Result<RollbackPreview, UserMessage> {
        let root = self.rollback_root()?;
        self.with(|s| Ok(s.rollback_preview(&root)?))
    }

    /// D7 / F11: our files and our `gameinfo.gi` line leave the selected profile's game folder (foreign files too when
    /// asked). E13 while the game runs.
    pub fn rollback(&self, remove_foreign: bool) -> Result<RolledBack, UserMessage> {
        let root = self.rollback_root()?;
        let running = self.game.running();
        let done = self.with(|s| Ok(s.rollback(&root, remove_foreign, running)?))?;
        self.edits_waiting.store(0, Ordering::Relaxed);
        Ok(done)
    }

    /// The "roll back first" of D8: every game folder deadloader has files or its line in; foreign files stay.
    pub fn rollback_all(&self) -> Result<Vec<RolledBack>, UserMessage> {
        if self.game.running() {
            return Err(UserMessage::new(MessageId::E13));
        }
        let mut known = self.game.installations();
        let manual = self.settings()?.game.manual_installations;
        known.extend(manual.into_iter().map(PathBuf::from));
        self.with(|s| Ok(s.rollback_all(&known, false)?))
    }

    /// Settings → Game → Game installations.
    pub fn installations(&self) -> Result<Vec<InstallationView>, UserMessage> {
        let detected = self.game.detected();
        self.with(|s| Ok(s.installations(&detected)))
    }

    /// "Add manually…" and the setup step's "Browse…": the Deadlock installation the chosen folder belongs to joins
    /// the list (unless Steam lists it already). E2 when the folder holds no Deadlock installation.
    pub fn add_installation(&self, chosen: &Path) -> Result<PathBuf, UserMessage> {
        let root = deadloader_core::setup::game_root_of(&deadloader_core::fs::Disk, chosen)
            .ok_or_else(|| UserMessage::new(MessageId::E2).arg("path", chosen.display()))?;
        let text = root.display().to_string();
        let same = |a: &str| a.eq_ignore_ascii_case(&text);
        let detected = self
            .game
            .installations()
            .iter()
            .any(|d| same(&d.display().to_string()));
        let mut settings = self.settings()?;
        if !detected && !settings.game.manual_installations.iter().any(|m| same(m)) {
            settings.game.manual_installations.push(text);
            self.update_settings(settings)?;
        }
        Ok(root)
    }

    /// Takes a folder added by hand off the list; nothing in it changes.
    pub fn remove_installation(&self, game_root: &str) -> Result<Settings, UserMessage> {
        let mut settings = self.settings()?;
        settings
            .game
            .manual_installations
            .retain(|m| !m.eq_ignore_ascii_case(game_root));
        self.update_settings(settings)
    }

    /// The game folder the selected profile uses, chosen in the setup step.
    pub fn set_game_root(&self, game_root: Option<&Path>) -> Result<(), UserMessage> {
        self.with(|s| Ok(s.set_game_root(game_root)?))
    }

    pub fn settings_text(&self) -> Result<String, UserMessage> {
        self.with(|s| Ok(s.settings_text()))
    }

    pub fn manifest_text(&self) -> Result<String, UserMessage> {
        self.with(|s| Ok(s.manifest_text()))
    }

    /// The Advanced save of `settings.json` (156): the inner error says why nothing was saved.
    pub fn save_settings_text(
        &self,
        text: &str,
    ) -> Result<Result<EditSaved, EditRefused>, UserMessage> {
        let at = deadloader_core::log::SystemClock.now();
        let result = self.with(|s| Ok(s.save_settings_text(text, at)))?;
        if result.is_ok() {
            self.tell_listeners()?;
        }
        Ok(result)
    }

    /// The Advanced save of `manifest.json` (156).
    pub fn save_manifest_text(
        &self,
        text: &str,
    ) -> Result<Result<EditSaved, EditRefused>, UserMessage> {
        let at = deadloader_core::log::SystemClock.now();
        self.with(|s| Ok(s.save_manifest_text(text, at)))
    }

    /// "Reset all settings" (100, 156): the dated backup, and the settings now.
    pub fn reset_settings(&self) -> Result<(EditSaved, Settings), UserMessage> {
        let at = deadloader_core::log::SystemClock.now();
        let saved = self.with(|s| Ok(s.reset_settings(at)?))?;
        let settings = self.tell_listeners()?;
        Ok((saved, settings))
    }

    /// The first-run notice was acknowledged (107–111): its version, the language it was read in, the catalog choice
    /// and the update-check answer (always off in local mode, 157).
    pub fn acknowledge_notice(
        &self,
        version: u32,
        language: Lang,
        catalog: CatalogChoice,
        check_app_updates: bool,
    ) -> Result<Settings, UserMessage> {
        let mut settings = self.settings()?;
        settings.acknowledge_notice(
            version,
            deadloader_core::log::SystemClock.now(),
            language,
            catalog,
        );
        settings
            .set_app_update_check(check_app_updates && catalog == CatalogChoice::Catalog)
            .map_err(|e| UserMessage::new(MessageId::Internal).arg("detail", format!("{e:?}")))?;
        self.update_settings(settings)
    }

    /// F13: the catalog on (after its network step was shown again) or off (local mode).
    pub fn choose_catalog(&self, enabled: bool) -> Result<Settings, UserMessage> {
        let mut settings = self.settings()?;
        settings.choose_catalog(if enabled {
            CatalogChoice::Catalog
        } else {
            CatalogChoice::LocalMode
        });
        self.update_settings(settings)
    }

    /// D8: what deleting all data removes.
    pub fn wipe_summary(
        &self,
        data_dir: &Path,
    ) -> Result<deadloader_core::wipe::Summary, UserMessage> {
        self.with(|s| {
            Ok(deadloader_core::wipe::summary(
                data_dir,
                s.manifest(),
                &s.profile_names(),
            ))
        })
    }

    /// Tells the listeners about settings the session saved itself.
    fn tell_listeners(&self) -> Result<Settings, UserMessage> {
        let saved = self.settings()?;
        for listener in self.lock_listeners().iter() {
            listener(&saved);
        }
        Ok(saved)
    }
}

/// A lock that outlives a panic elsewhere (the data is plain state).
fn lock<T>(m: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    m.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
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
        pub launched: Arc<std::sync::atomic::AtomicUsize>,
        pub installs: Vec<PathBuf>,
        pub recycled: Arc<std::sync::Mutex<Vec<PathBuf>>>,
        pub shown: Arc<std::sync::Mutex<Vec<ShellRequest>>>,
    }

    impl Game for FakeGame {
        fn running(&self) -> bool {
            self.running.load(Ordering::Relaxed)
        }
        fn installations(&self) -> Vec<PathBuf> {
            self.installs.clone()
        }
        fn launch(&self) -> std::io::Result<()> {
            self.launched.fetch_add(1, Ordering::Relaxed);
            Ok(())
        }
        fn move_to_recycle_bin(&self, path: &Path) -> std::io::Result<()> {
            self.recycled.lock().expect("lock").push(path.to_path_buf());
            deadloader_core::remove::Delete::delete(&deadloader_core::remove::Permanently, path)
        }
        fn shell(&self, request: &ShellRequest) -> std::io::Result<()> {
            self.shown.lock().expect("lock").push(request.clone());
            Ok(())
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
                "pending_count",
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
                "in_game",
                "name",
                "pending",
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
                "newer_in_library",
                "remote_file_id",
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
    fn an_installed_download_goes_into_the_game_at_once() {
        let f = Fixture::new("install");
        let game = FakeGame {
            installs: vec![f.game()],
            ..FakeGame::default()
        };
        let backend = f.backend(&game);
        // A profile that was applied once, so edits follow at once (138).
        backend.apply().expect("applied");
        let download = deadloader_core::session::Download {
            section: deadloader_core::library::Section::Mod,
            mod_id: 655_209,
            file_id: 1_480_331,
            file_name: "haze.vpk".to_owned(),
            mod_name: "Haze V2".to_owned(),
            author: None,
            version: None,
        };
        let installed = backend
            .install_download(
                &f.mod_file("17.bin", "models/haze/body.vmdl_c"),
                &download,
                &deadloader_core::import::NoHooks,
            )
            .expect("installed");
        assert_eq!(installed.outcome.mod_id, "gb-655209");
        let changed = installed.changed.expect("enabled");
        assert!(changed.applied);
        assert!(f.addons().join("pak01_dir.vpk").exists());
    }

    fn haze(file_id: u64) -> deadloader_core::session::Download {
        deadloader_core::session::Download {
            section: deadloader_core::library::Section::Mod,
            mod_id: 655_209,
            file_id,
            file_name: "haze.vpk".to_owned(),
            mod_name: "Haze V2".to_owned(),
            author: None,
            version: Some(format!("{file_id}.0")),
        }
    }

    #[test]
    fn a_clear_update_is_applied_and_goes_into_the_game() {
        let f = Fixture::new("update");
        let game = FakeGame {
            installs: vec![f.game()],
            ..FakeGame::default()
        };
        let backend = f.backend(&game);
        backend.apply().expect("applied");
        backend
            .install_download(
                &f.mod_file("1.bin", "models/haze/body.vmdl_c"),
                &haze(1),
                &deadloader_core::import::NoHooks,
            )
            .expect("v1");
        let step = backend
            .install_update_file(
                &f.mod_file("2.bin", "models/haze/body.vmdl_c"),
                &haze(2),
                1,
                &deadloader_core::import::NoHooks,
            )
            .expect("v2");
        let UpdateStep::Applied { mod_id, changed } = step else {
            panic!("{step:?}");
        };
        assert_eq!(mod_id, "gb-655209");
        assert!(changed.applied, "138: straight into the game");
        let library = backend.library().expect("library");
        let active = library.mods[0].variant_groups[0].active.clone();
        let v2 = library.mods[0]
            .files
            .iter()
            .find(|x| x.version.as_deref() == Some("2.0"));
        assert_eq!(active, v2.map(|x| x.id.clone()));
    }

    #[test]
    fn an_unclear_update_waits_for_the_user_and_then_applies_their_mapping() {
        let f = Fixture::new("updatemap");
        let backend = f.backend(&FakeGame::default());
        backend
            .install_download(
                &f.mod_file("1.bin", "models/haze/body.vmdl_c"),
                &haze(1),
                &deadloader_core::import::NoHooks,
            )
            .expect("v1");
        backend
            .install_download(
                &f.mod_file("1b.bin", "models/haze/hands.vmdl_c"),
                &haze(1),
                &deadloader_core::import::NoHooks,
            )
            .expect("v1, second file");
        let step = backend
            .install_update_file(
                &f.mod_file("2.bin", "models/haze/legs.vmdl_c"),
                &haze(2),
                1,
                &deadloader_core::import::NoHooks,
            )
            .expect("v2");
        let UpdateStep::NeedsMapping {
            mod_id,
            new_files,
            proposals,
            ..
        } = step
        else {
            panic!("{step:?}");
        };
        assert_eq!(proposals.len(), 2);
        // The user maps the body to the new file and keeps the hands as they are.
        let mappings: Vec<deadloader_core::updates::Mapping> = proposals
            .iter()
            .enumerate()
            .map(|(i, p)| deadloader_core::updates::Mapping {
                old: p.old.clone(),
                new: (i == 0).then(|| new_files[0].clone()),
            })
            .collect();
        backend
            .confirm_update(&mod_id, mappings, Some("2.0".to_owned()))
            .expect("confirmed");
        let library = backend.library().expect("library");
        let old = library.mods[0]
            .files
            .iter()
            .find(|x| x.id == proposals[0].old)
            .expect("kept");
        assert_eq!(old.newer_in_library.as_ref(), Some(&new_files[0]));
    }

    #[test]
    fn the_game_status_names_the_folder_the_profile_uses() {
        let f = Fixture::new("gamestatus");
        let none = f.backend(&FakeGame::default());
        assert_eq!(
            none.game_status().expect("status"),
            GameStatus {
                running: false,
                root: None,
                installations: 0
            }
        );
        let game = FakeGame {
            installs: vec![f.game(), f.dir.join("Other")],
            ..FakeGame::default()
        };
        let backend = f.backend(&game);
        game.running.store(true, Ordering::Relaxed);
        let status = backend.game_status().expect("status");
        assert!(status.running);
        assert_eq!(status.installations, 2);
        assert_eq!(
            status.root.as_deref(),
            Some(f.game().display().to_string().as_str())
        );
    }

    #[test]
    fn play_applies_the_profile_then_starts_the_game() {
        let f = Fixture::new("play");
        let game = FakeGame {
            installs: vec![f.game()],
            ..FakeGame::default()
        };
        let backend = f.backend(&game);
        backend
            .import_paths(&[f.mod_file("haze.vpk", "models/haze/body.vmdl_c")])
            .expect("imported");
        let played = backend.play().expect("played");
        assert!(played.changed);
        assert!(
            f.addons().join("pak01_dir.vpk").exists(),
            "applied before the start"
        );
        assert_eq!(game.launched.load(Ordering::Relaxed), 1);

        game.running.store(true, Ordering::Relaxed);
        assert_eq!(backend.play().expect_err("running").id, MessageId::E13);
        assert_eq!(
            game.launched.load(Ordering::Relaxed),
            1,
            "not started twice"
        );
    }

    #[test]
    fn saved_settings_reach_every_listener() {
        let f = Fixture::new("listeners");
        let backend = f.backend(&FakeGame::default());
        let seen = Arc::new(std::sync::Mutex::new(Vec::new()));
        let record = Arc::clone(&seen);
        backend.on_settings(Box::new(move |s: &Settings| {
            record.lock().expect("lock").push(s.network.retries);
        }));
        let mut settings = backend.settings().expect("settings");
        settings.network.retries = 2;
        backend.update_settings(settings.clone()).expect("saved");
        // A refused change reaches nobody.
        settings.network.retries = 9999;
        assert!(backend.update_settings(settings).is_err());
        assert_eq!(*seen.lock().expect("lock"), [2]);
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

        let removed = backend.remove(&id, None, None).expect("removed");
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
            backend.remove(&id, None, None).expect_err("refused").id,
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
        backend.remove(&id, None, None).expect("removed");
        assert_eq!(game.recycled.lock().expect("lock").len(), 1);
    }

    #[test]
    fn d6_can_send_one_deletion_to_the_recycle_bin_without_changing_the_setting() {
        let f = Fixture::new("recycleonce");
        let game = FakeGame::default();
        let backend = f.backend(&game);
        let outcomes = backend
            .import_paths(&[f.mod_file("haze.vpk", "models/haze/body.vmdl_c")])
            .expect("imported");
        let id = outcomes[0].as_ref().expect("ok").mod_id.clone();
        backend
            .remove(&id, None, Some(Deletion::RecycleBin))
            .expect("removed");
        assert_eq!(game.recycled.lock().expect("lock").len(), 1);
        assert_eq!(
            backend.settings().expect("settings").library.deletion,
            Deletion::Permanent
        );
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
    fn an_exported_profile_says_where_it_went_and_can_be_shown_in_its_folder() {
        let f = Fixture::new("profileshown");
        let game = FakeGame::default();
        let backend = f.backend(&game);
        assert!(
            backend.show(&Place::ExportedProfile, false).is_err(),
            "nothing exported yet"
        );
        let file = f.dir.join("Default.json");
        let saved = backend
            .export_profile_to("Default", &file)
            .expect("exported");
        assert_eq!(saved, file.display().to_string());
        // A later export that fails doesn't change which file is shown.
        assert!(
            backend
                .export_profile_to("Default", &f.dir.join("missing").join("x.json"))
                .is_err()
        );
        backend
            .show(&Place::ExportedProfile, false)
            .expect("folder");
        assert_eq!(
            game.shown.lock().expect("lock").clone(),
            [ShellRequest::Reveal(file)]
        );
    }

    #[test]
    fn a_file_the_core_found_is_shown_in_file_explorer() {
        let f = Fixture::new("showfile");
        let game = FakeGame::default();
        let backend = f.backend(&game);
        let archive = f.dir.join("downloads").join("17.rar");
        let request = backend.show_file(archive.clone()).expect("shown");
        assert_eq!(request, ShellRequest::Reveal(archive.clone()));
        assert_eq!(
            game.shown.lock().expect("lock").clone(),
            [ShellRequest::Reveal(archive)]
        );
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

    /// Two mods imported and applied to the game Steam knows.
    fn two_applied(f: &Fixture, game: &FakeGame) -> (Backend, String, String) {
        let backend = f.backend(game);
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
        (backend, ids[0].clone(), ids[1].clone())
    }

    fn with_game(f: &Fixture) -> FakeGame {
        FakeGame {
            installs: vec![f.game()],
            ..FakeGame::default()
        }
    }

    #[test]
    fn changes_made_while_the_game_runs_are_listed_and_can_be_taken_back() {
        let f = Fixture::new("pending");
        let game = with_game(&f);
        let (backend, first, _) = two_applied(&f, &game);
        game.running.store(true, Ordering::Relaxed);
        let changed = backend.set_enabled(&first, false).expect("off");
        assert_eq!(changed.library.pending_count, 1);
        let pending = backend.pending().expect("pending");
        assert_eq!(pending.view.changes.len(), 1);
        assert!(pending.applied_after_game.is_none());
        let changed = backend
            .revert_pending(&first, deadloader_core::pending::Aspect::Enabled)
            .expect("reverted");
        assert_eq!(changed.library.pending_count, 0);
        backend.set_enabled(&first, false).expect("off again");
        let changed = backend.discard_pending().expect("discarded");
        assert_eq!(changed.library.pending_count, 0);
    }

    #[test]
    fn what_was_applied_after_the_game_closed_can_be_undone_until_the_next_change() {
        let f = Fixture::new("undoauto");
        let game = with_game(&f);
        let (backend, first, _) = two_applied(&f, &game);
        game.running.store(true, Ordering::Relaxed);
        backend.set_enabled(&first, false).expect("off");
        game.running.store(false, Ordering::Relaxed);
        backend.game_closed().expect("applied").expect("changes");
        let pending = backend.pending().expect("pending");
        let applied = pending.applied_after_game.expect("undo offered");
        assert_eq!(applied.changes.len(), 1);
        assert!(pending.view.changes.is_empty());

        // 147: the Undo is an edit; while the game runs again it waits and is marked as the Undo.
        game.running.store(true, Ordering::Relaxed);
        let changed = backend.undo_applied().expect("undone");
        assert!(changed.waiting_for_game);
        let pending = backend.pending().expect("pending");
        assert_eq!(pending.undoing, Some(applied.at_unix_ms));
        assert_eq!(pending.view.changes.len(), 1);
        assert!(pending.applied_after_game.is_none());
        assert!(backend.undo_applied().is_err(), "only once");

        // The next change ends it.
        backend.set_enabled(&first, false).expect("off");
        assert!(backend.pending().expect("pending").undoing.is_none());
    }

    #[test]
    fn another_change_ends_the_undo_of_an_automatic_apply() {
        let f = Fixture::new("undoends");
        let game = with_game(&f);
        let (backend, first, second) = two_applied(&f, &game);
        game.running.store(true, Ordering::Relaxed);
        backend.set_enabled(&first, false).expect("off");
        game.running.store(false, Ordering::Relaxed);
        backend.game_closed().expect("applied");
        backend.set_enabled(&second, false).expect("another change");
        assert!(
            backend
                .pending()
                .expect("pending")
                .applied_after_game
                .is_none()
        );
    }

    fn foreign_pak(f: &Fixture) {
        std::fs::create_dir_all(f.addons()).expect("addons");
        std::fs::write(
            f.addons().join("pak01_dir.vpk"),
            vpk("models/heroes/haze/body.vmdl_c"),
        )
        .expect("pak");
    }

    #[test]
    fn foreign_files_are_listed_left_and_taken_under_management() {
        let f = Fixture::new("foreign");
        let game = with_game(&f);
        let backend = f.backend(&game);
        foreign_pak(&f);
        std::fs::write(f.addons().join("notes.txt"), b"x").expect("notes");
        let report = backend.foreign_files().expect("listed");
        assert_eq!(
            report.root.as_deref(),
            Some(f.game().display().to_string().as_str())
        );
        assert_eq!(report.items.len(), 2);
        let report = backend.leave_foreign("notes.txt").expect("left");
        assert!(report.items.iter().any(|i| i.name == "notes.txt" && i.left));

        game.running.store(true, Ordering::Relaxed);
        assert_eq!(
            backend
                .take_foreign("pak01_dir.vpk", "Haze")
                .expect_err("running")
                .id,
            MessageId::E13
        );
        game.running.store(false, Ordering::Relaxed);
        let taken = backend
            .take_foreign("pak01_dir.vpk", "Haze")
            .expect("taken");
        assert!(
            taken
                .changed
                .library
                .mods
                .iter()
                .any(|m| m.id == taken.mod_id && m.enabled)
        );
        assert!(
            !backend
                .foreign_files()
                .expect("listed")
                .items
                .iter()
                .any(|i| i.name == "pak01_dir.vpk")
        );
    }

    #[test]
    fn the_start_notices_name_foreign_files_once() {
        let f = Fixture::new("foreignnotice");
        let game = with_game(&f);
        foreign_pak(&f);
        let backend = f.backend(&game);
        assert_eq!(
            backend.take_notices(),
            [crate::events::Notice::Foreign { files: 1 }]
        );
        assert!(backend.take_notices().is_empty());
    }

    #[test]
    fn the_start_counts_foreign_files_not_left_as_they_are() {
        let f = Fixture::new("foreigncount");
        let game = with_game(&f);
        let backend = f.backend(&game);
        assert_eq!(backend.foreign_to_report(), 0);
        foreign_pak(&f);
        std::fs::write(f.addons().join("notes.txt"), b"x").expect("notes");
        assert_eq!(backend.foreign_to_report(), 2);
        backend.leave_foreign("notes.txt").expect("left");
        assert_eq!(backend.foreign_to_report(), 1);
    }

    fn dmm_gameinfo() -> String {
        VANILLA.replace(
            "\t\t\tGame\t\t\t\tcore\r\n",
            "\t\t\tGame\t\t\t\tcore\r\n\t\t\tGame\tcitadel/addons/dmm\r\n",
        )
    }

    #[test]
    fn gameinfo_changed_by_another_program_comes_with_its_differences() {
        let f = Fixture::new("gameinfo");
        let game = with_game(&f);
        let backend = f.backend(&game);
        let status = backend.gameinfo_status(false).expect("status");
        assert_eq!(status.state, GameinfoState::Vanilla);
        assert!(status.diff.is_none());
        let status = backend.gameinfo_status(true).expect("fixed");
        assert_eq!(status.state, GameinfoState::Ours);

        let file = f.game().join("game").join("citadel").join("gameinfo.gi");
        std::fs::write(&file, dmm_gameinfo()).expect("dmm");
        let status = backend.gameinfo_status(true).expect("status");
        assert_eq!(status.state, GameinfoState::Foreign);
        assert!(status.diff.expect("diff").found.iter().any(|l| l.marked));
        assert_eq!(
            status.path.as_deref(),
            Some(file.display().to_string().as_str())
        );

        let none = Fixture::new("gameinfonone");
        let missing = none
            .backend(&FakeGame::default())
            .gameinfo_status(false)
            .expect("status");
        assert_eq!(missing.state, GameinfoState::Missing);
    }

    #[test]
    fn opening_files_goes_only_to_places_the_core_knows() {
        let f = Fixture::new("shellplaces");
        let game = with_game(&f);
        let backend = f.backend(&game);
        foreign_pak(&f);
        let outcomes = backend
            .import_paths(&[f.mod_file("haze.vpk", "models/haze/body.vmdl_c")])
            .expect("imported");
        let id = outcomes[0].as_ref().expect("ok").mod_id.clone();
        let gi = f.game().join("game").join("citadel").join("gameinfo.gi");
        backend.show(&Place::Gameinfo, true).expect("notepad");
        backend.show(&Place::Gameinfo, false).expect("folder");
        backend
            .show(
                &Place::Foreign {
                    name: "pak01_dir.vpk".into(),
                },
                false,
            )
            .expect("foreign");
        backend
            .show(&Place::ModFiles { mod_id: id }, false)
            .expect("mod");
        backend.verify_game_files().expect("steam");
        let shown = game.shown.lock().expect("lock").clone();
        assert_eq!(shown[0], ShellRequest::Notepad(gi.clone()));
        assert_eq!(shown[1], ShellRequest::Reveal(gi));
        assert_eq!(
            shown[2],
            ShellRequest::Reveal(f.addons().join("pak01_dir.vpk"))
        );
        assert!(
            matches!(&shown[3], ShellRequest::Reveal(p) if p.starts_with(f.dir.join("deadloader-data")))
        );
        assert_eq!(shown[4], ShellRequest::VerifyGameFiles);
        for refused in ["../../notepad.exe", "sub/pak.vpk", "missing.vpk"] {
            assert!(
                backend
                    .show(
                        &Place::Foreign {
                            name: refused.into()
                        },
                        false
                    )
                    .is_err(),
                "{refused}"
            );
        }
        assert_eq!(game.shown.lock().expect("lock").len(), 5);
    }

    #[test]
    fn rollback_takes_the_selected_profile_out_of_its_game_folder() {
        let f = Fixture::new("rollback");
        let game = FakeGame {
            installs: vec![f.game()],
            ..FakeGame::default()
        };
        let backend = f.backend(&game);
        backend
            .import_paths(&[f.mod_file("haze.vpk", "models/haze/body.vmdl_c")])
            .expect("imported");
        backend.apply().expect("applied");
        let preview = backend.rollback_preview().expect("preview");
        assert_eq!(preview.files, 1);
        game.running.store(true, Ordering::Relaxed);
        assert_eq!(
            backend.rollback(false).expect_err("running").id,
            MessageId::E13
        );
        game.running.store(false, Ordering::Relaxed);
        let done = backend.rollback(false).expect("rolled back");
        assert_eq!(done.removed, 1);
        assert!(!f.addons().join("pak01_dir.vpk").exists());
        assert!(backend.library().expect("library").pending);
    }

    #[test]
    fn rollback_without_a_game_says_deadlock_was_not_found() {
        let f = Fixture::new("rollbacknogame");
        let backend = f.backend(&FakeGame::default());
        assert_eq!(
            backend.rollback_preview().expect_err("no game").id,
            MessageId::E2
        );
    }

    #[test]
    fn a_folder_added_by_hand_is_checked_and_listed() {
        let f = Fixture::new("addinstall");
        let backend = f.backend(&FakeGame::default());
        let root = f.game();
        let added = backend
            .add_installation(&root.join("game").join("citadel"))
            .expect("added");
        assert_eq!(added, root);
        assert_eq!(
            backend
                .settings()
                .expect("settings")
                .game
                .manual_installations,
            [root.display().to_string()]
        );
        // Adding it again doesn't list it twice; a folder without Deadlock is refused.
        backend.add_installation(&root).expect("again");
        assert_eq!(backend.installations().expect("list").len(), 1);
        assert_eq!(
            backend.add_installation(&f.dir).expect_err("no game").id,
            MessageId::E2
        );
        backend
            .remove_installation(&root.display().to_string())
            .expect("removed");
        assert!(backend.installations().expect("list").is_empty());
    }

    #[test]
    fn the_notice_records_its_answers_and_local_mode_keeps_the_update_check_off() {
        let f = Fixture::new("notice");
        let backend = f.backend(&FakeGame::default());
        let local = backend
            .acknowledge_notice(1, Lang::De, CatalogChoice::LocalMode, true)
            .expect("local");
        assert!(local.is_local_mode());
        assert!(!local.general.check_app_updates, "157");
        let notice = local.acknowledgements.notice.expect("recorded");
        assert_eq!((notice.version, notice.language), (1, Lang::De));
        let on = backend.choose_catalog(true).expect("catalog");
        assert!(!on.is_local_mode());
        assert_eq!(
            on.acknowledgements.notice.expect("kept").catalog,
            CatalogChoice::Catalog
        );
    }

    #[test]
    fn a_reset_and_an_advanced_save_reach_the_listeners() {
        let f = Fixture::new("resetlisteners");
        let backend = f.backend(&FakeGame::default());
        let seen = Arc::new(std::sync::Mutex::new(Vec::new()));
        let record = Arc::clone(&seen);
        backend.on_settings(Box::new(move |s: &Settings| {
            record.lock().expect("lock").push(s.network.retries);
        }));
        let mut settings = backend.settings().expect("settings");
        settings.network.retries = 2;
        backend.update_settings(settings).expect("saved");
        let text = backend
            .settings_text()
            .expect("text")
            .replace("\"retries\": 2", "\"retries\": 3");
        assert!(backend.save_settings_text(&text).expect("ok").is_ok());
        assert!(
            backend
                .save_settings_text("{ broken")
                .expect("answered")
                .is_err()
        );
        let (saved, after) = backend.reset_settings().expect("reset");
        assert!(saved.backup.is_some());
        assert_eq!(after.network.retries, 5);
        assert_eq!(*seen.lock().expect("lock"), [2, 3, 5]);
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

    #[test]
    fn favourites_are_kept_between_starts() {
        let f = Fixture::new("favourites");
        let backend = f.backend(&FakeGame::default());
        let haze = deadloader_core::favourites::Favourite {
            section: deadloader_core::library::Section::Mod,
            id: 655_209,
            name: "Haze V2".to_owned(),
            author: Some("Nyx".to_owned()),
            root_category: None,
            subcategory: None,
            nsfw: false,
            date_updated: None,
        };
        assert_eq!(backend.favourites().expect("none"), []);
        assert_eq!(
            backend.set_favourite(haze.clone(), true).expect("marked"),
            std::slice::from_ref(&haze)
        );
        drop(backend);
        let backend = f.backend(&FakeGame::default());
        assert_eq!(
            backend.favourites().expect("kept"),
            std::slice::from_ref(&haze)
        );
        assert!(
            backend
                .set_favourite(haze, false)
                .expect("unmarked")
                .is_empty()
        );
    }

    #[test]
    fn the_profiles_screen_sees_every_installation_and_profile() {
        let f = Fixture::new("profilesview");
        let game = FakeGame {
            installs: vec![f.game()],
            ..FakeGame::default()
        };
        let backend = f.backend(&game);
        let manual = f.dir.join("E-Deadlock");
        let mut settings = backend.settings().expect("settings");
        settings.game.manual_installations = vec![manual.display().to_string()];
        backend.update_settings(settings).expect("saved");
        backend.create_profile("Sasha").expect("created");
        backend.apply().expect("applied");

        let view = backend.profiles_view().expect("view");
        // The field names `ui/src/lib/api.ts` relies on.
        let json = serde_json::to_value(&view).expect("json");
        let mut keys: Vec<&String> = json["profiles"][0]
            .as_object()
            .expect("profile")
            .keys()
            .collect();
        keys.sort();
        assert_eq!(
            keys,
            [
                "applied",
                "game_root",
                "last_applied_unix_ms",
                "mod_names",
                "mods",
                "name"
            ]
        );
        assert_eq!(view.selected, "Default");
        assert_eq!(
            view.installations,
            [f.game().display().to_string(), manual.display().to_string()]
        );
        let names: Vec<(&str, bool)> = view
            .profiles
            .iter()
            .map(|p| (p.name.as_str(), p.applied))
            .collect();
        assert_eq!(names, [("Default", true), ("Sasha", false)]);

        let view = backend
            .set_profile_installation("Sasha", &manual.display().to_string())
            .expect("moved");
        let sasha = view
            .profiles
            .iter()
            .find(|p| p.name == "Sasha")
            .expect("sasha");
        assert_eq!(sasha.game_root, Some(manual.display().to_string()));
        assert_eq!(
            backend
                .set_profile_installation("Sasha", r"Z:\Nowhere")
                .expect_err("not a known installation")
                .id,
            MessageId::E2
        );
    }

    #[test]
    fn the_age_confirmation_is_saved_and_heard_by_the_listeners() {
        let f = Fixture::new("age");
        let backend = f.backend(&FakeGame::default());
        let heard = Arc::new(AtomicBool::new(false));
        let record = Arc::clone(&heard);
        backend.on_settings(Box::new(move |s: &Settings| {
            record.store(
                s.acknowledgements.age_confirmed_at.is_some(),
                Ordering::Relaxed,
            );
        }));
        let saved = backend.confirm_age().expect("saved");
        assert!(saved.acknowledgements.age_confirmed_at.is_some());
        assert!(heard.load(Ordering::Relaxed));
    }

    #[test]
    fn deleting_the_applied_profile_takes_its_mods_out_of_the_game() {
        let f = Fixture::new("deleteapplied");
        let game = FakeGame {
            installs: vec![f.game()],
            ..FakeGame::default()
        };
        let backend = f.backend(&game);
        backend
            .import_paths(&[f.mod_file("haze.vpk", "models/haze/body.vmdl_c")])
            .expect("imported");
        backend.apply().expect("applied");
        backend.create_profile("Other").expect("created");
        assert!(f.addons().join("pak01_dir.vpk").exists());

        game.running.store(true, Ordering::Relaxed);
        assert_eq!(
            backend.delete_profile("Default").expect_err("running").id,
            MessageId::E13
        );
        assert_eq!(backend.profile_names().expect("names").len(), 2);

        game.running.store(false, Ordering::Relaxed);
        backend.delete_profile("Default").expect("deleted");
        assert!(!f.addons().join("pak01_dir.vpk").exists());
        assert_eq!(backend.profile_names().expect("names"), ["Other"]);
        // The mod stays in the library.
        assert_eq!(backend.library().expect("library").mods.len(), 1);
    }

    #[test]
    fn a_profile_file_names_itself_in_the_import_result_and_its_errors() {
        let f = Fixture::new("profilefilename");
        let backend = f.backend(&FakeGame::default());
        let file = f.dir.join("night.json");
        backend
            .export_profile_to("Default", &file)
            .expect("exported");
        let imported = backend.import_profile_from(&file).expect("imported");
        assert_eq!(imported.file.as_deref(), Some("night.json"));
        assert_eq!(imported.listed, 0);
        let bad = f.dir.join("skins-backup.zip");
        std::fs::write(&bad, "PK").expect("write");
        let err = backend.import_profile_from(&bad).expect_err("refused");
        assert_eq!(err.id, MessageId::E17);
        assert_eq!(
            err.args.get("file").map(String::as_str),
            Some("skins-backup.zip")
        );
    }
}
