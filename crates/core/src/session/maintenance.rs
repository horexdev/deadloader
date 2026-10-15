//! Settings → Game, Advanced and Reset on the open session: rollback of a game folder (D7, F11), the installations
//! list, the selected profile's game folder, the Advanced editor's two files and "Reset all settings" (decisions 20,
//! 100, 156). File operations go through the logged file system or are recorded in the files log.

use super::*;
use crate::addons::{ForeignKind, Origin};
use crate::engine::{RollbackReport, scan_installation};
use crate::gamedir::remove_mods_path;
use crate::library::manifest_from_json;
use crate::settings::Problem;
use crate::setup::{GameinfoState, gameinfo_state};
use time::OffsetDateTime;

/// A file in a game's `addons` folder that deadloader didn't create.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ForeignFile {
    pub name: String,
    pub bytes: u64,
    /// Recognised as Deadlock Mod Manager's.
    pub from_dmm: bool,
}

/// Another game folder deadloader has files in (D7: "not touched").
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OtherInstallation {
    pub game_root: String,
    pub applied_profile: Option<String>,
    pub files: usize,
}

/// What rolling back one game folder would do (D7).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RollbackPreview {
    pub game_root: String,
    /// Our mod files in its `addons` folder now.
    pub files: usize,
    pub bytes: u64,
    pub gameinfo: GameinfoState,
    /// Foreign files (never folders) the user may remove too.
    pub foreign: Vec<ForeignFile>,
    /// Profiles that use this folder (the selected one first).
    pub profiles: Vec<String>,
    pub selected_profile: String,
    pub all_profiles: Vec<String>,
    pub library_mods: usize,
    pub library_bytes: u64,
    pub others: Vec<OtherInstallation>,
}

/// What a rollback did.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RolledBack {
    pub game_root: String,
    pub removed: usize,
    pub foreign_removed: Vec<String>,
    /// Our line was taken out of `gameinfo.gi`.
    pub gameinfo_restored: bool,
    /// `gameinfo.gi` afterwards.
    pub gameinfo: GameinfoState,
}

/// Where a game installation in the list comes from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InstallSource {
    /// Found through Steam (21/69).
    Steam,
    /// Added by hand in Settings → Game.
    Manual,
    /// Neither, but deadloader put files there before (moved, or Steam no longer lists it).
    Known,
}

/// A detected installation, as the core's Steam discovery returns it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Detected {
    pub game_root: PathBuf,
    pub library: Option<PathBuf>,
    pub build_id: Option<u64>,
}

/// One row of Settings → Game → Game installations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct InstallationView {
    pub game_root: String,
    pub source: InstallSource,
    pub library: Option<String>,
    pub build_id: Option<u64>,
    /// Profiles that use it; the selected profile is listed first.
    pub profiles: Vec<String>,
    pub applied_profile: Option<String>,
    /// The selected profile uses it.
    pub selected: bool,
    pub gameinfo: GameinfoState,
    /// Our files in it now, by the manifest.
    pub files: usize,
}

/// Why the Advanced editor didn't save a file (decision 156): nothing was written.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum EditRefused {
    /// Not valid JSON.
    Json { line: usize, column: usize },
    /// Made by a newer deadloader.
    NewerSchema { found: u32, supported: u32 },
    /// Valid JSON, but values that can't be used.
    Problems { problems: Vec<EditProblem> },
    /// Valid JSON, but not a file of this kind (no schema, wrong shape, an unsafe library path…).
    Invalid { detail: String },
    /// The file couldn't be written.
    Failed { message: UserMessage },
}

/// One field the editor refused, for the UI's own text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct EditProblem {
    /// Dotted path in the file, e.g. `downloads.parallel`.
    pub path: String,
    #[serde(flatten)]
    pub problem: ProblemKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "problem", rename_all = "snake_case")]
pub enum ProblemKind {
    UnknownField,
    /// The technical reason is English, for Details.
    InvalidValue {
        detail: String,
    },
    OutOfRange {
        found: u64,
        min: u32,
        max: u32,
    },
    InvalidEntry {
        entry: String,
    },
    DuplicateEntry {
        entry: String,
    },
    NeedsAgeConfirmation,
    UnavailableInLocalMode,
}

impl From<&SettingsError> for EditRefused {
    fn from(e: &SettingsError) -> Self {
        match e {
            SettingsError::Json { line, column, .. } => Self::Json {
                line: *line,
                column: *column,
            },
            SettingsError::NewerSchema { found, supported } => Self::NewerSchema {
                found: *found,
                supported: *supported,
            },
            SettingsError::Rejected(warnings) => Self::Problems {
                problems: warnings
                    .iter()
                    .map(|w| EditProblem {
                        path: w.path.clone(),
                        problem: match &w.problem {
                            Problem::UnknownField => ProblemKind::UnknownField,
                            Problem::InvalidValue { reason } => ProblemKind::InvalidValue {
                                detail: reason.clone(),
                            },
                            Problem::OutOfRange { found, min, max } => ProblemKind::OutOfRange {
                                found: *found,
                                min: *min,
                                max: *max,
                            },
                            Problem::InvalidEntry { entry } => ProblemKind::InvalidEntry {
                                entry: entry.clone(),
                            },
                            Problem::DuplicateEntry { entry } => ProblemKind::DuplicateEntry {
                                entry: entry.clone(),
                            },
                            Problem::NeedsAgeConfirmation => ProblemKind::NeedsAgeConfirmation,
                            Problem::UnavailableInLocalMode => ProblemKind::UnavailableInLocalMode,
                        },
                    })
                    .collect(),
            },
            SettingsError::Io(_) => Self::Failed {
                message: e.user_message(),
            },
            other => Self::Invalid {
                detail: other.to_string(),
            },
        }
    }
}

impl From<&ModelError> for EditRefused {
    fn from(e: &ModelError) -> Self {
        match e {
            ModelError::Json { line, column, .. } => Self::Json {
                line: *line,
                column: *column,
            },
            ModelError::NewerSchema { found, supported } => Self::NewerSchema {
                found: *found,
                supported: *supported,
            },
            ModelError::Io(_) => Self::Failed {
                message: data_file(e, "manifest.json"),
            },
            other => Self::Invalid {
                detail: other.to_string(),
            },
        }
    }
}

/// A file saved from the Advanced editor or by a reset.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct EditSaved {
    /// The dated backup of the previous file (`None` when there was no file yet).
    pub backup: Option<String>,
}

fn same_root(a: &str, b: &str) -> bool {
    let norm = |s: &str| {
        s.replace('/', "\\")
            .trim_end_matches('\\')
            .to_ascii_lowercase()
    };
    norm(a) == norm(b)
}

impl Session {
    /// Every profile, as saved.
    pub fn profiles(&self) -> &[Profile] {
        &self.profiles
    }

    /// The game folder the selected profile uses, set by hand (Settings → Game, the setup step); `None` lets it follow
    /// the first installation Steam finds again.
    pub fn set_game_root(&mut self, game_root: Option<&Path>) -> Result<(), SessionError> {
        let root = game_root.map(|r| r.display().to_string());
        self.edit_profile(|p| p.game_root = root)
    }

    /// Profiles that use `game_root`: their own folder, or the one they were last applied to. The selected one first.
    pub fn profiles_using(&self, game_root: &Path) -> Vec<String> {
        let root = game_root.display().to_string();
        let applied: Option<&str> = self
            .manifest
            .installations
            .iter()
            .find(|i| same_root(&i.game_root, &root))
            .and_then(|i| i.applied_profile.as_deref());
        let mut names: Vec<String> = self
            .profiles
            .iter()
            .filter(|p| match &p.game_root {
                Some(r) => same_root(r, &root),
                None => applied == Some(p.name.as_str()),
            })
            .map(|p| p.name.clone())
            .collect();
        names.sort_by_key(|n| *n != self.selected);
        names
    }

    /// Settings → Game → Game installations: those Steam finds, those added by hand, and any other folder deadloader
    /// still has files in.
    pub fn installations(&self, detected: &[Detected]) -> Vec<InstallationView> {
        fn add<'d>(
            rows: &mut Vec<(PathBuf, InstallSource, Option<&'d Detected>)>,
            root: PathBuf,
            source: InstallSource,
            found: Option<&'d Detected>,
        ) {
            let text = root.display().to_string();
            if !rows
                .iter()
                .any(|(r, _, _)| same_root(&r.display().to_string(), &text))
            {
                rows.push((root, source, found));
            }
        }
        let mut rows = Vec::new();
        for d in detected {
            add(
                &mut rows,
                d.game_root.clone(),
                InstallSource::Steam,
                Some(d),
            );
        }
        for m in &self.settings.game.manual_installations {
            add(&mut rows, PathBuf::from(m), InstallSource::Manual, None);
        }
        for i in &self.manifest.installations {
            if !i.applied.is_empty() {
                add(
                    &mut rows,
                    PathBuf::from(&i.game_root),
                    InstallSource::Known,
                    None,
                );
            }
        }
        let selected_root = self.game_root().map(|r| r.display().to_string());
        rows.into_iter()
            .map(|(root, source, found)| {
                let text = root.display().to_string();
                let state = self
                    .manifest
                    .installations
                    .iter()
                    .find(|i| same_root(&i.game_root, &text));
                InstallationView {
                    source,
                    library: found
                        .and_then(|d| d.library.as_ref())
                        .map(|l| l.display().to_string()),
                    build_id: found.and_then(|d| d.build_id),
                    profiles: self.profiles_using(&root),
                    applied_profile: state.and_then(|i| i.applied_profile.clone()),
                    selected: selected_root
                        .as_deref()
                        .is_some_and(|s| same_root(s, &text)),
                    gameinfo: gameinfo_state(&Disk, &root),
                    files: state.map_or(0, |i| i.applied.len()),
                    game_root: text,
                }
            })
            .collect()
    }

    /// D7: what rolling back `game_root` would remove, and what stays.
    pub fn rollback_preview(&self, game_root: &Path) -> Result<RollbackPreview, SessionError> {
        let found = scan_installation(&Disk, &self.manifest, game_root)?;
        let root = game_root.display().to_string();
        let records = self
            .manifest
            .installations
            .iter()
            .find(|i| same_root(&i.game_root, &root));
        let bytes = found
            .ours
            .iter()
            .filter_map(|o| {
                records.and_then(|i| i.applied.iter().find(|r| r.slot == o.slot).map(|r| r.len))
            })
            .sum();
        let foreign = found
            .foreign
            .iter()
            .filter(|f| f.kind != ForeignKind::Folder)
            .map(|f| ForeignFile {
                name: f.name.clone(),
                bytes: f.len,
                from_dmm: f.origin == Some(Origin::DeadlockModManager),
            })
            .collect();
        let others = self
            .manifest
            .installations
            .iter()
            .filter(|i| !same_root(&i.game_root, &root) && !i.applied.is_empty())
            .map(|i| OtherInstallation {
                game_root: i.game_root.clone(),
                applied_profile: i.applied_profile.clone(),
                files: i.applied.len(),
            })
            .collect();
        Ok(RollbackPreview {
            game_root: root,
            files: found.ours.len(),
            bytes,
            gameinfo: gameinfo_state(&Disk, game_root),
            foreign,
            profiles: self.profiles_using(game_root),
            selected_profile: self.selected.clone(),
            all_profiles: self.profile_names(),
            library_mods: self.manifest.mods.len(),
            library_bytes: self
                .manifest
                .mods
                .iter()
                .flat_map(|m| &m.files)
                .map(|f| f.len)
                .sum(),
            others,
        })
    }

    /// D7 / F11: takes our files out of `game_root` (and foreign files when asked) through the journal, then our line
    /// out of `gameinfo.gi` after a backup. The library and the profiles stay; Apply brings everything back.
    pub fn rollback(
        &mut self,
        game_root: &Path,
        remove_foreign: bool,
        game_running: bool,
    ) -> Result<RolledBack, SessionError> {
        let change = crate::log::InChange {
            log: &*self.log,
            txn: unique_number(),
            change: crate::log::Change::Rollback,
        };
        let fs = Logged {
            fs: &Disk,
            log: &change,
        };
        let journal = FileJournal::new(self.data_dir.join("journal.jsonl"));
        let engine = Engine {
            fs: &fs,
            journal: &journal,
            data_dir: &self.data_dir,
            direction: self.direction,
        };
        let manifest_path = self.manifest_path();
        let report: RollbackReport = engine
            .rollback(
                &mut self.manifest,
                game_root,
                remove_foreign,
                game_running,
                unique_number(),
                |m| save_io(&manifest_path, m),
            )
            .map_err(SessionError::Engine)?;
        let gameinfo = remove_mods_path(
            &fs,
            game_root,
            &self.data_dir.join("backups").join("gameinfo"),
            &format!("{}.gi", unique_number()),
        )?;
        Ok(RolledBack {
            game_root: game_root.display().to_string(),
            removed: report.removed,
            foreign_removed: report.foreign_removed,
            gameinfo_restored: matches!(gameinfo, GuardOutcome::Removed { .. }),
            gameinfo: gameinfo_state(&Disk, game_root),
        })
    }

    /// D8's "roll back first": every folder deadloader has files or its line in (`known` adds the installations Steam
    /// or the user listed). Foreign files stay. Stops at the first failure.
    pub fn rollback_all(
        &mut self,
        known: &[PathBuf],
        game_running: bool,
    ) -> Result<Vec<RolledBack>, SessionError> {
        let mut roots: Vec<PathBuf> = Vec::new();
        let recorded = self
            .manifest
            .installations
            .iter()
            .map(|i| PathBuf::from(&i.game_root));
        for root in recorded.chain(known.iter().cloned()) {
            let text = root.display().to_string();
            if !roots
                .iter()
                .any(|r| same_root(&r.display().to_string(), &text))
            {
                roots.push(root);
            }
        }
        let mut done = Vec::new();
        for root in roots {
            if gameinfo_state(&Disk, &root) == GameinfoState::Missing
                && !self.manifest.installations.iter().any(|i| {
                    same_root(&i.game_root, &root.display().to_string()) && !i.applied.is_empty()
                })
            {
                continue;
            }
            done.push(self.rollback(&root, false, game_running)?);
        }
        Ok(done)
    }

    /// The text of `settings.json` for the Advanced editor: the file as it is, or the current settings when there is
    /// no file yet.
    pub fn settings_text(&self) -> String {
        std::fs::read_to_string(self.data_dir.join(settings::FILE_NAME))
            .ok()
            .or_else(|| settings::to_json(&self.settings).ok())
            .unwrap_or_default()
    }

    /// The Advanced editor's save (156): checked first and never saved when anything is wrong; the previous file is kept
    /// as a dated backup.
    pub fn save_settings_text(
        &mut self,
        text: &str,
        at: OffsetDateTime,
    ) -> Result<EditSaved, EditRefused> {
        let parsed = settings::from_json_strict(text).map_err(|e| EditRefused::from(&e))?;
        let backup = self.keep_backup(&self.data_dir.join(settings::FILE_NAME), at)?;
        self.update_settings(parsed)
            .map_err(|e| EditRefused::Failed {
                message: e.user_message(),
            })?;
        self.record_saved(
            settings::FILE_NAME,
            "settings saved from the Advanced editor",
        );
        Ok(EditSaved { backup })
    }

    /// The text of `manifest.json` for the Advanced editor.
    pub fn manifest_text(&self) -> String {
        std::fs::read_to_string(self.manifest_path())
            .ok()
            .or_else(|| crate::library::to_json(&self.manifest).ok())
            .unwrap_or_default()
    }

    /// Like [`Session::save_settings_text`], for the manifest. Profiles lose entries for mods that are no longer in it.
    pub fn save_manifest_text(
        &mut self,
        text: &str,
        at: OffsetDateTime,
    ) -> Result<EditSaved, EditRefused> {
        let manifest = manifest_from_json(text).map_err(|e| EditRefused::from(&e))?;
        let backup = self.keep_backup(&self.manifest_path(), at)?;
        save_manifest(&self.manifest_path(), &manifest).map_err(|e| EditRefused::from(&e))?;
        self.manifest = manifest;
        self.record_saved("manifest.json", "manifest saved from the Advanced editor");
        let dir = self.profiles_dir();
        for profile in &mut self.profiles {
            if forget_missing(profile, &self.manifest) {
                profiles::save(&dir, profile).map_err(|e| EditRefused::Failed {
                    message: e.user_message(),
                })?;
            }
        }
        self.indexes.clear();
        self.damaged.clear();
        let ids: Vec<String> = self
            .manifest
            .mods
            .iter()
            .flat_map(|m| &m.files)
            .map(|f| f.id.clone())
            .collect();
        self.index(&ids);
        Ok(EditSaved { backup })
    }

    /// "Reset all settings" (100, 156): a dated backup, then the defaults (keeping what [`Settings::reset`] keeps).
    pub fn reset_settings(&mut self, at: OffsetDateTime) -> Result<EditSaved, SessionError> {
        let backup = self.keep_backup_io(&self.data_dir.join(settings::FILE_NAME), at)?;
        let fresh = self.settings.reset();
        self.update_settings(fresh)?;
        self.record_saved(settings::FILE_NAME, "settings reset to their defaults");
        Ok(EditSaved { backup })
    }

    fn keep_backup(&self, file: &Path, at: OffsetDateTime) -> Result<Option<String>, EditRefused> {
        self.keep_backup_io(file, at)
            .map_err(|e| EditRefused::Failed {
                message: from_io(&e, Area::Data),
            })
    }

    /// A dated backup of `file` (logged), and its file name.
    fn keep_backup_io(&self, file: &Path, at: OffsetDateTime) -> io::Result<Option<String>> {
        let Some(kept) = crate::backups::keep(file, at)? else {
            return Ok(None);
        };
        self.log.record(
            "backup kept",
            &FileEvent {
                src: Some(file.display().to_string()),
                dst: Some(kept.backup.display().to_string()),
                ..FileEvent::new(FileOp::Copy)
            },
        );
        for old in &kept.pruned {
            self.log.record(
                "old backup deleted",
                &FileEvent {
                    src: Some(old.display().to_string()),
                    ..FileEvent::new(FileOp::Delete)
                },
            );
        }
        Ok(kept
            .backup
            .file_name()
            .map(|n| n.to_string_lossy().into_owned()))
    }

    fn record_saved(&self, name: &str, msg: &str) {
        let path = self.data_dir.join(name);
        self.log.record(
            msg,
            &FileEvent {
                dst: Some(path.display().to_string()),
                bytes: std::fs::metadata(&path).ok().map(|m| m.len()),
                ..FileEvent::new(FileOp::Write)
            },
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::log::MemFileLog;
    use crate::testing::{remove_all, temp_dir, vpk};
    use std::sync::Arc;

    const VANILLA: &str = "\"GameInfo\"\r\n{\r\n\tFileSystem\r\n\t{\r\n\t\tSearchPaths\r\n\t\t{\r\n\t\t\tGame\t\t\t\tcitadel\r\n\t\t\tGame\t\t\t\tcore\r\n\t\t}\r\n\t}\r\n}\r\n";

    struct Fixture {
        dir: PathBuf,
        log: Arc<MemFileLog>,
    }

    impl Fixture {
        fn new(name: &str) -> Self {
            Self {
                dir: temp_dir("maintenance", name),
                log: Arc::new(MemFileLog::default()),
            }
        }
        fn data(&self) -> PathBuf {
            self.dir.join("deadloader-data")
        }
        fn game(&self, name: &str) -> PathBuf {
            let root = self.dir.join(name);
            let citadel = root.join("game").join("citadel");
            if !citadel.join("gameinfo.gi").exists() {
                std::fs::create_dir_all(&citadel).expect("game dir");
                std::fs::write(citadel.join("gameinfo.gi"), VANILLA).expect("gameinfo");
            }
            root
        }
        fn addons(&self, name: &str) -> PathBuf {
            self.game(name).join("game").join("citadel").join("addons")
        }
        fn source(&self, name: &str, path: &str) -> PathBuf {
            let file = self.dir.join(name);
            std::fs::write(&file, vpk(&[(path, 1)])).expect("vpk");
            file
        }
        fn open(&self) -> Session {
            Session::open(&self.data(), self.log.clone())
                .expect("opened")
                .session
        }
        /// Two mods applied to "Deadlock", with DMM's file and a foreign pak next to them.
        fn applied(&self) -> Session {
            let mut s = self.open();
            s.import_file(&self.source("haze.vpk", "models/haze/body.vmdl_c"), "Haze")
                .expect("haze");
            s.import_file(&self.source("ivy.vpk", "models/ivy/body.vmdl_c"), "Ivy")
                .expect("ivy");
            let addons = self.addons("Deadlock");
            std::fs::create_dir_all(&addons).expect("addons");
            std::fs::write(addons.join("pak01_dir.vpk"), b"someone else").expect("foreign");
            std::fs::write(addons.join(".dmm.json"), b"{}").expect("dmm");
            s.apply(&self.game("Deadlock"), false).expect("applied");
            s
        }
    }

    impl Drop for Fixture {
        fn drop(&mut self) {
            remove_all(&self.dir);
        }
    }

    fn gameinfo(root: &Path) -> String {
        std::fs::read_to_string(root.join("game").join("citadel").join("gameinfo.gi"))
            .expect("gameinfo")
    }

    #[test]
    fn the_preview_says_what_a_rollback_removes_and_what_stays() {
        let f = Fixture::new("preview");
        let s = f.applied();
        let p = s.rollback_preview(&f.game("Deadlock")).expect("preview");
        assert_eq!(p.files, 2);
        assert!(p.bytes > 0);
        assert_eq!(p.gameinfo, GameinfoState::Ours);
        let mut foreign: Vec<(&str, bool)> = p
            .foreign
            .iter()
            .map(|x| (x.name.as_str(), x.from_dmm))
            .collect();
        foreign.sort();
        assert_eq!(foreign, [(".dmm.json", true), ("pak01_dir.vpk", false)]);
        assert_eq!(p.profiles, ["Default"]);
        assert_eq!(p.selected_profile, "Default");
        assert_eq!(p.library_mods, 2);
        assert!(p.others.is_empty());
    }

    #[test]
    fn a_rollback_says_what_its_file_operations_were() {
        let f = Fixture::new("rollbackchange");
        let mut s = f.applied();
        let at = f.log.events().len();
        s.rollback(&f.game("Deadlock"), false, false)
            .expect("rolled back");
        let events = f.log.events();
        let since = &events[at..];
        assert!(!since.is_empty());
        assert!(
            since
                .iter()
                .all(|(_, e)| e.change == Some(crate::log::Change::Rollback) && e.txn.is_some()),
            "{since:?}"
        );
    }

    #[test]
    fn rollback_takes_our_files_and_line_out_and_apply_brings_them_back() {
        let f = Fixture::new("rollback");
        let mut s = f.applied();
        let game = f.game("Deadlock");
        let done = s.rollback(&game, false, false).expect("rolled back");
        assert_eq!(done.removed, 2);
        assert!(done.gameinfo_restored);
        assert_eq!(done.gameinfo, GameinfoState::Vanilla);
        assert_eq!(gameinfo(&game), VANILLA, "exactly as Steam installed it");
        let addons = f.addons("Deadlock");
        assert!(addons.join("pak01_dir.vpk").exists(), "foreign stays");
        assert!(!addons.join("pak02_dir.vpk").exists());
        assert!(!addons.join("pak03_dir.vpk").exists());
        // The library and the profile stay; the folder now differs from the profile.
        let view = s.library();
        assert_eq!(view.mods.len(), 2);
        assert_eq!(view.applied_profile, None);
        assert!(view.pending);
        // The gameinfo.gi backup is in the data folder.
        assert!(
            std::fs::read_dir(f.data().join("backups").join("gameinfo"))
                .expect("backups")
                .count()
                >= 2
        );
        s.apply(&game, false).expect("applied again");
        assert!(addons.join("pak02_dir.vpk").exists());
        assert_eq!(
            s.rollback_preview(&game).expect("preview").gameinfo,
            GameinfoState::Ours
        );
    }

    #[test]
    fn rollback_with_foreign_files_deletes_them_too() {
        let f = Fixture::new("foreign");
        let mut s = f.applied();
        let done = s
            .rollback(&f.game("Deadlock"), true, false)
            .expect("rolled back");
        assert_eq!(done.foreign_removed.len(), 2);
        let left: Vec<_> = std::fs::read_dir(f.addons("Deadlock"))
            .expect("addons")
            .collect();
        assert!(left.is_empty(), "{left:?}");
    }

    #[test]
    fn rollback_waits_for_the_game() {
        let f = Fixture::new("running");
        let mut s = f.applied();
        assert!(matches!(
            s.rollback(&f.game("Deadlock"), false, true),
            Err(SessionError::Engine(EngineError::GameRunning))
        ));
        assert!(f.addons("Deadlock").join("pak02_dir.vpk").exists());
    }

    #[test]
    fn rolling_back_everything_reaches_every_folder_with_our_files_or_line() {
        let f = Fixture::new("all");
        let mut s = f.applied();
        // A second installation with only our line (an apply that never put files there).
        let other = f.game("Other");
        std::fs::write(
            other.join("game").join("citadel").join("gameinfo.gi"),
            crate::gameinfo::add_addons_path(VANILLA).expect("ours"),
        )
        .expect("ours");
        let done = s
            .rollback_all(&[other.clone(), f.dir.join("Missing")], false)
            .expect("all");
        assert_eq!(done.len(), 2);
        assert_eq!(gameinfo(&other), VANILLA);
        assert_eq!(gameinfo(&f.game("Deadlock")), VANILLA);
        assert!(
            f.addons("Deadlock").join("pak01_dir.vpk").exists(),
            "foreign stays"
        );
    }

    #[test]
    fn the_installations_list_has_steam_manual_and_known_folders() {
        let f = Fixture::new("installs");
        let mut s = f.applied();
        let mut settings = s.settings().clone();
        settings.game.manual_installations = vec![f.game("Manual").display().to_string()];
        s.update_settings(settings).expect("saved");
        let detected = [Detected {
            game_root: f.game("Steam"),
            library: Some(f.dir.clone()),
            build_id: Some(20_314_556),
        }];
        let rows = s.installations(&detected);
        let summary: Vec<(InstallSource, bool, GameinfoState, usize)> = rows
            .iter()
            .map(|r| (r.source, r.selected, r.gameinfo, r.files))
            .collect();
        assert_eq!(
            summary,
            [
                (InstallSource::Steam, false, GameinfoState::Vanilla, 0),
                (InstallSource::Manual, false, GameinfoState::Vanilla, 0),
                (InstallSource::Known, true, GameinfoState::Ours, 2),
            ]
        );
        assert_eq!(rows[2].profiles, ["Default"]);
        assert_eq!(rows[2].applied_profile.as_deref(), Some("Default"));
        assert_eq!(rows[0].build_id, Some(20_314_556));
    }

    #[test]
    fn the_selected_profile_can_be_pointed_at_another_folder() {
        let f = Fixture::new("gameroot");
        let mut s = f.open();
        let other = f.game("Other");
        s.set_game_root(Some(&other)).expect("set");
        assert_eq!(s.game_root(), Some(other.clone()));
        assert_eq!(f.open().game_root(), Some(other), "saved in the profile");
        s.set_game_root(None).expect("cleared");
        assert_eq!(s.game_root(), None);
    }

    #[test]
    fn the_advanced_editor_saves_valid_settings_with_a_backup() {
        let f = Fixture::new("settingsjson");
        let mut s = f.open();
        s.update_settings(Settings::default()).expect("first file");
        let mut text = s.settings_text();
        assert!(text.contains("\"schema\": 1"));
        text = text.replace("\"retries\": 5", "\"retries\": 2");
        let saved = s
            .save_settings_text(&text, OffsetDateTime::now_utc())
            .expect("saved");
        assert!(
            saved
                .backup
                .expect("backup")
                .starts_with("settings.json.bak-")
        );
        assert_eq!(s.settings().network.retries, 2);
        assert!(
            f.log
                .events()
                .iter()
                .any(|(msg, _)| msg == "settings saved from the Advanced editor")
        );
    }

    #[test]
    fn the_advanced_editor_never_saves_an_invalid_file() {
        let f = Fixture::new("settingsbad");
        let mut s = f.open();
        s.update_settings(Settings::default()).expect("first file");
        let before = std::fs::read_to_string(f.data().join("settings.json")).expect("file");
        let broken = before.replacen("\"gif_autoplay\": false,", "\"gif_autoplay\": false", 1);
        assert!(matches!(
            s.save_settings_text(&broken, OffsetDateTime::now_utc()),
            Err(EditRefused::Json { .. })
        ));
        let out_of_range = before.replace("\"retries\": 5", "\"retries\": 99");
        let Err(EditRefused::Problems { problems }) =
            s.save_settings_text(&out_of_range, OffsetDateTime::now_utc())
        else {
            panic!("refused with problems");
        };
        assert_eq!(problems[0].path, "network.retries");
        assert_eq!(
            problems[0].problem,
            ProblemKind::OutOfRange {
                found: 99,
                min: 0,
                max: 10
            }
        );
        assert_eq!(
            std::fs::read_to_string(f.data().join("settings.json")).expect("file"),
            before,
            "nothing written"
        );
        assert!(
            crate::backups::list(&f.data().join("settings.json"))
                .expect("list")
                .is_empty()
        );
    }

    #[test]
    fn the_manifest_can_be_edited_and_is_checked() {
        let f = Fixture::new("manifestjson");
        let mut s = f.applied();
        let text = s.manifest_text();
        let renamed = text.replacen("\"name\": \"Haze\"", "\"name\": \"Haze (edited)\"", 1);
        s.save_manifest_text(&renamed, OffsetDateTime::now_utc())
            .expect("saved");
        assert!(s.library().mods.iter().any(|m| m.name == "Haze (edited)"));
        let unsafe_path =
            text.replacen("\"library_path\": \"library/", "\"library_path\": \"../", 1);
        assert!(matches!(
            s.save_manifest_text(&unsafe_path, OffsetDateTime::now_utc()),
            Err(EditRefused::Invalid { .. })
        ));
        assert!(s.library().mods.iter().any(|m| m.name == "Haze (edited)"));
    }

    #[test]
    fn reset_keeps_a_backup_and_what_is_not_a_preference() {
        let f = Fixture::new("reset");
        let mut s = f.open();
        let mut settings = s.settings().clone();
        settings.network.retries = 1;
        settings.catalog.enabled = true;
        s.update_settings(settings).expect("saved");
        let saved = s.reset_settings(OffsetDateTime::now_utc()).expect("reset");
        assert!(saved.backup.is_some());
        assert_eq!(s.settings().network.retries, 5);
        assert!(s.settings().catalog.enabled, "the catalog choice stays");
    }
}
