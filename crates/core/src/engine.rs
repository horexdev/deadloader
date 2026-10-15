//! Applying a profile to a game installation: scan → plan → journal → manifest, and finishing an interrupted apply on
//! start (layer 1 put together).
//!
//! The manifest is updated in the journal's commit hook, after the changes are durable and before the journal is
//! cleared; the new layout also travels in the transaction itself, so [`Engine::recover`] can record it if the process
//! died in between. Nothing is changed while the game is running (decision 17).

use crate::addons::{AppliedFile, ForeignItem, ForeignKind, scan};
use crate::apply::{Direction, PlanError, Wanted, layout, plan};
use crate::fs::FileOps;
use crate::gamedir::addons_path;
use crate::journal::{self, JournalStore, Recovery, Step, TxError};
use crate::library::{AppliedRecord, InstallationState, Manifest, Profile};
use serde::{Deserialize, Serialize};
use std::io;
use std::path::Path;

#[derive(Debug)]
pub struct Engine<'a, F, J> {
    pub fs: &'a F,
    pub journal: &'a J,
    /// `deadloader-data\`; library paths in the manifest are relative to it.
    pub data_dir: &'a Path,
    pub direction: Direction,
}

#[derive(Debug)]
pub struct ApplyReport {
    /// Whether anything in the game folder changed.
    pub changed: bool,
    pub steps: usize,
    pub foreign: Vec<ForeignItem>,
    pub leftovers_removed: usize,
    /// What the start-of-apply recovery found.
    pub recovered: Recovery,
}

#[derive(Debug)]
pub enum EngineError {
    GameRunning,
    UnknownMod(String),
    NotAFileOfMod {
        mod_id: String,
        file: String,
    },
    Plan(PlanError),
    Tx(TxError),
    Io(io::Error),
    /// A journal result that could not be read back.
    Meta(String),
}

impl std::fmt::Display for EngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GameRunning => {
                f.write_str("Deadlock is running; changes wait until it is closed")
            }
            Self::UnknownMod(id) => write!(
                f,
                "the profile lists a mod that is not in the library: {id}"
            ),
            Self::NotAFileOfMod { mod_id, file } => {
                write!(f, "the file {file} does not belong to the mod {mod_id}")
            }
            Self::Plan(e) => write!(f, "{e}"),
            Self::Tx(e) => write!(f, "{e}"),
            Self::Io(e) => write!(f, "{e}"),
            Self::Meta(e) => write!(f, "the interrupted change could not be recorded: {e}"),
        }
    }
}

impl std::error::Error for EngineError {}

/// What a rollback took out of one game installation (D7, decision 20).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RollbackReport {
    /// Whether anything in the game folder changed.
    pub changed: bool,
    /// Our pak files taken out.
    pub removed: usize,
    /// Foreign files deleted at the user's request (folders are never touched).
    pub foreign_removed: Vec<String>,
    /// What the start-of-rollback recovery found.
    pub recovered: Recovery,
}

/// Stored in the transaction so recovery knows what to record. `profile` is `None` after a rollback: nothing is
/// applied there any more.
#[derive(Debug, Serialize, Deserialize)]
struct Outcome {
    game_root: String,
    profile: Option<String>,
    applied: Vec<AppliedRecord>,
}

impl<F: FileOps, J: JournalStore> Engine<'_, F, J> {
    /// Finishes or undoes an interrupted change and records its result. Call at start, before anything else.
    pub fn recover(
        &self,
        manifest: &mut Manifest,
        mut save: impl FnMut(&Manifest) -> io::Result<()>,
    ) -> Result<Recovery, EngineError> {
        let (outcome, meta) =
            journal::recover_with_meta(self.fs, self.journal).map_err(EngineError::Io)?;
        if let Some(value) = meta {
            let o: Outcome =
                serde_json::from_value(value).map_err(|e| EngineError::Meta(e.to_string()))?;
            record(manifest, &o.game_root, o.profile.as_deref(), o.applied);
            save(manifest).map_err(EngineError::Io)?;
        }
        Ok(outcome)
    }

    /// Makes the game installation at `game_root` match `profile`. `tx` must be unique per change (it names the
    /// temporary files). `save` persists the manifest; it runs once the change is durable.
    pub fn apply(
        &self,
        manifest: &mut Manifest,
        profile: &Profile,
        game_root: &Path,
        game_running: bool,
        tx: u64,
        mut save: impl FnMut(&Manifest) -> io::Result<()>,
    ) -> Result<ApplyReport, EngineError> {
        if game_running {
            return Err(EngineError::GameRunning);
        }
        let recovered = self.recover(manifest, &mut save)?;
        let wanted = resolve(manifest, profile, self.data_dir)?;
        let root = game_root.to_string_lossy().into_owned();
        let applied_before: Vec<AppliedFile> = installation(manifest, &root)
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
        self.fs.create_dir_all(&addons).map_err(EngineError::Io)?;
        let found = scan(self.fs, &addons, &applied_before).map_err(EngineError::Io)?;
        let foreign_slots = found.foreign_slots();
        let mut steps = plan(
            &addons,
            tx,
            &found.ours,
            &foreign_slots,
            &wanted,
            self.direction,
        )
        .map_err(EngineError::Plan)?;
        steps.extend(
            found
                .leftovers
                .iter()
                .map(|p| Step::Delete { path: p.clone() }),
        );
        let records: Vec<AppliedRecord> = layout(&foreign_slots, &wanted, self.direction)
            .map_err(EngineError::Plan)?
            .into_iter()
            .map(|(slot, file)| {
                let len = manifest.file(&file).map_or(0, |(_, f)| f.len);
                AppliedRecord { slot, file, len }
            })
            .collect();

        let report = |changed: bool, steps: usize| ApplyReport {
            changed,
            steps,
            foreign: found.foreign.clone(),
            leftovers_removed: found.leftovers.len(),
            recovered,
        };
        if steps.is_empty() {
            let current = installation(manifest, &root).is_some_and(|i| {
                i.applied == records && i.applied_profile.as_deref() == Some(profile.name.as_str())
            });
            if !current {
                record(manifest, &root, Some(&profile.name), records);
                save(manifest).map_err(EngineError::Io)?;
            }
            return Ok(report(false, 0));
        }

        let meta = serde_json::to_value(Outcome {
            game_root: root.clone(),
            profile: Some(profile.name.clone()),
            applied: records.clone(),
        })
        .map_err(|e| EngineError::Meta(e.to_string()))?;
        let count = steps.len();
        journal::run_with(self.fs, self.journal, tx, &steps, meta, || {
            record(manifest, &root, Some(&profile.name), records);
            save(manifest)
        })
        .map_err(EngineError::Tx)?;
        Ok(report(true, count))
    }

    /// Full rollback of one installation (D7, decision 20): our pak files leave `addons` through the journal, and with
    /// `remove_foreign` the files deadloader didn't create too (never folders). The installation is then recorded as
    /// empty and applied by no profile, so Apply brings everything back. `gameinfo.gi` is the caller's next step
    /// ([`crate::gamedir::remove_mods_path`]). Nothing is changed while the game is running (decision 17).
    pub fn rollback(
        &self,
        manifest: &mut Manifest,
        game_root: &Path,
        remove_foreign: bool,
        game_running: bool,
        tx: u64,
        mut save: impl FnMut(&Manifest) -> io::Result<()>,
    ) -> Result<RollbackReport, EngineError> {
        if game_running {
            return Err(EngineError::GameRunning);
        }
        let recovered = self.recover(manifest, &mut save)?;
        let root = game_root.to_string_lossy().into_owned();
        let addons = addons_path(game_root);
        let found = scan_installation(self.fs, manifest, game_root).map_err(EngineError::Io)?;
        let mut counter = 0usize;
        let mut temp = || {
            counter += 1;
            addons.join(format!("~dl-{tx}-{counter}.tmp"))
        };
        let mut moves = Vec::new();
        let mut deletes: Vec<Step> = found
            .leftovers
            .iter()
            .map(|p| Step::Delete { path: p.clone() })
            .collect();
        for file in &found.ours {
            let t = temp();
            moves.push(Step::Rename {
                from: crate::apply::pak_path(&addons, file.slot),
                to: t.clone(),
            });
            deletes.push(Step::Delete { path: t });
        }
        let mut foreign_removed = Vec::new();
        if remove_foreign {
            for item in found
                .foreign
                .iter()
                .filter(|f| f.kind != ForeignKind::Folder)
            {
                let t = temp();
                moves.push(Step::Rename {
                    from: addons.join(&item.name),
                    to: t.clone(),
                });
                deletes.push(Step::Delete { path: t });
                foreign_removed.push(item.name.clone());
            }
        }
        let removed = found.ours.len();
        let recorded = installation(manifest, &root)
            .is_some_and(|i| !i.applied.is_empty() || i.applied_profile.is_some());
        if moves.is_empty() && deletes.is_empty() {
            if recorded {
                record(manifest, &root, None, Vec::new());
                save(manifest).map_err(EngineError::Io)?;
            }
            return Ok(RollbackReport {
                changed: false,
                removed: 0,
                foreign_removed,
                recovered,
            });
        }
        let meta = serde_json::to_value(Outcome {
            game_root: root.clone(),
            profile: None,
            applied: Vec::new(),
        })
        .map_err(|e| EngineError::Meta(e.to_string()))?;
        let steps: Vec<Step> = moves.into_iter().chain(deletes).collect();
        journal::run_with(self.fs, self.journal, tx, &steps, meta, || {
            record(manifest, &root, None, Vec::new());
            save(manifest)
        })
        .map_err(EngineError::Tx)?;
        Ok(RollbackReport {
            changed: true,
            removed,
            foreign_removed,
            recovered,
        })
    }
}

/// What is in an installation's `addons` folder now: our files (as the manifest recorded them), foreign items and
/// leftovers. For the rollback summary (D7) and the setup step (S1).
pub fn scan_installation(
    fs: &impl FileOps,
    manifest: &Manifest,
    game_root: &Path,
) -> io::Result<crate::addons::Scan> {
    let root = game_root.to_string_lossy();
    let applied: Vec<AppliedFile> = installation(manifest, &root)
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
    scan(fs, &addons_path(game_root), &applied)
}

/// The profile's files in priority order: per entry its chosen variants, then its add-ons.
fn resolve(
    manifest: &Manifest,
    profile: &Profile,
    data_dir: &Path,
) -> Result<Vec<Wanted>, EngineError> {
    let mut wanted = Vec::new();
    for entry in profile.entries.iter().filter(|e| e.enabled) {
        let m = manifest
            .mods
            .iter()
            .find(|m| m.id == entry.mod_id)
            .ok_or_else(|| EngineError::UnknownMod(entry.mod_id.clone()))?;
        for id in entry.variants.iter().chain(&entry.add_ons) {
            let f =
                m.files
                    .iter()
                    .find(|f| &f.id == id)
                    .ok_or_else(|| EngineError::NotAFileOfMod {
                        mod_id: m.id.clone(),
                        file: id.clone(),
                    })?;
            let source = f
                .library_path
                .split('/')
                .fold(data_dir.to_path_buf(), |p, c| p.join(c));
            wanted.push(Wanted {
                id: f.id.clone(),
                source,
                sha256: f.sha256,
            });
        }
    }
    Ok(wanted)
}

pub(crate) fn same_root(a: &str, b: &str) -> bool {
    let norm = |s: &str| {
        s.replace('/', "\\")
            .trim_end_matches('\\')
            .to_ascii_lowercase()
    };
    norm(a) == norm(b)
}

fn installation<'m>(manifest: &'m Manifest, root: &str) -> Option<&'m InstallationState> {
    manifest
        .installations
        .iter()
        .find(|i| same_root(&i.game_root, root))
}

fn record(manifest: &mut Manifest, root: &str, profile: Option<&str>, applied: Vec<AppliedRecord>) {
    let state = match manifest
        .installations
        .iter()
        .position(|i| same_root(&i.game_root, root))
    {
        Some(i) => &mut manifest.installations[i],
        None => {
            manifest.installations.push(InstallationState {
                game_root: root.to_owned(),
                applied_profile: None,
                applied: Vec::new(),
                ..InstallationState::default()
            });
            manifest.installations.last_mut().expect("just pushed")
        }
    };
    state.applied_profile = profile.map(str::to_owned);
    state.applied = applied;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fs::fake::MemFs;
    use crate::fs::sha256;
    use crate::journal::JournalStore;
    use crate::library::{Mod, ModFile, ProfileEntry, Section, Source};
    use std::cell::RefCell;

    const DATA: &str = r"D:\deadloader\deadloader-data";
    const ROOT: &str = r"D:\SteamLibrary\steamapps\common\Deadlock";

    fn addons(name: &str) -> String {
        format!(r"{ROOT}\game\citadel\addons\{name}")
    }

    #[derive(Debug, Default)]
    struct MemStore(RefCell<Vec<String>>);

    impl JournalStore for MemStore {
        fn append(&self, line: &str) -> io::Result<()> {
            self.0.borrow_mut().push(line.to_owned());
            Ok(())
        }
        fn read_all(&self) -> io::Result<String> {
            Ok(self.0.borrow().iter().map(|l| format!("{l}\n")).collect())
        }
        fn clear(&self) -> io::Result<()> {
            self.0.borrow_mut().clear();
            Ok(())
        }
    }

    fn file(id: &str) -> ModFile {
        ModFile {
            id: id.to_owned(),
            remote_file_id: None,
            archive_name: format!("{id}.zip"),
            inner_path: format!("{id}.vpk"),
            library_path: format!("library/{id}.vpk"),
            len: id.len() as u64,
            sha256: sha256(id.as_bytes()),
            version: None,
            successor: None,
            archive_md5: None,
        }
    }

    /// Mod "a" with one file, mod "b" with two colour variants.
    fn manifest() -> Manifest {
        Manifest {
            mods: vec![
                Mod {
                    id: "a".into(),
                    name: "A".into(),
                    author: None,
                    source: Source::Local {
                        imported_from: None,
                    },
                    files: vec![file("a1")],
                },
                Mod {
                    id: "b".into(),
                    name: "B".into(),
                    author: None,
                    source: Source::GameBanana {
                        section: Section::Mod,
                        mod_id: 7,
                    },
                    files: vec![file("b1"), file("b2")],
                },
            ],
            ..Manifest::default()
        }
    }

    fn fs() -> MemFs {
        let mut fs = MemFs::default().volume("D:", true);
        for id in ["a1", "b1", "b2"] {
            fs = fs.with(&format!(r"{DATA}\library\{id}.vpk"), id.as_bytes());
        }
        fs
    }

    fn profile(entries: &[(&str, &[&str])]) -> Profile {
        Profile {
            name: "Evening".into(),
            game_root: Some(ROOT.into()),
            entries: entries
                .iter()
                .map(|(m, v)| ProfileEntry {
                    mod_id: (*m).into(),
                    variants: v.iter().map(|s| (*s).to_owned()).collect(),
                    add_ons: vec![],
                    enabled: true,
                })
                .collect(),
        }
    }

    fn engine<'a>(fs: &'a MemFs, store: &'a MemStore) -> Engine<'a, MemFs, MemStore> {
        Engine {
            fs,
            journal: store,
            data_dir: Path::new(DATA),
            direction: Direction::LowerNumberWins,
        }
    }

    #[test]
    fn applying_puts_the_files_in_order_and_records_them() {
        let (fs, store) = (fs(), MemStore::default());
        let mut m = manifest();
        let mut saves = 0;
        let report = engine(&fs, &store)
            .apply(
                &mut m,
                &profile(&[("b", &["b2"]), ("a", &["a1"])]),
                Path::new(ROOT),
                false,
                1,
                |_| {
                    saves += 1;
                    Ok(())
                },
            )
            .expect("applied");
        assert_eq!(
            fs.read(&addons("pak01_dir.vpk")).as_deref(),
            Some(&b"b2"[..])
        );
        assert_eq!(
            fs.read(&addons("pak02_dir.vpk")).as_deref(),
            Some(&b"a1"[..])
        );
        let install = &m.installations[0];
        assert_eq!(install.applied_profile.as_deref(), Some("Evening"));
        assert_eq!(
            install.applied,
            [
                AppliedRecord {
                    slot: 1,
                    file: "b2".into(),
                    len: 2
                },
                AppliedRecord {
                    slot: 2,
                    file: "a1".into(),
                    len: 2
                }
            ]
        );
        assert_eq!(saves, 1);
        assert!(report.changed);
        assert!(store.read_all().expect("read").is_empty());
    }

    #[test]
    fn disabled_mods_keep_their_place_but_are_not_placed() {
        let (fs, store) = (fs(), MemStore::default());
        let mut m = manifest();
        let mut p = profile(&[("b", &["b2"]), ("a", &["a1"]), ("gone", &["x"])]);
        p.entries[0].enabled = false;
        p.entries[2].enabled = false; // a disabled entry for a mod no longer in the library is ignored
        engine(&fs, &store)
            .apply(&mut m, &p, Path::new(ROOT), false, 1, |_| Ok(()))
            .expect("applied");
        assert_eq!(
            fs.read(&addons("pak01_dir.vpk")).as_deref(),
            Some(&b"a1"[..])
        );
        assert!(fs.read(&addons("pak02_dir.vpk")).is_none());
    }

    #[test]
    fn nothing_happens_while_the_game_runs() {
        let (fs, store) = (fs(), MemStore::default());
        let before = fs.snapshot();
        let mut m = manifest();
        let err = engine(&fs, &store)
            .apply(
                &mut m,
                &profile(&[("a", &["a1"])]),
                Path::new(ROOT),
                true,
                1,
                |_| Ok(()),
            )
            .expect_err("running");
        assert!(matches!(err, EngineError::GameRunning));
        assert_eq!(fs.snapshot(), before);
        assert!(m.installations.is_empty());
    }

    #[test]
    fn applying_the_same_profile_again_changes_nothing() {
        let (fs, store) = (fs(), MemStore::default());
        let mut m = manifest();
        let p = profile(&[("a", &["a1"])]);
        engine(&fs, &store)
            .apply(&mut m, &p, Path::new(ROOT), false, 1, |_| Ok(()))
            .expect("first");
        let before = fs.snapshot();
        let report = engine(&fs, &store)
            .apply(&mut m, &p, Path::new(ROOT), false, 2, |_| Ok(()))
            .expect("again");
        assert!(!report.changed);
        assert_eq!(fs.snapshot(), before);
    }

    #[test]
    fn switching_variants_replaces_the_file_in_its_slot() {
        let (fs, store) = (fs(), MemStore::default());
        let mut m = manifest();
        engine(&fs, &store)
            .apply(
                &mut m,
                &profile(&[("b", &["b1"])]),
                Path::new(ROOT),
                false,
                1,
                |_| Ok(()),
            )
            .expect("first");
        engine(&fs, &store)
            .apply(
                &mut m,
                &profile(&[("b", &["b2"])]),
                Path::new(ROOT),
                false,
                2,
                |_| Ok(()),
            )
            .expect("switch");
        assert_eq!(
            fs.read(&addons("pak01_dir.vpk")).as_deref(),
            Some(&b"b2"[..])
        );
        assert_eq!(
            m.installations[0].applied,
            [AppliedRecord {
                slot: 1,
                file: "b2".into(),
                len: 2
            }]
        );
    }

    #[test]
    fn foreign_files_stay_and_keep_their_slot() {
        let fs = fs()
            .with(&addons("pak01_dir.vpk"), b"someone else")
            .with(&addons(".dmm.json"), b"{}");
        let store = MemStore::default();
        let mut m = manifest();
        let report = engine(&fs, &store)
            .apply(
                &mut m,
                &profile(&[("a", &["a1"])]),
                Path::new(ROOT),
                false,
                1,
                |_| Ok(()),
            )
            .expect("applied");
        assert_eq!(
            fs.read(&addons("pak01_dir.vpk")).as_deref(),
            Some(&b"someone else"[..])
        );
        assert_eq!(
            fs.read(&addons("pak02_dir.vpk")).as_deref(),
            Some(&b"a1"[..])
        );
        assert_eq!(report.foreign.len(), 2);
    }

    #[test]
    fn leftovers_of_an_interrupted_change_are_removed() {
        let fs = fs().with(&addons("~dl-3-1.tmp"), b"half");
        let store = MemStore::default();
        let mut m = manifest();
        let report = engine(&fs, &store)
            .apply(
                &mut m,
                &profile(&[("a", &["a1"])]),
                Path::new(ROOT),
                false,
                4,
                |_| Ok(()),
            )
            .expect("applied");
        assert_eq!(fs.read(&addons("~dl-3-1.tmp")), None);
        assert_eq!(report.leftovers_removed, 1);
    }

    #[test]
    fn a_profile_naming_an_unknown_mod_is_refused_before_any_change() {
        let (fs, store) = (fs(), MemStore::default());
        let before = fs.snapshot();
        let err = engine(&fs, &store)
            .apply(
                &mut manifest(),
                &profile(&[("nope", &[])]),
                Path::new(ROOT),
                false,
                1,
                |_| Ok(()),
            )
            .expect_err("unknown");
        assert!(
            matches!(err, EngineError::UnknownMod(ref id) if id == "nope"),
            "{err:?}"
        );
        assert_eq!(fs.snapshot(), before);
    }

    #[test]
    fn a_variant_of_another_mod_is_refused() {
        let (fs, store) = (fs(), MemStore::default());
        let err = engine(&fs, &store)
            .apply(
                &mut manifest(),
                &profile(&[("a", &["b1"])]),
                Path::new(ROOT),
                false,
                1,
                |_| Ok(()),
            )
            .expect_err("wrong mod");
        assert!(matches!(err, EngineError::NotAFileOfMod { .. }), "{err:?}");
    }

    #[test]
    fn a_crash_before_the_manifest_was_saved_is_finished_on_start() {
        let (fs, store) = (fs(), MemStore::default());
        let mut m = manifest();
        let err = engine(&fs, &store)
            .apply(
                &mut m,
                &profile(&[("a", &["a1"])]),
                Path::new(ROOT),
                false,
                1,
                |_| Err(io::Error::other("disk full")),
            )
            .expect_err("save failed");
        assert!(matches!(err, EngineError::Tx(_)), "{err:?}");

        // Next start: the manifest on disk is still the old one.
        let mut on_disk = manifest();
        let mut saved = None;
        let recovery = engine(&fs, &store)
            .recover(&mut on_disk, |m| {
                saved = Some(m.clone());
                Ok(())
            })
            .expect("recovered");
        assert_eq!(recovery, Recovery::Completed);
        assert_eq!(
            on_disk.installations[0].applied,
            [AppliedRecord {
                slot: 1,
                file: "a1".into(),
                len: 2
            }]
        );
        assert_eq!(saved.as_ref(), Some(&on_disk));
        assert!(store.read_all().expect("read").is_empty());
    }

    /// Applies mods "a" and "b" (b2) on top of a foreign pak and DMM's file, for the rollback tests.
    fn applied_with_foreign(fs: &MemFs, store: &MemStore) -> Manifest {
        let mut m = manifest();
        engine(fs, store)
            .apply(
                &mut m,
                &profile(&[("a", &["a1"]), ("b", &["b2"])]),
                Path::new(ROOT),
                false,
                1,
                |_| Ok(()),
            )
            .expect("applied");
        m
    }

    fn with_foreign() -> MemFs {
        fs().with(&addons("pak01_dir.vpk"), b"someone else")
            .with(&addons(".dmm.json"), b"{}")
            .with(&addons(r"replays\x.dem"), b"demo")
    }

    #[test]
    fn rollback_takes_our_files_out_and_leaves_foreign_ones() {
        let (fs, store) = (with_foreign(), MemStore::default());
        let mut m = applied_with_foreign(&fs, &store);
        assert!(fs.read(&addons("pak02_dir.vpk")).is_some());
        let mut saves = 0;
        let report = engine(&fs, &store)
            .rollback(&mut m, Path::new(ROOT), false, false, 2, |_| {
                saves += 1;
                Ok(())
            })
            .expect("rolled back");
        assert_eq!(report.removed, 2);
        assert!(report.foreign_removed.is_empty());
        assert!(report.changed);
        assert_eq!(fs.read(&addons("pak02_dir.vpk")), None);
        assert_eq!(fs.read(&addons("pak03_dir.vpk")), None);
        assert_eq!(
            fs.read(&addons("pak01_dir.vpk")).as_deref(),
            Some(&b"someone else"[..])
        );
        assert!(fs.read(&addons(".dmm.json")).is_some());
        assert!(
            fs.snapshot().keys().all(|k| !k.contains("~dl-")),
            "no temporary file left"
        );
        // The installation is recorded as empty and applied by no profile, so Apply brings everything back.
        assert_eq!(m.installations[0].applied, []);
        assert_eq!(m.installations[0].applied_profile, None);
        assert_eq!(saves, 1);
        assert!(store.read_all().expect("read").is_empty());
    }

    #[test]
    fn rollback_can_also_remove_foreign_files_but_never_folders() {
        let (fs, store) = (with_foreign(), MemStore::default());
        let mut m = applied_with_foreign(&fs, &store);
        let report = engine(&fs, &store)
            .rollback(&mut m, Path::new(ROOT), true, false, 2, |_| Ok(()))
            .expect("rolled back");
        assert_eq!(report.removed, 2);
        let mut foreign = report.foreign_removed.clone();
        foreign.sort();
        assert_eq!(foreign, [".dmm.json", "pak01_dir.vpk"]);
        assert_eq!(fs.read(&addons("pak01_dir.vpk")), None);
        assert_eq!(fs.read(&addons(".dmm.json")), None);
        assert!(fs.read(&addons(r"replays\x.dem")).is_some(), "folders stay");
    }

    #[test]
    fn rollback_waits_while_the_game_runs() {
        let (fs, store) = (with_foreign(), MemStore::default());
        let mut m = applied_with_foreign(&fs, &store);
        let before = fs.snapshot();
        let err = engine(&fs, &store)
            .rollback(&mut m, Path::new(ROOT), true, true, 2, |_| Ok(()))
            .expect_err("running");
        assert!(matches!(err, EngineError::GameRunning), "{err:?}");
        assert_eq!(fs.snapshot(), before);
    }

    #[test]
    fn rollback_of_a_folder_without_our_files_changes_nothing() {
        let (fs, store) = (with_foreign(), MemStore::default());
        let mut m = manifest();
        let before = fs.snapshot();
        let mut saves = 0;
        let report = engine(&fs, &store)
            .rollback(&mut m, Path::new(ROOT), false, false, 2, |_| {
                saves += 1;
                Ok(())
            })
            .expect("nothing");
        assert!(!report.changed);
        assert_eq!(report.removed, 0);
        assert_eq!(fs.snapshot(), before);
        assert_eq!(saves, 0);
    }

    #[test]
    fn a_crash_during_rollback_is_finished_on_start() {
        let (fs, store) = (with_foreign(), MemStore::default());
        let mut m = applied_with_foreign(&fs, &store);
        engine(&fs, &store)
            .rollback(&mut m, Path::new(ROOT), false, false, 2, |_| {
                Err(io::Error::other("disk full"))
            })
            .expect_err("save failed");
        let mut on_disk = applied_with_foreign_manifest();
        let recovery = engine(&fs, &store)
            .recover(&mut on_disk, |_| Ok(()))
            .expect("recovered");
        assert_eq!(recovery, Recovery::Completed);
        assert_eq!(on_disk.installations[0].applied, []);
        assert_eq!(on_disk.installations[0].applied_profile, None);
    }

    /// What the manifest said after [`applied_with_foreign`], as saved before a rollback.
    fn applied_with_foreign_manifest() -> Manifest {
        let (fs, store) = (with_foreign(), MemStore::default());
        applied_with_foreign(&fs, &store)
    }

    #[test]
    fn the_scan_shows_what_a_rollback_would_remove() {
        let (fs, store) = (with_foreign(), MemStore::default());
        let m = applied_with_foreign(&fs, &store);
        let found = scan_installation(&fs, &m, Path::new(ROOT)).expect("scan");
        assert_eq!(found.ours.len(), 2);
        let names: Vec<&str> = found.foreign.iter().map(|f| f.name.as_str()).collect();
        assert!(names.contains(&"pak01_dir.vpk") && names.contains(&".dmm.json"));
    }

    #[test]
    fn installations_are_matched_without_case() {
        let (fs, store) = (fs(), MemStore::default());
        let mut m = manifest();
        engine(&fs, &store)
            .apply(
                &mut m,
                &profile(&[("a", &["a1"])]),
                Path::new(ROOT),
                false,
                1,
                |_| Ok(()),
            )
            .expect("first");
        let lower = ROOT.to_ascii_lowercase();
        engine(&fs, &store)
            .apply(
                &mut m,
                &profile(&[("a", &["a1"])]),
                Path::new(&lower),
                false,
                2,
                |_| Ok(()),
            )
            .expect("again");
        assert_eq!(m.installations.len(), 1);
    }
}
