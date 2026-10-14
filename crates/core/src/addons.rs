//! What is in the game's `addons` folder: our files, foreign files and leftovers of an interrupted change.
//!
//! A `pakNN_dir.vpk` is ours only if the manifest says we put that file in that slot and its size still matches;
//! anything else is foreign and is never touched without an explicit user action (decision 16).

use crate::apply::{Installed, slot_of};
use crate::fs::FileOps;
use std::io;
use std::path::{Path, PathBuf};

/// What the manifest says we placed in this installation's addons folder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppliedFile {
    pub slot: u8,
    pub id: String,
    pub len: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ForeignKind {
    /// A pak file (or a data chunk of one) another tool or the user put there.
    ModFile,
    /// In one of our slots, but not the file we placed (size differs).
    ReplacedOurs,
    OtherFile,
    Folder,
}

/// Who probably created a foreign item, when it can be told from its name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Origin {
    DeadlockModManager,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForeignItem {
    pub name: String,
    pub kind: ForeignKind,
    /// The pak slot it occupies, if any.
    pub slot: Option<u8>,
    pub len: u64,
    pub origin: Option<Origin>,
}

#[derive(Debug, Default)]
pub struct Scan {
    pub ours: Vec<Installed>,
    pub foreign: Vec<ForeignItem>,
    /// Temporary files of ours left by an interrupted change (removed once the journal is recovered).
    pub leftovers: Vec<PathBuf>,
    /// Recorded files no longer there.
    pub missing: Vec<AppliedFile>,
}

impl Scan {
    /// Slots our numbering must skip.
    pub fn foreign_slots(&self) -> Vec<u8> {
        let mut slots: Vec<u8> = self.foreign.iter().filter_map(|f| f.slot).collect();
        slots.sort_unstable();
        slots.dedup();
        slots
    }
}

pub fn scan(fs: &impl FileOps, addons: &Path, applied: &[AppliedFile]) -> io::Result<Scan> {
    let entries = match fs.list_dir(addons) {
        Ok(entries) => entries,
        Err(e) if e.kind() == io::ErrorKind::NotFound => Vec::new(),
        Err(e) => return Err(e),
    };
    let mut result = Scan::default();
    let mut seen_slots = Vec::new();
    for entry in entries {
        let lower = entry.name.to_ascii_lowercase();
        if lower.starts_with("~dl-") && !entry.is_dir {
            result.leftovers.push(addons.join(&entry.name));
            continue;
        }
        if !entry.is_dir {
            if let Some(slot) = slot_of(&entry.name) {
                seen_slots.push(slot);
                match applied.iter().find(|a| a.slot == slot) {
                    Some(a) if a.len == entry.len => {
                        result.ours.push(Installed {
                            slot,
                            id: a.id.clone(),
                        });
                    }
                    Some(_) => {
                        result
                            .foreign
                            .push(foreign(entry, ForeignKind::ReplacedOurs, Some(slot)))
                    }
                    None => result
                        .foreign
                        .push(foreign(entry, ForeignKind::ModFile, Some(slot))),
                }
                continue;
            }
            if let Some(slot) = chunk_slot(&lower) {
                result
                    .foreign
                    .push(foreign(entry, ForeignKind::ModFile, Some(slot)));
                continue;
            }
        }
        let kind = if entry.is_dir {
            ForeignKind::Folder
        } else {
            ForeignKind::OtherFile
        };
        result.foreign.push(foreign(entry, kind, None));
    }
    result.ours.sort_by_key(|i| i.slot);
    result.missing = applied
        .iter()
        .filter(|a| !seen_slots.contains(&a.slot))
        .cloned()
        .collect();
    Ok(result)
}

fn foreign(entry: crate::fs::DirEntry, kind: ForeignKind, slot: Option<u8>) -> ForeignItem {
    let origin =
        (entry.name.eq_ignore_ascii_case(".dmm.json")).then_some(Origin::DeadlockModManager);
    ForeignItem {
        name: entry.name,
        kind,
        slot,
        len: entry.len,
        origin,
    }
}

/// `pakNN_000.vpk`-style data chunk of a multi-part pak: it belongs to slot NN.
fn chunk_slot(lower: &str) -> Option<u8> {
    let rest = lower.strip_prefix("pak")?;
    let (digits, tail) = rest.split_at_checked(2)?;
    let index = tail.strip_prefix('_')?.strip_suffix(".vpk")?;
    if index.len() != 3
        || !index.bytes().all(|b| b.is_ascii_digit())
        || !digits.bytes().all(|b| b.is_ascii_digit())
    {
        return None;
    }
    digits.parse().ok().filter(|&s| s >= 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fs::fake::MemFs;

    const ADDONS: &str = r"D:\game\citadel\addons";

    fn at(name: &str) -> String {
        format!(r"{ADDONS}\{name}")
    }

    fn record(slot: u8, id: &str, len: u64) -> AppliedFile {
        AppliedFile {
            slot,
            id: id.to_owned(),
            len,
        }
    }

    #[test]
    fn a_missing_addons_folder_is_empty() {
        let fs = MemFs::default();
        let scan = scan(&fs, Path::new(ADDONS), &[]).expect("scan");
        assert!(scan.ours.is_empty() && scan.foreign.is_empty() && scan.leftovers.is_empty());
    }

    #[test]
    fn files_matching_our_records_are_ours() {
        let fs = MemFs::default()
            .with(&at("pak01_dir.vpk"), b"12345")
            .with(&at("pak02_dir.vpk"), b"abc");
        let scan = scan(
            &fs,
            Path::new(ADDONS),
            &[record(1, "a", 5), record(2, "b", 3)],
        )
        .expect("scan");
        assert_eq!(
            scan.ours,
            [
                Installed {
                    slot: 1,
                    id: "a".into()
                },
                Installed {
                    slot: 2,
                    id: "b".into()
                }
            ]
        );
        assert!(scan.foreign.is_empty());
    }

    #[test]
    fn a_pak_file_we_did_not_put_there_is_foreign() {
        let fs = MemFs::default().with(&at("pak01_dir.vpk"), b"someone else's mod");
        let scan = scan(&fs, Path::new(ADDONS), &[]).expect("scan");
        assert!(scan.ours.is_empty());
        assert_eq!(scan.foreign.len(), 1);
        assert_eq!(scan.foreign[0].slot, Some(1));
        assert_eq!(scan.foreign[0].kind, ForeignKind::ModFile);
        assert_eq!(scan.foreign_slots(), [1]);
    }

    #[test]
    fn our_file_replaced_by_something_else_is_foreign() {
        // Recorded as 5 bytes, now 9: another tool wrote over it. We must not treat it as ours.
        let fs = MemFs::default().with(&at("pak01_dir.vpk"), b"different");
        let scan = scan(&fs, Path::new(ADDONS), &[record(1, "a", 5)]).expect("scan");
        assert!(scan.ours.is_empty());
        assert_eq!(scan.foreign[0].kind, ForeignKind::ReplacedOurs);
        assert_eq!(scan.foreign_slots(), [1]);
    }

    #[test]
    fn a_recorded_file_that_is_gone_is_reported_missing() {
        let fs = MemFs::default().with(&at("pak02_dir.vpk"), b"abc");
        let scan = scan(
            &fs,
            Path::new(ADDONS),
            &[record(1, "a", 5), record(2, "b", 3)],
        )
        .expect("scan");
        assert_eq!(scan.missing, [record(1, "a", 5)]);
        assert_eq!(scan.ours.len(), 1);
    }

    #[test]
    fn other_files_and_folders_are_foreign_and_recognised_when_known() {
        let fs = MemFs::default()
            .with(&at(".dmm.json"), b"{}")
            .with(&at(r"replays\match.dem"), b"x")
            .with(&at("readme.txt"), b"hi");
        let scan = scan(&fs, Path::new(ADDONS), &[]).expect("scan");
        let mut kinds: Vec<_> = scan
            .foreign
            .iter()
            .map(|f| (f.name.as_str(), f.kind, f.origin))
            .collect();
        kinds.sort();
        assert_eq!(
            kinds,
            [
                (
                    ".dmm.json",
                    ForeignKind::OtherFile,
                    Some(Origin::DeadlockModManager)
                ),
                ("readme.txt", ForeignKind::OtherFile, None),
                ("replays", ForeignKind::Folder, None),
            ]
        );
        assert!(scan.foreign_slots().is_empty());
    }

    #[test]
    fn our_temporary_names_are_leftovers() {
        let fs = MemFs::default()
            .with(&at("~dl-7-1.tmp"), b"x")
            .with(&at("pak01_dir.vpk"), b"12345");
        let scan = scan(&fs, Path::new(ADDONS), &[record(1, "a", 5)]).expect("scan");
        assert_eq!(scan.leftovers, [PathBuf::from(at("~dl-7-1.tmp"))]);
        assert!(scan.foreign.is_empty());
    }

    #[test]
    fn names_are_compared_without_case() {
        let fs = MemFs::default().with(&at("PAK01_DIR.VPK"), b"12345");
        let scan = scan(&fs, Path::new(ADDONS), &[record(1, "a", 5)]).expect("scan");
        assert_eq!(scan.ours.len(), 1);
    }

    #[test]
    fn multi_part_paks_of_other_tools_are_foreign_mod_files() {
        // pak05_000.vpk is a data chunk of someone's multi-part pak05; not one of our names.
        let fs = MemFs::default()
            .with(&at("pak05_dir.vpk"), b"d")
            .with(&at("pak05_000.vpk"), b"c");
        let scan = scan(&fs, Path::new(ADDONS), &[]).expect("scan");
        assert_eq!(scan.foreign_slots(), [5]);
        assert_eq!(scan.foreign.len(), 2);
    }
}
