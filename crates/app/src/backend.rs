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
    /// Starts Deadlock the way Steam does.
    fn launch(&self) -> std::io::Result<()>;
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
    /// Told about every saved settings change (the network client, the catalog).
    settings_listeners: Mutex<Vec<SettingsListener>>,
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
            },
            Err(e) => Self {
                session: Mutex::new(None),
                start_error: Some(e.user_message()),
                game,
                log,
                notices: Mutex::new(Vec::new()),
                edits_waiting: AtomicUsize::new(0),
                settings_listeners: Mutex::new(Vec::new()),
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
    pub fn install_download(
        &self,
        archive: &Path,
        download: &deadloader_core::session::Download,
    ) -> Result<Installed, UserMessage> {
        let outcome = self.with(|s| Ok(s.import_download(archive, download)?))?;
        let changed = if outcome.choices.is_empty() {
            Some(self.follow_up_edit()?)
        } else {
            None
        };
        Ok(Installed { outcome, changed })
    }

    /// A downloaded update of the GameBanana file `replaced` (D5, 169): imported without enabling it, then applied
    /// at once when every old file has a clear partner, otherwise handed to the user.
    pub fn install_update_file(
        &self,
        archive: &Path,
        download: &deadloader_core::session::Download,
        replaced: u64,
    ) -> Result<UpdateStep, UserMessage> {
        // The library mod may be a local one linked to this GameBanana mod (F10), not `gb-<id>`.
        let imported = self.with(|s| Ok(s.import_update_file(archive, download)?))?;
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
        pub launched: Arc<std::sync::atomic::AtomicUsize>,
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
        fn launch(&self) -> std::io::Result<()> {
            self.launched.fetch_add(1, Ordering::Relaxed);
            Ok(())
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
            .install_download(&f.mod_file("17.bin", "models/haze/body.vmdl_c"), &download)
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
            .install_download(&f.mod_file("1.bin", "models/haze/body.vmdl_c"), &haze(1))
            .expect("v1");
        let step = backend
            .install_update_file(&f.mod_file("2.bin", "models/haze/body.vmdl_c"), &haze(2), 1)
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
            .install_download(&f.mod_file("1.bin", "models/haze/body.vmdl_c"), &haze(1))
            .expect("v1");
        backend
            .install_download(&f.mod_file("1b.bin", "models/haze/hands.vmdl_c"), &haze(1))
            .expect("v1, second file");
        let step = backend
            .install_update_file(&f.mod_file("2.bin", "models/haze/legs.vmdl_c"), &haze(2), 1)
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
