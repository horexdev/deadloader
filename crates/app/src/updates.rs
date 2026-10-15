//! Mod update checks (decisions 38, 164): every GameBanana mod of the library against its current file list, at start
//! when the setting says so and on "Check for updates". Installing an update (D5) is a separate step.

use deadloader_catalog::{CatalogError, Initiator};
use deadloader_core::library::{Manifest, Section, Source};
use deadloader_core::updates::{self, ModCheck, RemoteMod, Status, UpdatePost};
use serde::Serialize;
use std::collections::{BTreeSet, HashMap};

/// Where the answers come from: the catalog, or a fake in tests.
pub trait Remote {
    fn remote_mod(
        &self,
        section: Section,
        id: u64,
        initiator: Initiator,
    ) -> Result<RemoteMod, CatalogError>;
    fn update_posts(
        &self,
        section: Section,
        id: u64,
        initiator: Initiator,
    ) -> Result<Vec<UpdatePost>, CatalogError>;
}

impl Remote for crate::catalog::AppCatalog {
    fn remote_mod(
        &self,
        section: Section,
        id: u64,
        initiator: Initiator,
    ) -> Result<RemoteMod, CatalogError> {
        deadloader_catalog::Catalog::remote_mod(self, gamebanana(section), id, initiator)
    }

    fn update_posts(
        &self,
        section: Section,
        id: u64,
        initiator: Initiator,
    ) -> Result<Vec<UpdatePost>, CatalogError> {
        deadloader_catalog::Catalog::update_posts(self, gamebanana(section), id, initiator)
    }
}

fn gamebanana(section: Section) -> deadloader_catalog::Section {
    match section {
        Section::Mod => deadloader_catalog::Section::Mod,
        Section::Sound => deadloader_catalog::Section::Sound,
    }
}

/// The result of one check: a status per library mod and the N3 count.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UpdateReport {
    pub checks: Vec<ModCheck>,
    /// Mods with an update (N3); new files alone don't count (164).
    pub available: usize,
    /// Why the check stopped before asking about every mod (local mode, offline, GameBanana busy or changed); the
    /// mods not asked are `not_checked`.
    pub stopped: Option<CatalogError>,
}

/// Asks about every GameBanana mod once, then fetches the update posts of the mods that have an update.
pub fn check_all(manifest: &Manifest, remote: &dyn Remote, initiator: Initiator) -> UpdateReport {
    let wanted: BTreeSet<(Section, u64)> = manifest
        .mods
        .iter()
        .filter_map(|m| match m.source {
            Source::GameBanana { section, mod_id } => Some((section, mod_id)),
            Source::Local { .. } => None,
        })
        .collect();
    let mut answers: HashMap<(Section, u64), RemoteMod> = HashMap::new();
    let mut stopped = None;
    for &(section, id) in &wanted {
        match remote.remote_mod(section, id, initiator) {
            Ok(answer) => {
                answers.insert((section, id), answer);
            }
            Err(e) if stops_the_check(&e) => {
                stopped = Some(e);
                break;
            }
            // This mod only (e.g. a redirect the allowlist refused); it stays "not checked".
            Err(_) => {}
        }
    }
    let first = updates::check(manifest, |s, id| answers.get(&(s, id)));
    if stopped.is_none() {
        for m in &manifest.mods {
            let Source::GameBanana { section, mod_id } = m.source else {
                continue;
            };
            let has_update = first
                .iter()
                .any(|c| c.mod_id == m.id && matches!(c.status, Status::UpdateAvailable(_)));
            if !has_update {
                continue;
            }
            if let Ok(posts) = remote.update_posts(section, mod_id, initiator)
                && let Some(RemoteMod::Listed(listed)) = answers.get_mut(&(section, mod_id))
            {
                listed.posts = posts;
            }
        }
    }
    let checks = updates::check(manifest, |s, id| answers.get(&(s, id)));
    UpdateReport {
        available: updates::updates_available(&checks),
        checks,
        stopped,
    }
}

/// The user's answer in D5 for one replaced GameBanana file: the listed file that replaces it, or `None` to keep the
/// old one and stop reporting it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize)]
pub struct UpdatePick {
    pub replaced: u64,
    pub file: Option<u64>,
}

/// What a pick takes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PickPlan {
    /// "Keep the old file": these library files get no successor and are not reported again.
    Keep { old_files: Vec<String> },
    /// The replacement is in the library already (downloaded before, decision 39's "download automatically").
    Present {
        replaced: u64,
        new_files: Vec<String>,
    },
    /// The replacement has to be downloaded (169: only what the user picked).
    Download { replaced: u64, file: u64 },
}

/// The library mod an update is for, with its GameBanana id.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateTarget {
    pub section: Section,
    pub mod_id: u64,
    pub name: String,
    pub author: Option<String>,
    pub plans: Vec<PickPlan>,
}

/// Turns the picks of D5 into what each needs, checking they belong to the mod.
pub fn plan_picks(
    manifest: &Manifest,
    library_mod: &str,
    picks: &[UpdatePick],
) -> Result<UpdateTarget, deadloader_core::message::UserMessage> {
    let mistake = |detail: String| {
        deadloader_core::message::UserMessage::new(deadloader_core::message::MessageId::Internal)
            .arg("detail", detail)
    };
    let m = manifest
        .mods
        .iter()
        .find(|m| m.id == library_mod)
        .ok_or_else(|| mistake(format!("no mod {library_mod}")))?;
    let Source::GameBanana { section, mod_id } = m.source else {
        return Err(mistake(format!("{library_mod} is not from GameBanana")));
    };
    let files_of = |remote: u64, current_only: bool| -> Vec<String> {
        m.files
            .iter()
            .filter(|f| f.remote_file_id == Some(remote))
            .filter(|f| {
                !current_only
                    || !matches!(
                        f.successor,
                        Some(deadloader_core::library::Successor::File(_))
                    )
            })
            .map(|f| f.id.clone())
            .collect()
    };
    let mut plans = Vec::new();
    for pick in picks {
        let old_files = files_of(pick.replaced, true);
        if old_files.is_empty() {
            return Err(mistake(format!(
                "{library_mod} has no current file from GameBanana file {}",
                pick.replaced
            )));
        }
        plans.push(match pick.file {
            None => PickPlan::Keep { old_files },
            Some(file) => {
                let present = files_of(file, false);
                if present.is_empty() {
                    PickPlan::Download {
                        replaced: pick.replaced,
                        file,
                    }
                } else {
                    PickPlan::Present {
                        replaced: pick.replaced,
                        new_files: present,
                    }
                }
            }
        });
    }
    Ok(UpdateTarget {
        section,
        mod_id,
        name: m.name.clone(),
        author: m.author.clone(),
        plans,
    })
}

/// Errors that would repeat for every other mod.
fn stops_the_check(e: &CatalogError) -> bool {
    matches!(
        e,
        CatalogError::LocalMode
            | CatalogError::Offline { .. }
            | CatalogError::RateLimited { .. }
            | CatalogError::ApiChanged { .. }
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use deadloader_core::library::{Mod, ModFile};
    use deadloader_core::updates::{ListedMod, RemoteFile, RemovedReason};
    use std::cell::RefCell;

    fn lib_file(id: &str, remote: u64, archive: &str) -> ModFile {
        ModFile {
            id: id.to_owned(),
            remote_file_id: Some(remote),
            archive_name: archive.to_owned(),
            inner_path: "pak01_dir.vpk".to_owned(),
            library_path: format!("library/m/{id}.vpk"),
            len: 1,
            sha256: [0; 32],
            version: None,
            successor: None,
        }
    }

    fn gb_mod(id: u64, files: Vec<ModFile>) -> Mod {
        Mod {
            id: format!("gb-{id}"),
            name: format!("mod {id}"),
            author: None,
            source: Source::GameBanana {
                section: Section::Mod,
                mod_id: id,
            },
            files,
        }
    }

    fn listed(files: &[(u64, &str)]) -> RemoteMod {
        RemoteMod::Listed(ListedMod {
            version: Some("2.0".to_owned()),
            date_updated: None,
            files: files
                .iter()
                .map(|&(id, name)| RemoteFile {
                    id,
                    file_name: name.to_owned(),
                    size_bytes: 1,
                    date_added: None,
                    version: None,
                    description: None,
                })
                .collect(),
            posts: Vec::new(),
        })
    }

    /// Answers from a table and records what was asked.
    struct Table {
        answers: HashMap<u64, Result<RemoteMod, CatalogError>>,
        asked: RefCell<Vec<String>>,
    }

    impl Remote for Table {
        fn remote_mod(&self, _: Section, id: u64, _: Initiator) -> Result<RemoteMod, CatalogError> {
            self.asked.borrow_mut().push(format!("files {id}"));
            self.answers
                .get(&id)
                .cloned()
                .unwrap_or(Err(CatalogError::NotFound { other_game: None }))
        }

        fn update_posts(
            &self,
            _: Section,
            id: u64,
            _: Initiator,
        ) -> Result<Vec<UpdatePost>, CatalogError> {
            self.asked.borrow_mut().push(format!("posts {id}"));
            Ok(vec![UpdatePost {
                id: 1,
                title: "v2".to_owned(),
                version: Some("2.0".to_owned()),
                date_added: None,
                changelog: Vec::new(),
                file_ids: vec![21],
                text_html: String::new(),
            }])
        }
    }

    fn manifest() -> Manifest {
        Manifest {
            mods: vec![
                gb_mod(1, vec![lib_file("gb-1-1-0", 11, "a.zip")]),
                gb_mod(2, vec![lib_file("gb-2-1-0", 20, "b.zip")]),
                gb_mod(3, vec![lib_file("gb-3-1-0", 30, "c.zip")]),
            ],
            ..Manifest::default()
        }
    }

    #[test]
    fn every_mod_is_asked_once_and_updates_get_their_posts() {
        let table = Table {
            answers: HashMap::from([
                (1, Ok(listed(&[(11, "a.zip")]))),
                // Our file 20 is gone; 21 has the same name.
                (2, Ok(listed(&[(21, "b.zip")]))),
                (3, Ok(RemoteMod::Removed(RemovedReason::Withheld))),
            ]),
            asked: RefCell::new(Vec::new()),
        };
        let report = check_all(&manifest(), &table, Initiator::User);
        assert_eq!(report.available, 1);
        assert!(report.stopped.is_none());
        assert_eq!(report.checks[0].status, Status::UpToDate);
        let Status::UpdateAvailable(update) = &report.checks[1].status else {
            panic!("{:?}", report.checks[1]);
        };
        assert_eq!(update.posts.len(), 1, "D5's changelog");
        assert!(matches!(report.checks[2].status, Status::Removed { .. }));
        assert_eq!(
            *table.asked.borrow(),
            ["files 1", "files 2", "files 3", "posts 2"]
        );
    }

    #[test]
    fn local_mode_or_offline_stops_the_check_at_once() {
        let table = Table {
            answers: HashMap::from([(1, Err(CatalogError::Offline { since_unix_ms: 5 }))]),
            asked: RefCell::new(Vec::new()),
        };
        let report = check_all(&manifest(), &table, Initiator::Auto);
        assert_eq!(
            report.stopped,
            Some(CatalogError::Offline { since_unix_ms: 5 })
        );
        assert_eq!(*table.asked.borrow(), ["files 1"]);
        assert!(report.checks.iter().all(|c| c.status == Status::NotChecked));
        assert_eq!(report.available, 0);
    }

    #[test]
    fn picks_become_keep_present_or_download() {
        let mut m = manifest();
        // File 21 of mod 2 was downloaded already.
        m.mods[1].files.push(lib_file("gb-2-2-0", 21, "b.zip"));
        let target = plan_picks(
            &m,
            "gb-2",
            &[UpdatePick {
                replaced: 20,
                file: Some(21),
            }],
        )
        .expect("planned");
        assert_eq!((target.section, target.mod_id), (Section::Mod, 2));
        assert_eq!(
            target.plans,
            [PickPlan::Present {
                replaced: 20,
                new_files: vec!["gb-2-2-0".to_owned()]
            }]
        );
        let target = plan_picks(
            &m,
            "gb-1",
            &[UpdatePick {
                replaced: 11,
                file: Some(12),
            }],
        )
        .expect("planned");
        assert_eq!(
            target.plans,
            [PickPlan::Download {
                replaced: 11,
                file: 12
            }]
        );
        let target = plan_picks(
            &m,
            "gb-3",
            &[UpdatePick {
                replaced: 30,
                file: None,
            }],
        )
        .expect("planned");
        assert_eq!(
            target.plans,
            [PickPlan::Keep {
                old_files: vec!["gb-3-1-0".to_owned()]
            }]
        );
    }

    #[test]
    fn picks_that_are_not_the_mods_are_refused() {
        let m = manifest();
        assert!(plan_picks(&m, "gb-9", &[]).is_err());
        assert!(
            plan_picks(
                &m,
                "gb-1",
                &[UpdatePick {
                    replaced: 99,
                    file: None
                }]
            )
            .is_err()
        );
    }

    #[test]
    fn a_failure_for_one_mod_leaves_the_others_checked() {
        let table = Table {
            answers: HashMap::from([
                (1, Ok(listed(&[(11, "a.zip")]))),
                (
                    2,
                    Err(CatalogError::Blocked {
                        message: deadloader_core::message::UserMessage::new(
                            deadloader_core::message::MessageId::E16,
                        ),
                    }),
                ),
                (3, Ok(listed(&[(30, "c.zip")]))),
            ]),
            asked: RefCell::new(Vec::new()),
        };
        let report = check_all(&manifest(), &table, Initiator::User);
        assert_eq!(report.checks[1].status, Status::NotChecked);
        assert_eq!(report.checks[2].status, Status::UpToDate);
    }
}
