//! Planning how to make the game's `addons` folder match a profile: which of our files move, appear or go, as
//! journal steps with two-phase renames.
//!
//! The order is always: our files that must move or go → temporary names; new files → temporary names; temporary
//! names → final `pakNN_dir.vpk`; leftovers deleted after the commit. Files already in their slot are not touched,
//! foreign files never are (their slots are skipped). Our slots are numbered contiguously in list order, so the
//! numbering is predictable and never runs out through gaps.

use crate::journal::Step;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

/// Slots `pak01_dir.vpk` … `pak99_dir.vpk`.
pub const MAX_SLOT: u8 = 99;

/// Which pak number wins when two files override the same game file. Not verified in game yet (decision 25); the
/// community says lower numbers win, so that is the default.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Direction {
    #[default]
    LowerNumberWins,
    HigherNumberWins,
}

/// One of our files currently in the game folder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Installed {
    pub slot: u8,
    /// Identity of the library file; equal ids mean equal content.
    pub id: String,
}

/// A library file the profile wants, in priority order (first = wins).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Wanted {
    pub id: String,
    pub source: PathBuf,
    pub sha256: [u8; 32],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlanError {
    TooManyMods { wanted: usize, free: usize },
    Duplicate(String),
}

impl std::fmt::Display for PlanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooManyMods { wanted, free } => {
                write!(
                    f,
                    "{wanted} mod files are enabled, but the game folder has room for {free}"
                )
            }
            Self::Duplicate(id) => write!(f, "the same mod file is in the list twice: {id}"),
        }
    }
}

impl std::error::Error for PlanError {}

pub fn plan(
    addons: &Path,
    tx: u64,
    installed: &[Installed],
    foreign_slots: &[u8],
    wanted: &[Wanted],
    direction: Direction,
) -> Result<Vec<Step>, PlanError> {
    let mut seen = HashSet::new();
    if let Some(dup) = wanted.iter().find(|w| !seen.insert(w.id.as_str())) {
        return Err(PlanError::Duplicate(dup.id.clone()));
    }
    let slots = layout(foreign_slots, wanted, direction)?;
    let target: HashMap<&str, u8> = slots
        .iter()
        .map(|(slot, id)| (id.as_str(), *slot))
        .collect();

    let mut temp_counter = 0usize;
    let mut temp = || {
        temp_counter += 1;
        addons.join(format!("~dl-{tx}-{temp_counter}.tmp"))
    };
    let mut moves = Vec::new();
    let mut places = Vec::new();
    let mut finals = Vec::new();
    let mut deletes = Vec::new();
    let mut staged: HashMap<&str, PathBuf> = HashMap::new();

    for file in installed {
        if target.get(file.id.as_str()) == Some(&file.slot) {
            continue;
        }
        let t = temp();
        moves.push(Step::Rename {
            from: pak_path(addons, file.slot),
            to: t.clone(),
        });
        if target.contains_key(file.id.as_str()) {
            staged.insert(file.id.as_str(), t);
        } else {
            deletes.push(Step::Delete { path: t });
        }
    }
    let present: HashSet<&str> = installed.iter().map(|i| i.id.as_str()).collect();
    for w in wanted {
        if !present.contains(w.id.as_str()) {
            let t = temp();
            places.push(Step::Place {
                source: w.source.clone(),
                dest: t.clone(),
                sha256: w.sha256,
            });
            staged.insert(w.id.as_str(), t);
        }
    }
    for w in wanted {
        if let Some(t) = staged.remove(w.id.as_str()) {
            finals.push(Step::Rename {
                from: t,
                to: pak_path(addons, target[w.id.as_str()]),
            });
        }
    }
    Ok(moves
        .into_iter()
        .chain(places)
        .chain(finals)
        .chain(deletes)
        .collect())
}

/// Which slot each wanted file gets: our slots numbered in list order around the foreign ones.
pub fn layout(
    foreign_slots: &[u8],
    wanted: &[Wanted],
    direction: Direction,
) -> Result<Vec<(u8, String)>, PlanError> {
    let free: Vec<u8> = (1..=MAX_SLOT)
        .filter(|s| !foreign_slots.contains(s))
        .collect();
    if wanted.len() > free.len() {
        return Err(PlanError::TooManyMods {
            wanted: wanted.len(),
            free: free.len(),
        });
    }
    Ok(wanted
        .iter()
        .enumerate()
        .map(|(i, w)| {
            let slot = match direction {
                Direction::LowerNumberWins => free[i],
                Direction::HigherNumberWins => free[wanted.len() - 1 - i],
            };
            (slot, w.id.clone())
        })
        .collect())
}

pub fn pak_path(addons: &Path, slot: u8) -> PathBuf {
    addons.join(format!("pak{slot:02}_dir.vpk"))
}

/// `pakNN_dir.vpk`, any case.
pub fn is_pak(name: &str) -> bool {
    slot_of(name).is_some()
}

pub fn slot_of(name: &str) -> Option<u8> {
    let lower = name.to_ascii_lowercase();
    let digits = lower.strip_prefix("pak")?.strip_suffix("_dir.vpk")?;
    if digits.len() != 2 || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    digits.parse().ok().filter(|&s| (1..=MAX_SLOT).contains(&s))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fs::fake::MemFs;
    use crate::fs::sha256;
    use crate::journal::{self, JournalStore};
    use std::cell::RefCell;
    use std::path::PathBuf;

    const ADDONS: &str = r"D:\game\citadel\addons";

    fn addons() -> PathBuf {
        PathBuf::from(ADDONS)
    }

    fn wanted(id: &str) -> Wanted {
        Wanted {
            id: id.to_owned(),
            source: PathBuf::from(format!(r"D:\lib\{id}.vpk")),
            sha256: sha256(id.as_bytes()),
        }
    }

    fn installed(slot: u8, id: &str) -> Installed {
        Installed {
            slot,
            id: id.to_owned(),
        }
    }

    fn pak(slot: u8) -> PathBuf {
        addons().join(format!("pak{slot:02}_dir.vpk"))
    }

    /// Library files contain their id; `installed` files are already in their slots.
    fn fs_with(installed: &[Installed], foreign: &[u8]) -> MemFs {
        let mut fs = MemFs::default().volume("D:", true);
        for id in ["a", "b", "c", "d"] {
            fs = fs.with(&format!(r"D:\lib\{id}.vpk"), id.as_bytes());
        }
        for i in installed {
            fs = fs.with(&pak(i.slot).to_string_lossy(), i.id.as_bytes());
        }
        for &slot in foreign {
            fs = fs.with(&pak(slot).to_string_lossy(), b"foreign");
        }
        fs
    }

    #[derive(Debug, Default)]
    struct MemStore(RefCell<Vec<String>>);

    impl JournalStore for MemStore {
        fn append(&self, line: &str) -> std::io::Result<()> {
            self.0.borrow_mut().push(line.to_owned());
            Ok(())
        }
        fn read_all(&self) -> std::io::Result<String> {
            Ok(self.0.borrow().join("\n"))
        }
        fn clear(&self) -> std::io::Result<()> {
            self.0.borrow_mut().clear();
            Ok(())
        }
    }

    /// Plans, runs the plan through the journal and returns slot → content of every pak file.
    fn apply(
        installed: &[Installed],
        foreign: &[u8],
        wanted: &[Wanted],
        direction: Direction,
    ) -> Vec<(u8, String)> {
        let fs = fs_with(installed, foreign);
        let steps = plan(&addons(), 1, installed, foreign, wanted, direction).expect("plan");
        journal::run(&fs, &MemStore::default(), 1, &steps).expect("run");
        (1..=MAX_SLOT)
            .filter_map(|s| {
                fs.read(&pak(s).to_string_lossy())
                    .map(|b| (s, String::from_utf8(b).expect("utf8")))
            })
            .collect()
    }

    fn slots(pairs: &[(u8, &str)]) -> Vec<(u8, String)> {
        pairs.iter().map(|&(s, c)| (s, c.to_owned())).collect()
    }

    #[test]
    fn a_fresh_install_numbers_the_list_from_the_top() {
        let result = apply(
            &[],
            &[],
            &[wanted("a"), wanted("b")],
            Direction::LowerNumberWins,
        );
        assert_eq!(result, slots(&[(1, "a"), (2, "b")]));
    }

    #[test]
    fn the_other_direction_gives_the_top_the_highest_number() {
        let result = apply(
            &[],
            &[],
            &[wanted("a"), wanted("b")],
            Direction::HigherNumberWins,
        );
        assert_eq!(result, slots(&[(1, "b"), (2, "a")]));
    }

    #[test]
    fn an_applied_profile_needs_no_steps() {
        let steps = plan(
            &addons(),
            1,
            &[installed(1, "a"), installed(2, "b")],
            &[],
            &[wanted("a"), wanted("b")],
            Direction::LowerNumberWins,
        )
        .expect("plan");
        assert!(steps.is_empty(), "{steps:?}");
    }

    #[test]
    fn swapping_two_mods_goes_through_temporary_names() {
        let steps = plan(
            &addons(),
            9,
            &[installed(1, "a"), installed(2, "b")],
            &[],
            &[wanted("b"), wanted("a")],
            Direction::LowerNumberWins,
        )
        .expect("plan");
        // Every rename into a pak name comes from a temporary name, never straight from another pak name.
        for step in &steps {
            if let Step::Rename { from, to } = step {
                let from = from.file_name().expect("name").to_string_lossy();
                let to = to.file_name().expect("name").to_string_lossy();
                assert!(!(is_pak(&from) && is_pak(&to)), "{from} -> {to}");
            }
        }
        let result = apply(
            &[installed(1, "a"), installed(2, "b")],
            &[],
            &[wanted("b"), wanted("a")],
            Direction::LowerNumberWins,
        );
        assert_eq!(result, slots(&[(1, "b"), (2, "a")]));
    }

    #[test]
    fn removed_mods_are_deleted_last_and_the_rest_close_up() {
        let steps = plan(
            &addons(),
            1,
            &[installed(1, "a"), installed(2, "b"), installed(3, "c")],
            &[],
            &[wanted("a"), wanted("c")],
            Direction::LowerNumberWins,
        )
        .expect("plan");
        assert!(matches!(steps.last(), Some(Step::Delete { .. })));
        let result = apply(
            &[installed(1, "a"), installed(2, "b"), installed(3, "c")],
            &[],
            &[wanted("a"), wanted("c")],
            Direction::LowerNumberWins,
        );
        assert_eq!(result, slots(&[(1, "a"), (2, "c")]));
    }

    #[test]
    fn foreign_files_keep_their_slots_and_are_never_touched() {
        let steps = plan(
            &addons(),
            1,
            &[],
            &[2],
            &[wanted("a"), wanted("b")],
            Direction::LowerNumberWins,
        )
        .expect("plan");
        assert!(steps.iter().all(|s| !touches(s, &pak(2))), "{steps:?}");
        let result = apply(
            &[],
            &[2],
            &[wanted("a"), wanted("b")],
            Direction::LowerNumberWins,
        );
        assert_eq!(result, slots(&[(1, "a"), (2, "foreign"), (3, "b")]));
    }

    #[test]
    fn a_new_mod_on_top_shifts_the_others_down() {
        let result = apply(
            &[installed(1, "a"), installed(2, "b")],
            &[],
            &[wanted("c"), wanted("a"), wanted("b")],
            Direction::LowerNumberWins,
        );
        assert_eq!(result, slots(&[(1, "c"), (2, "a"), (3, "b")]));
    }

    #[test]
    fn temporary_names_never_look_like_pak_files() {
        let steps = plan(
            &addons(),
            3,
            &[installed(1, "a")],
            &[],
            &[wanted("b"), wanted("a")],
            Direction::LowerNumberWins,
        )
        .expect("plan");
        for step in &steps {
            if let Step::Place { dest, .. } = step {
                let name = dest.file_name().expect("name").to_string_lossy();
                assert!(name.starts_with("~dl-") && !is_pak(&name), "{name}");
            }
        }
    }

    #[test]
    fn more_mods_than_free_slots_is_an_error() {
        let wanted: Vec<Wanted> = (0..MAX_SLOT).map(|i| wanted(&format!("m{i}"))).collect();
        let err = plan(&addons(), 1, &[], &[5], &wanted, Direction::LowerNumberWins)
            .expect_err("too many");
        assert_eq!(
            err,
            PlanError::TooManyMods {
                wanted: 99,
                free: 98
            }
        );
    }

    #[test]
    fn the_same_mod_twice_is_an_error() {
        let err = plan(
            &addons(),
            1,
            &[],
            &[],
            &[wanted("a"), wanted("a")],
            Direction::LowerNumberWins,
        )
        .expect_err("duplicate");
        assert_eq!(err, PlanError::Duplicate("a".to_owned()));
    }

    #[test]
    fn the_layout_lists_each_file_with_its_slot() {
        let l = layout(
            &[2],
            &[wanted("a"), wanted("b")],
            Direction::LowerNumberWins,
        )
        .expect("layout");
        assert_eq!(l, [(1, "a".to_owned()), (3, "b".to_owned())]);
        let l = layout(
            &[],
            &[wanted("a"), wanted("b")],
            Direction::HigherNumberWins,
        )
        .expect("layout");
        assert_eq!(l, [(2, "a".to_owned()), (1, "b".to_owned())]);
    }

    #[test]
    fn recognises_pak_file_names() {
        assert!(is_pak("pak01_dir.vpk"));
        assert!(is_pak("PAK99_DIR.VPK"));
        assert!(!is_pak("pak1_dir.vpk"));
        assert!(!is_pak("pak01_000.vpk"));
        assert!(!is_pak("~dl-1-0.tmp"));
        assert_eq!(slot_of("pak07_dir.vpk"), Some(7));
        assert_eq!(slot_of("pak00_dir.vpk"), None);
    }

    #[test]
    fn random_profiles_always_end_in_the_planned_layout() {
        // A small deterministic generator; no extra dependency.
        let mut seed = 0x2545_F491_4F6C_DD1Du64;
        let mut next = |n: u64| {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed % n
        };
        let ids = ["a", "b", "c", "d"];
        for _ in 0..300 {
            let mut pool: Vec<&str> = ids.to_vec();
            let mut installed_now = Vec::new();
            let foreign: Vec<u8> = (1..=6u8).filter(|_| next(4) == 0).collect();
            let mut slot = 1u8;
            for id in pool.clone() {
                if next(2) == 0 {
                    while foreign.contains(&slot) {
                        slot += 1;
                    }
                    installed_now.push(installed(slot, id));
                    slot += 1;
                }
            }
            let mut wanted_now = Vec::new();
            while !pool.is_empty() {
                let id = pool.remove(next(pool.len() as u64) as usize);
                if next(3) != 0 {
                    wanted_now.push(wanted(id));
                }
            }
            let direction = if next(2) == 0 {
                Direction::LowerNumberWins
            } else {
                Direction::HigherNumberWins
            };

            let result = apply(&installed_now, &foreign, &wanted_now, direction);

            let free: Vec<u8> = (1..=MAX_SLOT).filter(|s| !foreign.contains(s)).collect();
            let mut expected: Vec<(u8, String)> = wanted_now
                .iter()
                .enumerate()
                .map(|(i, w)| {
                    let s = match direction {
                        Direction::LowerNumberWins => free[i],
                        Direction::HigherNumberWins => free[wanted_now.len() - 1 - i],
                    };
                    (s, w.id.clone())
                })
                .chain(foreign.iter().map(|&s| (s, "foreign".to_owned())))
                .collect();
            expected.sort();
            assert_eq!(
                result, expected,
                "installed {installed_now:?} foreign {foreign:?} wanted {wanted_now:?}"
            );
        }
    }

    fn touches(step: &Step, path: &Path) -> bool {
        match step {
            Step::Place { dest, .. } => dest == path,
            Step::Rename { from, to } => from == path || to == path,
            Step::Delete { path: p } => p == path,
        }
    }
}
