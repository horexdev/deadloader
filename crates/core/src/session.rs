//! What the app works with: deadloader's data folder opened, the library as the Library screen shows it, and the
//! user's actions on it (S4, flows F2, F3, F6). Every change is saved before the call returns; the app layer only
//! translates calls and results.
//!
//! The folder layout is ADR 0003's. The library's VPK directories are read once when the session opens and after each
//! import; variant groups and conflicts are computed from them.

use crate::addons::{AppliedFile, ForeignKind, Origin, scan};
use crate::apply::{Direction, Wanted, layout, slot_of};
use crate::engine::{Engine, EngineError, same_root};
use crate::fs::{Counted, Disk, Logged};
use crate::gamedir::{GuardOutcome, addons_path, ensure_mods_load};
use crate::import::{ImportError, ImportRequest, NoHooks, import, limits_from};
use crate::journal::{self, FileJournal, Recovery, Step};
use crate::library::{
    InstallationState, Manifest, ModelError, Profile, ProfileEntry, Section, Source, load_manifest,
    save_manifest,
};
use crate::log::{FileEvent, FileLog, FileOp, Outcome};
use crate::message::{Area, MessageId, ToUserMessage, UserMessage, data_file, from_io};
use crate::overlap::{FileIndex, Grouping, conflicts, group};
use crate::pending::{self, Aspect, InGame, PendingChange};
use crate::profiles::{self, ProfileError};
use crate::remove::{
    self as removal, Delete, Permanently, RemovalPlan, RemoveError, Removed, Target,
    forget_missing, sweep,
};
use crate::selection;
use crate::settings::{self, Settings, SettingsError, Warning};
use crate::updates::{self, ConfirmedUpdate, OverlapOutcome, UpdateError};
use crate::vpk::read_index;
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Where the session reports file operations (the files log).
pub type SharedLog = Arc<dyn FileLog + Send + Sync>;

/// What linking a mod to its GameBanana page found, per archive of the mod (F10, 171).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LinkReport {
    pub mod_id: String,
    pub archives: Vec<LinkedArchive>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LinkedArchive {
    pub archive_name: String,
    /// The library files extracted from it.
    pub files: Vec<String>,
    /// The GameBanana file it is; `None` when nothing matched (link it by hand).
    pub remote_file_id: Option<u64>,
    pub matched_by: Option<LinkMatch>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LinkMatch {
    /// The same bytes: certain.
    Md5,
    /// The same name (ignoring case, and GameBanana's `_xxxxx` suffix): likely.
    Name,
}

/// The one listed file an archive is: by md5, else by name. Several equally good ones are left to the user.
fn match_listed(
    listed: &[updates::RemoteFile],
    md5: Option<&str>,
    name: &str,
) -> Option<(u64, LinkMatch)> {
    let unique = |hits: Vec<&updates::RemoteFile>| match hits.as_slice() {
        [one] => Some(one.id),
        _ => None,
    };
    if let Some(md5) = md5 {
        let hits = listed
            .iter()
            .filter(|l| {
                l.md5
                    .as_deref()
                    .is_some_and(|m| m.eq_ignore_ascii_case(md5))
            })
            .collect();
        if let Some(id) = unique(hits) {
            return Some((id, LinkMatch::Md5));
        }
    }
    let same = |a: &str, b: &str| a.to_lowercase() == b.to_lowercase();
    let exact = listed
        .iter()
        .filter(|l| same(&l.file_name, name))
        .collect::<Vec<_>>();
    if !exact.is_empty() {
        return unique(exact).map(|id| (id, LinkMatch::Name));
    }
    let bare = updates::without_suffix(name);
    let hits = listed
        .iter()
        .filter(|l| same(&updates::without_suffix(&l.file_name), &bare))
        .collect();
    unique(hits).map(|id| (id, LinkMatch::Name))
}

/// What applying the selected profile changes in the game folder, by mod name (D13).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct ApplyPreview {
    pub turned_off: Vec<String>,
    pub turned_on: Vec<String>,
}

/// Where a file of a replaced download goes (D5).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UpdateProposal {
    /// The old library file.
    pub old: String,
    pub proposed: Proposed,
}

impl UpdateProposal {
    /// One clear partner: applied without asking. A tie or no overlap needs the user (169).
    pub fn is_settled(&self) -> bool {
        matches!(
            self.proposed,
            Proposed::Pair { .. } | Proposed::Overlap { .. }
        )
    }

    /// The proposed new library file, when settled.
    pub fn new_file(&self) -> Option<&str> {
        match &self.proposed {
            Proposed::Pair { new } | Proposed::Overlap { new, .. } => Some(new),
            Proposed::Tie { .. } | Proposed::NoOverlap => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Proposed {
    /// The same place in the old and the new archive.
    Pair { new: String },
    /// The new file changing most of the same game files.
    Overlap {
        new: String,
        shared_paths: usize,
        identical: usize,
    },
    /// Several new files score the same: the user picks.
    Tie {
        candidates: Vec<String>,
        shared_paths: usize,
        identical: usize,
    },
    /// No new file changes any game file of the old one: the user maps it, or keeps it.
    NoOverlap,
}

/// What the download queue knows about a verified download: which GameBanana file of which mod it is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Download {
    pub section: Section,
    pub mod_id: u64,
    pub file_id: u64,
    /// The file's name on GameBanana.
    pub file_name: String,
    pub mod_name: String,
    pub author: Option<String>,
    pub version: Option<String>,
}

/// Something the user should hear about after start (a toast or banner).
#[derive(Debug)]
pub enum Notice {
    /// Settings were corrected while loading (unknown fields, values out of range).
    SettingsCorrected(Vec<Warning>),
    /// `settings.json` could not be read: defaults are used and the file is left as it is (E23).
    SettingsUnreadable(UserMessage),
    /// An interrupted change to the game folder was finished or undone (N5).
    Recovered(Recovery),
    /// Library files nothing referred to were deleted (left by a crash or a failed delete).
    Swept(usize),
    /// Profile files that could not be read; they are left as they are.
    BrokenProfiles(Vec<String>),
    /// Library files that are missing or no longer readable VPKs (E14).
    DamagedFiles(Vec<String>),
}

#[derive(Debug)]
pub struct Opened {
    pub session: Session,
    pub notices: Vec<Notice>,
}

#[derive(Debug)]
pub enum SessionError {
    /// `manifest.json` can't be read; the start stops rather than risk losing the library (E23).
    Manifest(ModelError),
    Settings(SettingsError),
    Profile(ProfileError),
    Engine(EngineError),
    Import(ImportError),
    Remove(RemoveError),
    UnknownMod(String),
    UnknownFile(String),
    Update(UpdateError),
    /// Linking files to GameBanana needs a mod linked to a GameBanana page first.
    NotLinked(String),
    /// A mod name can't be empty.
    EmptyName,
    /// The game folder has no `gameinfo.gi` (moved, uninstalled, or its drive is gone): nothing is written there (E2).
    GameNotFound(PathBuf),
    /// Only a self-contained `pakNN_dir.vpk` of another tool can be taken under management (D11).
    NotTakeable(String),
    /// The name is not a foreign item of the game's mod folder (F9).
    NotForeign(String),
    Io(io::Error),
}

impl std::fmt::Display for SessionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Manifest(e) => write!(f, "manifest.json: {e}"),
            Self::Settings(e) => write!(f, "settings.json: {e}"),
            Self::Profile(e) => write!(f, "{e}"),
            Self::Engine(e) => write!(f, "{e}"),
            Self::Import(e) => write!(f, "{e}"),
            Self::Remove(e) => write!(f, "{e}"),
            Self::UnknownMod(id) => write!(f, "the mod \"{id}\" is not in the library"),
            Self::UnknownFile(id) => write!(f, "the file \"{id}\" is not in the library"),
            Self::Update(e) => write!(f, "{e}"),
            Self::NotLinked(id) => write!(f, "the mod \"{id}\" is not linked to a GameBanana page"),
            Self::EmptyName => f.write_str("a mod name can't be empty"),
            Self::GameNotFound(root) => {
                write!(f, "no Deadlock installation in {}", root.display())
            }
            Self::NotForeign(name) => {
                write!(f, "{name} is not a foreign item of the game's mod folder")
            }
            Self::NotTakeable(name) => {
                write!(
                    f,
                    "{name} is not a mod file deadloader can take under management"
                )
            }
            Self::Io(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for SessionError {}

impl ToUserMessage for SessionError {
    fn user_message(&self) -> UserMessage {
        match self {
            Self::Manifest(e) => data_file(e, "manifest.json"),
            Self::Settings(e) => e.user_message(),
            Self::Profile(e) => e.user_message(),
            Self::Engine(e) => e.user_message(),
            Self::Import(e) => e.user_message(),
            Self::Remove(e) => e.user_message(),
            Self::Io(e) => from_io(e, Area::Data),
            Self::GameNotFound(root) => UserMessage::new(MessageId::E2)
                .arg("path", root.display())
                .arg("detail", self),
            Self::Update(e) => e.user_message(),
            Self::UnknownMod(_)
            | Self::UnknownFile(_)
            | Self::NotLinked(_)
            | Self::EmptyName
            | Self::NotTakeable(_)
            | Self::NotForeign(_) => UserMessage::new(MessageId::Internal).arg("detail", self),
        }
    }
}

impl From<ProfileError> for SessionError {
    fn from(e: ProfileError) -> Self {
        Self::Profile(e)
    }
}

impl From<ModelError> for SessionError {
    fn from(e: ModelError) -> Self {
        Self::Profile(ProfileError::Model(e))
    }
}

impl From<io::Error> for SessionError {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

/// The Library screen's content for the selected profile (S4).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LibraryView {
    pub selected_profile: String,
    /// The profile last applied to the game folder this profile uses, if any.
    pub applied_profile: Option<String>,
    /// The selected profile differs from what is in that game folder.
    pub pending: bool,
    /// How many pending changes S9 lists: 1 for a profile switch, else one per mod and aspect (the sidebar's count).
    pub pending_count: usize,
    /// The profile's mods in priority order (first wins), then library mods it doesn't use, switched off.
    pub mods: Vec<ModView>,
    pub conflicts: Vec<ConflictView>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ModView {
    pub id: String,
    pub name: String,
    pub author: Option<String>,
    pub source: Source,
    pub enabled: bool,
    pub files: Vec<FileView>,
    /// Files that change the same game files; one of each group is active.
    pub variant_groups: Vec<VariantGroupView>,
    /// Files overlapping nothing else in the mod (parts / add-ons) and whether each is on.
    pub add_ons: Vec<(String, bool)>,
    pub bytes: u64,
    /// Some change of this mod waits to be applied (S4's "Pending" badge).
    pub pending: bool,
    /// Files of this mod are in the game folder now (`None`: none are).
    pub in_game: Option<InGameView>,
}

/// How a mod's files are in the game folder: hard links take no extra space; copies do (124).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct InGameView {
    pub copied: bool,
    /// The size of its files there; extra disk space when `copied`.
    pub bytes: u64,
}

/// The pending-changes panel (S9).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PendingView {
    /// The selected profile isn't the one in its game folder: it waits for Apply or Play (138), shown as one change.
    pub switch: Option<ProfileSwitch>,
    /// Edits of the applied profile not in the game yet (while it runs, or after a failed apply).
    pub changes: Vec<PendingChange>,
    /// What applying does to the game folder.
    pub folder: FolderSummary,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProfileSwitch {
    /// The profile in the game folder now; `None` when deadloader never applied one there.
    pub from: Option<String>,
    pub to: String,
    pub turned_on: usize,
    pub turned_off: usize,
}

/// Applying's effect on the game's `addons` folder, in files.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct FolderSummary {
    pub added: usize,
    pub removed: usize,
    /// Files that stay but move to another `pakNN` slot.
    pub renumbered: usize,
    pub addons: Option<String>,
}

/// An item of the game's `addons` folder deadloader didn't put there (decision 16, F9).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ForeignView {
    pub name: String,
    pub kind: ForeignKind,
    pub slot: Option<u8>,
    /// A folder: everything in it.
    pub bytes: u64,
    /// A folder: how many files it holds, at every level (counted up to a limit).
    pub files: Option<u64>,
    pub modified_unix_ms: Option<i64>,
    pub origin: Option<Origin>,
    /// What a readable mod file contains.
    pub vpk: Option<VpkSummary>,
    /// D11 can take it: a self-contained `pakNN_dir.vpk`.
    pub takeable: bool,
    /// The user chose to leave it as it is.
    pub left: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct VpkSummary {
    pub entries: usize,
    /// The hero folder most of its files are in (the game's internal name), when there is one.
    pub hero: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FileView {
    pub id: String,
    pub archive_name: String,
    pub inner_path: String,
    pub version: Option<String>,
    pub len: u64,
    /// Missing from the library or no longer readable (E14).
    pub damaged: bool,
    /// The newest library file that replaced this one through confirmed updates; the UI hints at it when this old
    /// version is the active one (170).
    pub newer_in_library: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct VariantGroupView {
    pub files: Vec<String>,
    pub active: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ConflictView {
    pub winner_mod: String,
    pub winner_file: String,
    pub loser_mod: String,
    pub loser_file: String,
    pub paths: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ImportOutcome {
    pub mod_id: String,
    pub new_files: Vec<String>,
    /// Variant groups the user must pick from (D4); empty means the mod was enabled at once.
    pub choices: Vec<Vec<String>>,
    /// Archive entries that are not mod files (shown in the task details).
    pub skipped: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProfileImported {
    pub name: String,
    /// Mods and files this library doesn't have (D9).
    pub missing: Vec<profiles::Missing>,
}

#[derive(Debug)]
pub struct Applied {
    pub changed: bool,
    pub gameinfo: GuardOutcome,
    pub recovered: Recovery,
    pub foreign: usize,
}

pub struct Session {
    data_dir: PathBuf,
    log: SharedLog,
    manifest: Manifest,
    profiles: Vec<Profile>,
    settings: Settings,
    /// `settings.json` was unreadable: never overwrite the user's file.
    settings_locked: bool,
    selected: String,
    indexes: HashMap<String, FileIndex>,
    damaged: HashSet<String>,
    direction: Direction,
}

impl std::fmt::Debug for Session {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Session")
            .field("data_dir", &self.data_dir)
            .field("selected", &self.selected)
            .field("mods", &self.manifest.mods.len())
            .finish_non_exhaustive()
    }
}

/// Deletes through `inner` (permanently or to the recycle bin) and reports each deletion.
struct LoggedDelete<'a> {
    inner: &'a dyn Delete,
    log: &'a (dyn FileLog + Send + Sync),
}

impl Delete for LoggedDelete<'_> {
    fn delete(&self, path: &Path) -> io::Result<()> {
        let result = self.inner.delete(path);
        let mut event = FileEvent {
            src: Some(path.display().to_string()),
            ..FileEvent::new(FileOp::Delete)
        };
        if let Err(e) = &result {
            event.result = Outcome::Failed;
            event.error = Some(e.to_string());
        }
        self.log.record("library file deleted", &event);
        result
    }
}

impl Session {
    pub fn open(data_dir: &Path, log: SharedLog) -> Result<Opened, SessionError> {
        std::fs::create_dir_all(data_dir)?;
        let mut notices = Vec::new();
        let (settings, settings_locked) = match settings::load(&data_dir.join("settings.json")) {
            Ok(loaded) => {
                if !loaded.warnings.is_empty() {
                    notices.push(Notice::SettingsCorrected(loaded.warnings));
                }
                (loaded.settings, false)
            }
            Err(e) => {
                notices.push(Notice::SettingsUnreadable(e.user_message()));
                (Settings::default(), true)
            }
        };
        let mut manifest =
            load_manifest(&data_dir.join("manifest.json")).map_err(SessionError::Manifest)?;

        let direction = Direction::default();
        let journal = FileJournal::new(data_dir.join("journal.jsonl"));
        let fs = Logged {
            fs: &Disk,
            log: &*log,
        };
        let engine = Engine {
            fs: &fs,
            journal: &journal,
            data_dir,
            direction,
        };
        let manifest_path = data_dir.join("manifest.json");
        let recovered = engine
            .recover(&mut manifest, |m| save_io(&manifest_path, m))
            .map_err(SessionError::Engine)?;
        if recovered != Recovery::Clean {
            notices.push(Notice::Recovered(recovered));
        }

        let profiles_dir = data_dir.join("profiles");
        profiles::ensure_default(&profiles_dir)?;
        let listed = profiles::list(&profiles_dir)?;
        if !listed.broken.is_empty() {
            notices.push(Notice::BrokenProfiles(
                listed.broken.into_iter().map(|(name, _)| name).collect(),
            ));
        }
        let selected = settings
            .ui
            .last_selected_profile
            .clone()
            .filter(|name| listed.profiles.iter().any(|p| &p.name == name))
            .or_else(|| {
                listed
                    .profiles
                    .iter()
                    .find(|p| p.name == profiles::DEFAULT_NAME)
                    .or(listed.profiles.first())
                    .map(|p| p.name.clone())
            })
            .unwrap_or_else(|| profiles::DEFAULT_NAME.to_owned());

        let swept = sweep(
            data_dir,
            &manifest,
            &LoggedDelete {
                inner: &Permanently,
                log: &*log,
            },
        )?;
        if !swept.is_empty() {
            notices.push(Notice::Swept(swept.len()));
        }

        let mut session = Session {
            data_dir: data_dir.to_path_buf(),
            log,
            manifest,
            profiles: listed.profiles,
            settings,
            settings_locked,
            selected,
            indexes: HashMap::new(),
            damaged: HashSet::new(),
            direction,
        };
        let ids: Vec<String> = session
            .manifest
            .mods
            .iter()
            .flat_map(|m| &m.files)
            .map(|f| f.id.clone())
            .collect();
        session.index(&ids);
        if !session.damaged.is_empty() {
            let mut damaged: Vec<String> = session.damaged.iter().cloned().collect();
            damaged.sort();
            notices.push(Notice::DamagedFiles(damaged));
        }
        Ok(Opened { session, notices })
    }

    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    pub fn manifest(&self) -> &Manifest {
        &self.manifest
    }

    pub fn selected_profile(&self) -> &str {
        &self.selected
    }

    pub fn profile_names(&self) -> Vec<String> {
        self.profiles.iter().map(|p| p.name.clone()).collect()
    }

    /// The game folder the selected profile uses: its own (set when it was applied), else the one it was last
    /// applied to. `None` until the user picks or applies one.
    pub fn game_root(&self) -> Option<PathBuf> {
        self.profile()
            .and_then(|p| p.game_root.clone())
            .or_else(|| self.installation().map(|i| i.game_root.clone()))
            .map(PathBuf::from)
    }

    pub fn library(&self) -> LibraryView {
        let profile = self.profile();
        let mut mods = Vec::new();
        let mut shown = HashSet::new();
        let empty = Vec::new();
        for entry in profile.map(|p| &p.entries).unwrap_or(&empty) {
            if let Some(view) = self.mod_view(&entry.mod_id, Some(entry)) {
                shown.insert(entry.mod_id.clone());
                mods.push(view);
            }
        }
        for m in &self.manifest.mods {
            if !shown.contains(&m.id)
                && let Some(view) = self.mod_view(&m.id, None)
            {
                mods.push(view);
            }
        }
        let wanted = self.wanted_files();
        let owner: HashMap<&str, &str> = self
            .manifest
            .mods
            .iter()
            .flat_map(|m| m.files.iter().map(move |f| (f.id.as_str(), m.id.as_str())))
            .collect();
        let ordered: Vec<FileIndex> = wanted
            .iter()
            .filter_map(|id| self.indexes.get(id).cloned())
            .collect();
        let conflicts = conflicts(&ordered)
            .into_iter()
            .filter_map(|c| {
                let (wm, lm) = (owner.get(c.winner.as_str())?, owner.get(c.loser.as_str())?);
                (wm != lm).then(|| ConflictView {
                    winner_mod: (*wm).to_owned(),
                    winner_file: c.winner,
                    loser_mod: (*lm).to_owned(),
                    loser_file: c.loser,
                    paths: c.paths,
                })
            })
            .collect();
        let installation = self.installation();
        // A profile without a game folder of its own goes to the first one (as Apply does with Steam's first).
        let target = installation.or_else(|| {
            self.profile()
                .filter(|p| p.game_root.is_none())
                .and_then(|_| self.manifest.installations.first())
        });
        let (switch, changes) = self.pending_parts(target);
        for m in &mut mods {
            m.pending = changes.iter().any(|c| c.mod_id == m.id);
        }
        LibraryView {
            selected_profile: self.selected.clone(),
            applied_profile: installation.and_then(|i| i.applied_profile.clone()),
            pending: self.pending(&wanted),
            pending_count: if switch.is_some() { 1 } else { changes.len() },
            mods,
            conflicts,
        }
    }

    /// Imports a `.vpk` / `.zip` / `.7z` / `.rar` from disk as a new local mod (decision 134) and enables it on top
    /// of the selected profile, unless the user has to pick variants first (then call [`Session::choose`]).
    pub fn import_file(&mut self, path: &Path, name: &str) -> Result<ImportOutcome, SessionError> {
        let mod_id = self.free_local_id();
        let req = ImportRequest {
            source: path.to_path_buf(),
            mod_id,
            name: name.to_owned(),
            mod_source: Source::Local {
                imported_from: Some(path.display().to_string()),
            },
            version: None,
            remote_file_id: None,
            archive_name: None,
        };
        self.import_request(&req, None, true)
    }

    /// A verified download from the catalog (F2): its files join the library mod of that GameBanana mod, a new one
    /// the first time (144), with the GameBanana file id and name for update detection (164). Then as
    /// [`Session::import_file`]: enabled on top (136), unless the user has to pick variants first.
    pub fn import_download(
        &mut self,
        path: &Path,
        download: &Download,
    ) -> Result<ImportOutcome, SessionError> {
        let req = self.download_request(path, download);
        self.import_request(&req, download.author.as_deref(), true)
    }

    /// A downloaded update (D5): its files join the mod but nothing is enabled; [`Session::propose_update`] and
    /// [`Session::apply_update`] put them where the old files were. Returns the new library file ids.
    pub fn import_update_file(
        &mut self,
        path: &Path,
        download: &Download,
    ) -> Result<ImportOutcome, SessionError> {
        let req = self.download_request(path, download);
        self.import_request(&req, download.author.as_deref(), false)
    }

    /// The library mod of a GameBanana mod: one linked to it already (a local mod linked in F10), else `gb-<id>`.
    fn download_request(&self, path: &Path, download: &Download) -> ImportRequest {
        let source = Source::GameBanana {
            section: download.section,
            mod_id: download.mod_id,
        };
        let mod_id = self
            .manifest
            .mods
            .iter()
            .find(|m| m.source == source)
            .map(|m| m.id.clone())
            .unwrap_or_else(|| match download.section {
                Section::Mod => format!("gb-{}", download.mod_id),
                Section::Sound => format!("gb-sound-{}", download.mod_id),
            });
        ImportRequest {
            source: path.to_path_buf(),
            mod_id,
            name: download.mod_name.clone(),
            mod_source: Source::GameBanana {
                section: download.section,
                mod_id: download.mod_id,
            },
            version: download.version.clone(),
            remote_file_id: Some(download.file_id),
            archive_name: Some(download.file_name.clone()),
        }
    }

    /// Which new library file holds the newer version of each file of the replaced GameBanana file (step 2 of
    /// decision 164): by place in the archive first, then by the game files they change. Only a settled proposal
    /// for every file may be applied without asking (169).
    pub fn propose_update(
        &self,
        mod_id: &str,
        replaced_remote_id: u64,
        new_files: &[String],
    ) -> Result<Vec<UpdateProposal>, SessionError> {
        let m = self
            .manifest
            .mods
            .iter()
            .find(|m| m.id == mod_id)
            .ok_or_else(|| SessionError::UnknownMod(mod_id.to_owned()))?;
        let old: Vec<&crate::library::ModFile> = m
            .files
            .iter()
            .filter(|f| {
                f.remote_file_id == Some(replaced_remote_id)
                    && !matches!(f.successor, Some(crate::library::Successor::File(_)))
            })
            .collect();
        let mut new: Vec<&crate::library::ModFile> = Vec::new();
        for id in new_files {
            new.push(
                m.files
                    .iter()
                    .find(|f| &f.id == id)
                    .ok_or_else(|| SessionError::UnknownFile(id.clone()))?,
            );
        }
        let pairing = updates::pair_within_archive(&old, &new);
        let mut proposals: Vec<UpdateProposal> = pairing
            .pairs
            .into_iter()
            .map(|p| UpdateProposal {
                old: p.old,
                proposed: Proposed::Pair { new: p.new },
            })
            .collect();
        let index_of = |id: &String| {
            self.indexes.get(id).cloned().unwrap_or(FileIndex {
                id: id.clone(),
                entries: Vec::new(),
            })
        };
        let old_left: Vec<FileIndex> = pairing.old_left.iter().map(index_of).collect();
        let new_left: Vec<FileIndex> = pairing.new_left.iter().map(index_of).collect();
        proposals.extend(
            updates::match_by_overlap(&old_left, &new_left)
                .into_iter()
                .map(|m| UpdateProposal {
                    old: m.old,
                    proposed: match m.outcome {
                        OverlapOutcome::Match {
                            new,
                            shared_paths,
                            identical,
                        } => Proposed::Overlap {
                            new,
                            shared_paths,
                            identical,
                        },
                        OverlapOutcome::Tie {
                            candidates,
                            shared_paths,
                            identical,
                        } => Proposed::Tie {
                            candidates,
                            shared_paths,
                            identical,
                        },
                        OverlapOutcome::NoOverlap => Proposed::NoOverlap,
                    },
                }),
        );
        Ok(proposals)
    }

    /// A confirmed update (39): the old files stay (22) and point at their successors, and the new files are active
    /// wherever the old ones were (137). The manifest and the changed profiles are saved; nothing changes on error.
    pub fn apply_update(&mut self, update: &ConfirmedUpdate) -> Result<(), SessionError> {
        let mut manifest = self.manifest.clone();
        let mut profiles = self.profiles.clone();
        let changed = updates::apply_update(&mut manifest, &mut profiles, update)
            .map_err(SessionError::Update)?;
        save_manifest(&self.manifest_path(), &manifest)?;
        for &i in &changed {
            profiles::save(&self.profiles_dir(), &profiles[i])?;
        }
        self.manifest = manifest;
        self.profiles = profiles;
        let added: Vec<String> = update.added.iter().map(|f| f.id.clone()).collect();
        self.index(&added);
        Ok(())
    }

    fn import_request(
        &mut self,
        req: &ImportRequest,
        author: Option<&str>,
        install: bool,
    ) -> Result<ImportOutcome, SessionError> {
        let path = &req.source;
        let mod_id = req.mod_id.clone();
        let mut updated = self.manifest.clone();
        let imported = import(
            &self.data_dir,
            &mut updated,
            req,
            // The limits the user set (127), not the defaults.
            &limits_from(&self.settings.downloads),
            &NoHooks,
        )
        .map_err(SessionError::Import)?;
        if let Some(author) = author
            && let Some(m) = updated.mods.iter_mut().find(|m| m.id == mod_id)
            && m.author.is_none()
        {
            m.author = Some(author.to_owned());
        }
        save_manifest(&self.manifest_path(), &updated)?;
        self.manifest = updated;
        for id in &imported.new_files {
            if let Some((_, f)) = self.manifest.file(id) {
                let event = FileEvent {
                    src: Some(format!("{} | {}", path.display(), f.inner_path)),
                    dst: Some(
                        library_path(&self.data_dir, &f.library_path)
                            .display()
                            .to_string(),
                    ),
                    bytes: Some(f.len),
                    sha256: Some(f.sha256),
                    ..FileEvent::new(FileOp::Copy)
                };
                self.log.record("imported into the library", &event);
            }
        }
        self.index(&imported.new_files);
        if !install {
            return Ok(ImportOutcome {
                mod_id,
                new_files: imported.new_files,
                choices: Vec::new(),
                skipped: imported.skipped,
            });
        }
        let choices = selection::choices_needed(&imported.grouping, &imported.new_files);
        if choices.is_empty() {
            let grouping = imported.grouping.clone();
            let new_files = imported.new_files.clone();
            self.edit_profile(|p| selection::install(p, &mod_id, &grouping, &new_files))?;
        }
        Ok(ImportOutcome {
            mod_id,
            new_files: imported.new_files,
            choices,
            skipped: imported.skipped,
        })
    }

    /// F10 + 171: a mod imported from disk is linked to its GameBanana page. Each of its archives is matched to a
    /// listed file by md5, else by name (ignoring case and GameBanana's `_xxxxx` suffix); a matched archive's files get
    /// that file's id, so the update check can follow them. What nothing matches stays unlinked ([`Session::link_files`]).
    pub fn link_mod(
        &mut self,
        mod_id: &str,
        section: Section,
        gamebanana_id: u64,
        listed: &[updates::RemoteFile],
    ) -> Result<LinkReport, SessionError> {
        let mut updated = self.manifest.clone();
        let m = updated
            .mods
            .iter_mut()
            .find(|m| m.id == mod_id)
            .ok_or_else(|| SessionError::UnknownMod(mod_id.to_owned()))?;
        m.source = Source::GameBanana {
            section,
            mod_id: gamebanana_id,
        };
        let mut archives: Vec<LinkedArchive> = Vec::new();
        let mut md5s: Vec<Option<String>> = Vec::new();
        for f in &m.files {
            let key = (f.archive_md5.clone(), f.archive_name.clone());
            match archives
                .iter_mut()
                .zip(&md5s)
                .find(|(a, md5)| (*md5).clone() == key.0 && a.archive_name == key.1)
            {
                Some((a, _)) => a.files.push(f.id.clone()),
                None => {
                    archives.push(LinkedArchive {
                        archive_name: f.archive_name.clone(),
                        files: vec![f.id.clone()],
                        remote_file_id: None,
                        matched_by: None,
                    });
                    md5s.push(key.0);
                }
            }
        }
        for (archive, md5) in archives.iter_mut().zip(&md5s) {
            let found = match_listed(listed, md5.as_deref(), &archive.archive_name);
            if let Some((id, by)) = found {
                archive.remote_file_id = Some(id);
                archive.matched_by = Some(by);
                for f in m.files.iter_mut().filter(|f| archive.files.contains(&f.id)) {
                    f.remote_file_id = Some(id);
                }
            }
        }
        save_manifest(&self.manifest_path(), &updated)?;
        self.manifest = updated;
        Ok(LinkReport {
            mod_id: mod_id.to_owned(),
            archives,
        })
    }

    /// "Link" by hand (171): these files of a linked mod came from GameBanana file `remote_file_id` (`None` unlinks).
    pub fn link_files(
        &mut self,
        mod_id: &str,
        files: &[String],
        remote_file_id: Option<u64>,
    ) -> Result<(), SessionError> {
        let mut updated = self.manifest.clone();
        let m = updated
            .mods
            .iter_mut()
            .find(|m| m.id == mod_id)
            .ok_or_else(|| SessionError::UnknownMod(mod_id.to_owned()))?;
        if !matches!(m.source, Source::GameBanana { .. }) {
            return Err(SessionError::NotLinked(mod_id.to_owned()));
        }
        for id in files {
            let f = m
                .files
                .iter_mut()
                .find(|f| &f.id == id)
                .ok_or_else(|| SessionError::UnknownFile(id.clone()))?;
            f.remote_file_id = remote_file_id;
        }
        save_manifest(&self.manifest_path(), &updated)?;
        self.manifest = updated;
        Ok(())
    }

    /// F10: the name a mod is shown under.
    pub fn rename_mod(&mut self, mod_id: &str, name: &str) -> Result<(), SessionError> {
        let name = name.trim();
        if name.is_empty() {
            return Err(SessionError::EmptyName);
        }
        let mut updated = self.manifest.clone();
        let m = updated
            .mods
            .iter_mut()
            .find(|m| m.id == mod_id)
            .ok_or_else(|| SessionError::UnknownMod(mod_id.to_owned()))?;
        m.name = name.to_owned();
        save_manifest(&self.manifest_path(), &updated)?;
        self.manifest = updated;
        Ok(())
    }

    /// D13: which mods applying the selected profile takes out of the game folder and which it brings in (by name,
    /// in library order). Play asks first when the selected profile isn't the applied one and mods would go (146).
    /// `fallback_root` is where Apply goes for a profile that has no game folder yet (the first one Steam knows).
    pub fn apply_preview(&self, fallback_root: Option<&Path>) -> ApplyPreview {
        let Some(root) = self
            .game_root()
            .or_else(|| fallback_root.map(Path::to_path_buf))
        else {
            return ApplyPreview::default();
        };
        let root = root.display().to_string();
        let in_game: HashSet<&str> = self
            .manifest
            .installations
            .iter()
            .find(|i| i.game_root == root)
            .map(|i| i.applied.iter().map(|r| r.file.as_str()).collect())
            .unwrap_or_default();
        let wanted: HashSet<String> = self.wanted_files().into_iter().collect();
        let mut preview = ApplyPreview::default();
        for m in &self.manifest.mods {
            let now = m.files.iter().any(|f| in_game.contains(f.id.as_str()));
            let then = m.files.iter().any(|f| wanted.contains(&f.id));
            if now && !then {
                preview.turned_off.push(m.name.clone());
            } else if then && !now {
                preview.turned_on.push(m.name.clone());
            }
        }
        preview
    }

    /// "Hide" on "new files available" (168): these GameBanana files of the mod are not reported again.
    pub fn dismiss_new_files(
        &mut self,
        mod_id: &str,
        file_ids: &[u64],
    ) -> Result<(), SessionError> {
        if !self.manifest.mods.iter().any(|m| m.id == mod_id) {
            return Err(SessionError::UnknownMod(mod_id.to_owned()));
        }
        let mut updated = self.manifest.clone();
        let hidden = updated
            .dismissed_new_files
            .entry(mod_id.to_owned())
            .or_default();
        hidden.extend(file_ids);
        hidden.sort_unstable();
        hidden.dedup();
        save_manifest(&self.manifest_path(), &updated)?;
        self.manifest = updated;
        Ok(())
    }

    /// The user's pick in D4: `chosen` become the active variants; the mod's add-ons are enabled with them.
    pub fn choose(&mut self, mod_id: &str, chosen: &[String]) -> Result<(), SessionError> {
        let grouping = self.grouping(mod_id)?;
        for file in chosen {
            if !self.mod_files(mod_id)?.contains(file) {
                return Err(SessionError::UnknownFile(file.clone()));
            }
        }
        let mut files = chosen.to_vec();
        files.extend(grouping.standalone.iter().cloned());
        self.edit_profile(|p| selection::install(p, mod_id, &grouping, &files))
    }

    /// The toggle in the list. A mod the profile doesn't use yet is added on top with its first variant of each
    /// group and all add-ons.
    pub fn set_enabled(&mut self, mod_id: &str, enabled: bool) -> Result<(), SessionError> {
        let grouping = self.grouping(mod_id)?;
        let present = self
            .profile()
            .is_some_and(|p| p.entries.iter().any(|e| e.mod_id == mod_id));
        if present {
            self.edit_profile(|p| {
                selection::set_enabled(p, mod_id, enabled);
            })
        } else if enabled {
            let defaults = default_files(&grouping);
            self.edit_profile(|p| selection::install(p, mod_id, &grouping, &defaults))
        } else {
            Ok(())
        }
    }

    /// Moves a mod to `index` in the priority order (0 = wins over all).
    pub fn move_mod(&mut self, mod_id: &str, index: usize) -> Result<(), SessionError> {
        let grouping = self.grouping(mod_id)?;
        self.edit_profile(|p| {
            let entry = match p.entries.iter().position(|e| e.mod_id == mod_id) {
                Some(i) => p.entries.remove(i),
                None => {
                    let mut e = ProfileEntry {
                        mod_id: mod_id.to_owned(),
                        variants: Vec::new(),
                        add_ons: Vec::new(),
                        enabled: false,
                    };
                    for f in default_files(&grouping) {
                        selection::switch_to(&mut e, &grouping, &f);
                    }
                    e
                }
            };
            let at = index.min(p.entries.len());
            p.entries.insert(at, entry);
        })
    }

    /// "Switch to" a variant, or enable an add-on.
    pub fn switch_variant(&mut self, mod_id: &str, file: &str) -> Result<(), SessionError> {
        let grouping = self.grouping(mod_id)?;
        if !self.mod_files(mod_id)?.iter().any(|f| f == file) {
            return Err(SessionError::UnknownFile(file.to_owned()));
        }
        let present = self
            .profile()
            .is_some_and(|p| p.entries.iter().any(|e| e.mod_id == mod_id));
        if present {
            self.edit_profile(|p| {
                if let Some(e) = p.entries.iter_mut().find(|e| e.mod_id == mod_id) {
                    selection::switch_to(e, &grouping, file);
                }
            })
        } else {
            let file = file.to_owned();
            self.edit_profile(|p| selection::install(p, mod_id, &grouping, &[file]))
        }
    }

    pub fn select_profile(&mut self, name: &str) -> Result<(), SessionError> {
        let found = self
            .profiles
            .iter()
            .find(|p| p.name.to_lowercase() == name.to_lowercase())
            .map(|p| p.name.clone())
            .ok_or_else(|| ProfileError::NotFound(name.to_owned()))?;
        self.selected = found.clone();
        self.settings.ui.last_selected_profile = Some(found);
        self.save_settings()
    }

    pub fn create_profile(&mut self, name: &str) -> Result<(), SessionError> {
        profiles::create(
            &self.profiles_dir(),
            &Profile {
                name: name.to_owned(),
                game_root: None,
                entries: Vec::new(),
            },
        )?;
        self.reload_profiles()
    }

    /// What removing `target` means, for D6.
    pub fn removal_plan(&self, target: &Target) -> Result<RemovalPlan, SessionError> {
        removal::plan(&self.manifest, target, &self.profiles).map_err(SessionError::Remove)
    }

    /// Removes a mod or some of its files (decision 42), through `deleter` (permanently or to the recycle bin), and
    /// drops them from every profile. Files still in a game folder are refused: apply without them first.
    pub fn remove(
        &mut self,
        target: &Target,
        deleter: &dyn Delete,
    ) -> Result<Removed, SessionError> {
        let logged = LoggedDelete {
            inner: deleter,
            log: &*self.log,
        };
        let manifest_path = self.manifest_path();
        let removed = removal::remove(&self.data_dir, &mut self.manifest, target, &logged, |m| {
            save_io(&manifest_path, m)
        })
        .map_err(SessionError::Remove)?;
        for id in &removed.files {
            self.indexes.remove(id);
            self.damaged.remove(id);
        }
        let dir = self.profiles_dir();
        for profile in &mut self.profiles {
            if forget_missing(profile, &self.manifest) {
                profiles::save(&dir, profile)?;
            }
        }
        Ok(removed)
    }

    pub fn rename_profile(&mut self, old: &str, new: &str) -> Result<(), SessionError> {
        let manifest_path = self.manifest_path();
        profiles::rename(&self.profiles_dir(), &mut self.manifest, old, new, |m| {
            save_io(&manifest_path, m)
        })?;
        self.reload_profiles()?;
        if self.selected.to_lowercase() == old.to_lowercase() {
            self.select_profile(new)?;
        }
        Ok(())
    }

    pub fn duplicate_profile(&mut self, name: &str, new_name: &str) -> Result<(), SessionError> {
        profiles::duplicate(&self.profiles_dir(), name, new_name)?;
        self.reload_profiles()
    }

    /// Deletes a profile (the UI confirms first). If it was selected, the default profile (or the first) is
    /// selected instead.
    pub fn delete_profile(&mut self, name: &str) -> Result<(), SessionError> {
        let manifest_path = self.manifest_path();
        profiles::delete(&self.profiles_dir(), &mut self.manifest, name, |m| {
            save_io(&manifest_path, m)
        })?;
        self.reload_profiles()?;
        if self.selected.to_lowercase() == name.to_lowercase() {
            let next = self
                .profiles
                .iter()
                .find(|p| p.name == profiles::DEFAULT_NAME)
                .or(self.profiles.first())
                .map(|p| p.name.clone())
                .ok_or(ProfileError::LastProfile)?;
            self.select_profile(&next)?;
        }
        Ok(())
    }

    /// The shareable JSON of a profile (F14).
    pub fn export_profile(&self, name: &str) -> Result<String, SessionError> {
        let profile = self
            .profiles
            .iter()
            .find(|p| p.name.to_lowercase() == name.to_lowercase())
            .ok_or_else(|| ProfileError::NotFound(name.to_owned()))?;
        Ok(profiles::export(profile, &self.manifest)?)
    }

    /// Imports an exported profile under a free name and selects it (not applied, F14); what this library lacks is
    /// listed for D9.
    pub fn import_profile(&mut self, text: &str) -> Result<ProfileImported, SessionError> {
        let imported = profiles::import(text, &self.manifest)?;
        let mut profile = imported.profile;
        profile.name = profiles::free_name(&self.profiles, &profile.name);
        profiles::create(&self.profiles_dir(), &profile)?;
        self.reload_profiles()?;
        self.select_profile(&profile.name)?;
        Ok(ProfileImported {
            name: profile.name,
            missing: imported.missing,
        })
    }

    /// Replaces the settings; refused (and nothing saved) if any value is invalid. An unreadable `settings.json`
    /// kept from start is overwritten only now, by the user's own change; the previous file stays as `.bak`.
    pub fn update_settings(&mut self, settings: Settings) -> Result<(), SessionError> {
        settings::save(&self.data_dir.join("settings.json"), &settings)
            .map_err(SessionError::Settings)?;
        self.settings = settings;
        self.settings_locked = false;
        Ok(())
    }

    /// Puts the selected profile into the game folder `game_root` (F3/F6): our line in `gameinfo.gi` if it is
    /// vanilla, then the files. Nothing is touched while the game runs.
    pub fn apply(&mut self, game_root: &Path, game_running: bool) -> Result<Applied, SessionError> {
        if game_running {
            return Err(SessionError::Engine(EngineError::GameRunning));
        }
        let mut profile = self
            .profile()
            .cloned()
            .ok_or_else(|| ProfileError::NotFound(self.selected.clone()))?;
        let backup_name = format!("{}.gi", unique_number());
        let fs = Logged {
            fs: &Disk,
            log: &*self.log,
        };
        let gameinfo = ensure_mods_load(
            &fs,
            game_root,
            &self.data_dir.join("backups").join("gameinfo"),
            &backup_name,
        )?;
        // Never recreate a game folder that is gone: the files would land in an empty tree no game reads.
        if gameinfo == GuardOutcome::Missing {
            return Err(SessionError::GameNotFound(game_root.to_path_buf()));
        }
        let journal = FileJournal::new(self.data_dir.join("journal.jsonl"));
        let counted = Counted::new(&fs);
        let engine = Engine {
            fs: &counted,
            journal: &journal,
            data_dir: &self.data_dir,
            direction: self.direction,
        };
        let manifest_path = self.manifest_path();
        let report = engine
            .apply(
                &mut self.manifest,
                &profile,
                game_root,
                false,
                unique_number(),
                |m| save_io(&manifest_path, m),
            )
            .map_err(SessionError::Engine)?;
        let root = game_root.display().to_string();
        if counted.linked() + counted.copied() > 0 {
            let copies = Some(counted.copied() > 0);
            if let Some(i) = self
                .manifest
                .installations
                .iter_mut()
                .find(|i| same_root(&i.game_root, &root))
                && i.copies != copies
            {
                i.copies = copies;
                save_manifest(&manifest_path, &self.manifest)?;
            }
        }
        if profile.game_root.as_deref() != Some(root.as_str()) {
            profile.game_root = Some(root);
            profiles::save(&self.profiles_dir(), &profile)?;
            self.reload_profiles()?;
        }
        Ok(Applied {
            changed: report.changed,
            gameinfo,
            recovered: report.recovered,
            foreign: report.foreign.len(),
        })
    }

    /// S9: what applying the selected profile would change in its game folder (`fallback_root` as in
    /// [`Session::apply_preview`]).
    pub fn pending_view(&self, fallback_root: Option<&Path>) -> PendingView {
        let root = self.root_or(fallback_root);
        let installation = root.as_deref().and_then(|r| self.installation_at(r));
        let (switch, changes) = self.pending_parts(installation);
        PendingView {
            switch,
            changes,
            folder: self.folder_summary(root.as_deref(), installation),
        }
    }

    /// S9 "Undo this change": that part of the mod's entry goes back to what the game folder has.
    pub fn revert_pending(
        &mut self,
        mod_id: &str,
        aspect: Aspect,
        fallback_root: Option<&Path>,
    ) -> Result<(), SessionError> {
        let game = self.game_side(fallback_root);
        let mut profile = self
            .profile()
            .cloned()
            .ok_or_else(|| ProfileError::NotFound(self.selected.clone()))?;
        pending::revert(
            &mut profile,
            &game,
            &|id| self.grouping(id).unwrap_or_default(),
            mod_id,
            aspect,
        );
        self.edit_profile(|p| p.entries = profile.entries)
    }

    /// S9 "Discard all": a profile switch selects the applied profile again ("Keep Evening"); edits make the profile
    /// match the game folder.
    pub fn discard_pending(&mut self, fallback_root: Option<&Path>) -> Result<(), SessionError> {
        let root = self.root_or(fallback_root);
        let applied = root
            .as_deref()
            .and_then(|r| self.installation_at(r))
            .and_then(|i| i.applied_profile.clone());
        match applied {
            Some(name) if name == self.selected => {
                let matched = self
                    .profile_matching_game(fallback_root)
                    .ok_or_else(|| ProfileError::NotFound(self.selected.clone()))?;
                self.restore_profile(&matched)
            }
            Some(name) => self.select_profile(&name),
            None => Ok(()),
        }
    }

    /// The selected profile as the game folder has it now: what Undo after an automatic apply goes back to (143, 147).
    pub fn profile_matching_game(&self, fallback_root: Option<&Path>) -> Option<Profile> {
        let game = self.game_side(fallback_root);
        let profile = self.profile()?;
        Some(pending::matching_game(profile, &game, &|id| {
            self.grouping(id).unwrap_or_default()
        }))
    }

    /// Puts a profile's mods back as they were (Undo); only for the selected profile.
    pub fn restore_profile(&mut self, profile: &Profile) -> Result<(), SessionError> {
        if profile.name != self.selected {
            return Err(ProfileError::NotFound(profile.name.clone()).into());
        }
        let entries = profile.entries.clone();
        self.edit_profile(|p| p.entries = entries)
    }

    /// F9: what is in the game's `addons` folder that deadloader didn't put there, by name.
    pub fn foreign(&self, game_root: &Path) -> Vec<ForeignView> {
        let installation = self.installation_at(game_root);
        let applied: Vec<AppliedFile> = installation
            .map(|i| {
                i.applied
                    .iter()
                    .map(|r| AppliedFile {
                        slot: r.slot,
                        id: r.file.clone(),
                        len: r.len,
                    })
                    .collect()
            })
            .unwrap_or_default();
        let addons = addons_path(game_root);
        let Ok(found) = scan(&Disk, &addons, &applied) else {
            return Vec::new();
        };
        let chunked = |slot: u8| {
            found.foreign.iter().any(|o| {
                o.kind == ForeignKind::ModFile && o.slot == Some(slot) && slot_of(&o.name).is_none()
            })
        };
        let mut out: Vec<ForeignView> = found
            .foreign
            .iter()
            .map(|item| {
                let path = addons.join(&item.name);
                let modified_unix_ms = std::fs::metadata(&path)
                    .and_then(|m| m.modified())
                    .ok()
                    .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                    .and_then(|d| i64::try_from(d.as_millis()).ok());
                let (bytes, files) = if item.kind == ForeignKind::Folder {
                    let (bytes, files) = folder_size(&path);
                    (bytes, Some(files))
                } else {
                    (item.len, None)
                };
                let dir_file =
                    matches!(item.kind, ForeignKind::ModFile | ForeignKind::ReplacedOurs)
                        && slot_of(&item.name).is_some();
                let read = dir_file.then(|| summarize(&path)).flatten();
                let takeable = read.as_ref().is_some_and(|(_, inline)| *inline)
                    && !item.slot.is_some_and(chunked);
                ForeignView {
                    name: item.name.clone(),
                    kind: item.kind,
                    slot: item.slot,
                    bytes,
                    files,
                    modified_unix_ms,
                    origin: item.origin,
                    vpk: read.map(|(summary, _)| summary),
                    takeable,
                    left: installation.is_some_and(|i| {
                        i.left_foreign
                            .iter()
                            .any(|n| n.eq_ignore_ascii_case(&item.name))
                    }),
                }
            })
            .collect();
        out.sort_by_key(|i| i.name.to_lowercase());
        out
    }

    /// F9 "Leave as is": the item stays where it is and listed, but is never reported again.
    pub fn leave_foreign(&mut self, game_root: &Path, name: &str) -> Result<(), SessionError> {
        let name = self
            .foreign(game_root)
            .into_iter()
            .find(|i| i.name.eq_ignore_ascii_case(name))
            .map(|i| i.name)
            .ok_or_else(|| SessionError::NotForeign(name.to_owned()))?;
        let name = name.as_str();
        let root = game_root.display().to_string();
        let mut updated = self.manifest.clone();
        let index = match updated
            .installations
            .iter()
            .position(|i| same_root(&i.game_root, &root))
        {
            Some(i) => i,
            None => {
                updated.installations.push(InstallationState {
                    game_root: root,
                    ..InstallationState::default()
                });
                updated.installations.len() - 1
            }
        };
        let left = &mut updated.installations[index].left_foreign;
        if !left.iter().any(|n| n.eq_ignore_ascii_case(name)) {
            left.push(name.to_owned());
            left.sort_by_key(|n| n.to_lowercase());
        }
        save_manifest(&self.manifest_path(), &updated)?;
        self.manifest = updated;
        Ok(())
    }

    /// D11: a foreign mod file becomes a local mod. It is copied into the library (the copy keeps its bytes, so this
    /// can be undone), the original leaves the game folder through the intent journal, and the mod is enabled at the
    /// bottom of the selected profile; the next apply puts our copy in. Returns the new mod's id.
    pub fn take_foreign(
        &mut self,
        game_root: &Path,
        name: &str,
        mod_name: &str,
        game_running: bool,
    ) -> Result<String, SessionError> {
        if game_running {
            return Err(SessionError::Engine(EngineError::GameRunning));
        }
        let item = self
            .foreign(game_root)
            .into_iter()
            .find(|i| i.name.eq_ignore_ascii_case(name) && i.takeable)
            .ok_or_else(|| SessionError::NotTakeable(name.to_owned()))?;
        let path = addons_path(game_root).join(&item.name);
        let title = match mod_name.trim() {
            "" => item.name.trim_end_matches(".vpk").to_owned(),
            given => given.to_owned(),
        };
        let req = ImportRequest {
            source: path.clone(),
            mod_id: self.free_local_id(),
            name: title,
            mod_source: Source::Local {
                imported_from: Some(path.display().to_string()),
            },
            version: None,
            remote_file_id: None,
            archive_name: None,
        };
        let imported = self.import_request(&req, None, false)?;
        let journal = FileJournal::new(self.data_dir.join("journal.jsonl"));
        let fs = Logged {
            fs: &Disk,
            log: &*self.log,
        };
        if let Err(e) = journal::run(&fs, &journal, unique_number(), &[Step::Delete { path }]) {
            // The original stays where it was; the copy goes again.
            let _ = self.remove(&Target::Mod(imported.mod_id.clone()), &Permanently);
            return Err(SessionError::Engine(EngineError::Tx(e)));
        }
        let grouping = self.grouping(&imported.mod_id)?;
        let mod_id = imported.mod_id.clone();
        self.edit_profile(|p| {
            p.entries.retain(|e| e.mod_id != mod_id);
            p.entries.push(ProfileEntry {
                mod_id: mod_id.clone(),
                variants: grouping
                    .variant_groups
                    .iter()
                    .filter_map(|g| g.first().cloned())
                    .collect(),
                add_ons: grouping.standalone.clone(),
                enabled: true,
            });
        })?;
        Ok(imported.mod_id)
    }

    /// D10 "Check again": our line goes into a `gameinfo.gi` that is the original again (after Steam restored it);
    /// a file another program changed is left as it is. Never while the game runs.
    pub fn fix_gameinfo(
        &mut self,
        game_root: &Path,
        game_running: bool,
    ) -> Result<GuardOutcome, SessionError> {
        if game_running {
            return Err(SessionError::Engine(EngineError::GameRunning));
        }
        let fs = Logged {
            fs: &Disk,
            log: &*self.log,
        };
        Ok(ensure_mods_load(
            &fs,
            game_root,
            &self.data_dir.join("backups").join("gameinfo"),
            &format!("{}.gi", unique_number()),
        )?)
    }

    /// Where a mod's first file is in the library ("Open folder" in the mod details).
    pub fn mod_file_path(&self, mod_id: &str) -> Option<PathBuf> {
        let m = self.manifest.mods.iter().find(|m| m.id == mod_id)?;
        let f = m.files.first()?;
        Some(library_path(&self.data_dir, &f.library_path))
    }

    /// The pending switch or the pending edits against `installation`.
    fn pending_parts(
        &self,
        installation: Option<&InstallationState>,
    ) -> (Option<ProfileSwitch>, Vec<PendingChange>) {
        let Some(profile) = self.profile() else {
            return (None, Vec::new());
        };
        let game = pending::in_game(&self.manifest, installation, self.direction);
        let changes = pending::changes(&self.manifest, profile, &game);
        let applied = installation.and_then(|i| i.applied_profile.clone());
        if applied.as_deref() == Some(self.selected.as_str()) {
            return (None, changes);
        }
        // A profile never applied to an empty folder has nothing to switch.
        if applied.is_none() && changes.is_empty() {
            return (None, Vec::new());
        }
        let count = |kind: pending::Change| changes.iter().filter(|c| c.change == kind).count();
        (
            Some(ProfileSwitch {
                from: applied,
                to: self.selected.clone(),
                turned_on: count(pending::Change::Enable),
                turned_off: count(pending::Change::Disable),
            }),
            Vec::new(),
        )
    }

    /// Files added, removed and renumbered in `root`'s `addons` folder by applying the selected profile.
    fn folder_summary(
        &self,
        root: Option<&Path>,
        installation: Option<&InstallationState>,
    ) -> FolderSummary {
        let applied: Vec<&crate::library::AppliedRecord> = installation
            .map(|i| i.applied.iter().collect())
            .unwrap_or_default();
        let foreign_slots = root
            .and_then(|r| {
                let records: Vec<AppliedFile> = applied
                    .iter()
                    .map(|r| AppliedFile {
                        slot: r.slot,
                        id: r.file.clone(),
                        len: r.len,
                    })
                    .collect();
                scan(&Disk, &addons_path(r), &records).ok()
            })
            .map(|s| s.foreign_slots())
            .unwrap_or_default();
        let wanted: Vec<Wanted> = self
            .wanted_files()
            .into_iter()
            .map(|id| Wanted {
                id,
                source: PathBuf::new(),
                sha256: [0; 32],
            })
            .collect();
        let target = layout(&foreign_slots, &wanted, self.direction).unwrap_or_default();
        let slot_now = |id: &str| applied.iter().find(|r| r.file == id).map(|r| r.slot);
        FolderSummary {
            added: target
                .iter()
                .filter(|(_, id)| slot_now(id).is_none())
                .count(),
            removed: applied
                .iter()
                .filter(|r| !target.iter().any(|(_, id)| *id == r.file))
                .count(),
            renumbered: target
                .iter()
                .filter(|(slot, id)| slot_now(id).is_some_and(|s| s != *slot))
                .count(),
            addons: root.map(|r| addons_path(r).display().to_string()),
        }
    }

    /// What deadloader put into the game folder the selected profile goes to, by mod.
    fn game_side(&self, fallback_root: Option<&Path>) -> Vec<InGame> {
        let root = self.root_or(fallback_root);
        let installation = root.as_deref().and_then(|r| self.installation_at(r));
        pending::in_game(&self.manifest, installation, self.direction)
    }

    fn root_or(&self, fallback_root: Option<&Path>) -> Option<PathBuf> {
        self.game_root()
            .or_else(|| fallback_root.map(Path::to_path_buf))
    }

    fn installation_at(&self, root: &Path) -> Option<&InstallationState> {
        let root = root.display().to_string();
        self.manifest
            .installations
            .iter()
            .find(|i| same_root(&i.game_root, &root))
    }

    fn profile(&self) -> Option<&Profile> {
        self.profiles.iter().find(|p| p.name == self.selected)
    }

    fn edit_profile(&mut self, change: impl FnOnce(&mut Profile)) -> Result<(), SessionError> {
        let mut profile = self
            .profile()
            .cloned()
            .ok_or_else(|| ProfileError::NotFound(self.selected.clone()))?;
        change(&mut profile);
        profiles::save(&self.profiles_dir(), &profile)?;
        if let Some(p) = self.profiles.iter_mut().find(|p| p.name == profile.name) {
            *p = profile;
        }
        Ok(())
    }

    fn reload_profiles(&mut self) -> Result<(), SessionError> {
        self.profiles = profiles::list(&self.profiles_dir())?.profiles;
        Ok(())
    }

    fn save_settings(&self) -> Result<(), SessionError> {
        if self.settings_locked {
            return Ok(());
        }
        settings::save(&self.data_dir.join("settings.json"), &self.settings)
            .map_err(SessionError::Settings)
    }

    fn mod_files(&self, mod_id: &str) -> Result<Vec<String>, SessionError> {
        self.manifest
            .mods
            .iter()
            .find(|m| m.id == mod_id)
            .map(|m| m.files.iter().map(|f| f.id.clone()).collect())
            .ok_or_else(|| SessionError::UnknownMod(mod_id.to_owned()))
    }

    fn grouping(&self, mod_id: &str) -> Result<Grouping, SessionError> {
        let files = self.mod_files(mod_id)?;
        let indexes: Vec<FileIndex> = files
            .iter()
            .map(|id| {
                self.indexes.get(id).cloned().unwrap_or(FileIndex {
                    id: id.clone(),
                    entries: Vec::new(),
                })
            })
            .collect();
        Ok(group(&indexes))
    }

    fn mod_view(&self, mod_id: &str, entry: Option<&ProfileEntry>) -> Option<ModView> {
        fn newest_successor(
            m: &crate::library::Mod,
            f: &crate::library::ModFile,
        ) -> Option<String> {
            let mut newest: Option<&str> = None;
            let mut at = f;
            // The manifest refuses successor loops; the bound only guards against a damaged one.
            for _ in 0..m.files.len() {
                let Some(crate::library::Successor::File(next)) = &at.successor else {
                    break;
                };
                let Some(file) = m.files.iter().find(|x| &x.id == next) else {
                    break;
                };
                newest = Some(&file.id);
                at = file;
            }
            newest.map(str::to_owned)
        }
        let m = self.manifest.mods.iter().find(|m| m.id == mod_id)?;
        let grouping = self.grouping(mod_id).ok()?;
        let active =
            |id: &String| entry.is_some_and(|e| e.variants.contains(id) || e.add_ons.contains(id));
        Some(ModView {
            id: m.id.clone(),
            name: m.name.clone(),
            author: m.author.clone(),
            source: m.source.clone(),
            enabled: entry.is_some_and(|e| e.enabled),
            files: m
                .files
                .iter()
                .map(|f| FileView {
                    id: f.id.clone(),
                    archive_name: f.archive_name.clone(),
                    inner_path: f.inner_path.clone(),
                    version: f.version.clone(),
                    len: f.len,
                    damaged: self.damaged.contains(&f.id),
                    newer_in_library: newest_successor(m, f),
                })
                .collect(),
            variant_groups: grouping
                .variant_groups
                .iter()
                .map(|g| VariantGroupView {
                    files: g.clone(),
                    active: g.iter().find(|f| active(f)).cloned(),
                })
                .collect(),
            add_ons: grouping
                .standalone
                .iter()
                .map(|f| (f.clone(), active(f)))
                .collect(),
            bytes: m.files.iter().map(|f| f.len).sum(),
            pending: false,
            in_game: self.in_game_view(m),
        })
    }

    /// The files of `m` deadloader put into the selected profile's game folder.
    fn in_game_view(&self, m: &crate::library::Mod) -> Option<InGameView> {
        let installation = self.installation()?;
        let bytes: u64 = installation
            .applied
            .iter()
            .filter(|r| m.files.iter().any(|f| f.id == r.file))
            .map(|r| r.len)
            .sum();
        let any = installation
            .applied
            .iter()
            .any(|r| m.files.iter().any(|f| f.id == r.file));
        any.then_some(InGameView {
            copied: installation.copies == Some(true),
            bytes,
        })
    }

    /// The files the selected profile puts into the game, in priority order.
    fn wanted_files(&self) -> Vec<String> {
        self.profile()
            .map(|p| {
                p.entries
                    .iter()
                    .filter(|e| e.enabled)
                    .flat_map(|e| e.variants.iter().chain(&e.add_ons).cloned())
                    .filter(|id| self.manifest.file(id).is_some())
                    .collect()
            })
            .unwrap_or_default()
    }

    /// The game folder the selected profile uses: its own, or the one it was last applied to.
    fn installation(&self) -> Option<&crate::library::InstallationState> {
        let profile = self.profile()?;
        match &profile.game_root {
            Some(root) => self
                .manifest
                .installations
                .iter()
                .find(|i| i.game_root == *root),
            None => self
                .manifest
                .installations
                .iter()
                .find(|i| i.applied_profile.as_deref() == Some(self.selected.as_str())),
        }
    }

    fn pending(&self, wanted: &[String]) -> bool {
        let wanted: Vec<Wanted> = wanted
            .iter()
            .filter_map(|id| {
                let (_, f) = self.manifest.file(id)?;
                Some(Wanted {
                    id: f.id.clone(),
                    source: PathBuf::new(),
                    sha256: f.sha256,
                })
            })
            .collect();
        let Ok(mut expected) = layout(&[], &wanted, self.direction) else {
            return true;
        };
        expected.sort();
        let mut applied: Vec<(u8, String)> = self
            .installation()
            .map(|i| i.applied.iter().map(|r| (r.slot, r.file.clone())).collect())
            .unwrap_or_default();
        applied.sort();
        let ids = |v: &[(u8, String)]| v.iter().map(|(_, id)| id.clone()).collect::<Vec<_>>();
        ids(&expected) != ids(&applied)
    }

    fn index(&mut self, ids: &[String]) {
        for id in ids {
            let Some((_, f)) = self.manifest.file(id) else {
                continue;
            };
            let path = library_path(&self.data_dir, &f.library_path);
            let read = std::fs::File::open(&path)
                .ok()
                .and_then(|file| read_index(&mut io::BufReader::new(file)).ok());
            match read {
                Some(index) => {
                    self.damaged.remove(id);
                    self.indexes.insert(
                        id.clone(),
                        FileIndex {
                            id: id.clone(),
                            entries: index
                                .entries
                                .into_iter()
                                .map(|e| (e.path, e.crc32))
                                .collect(),
                        },
                    );
                }
                None => {
                    self.damaged.insert(id.clone());
                }
            }
        }
    }

    fn free_local_id(&self) -> String {
        (1..)
            .map(|n| format!("local-{n}"))
            .find(|id| self.manifest.mods.iter().all(|m| &m.id != id))
            .expect("a free id")
    }

    fn manifest_path(&self) -> PathBuf {
        self.data_dir.join("manifest.json")
    }

    fn profiles_dir(&self) -> PathBuf {
        self.data_dir.join("profiles")
    }
}

/// The first file of each variant group and every add-on.
fn default_files(grouping: &Grouping) -> Vec<String> {
    grouping
        .variant_groups
        .iter()
        .filter_map(|g| g.first().cloned())
        .chain(grouping.standalone.iter().cloned())
        .collect()
}

/// At most this many files of a foreign folder are counted (a replays folder can be big).
const FOLDER_COUNT_LIMIT: u64 = 10_000;

/// The size and number of files of a folder, at every level, up to [`FOLDER_COUNT_LIMIT`] files.
fn folder_size(dir: &Path) -> (u64, u64) {
    let mut bytes = 0u64;
    let mut files = 0u64;
    let mut pending = vec![dir.to_path_buf()];
    while let Some(at) = pending.pop() {
        let Ok(entries) = std::fs::read_dir(&at) else {
            continue;
        };
        for entry in entries.flatten() {
            let Ok(kind) = entry.file_type() else {
                continue;
            };
            if kind.is_dir() {
                pending.push(entry.path());
            } else {
                files += 1;
                bytes += entry.metadata().map_or(0, |m| m.len());
                if files >= FOLDER_COUNT_LIMIT {
                    return (bytes, files);
                }
            }
        }
    }
    (bytes, files)
}

/// What a foreign `pakNN_dir.vpk` holds, and whether all its data is inside it (no `pakNN_000.vpk` chunks needed).
fn summarize(path: &Path) -> Option<(VpkSummary, bool)> {
    let file = std::fs::File::open(path).ok()?;
    let index = read_index(&mut io::BufReader::new(file)).ok()?;
    let inline = index.entries.iter().all(crate::vpk::Entry::in_dir_file);
    let mut heroes: HashMap<String, usize> = HashMap::new();
    for e in &index.entries {
        let parts: Vec<&str> = e.path.split('/').collect();
        if let Some(i) = parts
            .iter()
            .position(|p| p.to_ascii_lowercase().starts_with("heroes"))
            && let Some(hero) = parts.get(i + 1).filter(|_| i + 2 < parts.len())
        {
            *heroes.entry(hero.to_ascii_lowercase()).or_default() += 1;
        }
    }
    let hero = heroes
        .into_iter()
        .max_by(|a, b| a.1.cmp(&b.1).then_with(|| b.0.cmp(&a.0)))
        .map(|(name, _)| name);
    Some((
        VpkSummary {
            entries: index.entries.len(),
            hero,
        },
        inline,
    ))
}

fn library_path(data_dir: &Path, relative: &str) -> PathBuf {
    relative
        .split('/')
        .fold(data_dir.to_path_buf(), |p, c| p.join(c))
}

fn save_io(path: &Path, manifest: &Manifest) -> io::Result<()> {
    save_manifest(path, manifest).map_err(|e| match e {
        ModelError::Io(text) => io::Error::other(text),
        other => io::Error::new(io::ErrorKind::InvalidData, other.to_string()),
    })
}

/// Distinct per call within this process and across restarts: transaction ids and backup names.
fn unique_number() -> u64 {
    use std::sync::atomic::{AtomicU64, Ordering};
    static LAST: AtomicU64 = AtomicU64::new(0);
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| u64::try_from(d.as_micros()).unwrap_or(u64::MAX))
        .unwrap_or(0);
    let mut last = LAST.load(Ordering::Relaxed);
    loop {
        let next = now.max(last + 1);
        match LAST.compare_exchange(last, next, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => return next,
            Err(seen) => last = seen,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::log::{FileOp, MemFileLog};
    use crate::testing::{remove_all, temp_dir, vpk};
    use crate::updates::Mapping;
    use std::path::PathBuf;
    use std::sync::Arc;

    const VANILLA: &str = "\"GameInfo\"\r\n{\r\n\tFileSystem\r\n\t{\r\n\t\tSearchPaths\r\n\t\t{\r\n\t\t\tGame\t\t\t\tcitadel\r\n\t\t\tGame\t\t\t\tcore\r\n\t\t}\r\n\t}\r\n}\r\n";

    struct Fixture {
        dir: PathBuf,
        log: Arc<MemFileLog>,
    }

    impl Fixture {
        fn new(name: &str) -> Self {
            Self {
                dir: temp_dir("session", name),
                log: Arc::new(MemFileLog::default()),
            }
        }
        fn ops(&self) -> Vec<(FileOp, String)> {
            self.log
                .events()
                .into_iter()
                .map(|(_, e)| (e.op, e.dst.or(e.src).unwrap_or_default()))
                .collect()
        }
        fn data(&self) -> PathBuf {
            self.dir.join("deadloader-data")
        }
        fn game(&self) -> PathBuf {
            let root = self.dir.join("Deadlock");
            let citadel = root.join("game").join("citadel");
            if !citadel.join("gameinfo.gi").exists() {
                std::fs::create_dir_all(&citadel).expect("game dir");
                std::fs::write(citadel.join("gameinfo.gi"), VANILLA).expect("gameinfo");
            }
            root
        }
        fn source(&self, name: &str, files: &[(&str, u32)]) -> PathBuf {
            let path = self.dir.join(name);
            std::fs::write(&path, vpk(files)).expect("source vpk");
            path
        }
    }

    impl Drop for Fixture {
        fn drop(&mut self) {
            remove_all(&self.dir);
        }
    }

    fn open(f: &Fixture) -> Session {
        Session::open(&f.data(), f.log.clone())
            .expect("opened")
            .session
    }

    fn names(view: &LibraryView) -> Vec<(&str, bool)> {
        view.mods
            .iter()
            .map(|m| (m.name.as_str(), m.enabled))
            .collect()
    }

    fn haze_download(file_id: u64, file_name: &str) -> Download {
        Download {
            section: crate::library::Section::Mod,
            mod_id: 655_209,
            file_id,
            file_name: file_name.to_owned(),
            mod_name: "Haze V2".to_owned(),
            author: Some("Nyx".to_owned()),
            version: Some("2.0".to_owned()),
        }
    }

    #[test]
    fn a_download_becomes_a_gamebanana_mod_enabled_on_top() {
        let f = Fixture::new("download");
        let mut s = open(&f);
        s.import_file(
            &f.source("old.vpk", &[("models/old/body.vmdl_c", 1)]),
            "Old",
        )
        .expect("local mod");
        // The queue keeps the archive under its task number.
        let queued = f.source("17.bin", &[("models/haze/body.vmdl_c", 1)]);
        let outcome = s
            .import_download(&queued, &haze_download(1_480_331, "haze_v2.vpk"))
            .expect("imported");
        assert_eq!(outcome.mod_id, "gb-655209");
        assert!(outcome.choices.is_empty());
        let view = s.library();
        assert_eq!(
            names(&view),
            [("Haze V2", true), ("Old", true)],
            "136: on top"
        );
        let haze = &view.mods[0];
        assert_eq!(haze.author.as_deref(), Some("Nyx"));
        assert_eq!(
            haze.source,
            Source::GameBanana {
                section: crate::library::Section::Mod,
                mod_id: 655_209
            }
        );
        assert_eq!(haze.files[0].archive_name, "haze_v2.vpk");
        assert_eq!(haze.files[0].version.as_deref(), Some("2.0"));
        let (_, file) = s.manifest.file(&haze.files[0].id).expect("file");
        assert_eq!(file.remote_file_id, Some(1_480_331));
    }

    #[test]
    fn another_file_of_the_same_mod_joins_it() {
        let f = Fixture::new("download2");
        let mut s = open(&f);
        s.import_download(
            &f.source("17.bin", &[("models/haze/body.vmdl_c", 1)]),
            &haze_download(1, "haze_pink.vpk"),
        )
        .expect("first");
        s.import_download(
            &f.source("18.bin", &[("models/haze/body.vmdl_c", 2)]),
            &haze_download(2, "haze_blue.vpk"),
        )
        .expect("second");
        let view = s.library();
        assert_eq!(view.mods.len(), 1, "one GameBanana mod (144)");
        assert_eq!(view.mods[0].files.len(), 2);
    }

    /// v1 of Haze installed and active, v2 imported as its update (D5): the old one stays active until confirmed.
    fn haze_v1_and_its_update(f: &Fixture, v2_path: &str) -> (Session, String, Vec<String>) {
        let mut s = open(f);
        s.import_download(
            &f.source("1.bin", &[("models/haze/body.vmdl_c", 1)]),
            &haze_download(1, "haze.vpk"),
        )
        .expect("v1");
        let v1 = s.manifest.mods[0].files[0].id.clone();
        let new = s
            .import_update_file(
                &f.source("2.bin", &[(v2_path, 2)]),
                &haze_download(2, "haze.vpk"),
            )
            .expect("v2");
        (s, v1, new.new_files)
    }

    #[test]
    fn an_update_file_joins_the_mod_without_being_enabled() {
        let f = Fixture::new("updatefile");
        let (s, v1, new) = haze_v1_and_its_update(&f, "models/haze/body.vmdl_c");
        assert_eq!(new.len(), 1);
        let view = s.library();
        assert_eq!(view.mods[0].files.len(), 2);
        let group = &view.mods[0].variant_groups[0];
        assert_eq!(
            group.active.as_deref(),
            Some(v1.as_str()),
            "v1 stays active"
        );
    }

    #[test]
    fn the_update_is_paired_by_its_place_in_the_archive_and_applied() {
        let f = Fixture::new("updateapply");
        let (mut s, v1, new) = haze_v1_and_its_update(&f, "models/haze/body.vmdl_c");
        let proposals = s.propose_update("gb-655209", 1, &new).expect("proposed");
        assert_eq!(
            proposals,
            [UpdateProposal {
                old: v1.clone(),
                proposed: Proposed::Pair {
                    new: new[0].clone()
                },
            }]
        );
        assert!(proposals.iter().all(UpdateProposal::is_settled));
        s.apply_update(&ConfirmedUpdate {
            mod_id: "gb-655209".to_owned(),
            added: Vec::new(),
            mappings: vec![Mapping {
                old: v1.clone(),
                new: Some(new[0].clone()),
            }],
            version: Some("2.0".to_owned()),
        })
        .expect("applied");
        // 137: the new version is active where the old one was; the old one stays (22) and knows its successor.
        let view = open(&f).library();
        assert_eq!(
            view.mods[0].variant_groups[0].active.as_deref(),
            Some(new[0].as_str())
        );
        let old = view.mods[0]
            .files
            .iter()
            .find(|x| x.id == v1)
            .expect("kept");
        assert_eq!(old.newer_in_library.as_deref(), Some(new[0].as_str()));
    }

    #[test]
    fn an_update_that_changes_other_game_files_needs_the_user() {
        let f = Fixture::new("updatenooverlap");
        let (mut s, v1, new) = haze_v1_and_its_update(&f, "models/haze/legs.vmdl_c");
        // Different archive layout: one file each still pairs by place; force overlap matching with two old files.
        s.import_download(
            &f.source("3.bin", &[("models/haze/hands.vmdl_c", 1)]),
            &haze_download(1, "haze.vpk"),
        )
        .expect("a second file of v1's download");
        let proposals = s.propose_update("gb-655209", 1, &new).expect("proposed");
        assert!(proposals.iter().any(|p| p.old == v1));
        assert!(
            proposals
                .iter()
                .all(|p| matches!(p.proposed, Proposed::NoOverlap)),
            "{proposals:?}"
        );
        assert!(!proposals.iter().any(UpdateProposal::is_settled));
    }

    #[test]
    fn an_old_version_shows_the_newest_one_in_the_library() {
        let f = Fixture::new("newer");
        let mut s = open(&f);
        for (n, crc) in [(1, 1), (2, 2), (3, 3)] {
            s.import_download(
                &f.source(&format!("{n}.bin"), &[("models/haze/body.vmdl_c", crc)]),
                &haze_download(n, &format!("haze_v{n}.vpk")),
            )
            .expect("imported");
        }
        let ids: Vec<String> = s.manifest.mods[0]
            .files
            .iter()
            .map(|f| f.id.clone())
            .collect();
        // v1 → v2 → v3, as two confirmed updates leave it (170).
        s.manifest.mods[0].files[0].successor =
            Some(crate::library::Successor::File(ids[1].clone()));
        s.manifest.mods[0].files[1].successor =
            Some(crate::library::Successor::File(ids[2].clone()));
        let view = s.library();
        let newer: Vec<Option<&str>> = view.mods[0]
            .files
            .iter()
            .map(|f| f.newer_in_library.as_deref())
            .collect();
        assert_eq!(newer, [Some(ids[2].as_str()), Some(ids[2].as_str()), None]);
    }

    #[test]
    fn hidden_new_files_are_remembered_across_starts() {
        let f = Fixture::new("dismiss");
        let mut s = open(&f);
        s.import_download(
            &f.source("17.bin", &[("models/haze/body.vmdl_c", 1)]),
            &haze_download(1, "haze.vpk"),
        )
        .expect("imported");
        s.dismiss_new_files("gb-655209", &[7, 8]).expect("hidden");
        s.dismiss_new_files("gb-655209", &[8, 9])
            .expect("hidden again");
        assert_eq!(
            open(&f).manifest().dismissed_new_files["gb-655209"],
            [7, 8, 9]
        );
        assert!(matches!(
            s.dismiss_new_files("gb-1", &[1]),
            Err(SessionError::UnknownMod(_))
        ));
    }

    fn listed_file(id: u64, name: &str, md5: Option<String>) -> crate::updates::RemoteFile {
        crate::updates::RemoteFile {
            id,
            file_name: name.to_owned(),
            size_bytes: 1,
            date_added: None,
            version: None,
            description: None,
            md5,
        }
    }

    const HAZE: (crate::library::Section, u64) = (crate::library::Section::Mod, 655_209);

    #[test]
    fn applying_a_switched_profile_says_which_mods_leave_the_game() {
        let f = Fixture::new("turnoff");
        let mut s = open(&f);
        s.import_file(
            &f.source("haze.vpk", &[("models/haze/body.vmdl_c", 1)]),
            "Haze",
        )
        .expect("haze");
        s.import_file(
            &f.source("seven.vpk", &[("models/seven/body.vmdl_c", 1)]),
            "Seven",
        )
        .expect("seven");
        let game = f.game();
        s.apply(&game, false).expect("applied");
        assert_eq!(
            s.apply_preview(Some(&game)),
            ApplyPreview::default(),
            "nothing waits"
        );

        // D13: another profile keeps Seven, adds Wraith and leaves Haze out.
        s.create_profile("Evening").expect("created");
        s.select_profile("Evening").expect("selected");
        s.set_enabled("local-2", true).expect("seven on");
        s.import_file(
            &f.source("wraith.vpk", &[("models/wraith/body.vmdl_c", 1)]),
            "Wraith",
        )
        .expect("wraith");
        // The new profile has no game folder yet: Apply would use the one Steam knows.
        assert_eq!(
            s.apply_preview(Some(&game)),
            ApplyPreview {
                turned_off: vec!["Haze".to_owned()],
                turned_on: vec!["Wraith".to_owned()],
            }
        );
        assert_eq!(
            s.apply_preview(None),
            ApplyPreview::default(),
            "no game folder at all"
        );
    }

    #[test]
    fn a_local_mod_is_linked_by_the_md5_of_its_archive() {
        let f = Fixture::new("linkmd5");
        let mut s = open(&f);
        // Downloaded by hand in a browser and renamed: only the md5 can tell.
        let source = f.source("my haze.vpk", &[("models/haze/body.vmdl_c", 1)]);
        let md5 = crate::fs::md5_file(&source).expect("md5");
        s.import_file(&source, "Haze").expect("imported");
        let listed = [
            listed_file(101, "haze_v2.vpk", Some(md5)),
            listed_file(102, "other.vpk", Some("00".repeat(16))),
        ];
        let report = s
            .link_mod("local-1", HAZE.0, HAZE.1, &listed)
            .expect("linked");
        assert_eq!(report.archives.len(), 1);
        assert_eq!(report.archives[0].remote_file_id, Some(101));
        assert_eq!(report.archives[0].matched_by, Some(LinkMatch::Md5));
        let reopened = open(&f);
        let m = &reopened.manifest().mods[0];
        assert_eq!(
            m.source,
            Source::GameBanana {
                section: HAZE.0,
                mod_id: HAZE.1
            }
        );
        assert_eq!(m.files[0].remote_file_id, Some(101));
    }

    #[test]
    fn without_a_matching_md5_the_archive_name_links_it() {
        let f = Fixture::new("linkname");
        let mut s = open(&f);
        s.import_file(
            &f.source("Haze_V2.vpk", &[("models/haze/body.vmdl_c", 1)]),
            "Haze",
        )
        .expect("imported");
        let listed = [
            listed_file(101, "haze_v2_ea17d.vpk", Some("11".repeat(16))),
            listed_file(102, "other.vpk", None),
        ];
        let report = s
            .link_mod("local-1", HAZE.0, HAZE.1, &listed)
            .expect("linked");
        assert_eq!(report.archives[0].remote_file_id, Some(101));
        assert_eq!(report.archives[0].matched_by, Some(LinkMatch::Name));
    }

    #[test]
    fn an_archive_nothing_matches_is_linked_by_hand() {
        let f = Fixture::new("linkhand");
        let mut s = open(&f);
        s.import_file(
            &f.source("repack.vpk", &[("models/haze/body.vmdl_c", 1)]),
            "Haze",
        )
        .expect("imported");
        let listed = [listed_file(101, "haze_v2.vpk", None)];
        let report = s
            .link_mod("local-1", HAZE.0, HAZE.1, &listed)
            .expect("linked");
        assert_eq!(report.archives[0].remote_file_id, None);
        assert_eq!(report.archives[0].matched_by, None);
        let files = report.archives[0].files.clone();
        s.link_files("local-1", &files, Some(101))
            .expect("linked by hand");
        assert_eq!(
            open(&f).manifest().mods[0].files[0].remote_file_id,
            Some(101)
        );
        // A file of another mod, or a mod that is still local, is refused.
        assert!(
            s.link_files("local-1", &["local-9-1-0".to_owned()], Some(1))
                .is_err()
        );
    }

    #[test]
    fn a_linked_mod_takes_its_later_downloads() {
        let f = Fixture::new("linkjoin");
        let mut s = open(&f);
        s.import_file(
            &f.source("haze.vpk", &[("models/haze/body.vmdl_c", 1)]),
            "Haze",
        )
        .expect("imported");
        s.link_mod(
            "local-1",
            HAZE.0,
            HAZE.1,
            &[listed_file(101, "haze.vpk", None)],
        )
        .expect("linked");
        let outcome = s
            .import_download(
                &f.source("7.bin", &[("models/haze/hands.vmdl_c", 1)]),
                &haze_download(102, "hands.vpk"),
            )
            .expect("downloaded");
        assert_eq!(
            outcome.mod_id, "local-1",
            "one library mod per GameBanana mod (144)"
        );
        assert_eq!(s.library().mods.len(), 1);
    }

    #[test]
    fn a_mod_can_be_renamed() {
        let f = Fixture::new("rename");
        let mut s = open(&f);
        s.import_file(&f.source("x.vpk", &[("models/haze/body.vmdl_c", 1)]), "x")
            .expect("imported");
        s.rename_mod("local-1", "  Haze, blue  ").expect("renamed");
        assert_eq!(open(&f).library().mods[0].name, "Haze, blue");
        assert!(s.rename_mod("local-1", "   ").is_err(), "a name is needed");
    }

    #[test]
    fn imports_keep_to_the_archive_limits_in_the_settings() {
        use std::io::Write as _;
        let f = Fixture::new("limits");
        let mut s = open(&f);
        let mut settings = s.settings().clone();
        settings.downloads.max_vpk_files = 1;
        s.update_settings(settings).expect("saved");
        let archive = f.dir.join("two.zip");
        let mut w = zip::ZipWriter::new(std::fs::File::create(&archive).expect("zip"));
        let opts = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Stored);
        for name in ["a/pak01_dir.vpk", "b/pak01_dir.vpk"] {
            w.start_file(name, opts).expect("entry");
            w.write_all(&vpk(&[("models/haze/body.vmdl_c", 1)]))
                .expect("write");
        }
        w.finish().expect("finish");
        let err = s.import_file(&archive, "Two").expect_err("over the limit");
        assert!(
            matches!(err, SessionError::Import(ImportError::Archive(_))),
            "{err}"
        );
    }

    #[test]
    fn sound_mods_have_their_own_ids() {
        let f = Fixture::new("downloadsound");
        let mut s = open(&f);
        let download = Download {
            section: crate::library::Section::Sound,
            ..haze_download(1, "haze.vpk")
        };
        let outcome = s
            .import_download(
                &f.source("1.bin", &[("sounds/haze/a.vsnd_c", 1)]),
                &download,
            )
            .expect("imported");
        assert_eq!(outcome.mod_id, "gb-sound-655209");
    }

    #[test]
    fn a_first_start_has_an_empty_library_and_the_default_profile() {
        let f = Fixture::new("first");
        let opened = Session::open(&f.data(), f.log.clone()).expect("opened");
        assert!(opened.notices.is_empty(), "{:?}", opened.notices);
        let s = opened.session;
        assert_eq!(s.selected_profile(), crate::profiles::DEFAULT_NAME);
        assert_eq!(
            s.profile_names(),
            [crate::profiles::DEFAULT_NAME.to_owned()]
        );
        assert!(s.library().mods.is_empty());
        assert!(f.data().join("profiles").join("Default.json").exists());
    }

    #[test]
    fn an_imported_mod_is_enabled_on_top_and_survives_a_restart() {
        let f = Fixture::new("import");
        let mut s = open(&f);
        let first = f.source("haze.vpk", &[("models/haze/body.vmdl_c", 1)]);
        let second = f.source("sounds.vpk", &[("sounds/haze/ult.vsnd_c", 2)]);
        let a = s.import_file(&first, "Haze skin").expect("imported");
        assert!(a.choices.is_empty());
        s.import_file(&second, "Haze sounds").expect("imported");

        let view = s.library();
        assert_eq!(names(&view), [("Haze sounds", true), ("Haze skin", true)]);
        assert_eq!(view.mods[1].files.len(), 1);
        assert_eq!(
            view.mods[1].files[0].len,
            vpk(&[("models/haze/body.vmdl_c", 1)]).len() as u64
        );

        let again = open(&f);
        assert_eq!(names(&again.library()), names(&view));
    }

    #[test]
    fn colour_options_in_one_archive_wait_for_a_choice() {
        let f = Fixture::new("choice");
        let mut s = open(&f);
        let archive = f.dir.join("colours.zip");
        {
            use std::io::Write;
            let mut w = zip::ZipWriter::new(std::fs::File::create(&archive).expect("zip"));
            let opts = zip::write::SimpleFileOptions::default()
                .compression_method(zip::CompressionMethod::Stored);
            for (name, crc) in [("red.vpk", 1), ("blue.vpk", 2)] {
                w.start_file(name, opts).expect("entry");
                w.write_all(&vpk(&[("models/haze/body.vmdl_c", crc)]))
                    .expect("write");
            }
            w.finish().expect("finish");
        }
        let imported = s.import_file(&archive, "Haze colours").expect("imported");
        assert_eq!(imported.choices.len(), 1);
        assert_eq!(imported.choices[0].len(), 2);
        // Until the user picks, the mod is in the library but off.
        assert_eq!(names(&s.library()), [("Haze colours", false)]);

        let blue = imported.choices[0][1].clone();
        s.choose(&imported.mod_id, std::slice::from_ref(&blue))
            .expect("chosen");
        let view = s.library();
        assert!(view.mods[0].enabled);
        assert_eq!(view.mods[0].variant_groups.len(), 1);
        assert_eq!(
            view.mods[0].variant_groups[0].active.as_deref(),
            Some(blue.as_str())
        );
    }

    #[test]
    fn switching_a_mod_off_keeps_its_place_and_order_can_change() {
        let f = Fixture::new("order");
        let mut s = open(&f);
        for (file, name, path) in [
            ("a.vpk", "A", "models/a/x.vmdl_c"),
            ("b.vpk", "B", "models/b/x.vmdl_c"),
            ("c.vpk", "C", "models/c/x.vmdl_c"),
        ] {
            let src = f.source(file, &[(path, 1)]);
            s.import_file(&src, name).expect("imported");
        }
        assert_eq!(names(&s.library()), [("C", true), ("B", true), ("A", true)]);
        let b = s.library().mods[1].id.clone();
        s.set_enabled(&b, false).expect("off");
        assert_eq!(
            names(&s.library()),
            [("C", true), ("B", false), ("A", true)]
        );
        s.move_mod(&b, 0).expect("moved");
        assert_eq!(
            names(&open(&f).library()),
            [("B", false), ("C", true), ("A", true)]
        );
    }

    #[test]
    fn conflicts_between_enabled_mods_are_shown_with_the_winner_first() {
        let f = Fixture::new("conflict");
        let mut s = open(&f);
        let old = f.source("old.vpk", &[("models/yamato/body.vmdl_c", 1)]);
        let new = f.source("new.vpk", &[("models/yamato/body.vmdl_c", 2)]);
        s.import_file(&old, "Old").expect("imported");
        s.import_file(&new, "New").expect("imported");
        let view = s.library();
        assert_eq!(view.conflicts.len(), 1);
        assert_eq!(view.conflicts[0].winner_mod, view.mods[0].id);
        assert_eq!(view.conflicts[0].loser_mod, view.mods[1].id);
        assert_eq!(view.conflicts[0].paths, ["models/yamato/body.vmdl_c"]);
    }

    #[test]
    fn applying_puts_the_files_into_the_game_and_adds_our_line() {
        let f = Fixture::new("apply");
        let mut s = open(&f);
        let src = f.source("haze.vpk", &[("models/haze/body.vmdl_c", 1)]);
        s.import_file(&src, "Haze").expect("imported");
        let game = f.game();
        let applied = s.apply(&game, false).expect("applied");
        assert!(applied.changed);
        assert!(matches!(applied.gameinfo, GuardOutcome::Added { .. }));
        let addons = game.join("game").join("citadel").join("addons");
        assert_eq!(
            std::fs::read(addons.join("pak01_dir.vpk")).expect("placed"),
            vpk(&[("models/haze/body.vmdl_c", 1)])
        );
        let gameinfo =
            std::fs::read_to_string(game.join("game").join("citadel").join("gameinfo.gi"))
                .expect("gameinfo");
        assert!(gameinfo.contains("citadel/addons"));
        let view = s.library();
        assert_eq!(
            view.applied_profile.as_deref(),
            Some(crate::profiles::DEFAULT_NAME)
        );
        assert!(!view.pending, "nothing left to apply");

        // Switching the mod off is now a pending change until it is applied again.
        let id = view.mods[0].id.clone();
        s.set_enabled(&id, false).expect("off");
        assert!(s.library().pending);
        s.apply(&game, false).expect("applied");
        assert!(!addons.join("pak01_dir.vpk").exists());
    }

    #[test]
    fn a_game_folder_that_is_gone_is_not_recreated() {
        let f = Fixture::new("gone");
        let mut s = open(&f);
        let src = f.source("haze.vpk", &[("models/haze/body.vmdl_c", 1)]);
        s.import_file(&src, "Haze").expect("imported");
        // Steam moved the game, or its drive is unplugged: the folder the profile remembers has no gameinfo.gi.
        let gone = f.dir.join("Deadlock");
        let err = s.apply(&gone, false).expect_err("no game there");
        assert!(
            matches!(err, SessionError::GameNotFound(ref p) if p == &gone),
            "{err}"
        );
        assert_eq!(err.user_message().id, MessageId::E2);
        assert!(!gone.exists(), "nothing was created");
        assert!(s.library().applied_profile.is_none());
    }

    #[test]
    fn the_game_folder_is_remembered_by_the_profile_after_applying() {
        let f = Fixture::new("root");
        let mut s = open(&f);
        assert_eq!(s.game_root(), None, "nothing chosen yet");
        let game = f.game();
        s.apply(&game, false).expect("applied");
        assert_eq!(s.game_root(), Some(game.clone()));
        assert_eq!(open(&f).game_root(), Some(game));
    }

    #[test]
    fn files_put_into_the_library_and_the_game_are_logged() {
        let f = Fixture::new("logged");
        let mut s = open(&f);
        let src = f.source("haze.vpk", &[("models/haze/body.vmdl_c", 1)]);
        s.import_file(&src, "Haze").expect("imported");
        let ops = f.ops();
        assert!(
            ops.iter().any(|(op, dst)| *op == FileOp::Copy
                && dst.contains("library")
                && dst.ends_with("0.vpk")),
            "{ops:?}"
        );
        s.apply(&f.game(), false).expect("applied");
        let ops = f.ops();
        // Two-phase: linked under a temporary name in addons, then renamed into its slot.
        assert!(
            ops.iter()
                .any(|(op, dst)| matches!(op, FileOp::Hardlink | FileOp::Copy)
                    && dst.contains("addons")),
            "{ops:?}"
        );
        assert!(
            ops.iter()
                .any(|(op, dst)| *op == FileOp::Rename && dst.ends_with("pak01_dir.vpk")),
            "{ops:?}"
        );
        assert!(
            ops.iter()
                .any(|(op, dst)| *op == FileOp::Rename && dst.ends_with("gameinfo.gi")),
            "{ops:?}"
        );
        assert!(
            f.log
                .events()
                .iter()
                .all(|(_, e)| e.result == crate::log::Outcome::Ok),
            "a normal apply logs no failures: {ops:?}"
        );
    }

    #[test]
    fn stray_library_files_are_deleted_on_start_and_logged() {
        let f = Fixture::new("stray");
        open(&f);
        let stray = f.data().join("library").join("ghost").join("1");
        std::fs::create_dir_all(&stray).expect("dir");
        std::fs::write(stray.join("0.vpk"), b"x").expect("write");
        let opened = Session::open(&f.data(), f.log.clone()).expect("opened");
        assert!(opened.notices.iter().any(|n| matches!(n, Notice::Swept(1))));
        assert!(
            f.ops()
                .iter()
                .any(|(op, path)| *op == FileOp::Delete && path.ends_with("0.vpk")),
            "{:?}",
            f.ops()
        );
    }

    #[test]
    fn nothing_is_applied_while_the_game_runs() {
        let f = Fixture::new("running");
        let mut s = open(&f);
        let src = f.source("haze.vpk", &[("models/haze/body.vmdl_c", 1)]);
        s.import_file(&src, "Haze").expect("imported");
        let game = f.game();
        let err = s.apply(&game, true).expect_err("running");
        assert!(matches!(
            err,
            SessionError::Engine(crate::engine::EngineError::GameRunning)
        ));
        assert!(!game.join("game").join("citadel").join("addons").exists());
    }

    #[test]
    fn a_removed_mod_leaves_the_library_the_disk_and_every_profile() {
        let f = Fixture::new("remove");
        let mut s = open(&f);
        let src = f.source("haze.vpk", &[("models/haze/body.vmdl_c", 1)]);
        let id = s.import_file(&src, "Haze").expect("imported").mod_id;
        s.create_profile("Evening").expect("created");
        s.select_profile("Evening").expect("selected");
        s.set_enabled(&id, true).expect("on");

        let plan = s.removal_plan(&Target::Mod(id.clone())).expect("plan");
        assert_eq!(plan.profiles.len(), 2);
        assert!(plan.bytes > 0 && plan.applied_in.is_empty());

        s.remove(&Target::Mod(id.clone()), &Permanently)
            .expect("removed");
        assert!(s.library().mods.is_empty());
        assert!(!f.data().join("library").join(&id).exists());
        let again = open(&f);
        assert!(again.library().mods.is_empty());
        assert!(
            f.ops().iter().any(|(op, _)| *op == FileOp::Delete),
            "deletions are logged"
        );
    }

    #[test]
    fn a_mod_still_in_the_game_is_not_removed() {
        let f = Fixture::new("removeapplied");
        let mut s = open(&f);
        let src = f.source("haze.vpk", &[("models/haze/body.vmdl_c", 1)]);
        let id = s.import_file(&src, "Haze").expect("imported").mod_id;
        s.apply(&f.game(), false).expect("applied");
        let err = s
            .remove(&Target::Mod(id.clone()), &Permanently)
            .expect_err("refused");
        assert!(matches!(
            err,
            SessionError::Remove(RemoveError::StillInGame(_))
        ));
        assert_eq!(s.library().mods.len(), 1);
    }

    #[test]
    fn profiles_can_be_renamed_duplicated_and_deleted() {
        let f = Fixture::new("profileops");
        let mut s = open(&f);
        let src = f.source("haze.vpk", &[("models/haze/body.vmdl_c", 1)]);
        s.import_file(&src, "Haze").expect("imported");
        s.rename_profile(crate::profiles::DEFAULT_NAME, "Main")
            .expect("renamed");
        assert_eq!(s.selected_profile(), "Main", "the selection follows");
        s.duplicate_profile("Main", "Copy").expect("duplicated");
        s.select_profile("Copy").expect("selected");
        assert_eq!(
            names(&s.library()),
            [("Haze", true)],
            "the copy has the same mods"
        );
        s.delete_profile("Copy").expect("deleted");
        assert_eq!(s.selected_profile(), "Main", "another profile is selected");
        assert_eq!(s.profile_names(), ["Main"]);
        assert!(matches!(
            s.delete_profile("Main"),
            Err(SessionError::Profile(ProfileError::LastProfile))
        ));
        assert_eq!(open(&f).selected_profile(), "Main");
    }

    #[test]
    fn an_exported_profile_imports_under_a_free_name() {
        let f = Fixture::new("exchange");
        let mut s = open(&f);
        let src = f.source("haze.vpk", &[("models/haze/body.vmdl_c", 1)]);
        s.import_file(&src, "Haze").expect("imported");
        let text = s
            .export_profile(crate::profiles::DEFAULT_NAME)
            .expect("exported");
        let imported = s.import_profile(&text).expect("imported");
        assert_eq!(imported.name, "Default (2)");
        assert!(imported.missing.is_empty());
        assert_eq!(
            s.selected_profile(),
            "Default (2)",
            "selected, not applied (F14)"
        );
        assert_eq!(names(&s.library()), [("Haze", true)]);
    }

    #[test]
    fn settings_changes_are_saved_and_invalid_ones_refused() {
        let f = Fixture::new("settings");
        let mut s = open(&f);
        let mut changed = s.settings().clone();
        changed.downloads.parallel = 2;
        s.update_settings(changed).expect("saved");
        assert_eq!(open(&f).settings().downloads.parallel, 2);
        let mut bad = s.settings().clone();
        bad.downloads.parallel = 999;
        assert!(matches!(
            s.update_settings(bad),
            Err(SessionError::Settings(_))
        ));
        assert_eq!(open(&f).settings().downloads.parallel, 2);
    }

    #[test]
    fn profiles_can_be_switched_and_each_keeps_its_own_setup() {
        let f = Fixture::new("profiles");
        let mut s = open(&f);
        let src = f.source("haze.vpk", &[("models/haze/body.vmdl_c", 1)]);
        s.import_file(&src, "Haze").expect("imported");
        s.create_profile("Evening").expect("created");
        s.select_profile("Evening").expect("selected");
        // A new profile starts empty: the mod is in the library but off here.
        assert_eq!(names(&s.library()), [("Haze", false)]);
        s.select_profile(crate::profiles::DEFAULT_NAME)
            .expect("selected");
        assert_eq!(names(&s.library()), [("Haze", true)]);
        assert_eq!(open(&f).selected_profile(), crate::profiles::DEFAULT_NAME);
        s.select_profile("Evening").expect("selected");
        assert_eq!(
            open(&f).selected_profile(),
            "Evening",
            "the selection is remembered"
        );
    }

    #[test]
    fn a_damaged_manifest_stops_the_start_instead_of_losing_the_library() {
        let f = Fixture::new("damaged");
        open(&f);
        std::fs::write(f.data().join("manifest.json"), "{ not json").expect("damage");
        let err = Session::open(&f.data(), f.log.clone()).expect_err("refused");
        assert!(matches!(err, SessionError::Manifest(_)));
        assert!(f.data().join("manifest.json").exists(), "left for the user");
    }

    /// Haze and Wraith imported (Wraith on top) and applied to a fresh game folder.
    fn applied_two(f: &Fixture) -> (Session, PathBuf, String, String) {
        let mut s = open(f);
        let haze = s
            .import_file(
                &f.source("haze.vpk", &[("models/haze/body.vmdl_c", 1)]),
                "Haze",
            )
            .expect("haze")
            .mod_id;
        let wraith = s
            .import_file(
                &f.source("wraith.vpk", &[("models/wraith/body.vmdl_c", 1)]),
                "Wraith",
            )
            .expect("wraith")
            .mod_id;
        let game = f.game();
        s.apply(&game, false).expect("applied");
        (s, game, haze, wraith)
    }

    fn addons(game: &Path) -> PathBuf {
        game.join("game").join("citadel").join("addons")
    }

    #[test]
    fn edits_not_in_the_game_yet_are_listed_counted_and_badged() {
        let f = Fixture::new("pendinglist");
        let (mut s, _, haze, wraith) = applied_two(&f);
        assert!(s.pending_view(None).changes.is_empty());
        assert_eq!(s.library().pending_count, 0);

        s.set_enabled(&haze, false).expect("off");
        let view = s.pending_view(None);
        assert!(view.switch.is_none());
        assert_eq!(view.changes.len(), 1);
        assert_eq!(view.changes[0].mod_id, haze);
        assert_eq!(view.changes[0].change, crate::pending::Change::Disable);
        assert_eq!(
            (
                view.folder.added,
                view.folder.removed,
                view.folder.renumbered
            ),
            (0, 1, 0)
        );
        let library = s.library();
        assert_eq!(library.pending_count, 1);
        let badge = |id: &str| {
            library
                .mods
                .iter()
                .find(|m| m.id == id)
                .expect("mod")
                .pending
        };
        assert!(badge(&haze) && !badge(&wraith));
    }

    #[test]
    fn a_pending_change_is_taken_back_one_by_one_or_all_at_once() {
        let f = Fixture::new("pendingrevert");
        let (mut s, _, haze, wraith) = applied_two(&f);
        s.set_enabled(&haze, false).expect("off");
        s.move_mod(&haze, 0).expect("moved");
        s.revert_pending(&haze, crate::pending::Aspect::Enabled, None)
            .expect("reverted");
        assert_eq!(s.pending_view(None).changes.len(), 1, "the move is left");
        s.discard_pending(None).expect("discarded");
        assert!(s.pending_view(None).changes.is_empty());
        let order: Vec<String> = s.library().mods.into_iter().map(|m| m.id).collect();
        assert_eq!(order, [wraith.clone(), haze.clone()]);
        assert_eq!(open(&f).library().pending_count, 0, "saved");
    }

    #[test]
    fn a_profile_switch_is_one_change_and_discarding_it_selects_the_applied_profile_again() {
        let f = Fixture::new("pendingswitch");
        let (mut s, game, _, _) = applied_two(&f);
        s.create_profile("Evening").expect("created");
        s.select_profile("Evening").expect("selected");
        // A profile that was never applied goes to the first game folder (the caller passes Steam's).
        let view = s.pending_view(Some(&game));
        let switch = view.switch.expect("a switch");
        assert_eq!(switch.from.as_deref(), Some(crate::profiles::DEFAULT_NAME));
        assert_eq!(switch.to, "Evening");
        assert_eq!((switch.turned_on, switch.turned_off), (0, 2));
        assert!(view.changes.is_empty());
        assert_eq!(view.folder.removed, 2);
        assert_eq!(s.library().pending_count, 1);
        s.discard_pending(Some(&game))
            .expect("kept the applied one");
        assert_eq!(s.selected_profile(), crate::profiles::DEFAULT_NAME);
    }

    #[test]
    fn a_fresh_empty_profile_has_nothing_pending() {
        let f = Fixture::new("pendingfresh");
        let s = open(&f);
        let view = s.pending_view(Some(&f.game()));
        assert!(view.switch.is_none() && view.changes.is_empty());
        assert_eq!(s.library().pending_count, 0);
    }

    #[test]
    fn the_game_state_before_a_change_can_be_restored_as_an_edit() {
        let f = Fixture::new("pendingrestore");
        let (mut s, game, haze, _) = applied_two(&f);
        s.set_enabled(&haze, false).expect("off");
        // What the game had before the waiting change goes in (143's Undo).
        let before = s.profile_matching_game(None).expect("a profile");
        s.apply(&game, false).expect("applied");
        assert!(s.pending_view(None).changes.is_empty());
        s.restore_profile(&before).expect("restored");
        let changes = s.pending_view(None).changes;
        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0].change, crate::pending::Change::Enable);
    }

    #[test]
    fn mods_in_the_game_say_how_they_got_there() {
        let f = Fixture::new("placement");
        let (s, _, haze, _) = applied_two(&f);
        let view = s.library();
        let m = view.mods.iter().find(|m| m.id == haze).expect("haze");
        // The test folders share a drive, so the files are hard links.
        assert_eq!(
            m.in_game,
            Some(InGameView {
                copied: false,
                bytes: m.bytes
            })
        );
        let mut s = s;
        s.set_enabled(&haze, false).expect("off");
        assert!(
            s.library()
                .mods
                .iter()
                .find(|m| m.id == haze)
                .expect("haze")
                .in_game
                .is_some(),
            "still in the game until applied"
        );
    }

    fn foreign_game(f: &Fixture) -> PathBuf {
        let game = f.game();
        let addons = addons(&game);
        std::fs::create_dir_all(addons.join("replays")).expect("addons");
        std::fs::write(
            addons.join("pak01_dir.vpk"),
            vpk(&[
                ("models/heroes/haze/body.vmdl_c", 1),
                ("models/heroes/haze/head.vmdl_c", 2),
            ]),
        )
        .expect("pak");
        std::fs::write(addons.join(".dmm.json"), b"{}").expect("dmm");
        std::fs::write(addons.join("replays").join("a.dem"), b"aaaa").expect("dem");
        std::fs::write(addons.join("replays").join("b.dem"), b"bb").expect("dem");
        game
    }

    #[test]
    fn foreign_items_are_listed_with_what_they_seem_to_be() {
        let f = Fixture::new("foreignlist");
        let s = open(&f);
        let game = foreign_game(&f);
        let items = s.foreign(&game);
        let names: Vec<&str> = items.iter().map(|i| i.name.as_str()).collect();
        assert_eq!(names, [".dmm.json", "pak01_dir.vpk", "replays"]);
        let pak = &items[1];
        assert_eq!(pak.kind, crate::addons::ForeignKind::ModFile);
        assert_eq!(pak.slot, Some(1));
        assert!(pak.takeable && !pak.left);
        assert_eq!(
            pak.vpk,
            Some(VpkSummary {
                entries: 2,
                hero: Some("haze".into())
            })
        );
        assert!(pak.modified_unix_ms.is_some());
        let dmm = &items[0];
        assert_eq!(dmm.origin, Some(crate::addons::Origin::DeadlockModManager));
        assert!(!dmm.takeable && dmm.vpk.is_none());
        let replays = &items[2];
        assert_eq!(replays.kind, crate::addons::ForeignKind::Folder);
        assert_eq!((replays.files, replays.bytes), (Some(2), 6));
    }

    #[test]
    fn leaving_a_foreign_item_as_it_is_is_remembered() {
        let f = Fixture::new("foreignleave");
        let mut s = open(&f);
        let game = foreign_game(&f);
        s.leave_foreign(&game, "replays").expect("left");
        let again = open(&f);
        let items = again.foreign(&game);
        assert!(
            items
                .iter()
                .find(|i| i.name == "replays")
                .expect("listed")
                .left
        );
        assert!(
            !items
                .iter()
                .find(|i| i.name == "pak01_dir.vpk")
                .expect("listed")
                .left
        );
        assert!(addons(&game).join("replays").exists(), "never touched");
    }

    #[test]
    fn only_a_listed_foreign_item_can_be_left_as_it_is() {
        let f = Fixture::new("foreignleaveunknown");
        let mut s = open(&f);
        let game = foreign_game(&f);
        assert!(matches!(
            s.leave_foreign(&game, "nothing_here.vpk"),
            Err(SessionError::NotForeign(_))
        ));
        assert!(
            open(&f)
                .manifest()
                .installations
                .iter()
                .all(|i| i.left_foreign.is_empty()),
            "nothing written"
        );
    }

    #[test]
    fn a_foreign_mod_taken_under_management_is_copied_and_then_replaced_by_ours() {
        let f = Fixture::new("foreigntake");
        let mut s = open(&f);
        let game = foreign_game(&f);
        let original = std::fs::read(addons(&game).join("pak01_dir.vpk")).expect("pak");
        s.import_file(
            &f.source("wraith.vpk", &[("models/wraith/body.vmdl_c", 1)]),
            "Wraith",
        )
        .expect("wraith");
        let id = s
            .take_foreign(&game, "pak01_dir.vpk", "Haze skin", false)
            .expect("taken");
        assert!(
            !addons(&game).join("pak01_dir.vpk").exists(),
            "the original left"
        );
        let view = s.library();
        let m = view
            .mods
            .iter()
            .find(|m| m.id == id)
            .expect("in the library");
        assert_eq!(m.name, "Haze skin");
        assert!(m.enabled);
        assert_eq!(
            view.mods.last().map(|m| m.id.as_str()),
            Some(id.as_str()),
            "at the bottom"
        );
        assert!(
            matches!(&m.source, Source::Local { imported_from: Some(p) } if p.ends_with("pak01_dir.vpk"))
        );
        assert!(
            f.ops()
                .iter()
                .any(|(op, path)| *op == FileOp::Delete && path.ends_with("pak01_dir.vpk")),
            "logged"
        );

        s.apply(&game, false).expect("applied");
        // Ours now: Wraith on top in slot 1, the taken mod below it in slot 2.
        assert_eq!(
            std::fs::read(addons(&game).join("pak02_dir.vpk")).expect("ours"),
            original
        );
        assert!(
            s.foreign(&game)
                .iter()
                .all(|i| i.name != "pak01_dir.vpk" && i.name != "pak02_dir.vpk")
        );
    }

    #[test]
    fn taking_over_waits_while_the_game_runs_and_refuses_what_is_not_a_mod() {
        let f = Fixture::new("foreignrefuse");
        let mut s = open(&f);
        let game = foreign_game(&f);
        assert!(matches!(
            s.take_foreign(&game, "pak01_dir.vpk", "Haze", true),
            Err(SessionError::Engine(
                crate::engine::EngineError::GameRunning
            ))
        ));
        assert!(matches!(
            s.take_foreign(&game, "replays", "Replays", false),
            Err(SessionError::NotTakeable(_))
        ));
        assert!(addons(&game).join("pak01_dir.vpk").exists());
        assert!(s.library().mods.is_empty());
    }

    #[test]
    fn a_multi_part_pak_of_another_tool_is_not_taken() {
        let f = Fixture::new("foreignchunks");
        let s = open(&f);
        let game = foreign_game(&f);
        std::fs::write(addons(&game).join("pak01_000.vpk"), b"data").expect("chunk");
        let items = s.foreign(&game);
        assert!(
            !items
                .iter()
                .find(|i| i.name == "pak01_dir.vpk")
                .expect("listed")
                .takeable
        );
    }

    #[test]
    fn checking_gameinfo_again_adds_our_line_only_to_the_original() {
        let f = Fixture::new("gifix");
        let mut s = open(&f);
        let game = f.game();
        let file = game.join("game").join("citadel").join("gameinfo.gi");
        assert!(matches!(
            s.fix_gameinfo(&game, true),
            Err(SessionError::Engine(
                crate::engine::EngineError::GameRunning
            ))
        ));
        assert!(matches!(
            s.fix_gameinfo(&game, false).expect("fixed"),
            GuardOutcome::Added { .. }
        ));
        assert!(
            std::fs::read_to_string(&file)
                .expect("gi")
                .contains("citadel/addons")
        );
        let foreign = VANILLA.replace(
            "\t\t\tGame\t\t\t\tcore\r\n",
            "\t\t\tGame\t\t\t\tcore\r\n\t\t\tGame\tcitadel/addons/dmm\r\n",
        );
        std::fs::write(&file, &foreign).expect("dmm");
        assert!(matches!(
            s.fix_gameinfo(&game, false).expect("checked"),
            GuardOutcome::Foreign(_)
        ));
        assert_eq!(std::fs::read_to_string(&file).expect("gi"), foreign);
    }

    #[test]
    fn unknown_mods_are_refused_plainly() {
        let f = Fixture::new("unknown");
        let mut s = open(&f);
        assert!(matches!(
            s.set_enabled("nope", true),
            Err(SessionError::UnknownMod(id)) if id == "nope"
        ));
        assert!(matches!(
            s.move_mod("nope", 0),
            Err(SessionError::UnknownMod(_))
        ));
    }
}
