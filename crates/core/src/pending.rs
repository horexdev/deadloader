//! Pending changes (S9, decisions 17, 138, 143): how the selected profile differs from what is in its game folder,
//! one change per mod and aspect, and taking a change back by making the profile match the game again.
//!
//! The game side is what deadloader recorded when it last applied (the manifest's slots), in priority order. Taking a
//! change back edits only the profile; applying it is the caller's business, like any other edit.

use crate::apply::Direction;
use crate::library::{InstallationState, Manifest, Profile, ProfileEntry, Successor};
use crate::overlap::Grouping;
use serde::Serialize;
use std::collections::{HashMap, HashSet};

/// One mod in the game folder: its files there, in priority order of the mods.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InGame {
    pub mod_id: String,
    pub files: Vec<String>,
}

/// What deadloader put into this game folder, by mod, the winning mod first.
pub fn in_game(
    manifest: &Manifest,
    installation: Option<&InstallationState>,
    direction: Direction,
) -> Vec<InGame> {
    let mut records: Vec<(u8, &str)> = installation
        .map(|i| {
            i.applied
                .iter()
                .map(|r| (r.slot, r.file.as_str()))
                .collect()
        })
        .unwrap_or_default();
    records.sort_by_key(|(slot, _)| *slot);
    if direction == Direction::HigherNumberWins {
        records.reverse();
    }
    let mut out: Vec<InGame> = Vec::new();
    for (_, file) in records {
        let Some((m, _)) = manifest.file(file) else {
            continue;
        };
        match out.iter_mut().find(|g| g.mod_id == m.id) {
            Some(g) => g.files.push(file.to_owned()),
            None => out.push(InGame {
                mod_id: m.id.clone(),
                files: vec![file.to_owned()],
            }),
        }
    }
    out
}

/// One pending change of one mod (S9 groups them by kind).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PendingChange {
    pub mod_id: String,
    pub name: String,
    #[serde(flatten)]
    pub change: Change,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Change {
    /// Off in the game, on in the profile.
    Enable,
    /// In the game, off (or gone) in the profile.
    Disable,
    /// Its place among the mods in the game changed: it is now right above `above` (or, at the bottom, right below
    /// `below`). Mod ids.
    Order {
        above: Option<String>,
        below: Option<String>,
    },
    /// Another variant or add-on: library files leaving the game and coming in.
    Variant { from: Vec<String>, to: Vec<String> },
    /// A newer or older version of a file (they are linked by updates).
    Version { from: String, to: String },
}

impl Change {
    /// What taking this change back touches.
    pub fn aspect(&self) -> Aspect {
        match self {
            Self::Enable | Self::Disable => Aspect::Enabled,
            Self::Order { .. } => Aspect::Order,
            Self::Variant { .. } | Self::Version { .. } => Aspect::Files,
        }
    }
}

/// The part of a mod's profile entry a change is about.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Aspect {
    Enabled,
    Files,
    Order,
}

/// The files of each enabled mod of the profile that exist in the library, in priority order.
fn wanted(manifest: &Manifest, profile: &Profile) -> Vec<InGame> {
    profile
        .entries
        .iter()
        .filter(|e| e.enabled)
        .map(|e| InGame {
            mod_id: e.mod_id.clone(),
            files: e
                .variants
                .iter()
                .chain(&e.add_ons)
                .filter(|f| manifest.file(f).is_some())
                .cloned()
                .collect(),
        })
        .filter(|w| !w.files.is_empty())
        .collect()
}

/// Every change between `game` and `profile`: enable, disable and file changes in the profile's order (mods only in
/// the game after them), then the moves.
pub fn changes(manifest: &Manifest, profile: &Profile, game: &[InGame]) -> Vec<PendingChange> {
    let wanted = wanted(manifest, profile);
    let name = |id: &str| {
        manifest
            .mods
            .iter()
            .find(|m| m.id == id)
            .map_or_else(|| id.to_owned(), |m| m.name.clone())
    };
    let item = |mod_id: &str, change: Change| PendingChange {
        mod_id: mod_id.to_owned(),
        name: name(mod_id),
        change,
    };
    let mut out = Vec::new();
    for w in &wanted {
        match game.iter().find(|g| g.mod_id == w.mod_id) {
            None => out.push(item(&w.mod_id, Change::Enable)),
            Some(g) => {
                for change in file_changes(manifest, &g.files, &w.files) {
                    out.push(item(&w.mod_id, change));
                }
            }
        }
    }
    for g in game {
        if !wanted.iter().any(|w| w.mod_id == g.mod_id) {
            out.push(item(&g.mod_id, Change::Disable));
        }
    }
    let common: Vec<&str> = wanted
        .iter()
        .map(|w| w.mod_id.as_str())
        .filter(|id| game.iter().any(|g| g.mod_id == *id))
        .collect();
    let game_pos: HashMap<&str, usize> = game
        .iter()
        .enumerate()
        .map(|(i, g)| (g.mod_id.as_str(), i))
        .collect();
    let kept = longest_increasing(&common.iter().map(|id| game_pos[id]).collect::<Vec<_>>());
    for (i, id) in common.iter().enumerate() {
        if !kept.contains(&i) {
            out.push(item(
                id,
                Change::Order {
                    above: common.get(i + 1).map(|s| (*s).to_owned()),
                    below: i.checked_sub(1).map(|j| common[j].to_owned()),
                },
            ));
        }
    }
    out
}

/// Files leaving and coming in for one mod; a pair linked by an update is a version change.
fn file_changes(manifest: &Manifest, game: &[String], wanted: &[String]) -> Vec<Change> {
    let mut removed: Vec<&String> = game.iter().filter(|f| !wanted.contains(f)).collect();
    let mut added: Vec<&String> = wanted.iter().filter(|f| !game.contains(f)).collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < removed.len() {
        let old = removed[i];
        match added
            .iter()
            .position(|new| updated(manifest, old, new) || updated(manifest, new, old))
        {
            Some(j) => {
                out.push(Change::Version {
                    from: old.clone(),
                    to: added.remove(j).clone(),
                });
                removed.remove(i);
            }
            None => i += 1,
        }
    }
    if !removed.is_empty() || !added.is_empty() {
        out.push(Change::Variant {
            from: removed.into_iter().cloned().collect(),
            to: added.into_iter().cloned().collect(),
        });
    }
    out
}

/// `newer` is reached from `older` through the successors updates recorded.
fn updated(manifest: &Manifest, older: &str, newer: &str) -> bool {
    let mut at = older;
    // The manifest refuses successor loops; the bound only guards against a damaged one.
    for _ in 0..64 {
        let Some((_, f)) = manifest.file(at) else {
            return false;
        };
        match &f.successor {
            Some(Successor::File(next)) if next == newer => return true,
            Some(Successor::File(next)) => at = next,
            _ => return false,
        }
    }
    false
}

/// Indexes of one longest strictly increasing subsequence of `values` (the mods whose relative order is unchanged).
fn longest_increasing(values: &[usize]) -> HashSet<usize> {
    let n = values.len();
    let mut len = vec![1usize; n];
    let mut prev: Vec<Option<usize>> = vec![None; n];
    for i in 0..n {
        for j in 0..i {
            if values[j] < values[i] && len[j] + 1 > len[i] {
                len[i] = len[j] + 1;
                prev[i] = Some(j);
            }
        }
    }
    let mut kept = HashSet::new();
    let mut at = (0..n).max_by_key(|&i| (len[i], std::cmp::Reverse(i)));
    while let Some(i) = at {
        kept.insert(i);
        at = prev[i];
    }
    kept
}

/// The profile changed only as far as needed to match the game: the mods in the game take the places the profile
/// gives its in-game mods, in the game's order, on and with the files the game has; every other mod is off and keeps
/// its place and choices.
pub fn matching_game(
    profile: &Profile,
    game: &[InGame],
    grouping_of: &dyn Fn(&str) -> Grouping,
) -> Profile {
    let mut out = profile.clone();
    out.entries = merged_order(profile, game)
        .into_iter()
        .map(|id| {
            let mut entry = profile
                .entries
                .iter()
                .find(|e| e.mod_id == id)
                .cloned()
                .unwrap_or_else(|| ProfileEntry {
                    mod_id: id.clone(),
                    variants: Vec::new(),
                    add_ons: Vec::new(),
                    enabled: false,
                });
            match game.iter().find(|g| g.mod_id == id) {
                Some(g) => {
                    entry.enabled = true;
                    set_files(&mut entry, &g.files, &grouping_of(&id));
                }
                None => entry.enabled = false,
            }
            entry
        })
        .collect();
    out
}

/// Takes back one aspect of one mod's change (S9 "Undo this change").
pub fn revert(
    profile: &mut Profile,
    game: &[InGame],
    grouping_of: &dyn Fn(&str) -> Grouping,
    mod_id: &str,
    aspect: Aspect,
) {
    let in_game = game.iter().find(|g| g.mod_id == mod_id);
    match aspect {
        Aspect::Enabled => match (
            in_game,
            profile.entries.iter_mut().find(|e| e.mod_id == mod_id),
        ) {
            (None, Some(entry)) => entry.enabled = false,
            (Some(g), Some(entry)) => {
                entry.enabled = true;
                set_files(entry, &g.files, &grouping_of(mod_id));
            }
            (Some(_), None) => *profile = matching_game(profile, game, grouping_of),
            (None, None) => {}
        },
        Aspect::Files => {
            if let (Some(g), Some(entry)) = (
                in_game,
                profile.entries.iter_mut().find(|e| e.mod_id == mod_id),
            ) {
                set_files(entry, &g.files, &grouping_of(mod_id));
            }
        }
        Aspect::Order => {
            let target = merged_order(profile, game);
            let (Some(from), Some(to)) = (
                profile.entries.iter().position(|e| e.mod_id == mod_id),
                target.iter().position(|id| id == mod_id),
            ) else {
                return;
            };
            let entry = profile.entries.remove(from);
            let to = to.min(profile.entries.len());
            profile.entries.insert(to, entry);
        }
    }
}

/// The profile's mod order with its in-game mods put into the game's order, in the places they hold; mods in the game
/// the profile lacks go after the last of those places.
fn merged_order(profile: &Profile, game: &[InGame]) -> Vec<String> {
    let mut order: Vec<String> = profile.entries.iter().map(|e| e.mod_id.clone()).collect();
    let places: Vec<usize> = order
        .iter()
        .enumerate()
        .filter(|(_, id)| game.iter().any(|g| &g.mod_id == *id))
        .map(|(i, _)| i)
        .collect();
    let mut game_ids = game.iter().map(|g| g.mod_id.clone());
    for &place in &places {
        if let Some(id) = game_ids.next() {
            order[place] = id;
        }
    }
    let rest: Vec<String> = game_ids.collect();
    let at = places.last().map_or(order.len(), |p| p + 1);
    order.splice(at..at, rest);
    order
}

/// The entry's files become exactly `files`: variants where they belong to a variant group, add-ons otherwise.
fn set_files(entry: &mut ProfileEntry, files: &[String], grouping: &Grouping) {
    let in_group = |f: &String| grouping.variant_groups.iter().any(|g| g.contains(f));
    entry.variants = files.iter().filter(|f| in_group(f)).cloned().collect();
    entry.add_ons = files.iter().filter(|f| !in_group(f)).cloned().collect();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::{AppliedRecord, Mod, ModFile, Source};

    fn file(id: &str, successor: Option<&str>) -> ModFile {
        ModFile {
            id: id.to_owned(),
            remote_file_id: None,
            archive_name: format!("{id}.zip"),
            inner_path: format!("{id}.vpk"),
            library_path: format!("library/{id}.vpk"),
            len: 10,
            sha256: [0; 32],
            version: None,
            successor: successor.map(|s| Successor::File(s.to_owned())),
            archive_md5: None,
        }
    }

    fn library(mods: &[(&str, &[&str])]) -> Manifest {
        Manifest {
            mods: mods
                .iter()
                .map(|(id, files)| Mod {
                    id: (*id).to_owned(),
                    name: id.to_uppercase(),
                    author: None,
                    source: Source::Local {
                        imported_from: None,
                    },
                    files: files.iter().map(|f| file(f, None)).collect(),
                })
                .collect(),
            ..Manifest::default()
        }
    }

    fn entry(id: &str, files: &[&str], enabled: bool) -> ProfileEntry {
        ProfileEntry {
            mod_id: id.to_owned(),
            variants: files.iter().map(|f| (*f).to_owned()).collect(),
            add_ons: Vec::new(),
            enabled,
        }
    }

    fn profile(entries: Vec<ProfileEntry>) -> Profile {
        Profile {
            name: "Evening".into(),
            game_root: None,
            entries,
        }
    }

    fn game(mods: &[(&str, &[&str])]) -> Vec<InGame> {
        mods.iter()
            .map(|(id, files)| InGame {
                mod_id: (*id).to_owned(),
                files: files.iter().map(|f| (*f).to_owned()).collect(),
            })
            .collect()
    }

    /// Every file is its own variant group (one-file mods), unless the test says otherwise.
    fn groups(_: &str) -> Grouping {
        Grouping {
            variant_groups: Vec::new(),
            standalone: Vec::new(),
        }
    }

    fn kinds(changes: &[PendingChange]) -> Vec<(&str, &Change)> {
        changes
            .iter()
            .map(|c| (c.mod_id.as_str(), &c.change))
            .collect()
    }

    #[test]
    fn the_game_side_is_read_from_the_slots_by_mod_winner_first() {
        let manifest = library(&[("a", &["a1", "a2"]), ("b", &["b1"])]);
        let installation = InstallationState {
            game_root: "D:\\Deadlock".into(),
            applied_profile: Some("Evening".into()),
            applied: vec![
                AppliedRecord {
                    slot: 3,
                    file: "a2".into(),
                    len: 1,
                },
                AppliedRecord {
                    slot: 1,
                    file: "b1".into(),
                    len: 1,
                },
                AppliedRecord {
                    slot: 2,
                    file: "a1".into(),
                    len: 1,
                },
            ],
            ..InstallationState::default()
        };
        assert_eq!(
            in_game(&manifest, Some(&installation), Direction::LowerNumberWins),
            game(&[("b", &["b1"]), ("a", &["a1", "a2"])])
        );
        assert_eq!(
            in_game(&manifest, Some(&installation), Direction::HigherNumberWins),
            game(&[("a", &["a2", "a1"]), ("b", &["b1"])])
        );
        assert!(in_game(&manifest, None, Direction::LowerNumberWins).is_empty());
    }

    #[test]
    fn nothing_differs_when_the_profile_is_what_the_game_has() {
        let manifest = library(&[("a", &["a1"]), ("b", &["b1"])]);
        let p = profile(vec![entry("a", &["a1"], true), entry("b", &["b1"], false)]);
        assert!(changes(&manifest, &p, &game(&[("a", &["a1"])])).is_empty());
    }

    #[test]
    fn switching_on_and_off_are_changes() {
        let manifest = library(&[("a", &["a1"]), ("b", &["b1"])]);
        let p = profile(vec![entry("a", &["a1"], false), entry("b", &["b1"], true)]);
        let found = changes(&manifest, &p, &game(&[("a", &["a1"])]));
        assert_eq!(
            kinds(&found),
            [("b", &Change::Enable), ("a", &Change::Disable)]
        );
        assert_eq!(found[0].name, "B");
    }

    #[test]
    fn another_variant_is_a_variant_change_and_a_successor_a_version_change() {
        let mut manifest = library(&[("faun", &["pink", "red"]), ("yam", &["v1", "v2"])]);
        manifest.mods[1].files[0].successor = Some(Successor::File("v2".into()));
        let p = profile(vec![
            entry("faun", &["red"], true),
            entry("yam", &["v2"], true),
        ]);
        let found = changes(
            &manifest,
            &p,
            &game(&[("faun", &["pink"]), ("yam", &["v1"])]),
        );
        assert_eq!(
            kinds(&found),
            [
                (
                    "faun",
                    &Change::Variant {
                        from: vec!["pink".into()],
                        to: vec!["red".into()]
                    }
                ),
                (
                    "yam",
                    &Change::Version {
                        from: "v1".into(),
                        to: "v2".into()
                    }
                ),
            ]
        );
        // Back to the old version: still a version change.
        let back = profile(vec![entry("yam", &["v1"], true)]);
        assert_eq!(
            kinds(&changes(&manifest, &back, &game(&[("yam", &["v2"])]))),
            [(
                "yam",
                &Change::Version {
                    from: "v2".into(),
                    to: "v1".into()
                }
            )]
        );
    }

    #[test]
    fn only_the_mods_that_moved_are_order_changes() {
        let manifest = library(&[("a", &["a1"]), ("b", &["b1"]), ("c", &["c1"])]);
        // Game: a b c. Profile: c on top.
        let p = profile(vec![
            entry("c", &["c1"], true),
            entry("a", &["a1"], true),
            entry("b", &["b1"], true),
        ]);
        let g = game(&[("a", &["a1"]), ("b", &["b1"]), ("c", &["c1"])]);
        assert_eq!(
            kinds(&changes(&manifest, &p, &g)),
            [(
                "c",
                &Change::Order {
                    above: Some("a".into()),
                    below: None
                }
            )]
        );
    }

    #[test]
    fn a_disabled_mod_between_others_is_not_a_move() {
        let manifest = library(&[("a", &["a1"]), ("b", &["b1"]), ("x", &["x1"])]);
        let p = profile(vec![
            entry("a", &["a1"], true),
            entry("x", &["x1"], false),
            entry("b", &["b1"], true),
        ]);
        let g = game(&[("a", &["a1"]), ("b", &["b1"])]);
        assert!(changes(&manifest, &p, &g).is_empty());
    }

    #[test]
    fn taking_back_each_change_leaves_nothing_pending() {
        let mut manifest = library(&[
            ("a", &["a1"]),
            ("b", &["b1"]),
            ("c", &["c1"]),
            ("v", &["v1", "v2"]),
        ]);
        manifest.mods[3].files[0].successor = Some(Successor::File("v2".into()));
        let g = game(&[("a", &["a1"]), ("b", &["b1"]), ("v", &["v1"])]);
        let mut p = profile(vec![
            entry("c", &["c1"], true),
            entry("b", &["b1"], true),
            entry("a", &["a1"], true),
            entry("v", &["v2"], true),
        ]);
        for _ in 0..10 {
            let Some(first) = changes(&manifest, &p, &g).into_iter().next() else {
                break;
            };
            revert(&mut p, &g, &groups, &first.mod_id, first.change.aspect());
        }
        assert!(changes(&manifest, &p, &g).is_empty(), "{p:?}");
        // The mod the game doesn't have is still in the profile, switched off.
        assert!(p.entries.iter().any(|e| e.mod_id == "c" && !e.enabled));
    }

    #[test]
    fn a_mod_switched_off_comes_back_with_the_files_in_the_game() {
        let g = game(&[("faun", &["pink"])]);
        let mut p = profile(vec![entry("faun", &["red"], false)]);
        let grouping = |_: &str| Grouping {
            variant_groups: vec![vec!["pink".into(), "red".into()]],
            standalone: Vec::new(),
        };
        revert(&mut p, &g, &grouping, "faun", Aspect::Enabled);
        assert_eq!(p.entries[0], entry("faun", &["pink"], true));
    }

    #[test]
    fn matching_the_game_keeps_the_other_mods_in_their_places() {
        let g = game(&[("a", &["a1"]), ("b", &["b1"])]);
        let p = profile(vec![
            entry("b", &["b1"], true),
            entry("x", &["x1"], true),
            entry("a", &["a1"], true),
        ]);
        let matched = matching_game(&p, &g, &groups);
        let order: Vec<(&str, bool)> = matched
            .entries
            .iter()
            .map(|e| (e.mod_id.as_str(), e.enabled))
            .collect();
        assert_eq!(order, [("a", true), ("x", false), ("b", true)]);
        // Add-ons stay add-ons.
        assert_eq!(matched.entries[0].add_ons, ["a1"]);
    }

    #[test]
    fn a_mod_in_the_game_the_profile_lost_is_added_back() {
        let g = game(&[("a", &["a1"])]);
        let p = profile(vec![entry("x", &["x1"], true)]);
        let matched = matching_game(&p, &g, &groups);
        let order: Vec<(&str, bool)> = matched
            .entries
            .iter()
            .map(|e| (e.mod_id.as_str(), e.enabled))
            .collect();
        assert_eq!(order, [("x", false), ("a", true)]);
    }

    #[test]
    fn changes_are_tagged_for_the_ui() {
        let change = PendingChange {
            mod_id: "a".into(),
            name: "A".into(),
            change: Change::Order {
                above: Some("b".into()),
                below: None,
            },
        };
        assert_eq!(
            serde_json::to_value(&change).expect("json"),
            serde_json::json!({"mod_id": "a", "name": "A", "kind": "order", "above": "b", "below": null})
        );
    }
}
