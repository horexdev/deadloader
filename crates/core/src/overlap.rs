//! Which mod files change the same game files: variant groups inside a mod (decision 144) and conflicts between
//! mods (decision 133).
//!
//! Works on VPK directory entries (path + CRC32). Paths are compared without case, like the game's filesystem.
//! Files in the package root (`README.txt`, credits) are ignored: authors put them in every VPK and the game doesn't
//! load them, so they would make unrelated files look like variants or conflicts.

use std::collections::{BTreeMap, BTreeSet, HashMap};

/// The directory of one mod file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileIndex {
    pub id: String,
    pub entries: Vec<(String, u32)>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Grouping {
    /// Files that change the same game files; one per group can be active. Members in input order.
    pub variant_groups: Vec<Vec<String>>,
    /// Files that overlap with no other file of the mod: enabled together (parts / add-ons).
    pub standalone: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Conflict {
    pub winner: String,
    pub loser: String,
    /// Game files both change differently, sorted.
    pub paths: Vec<String>,
}

/// The key a game file is compared by, or `None` for files in the package root.
pub(crate) fn game_path(path: &str) -> Option<String> {
    path.contains('/').then(|| path.to_ascii_lowercase())
}

/// Groups the files of one mod into variant groups and standalone files.
pub fn group(files: &[FileIndex]) -> Grouping {
    let mut parent: Vec<usize> = (0..files.len()).collect();
    fn root(parent: &mut [usize], mut i: usize) -> usize {
        while parent[i] != i {
            parent[i] = parent[parent[i]];
            i = parent[i];
        }
        i
    }
    let mut owner: HashMap<String, usize> = HashMap::new();
    for (i, file) in files.iter().enumerate() {
        for (path, _) in &file.entries {
            let Some(key) = game_path(path) else { continue };
            match owner.get(&key) {
                Some(&j) => {
                    let (a, b) = (root(&mut parent, i), root(&mut parent, j));
                    parent[a.max(b)] = a.min(b);
                }
                None => {
                    owner.insert(key, i);
                }
            }
        }
    }
    let mut components: BTreeMap<usize, Vec<String>> = BTreeMap::new();
    for (i, file) in files.iter().enumerate() {
        let r = root(&mut parent, i);
        components.entry(r).or_default().push(file.id.clone());
    }
    let mut grouping = Grouping::default();
    for members in components.into_values() {
        if members.len() > 1 {
            grouping.variant_groups.push(members);
        } else {
            grouping.standalone.extend(members);
        }
    }
    grouping
}

/// Conflicts between enabled files in priority order (first wins). Identical content (same CRC32) is not a conflict.
pub fn conflicts(ordered: &[FileIndex]) -> Vec<Conflict> {
    let mut first: HashMap<String, (usize, u32, &str)> = HashMap::new();
    let mut pairs: BTreeMap<(usize, usize), BTreeSet<String>> = BTreeMap::new();
    for (i, file) in ordered.iter().enumerate() {
        for (path, crc) in &file.entries {
            let Some(key) = game_path(path) else { continue };
            match first.get(&key) {
                Some(&(w, wcrc, wpath)) if w != i && wcrc != *crc => {
                    pairs.entry((w, i)).or_default().insert(wpath.to_owned());
                }
                Some(_) => {}
                None => {
                    first.insert(key, (i, *crc, path.as_str()));
                }
            }
        }
    }
    pairs
        .into_iter()
        .map(|((w, l), paths)| Conflict {
            winner: ordered[w].id.clone(),
            loser: ordered[l].id.clone(),
            paths: paths.into_iter().collect(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn index(id: &str, entries: &[(&str, u32)]) -> FileIndex {
        FileIndex {
            id: id.to_owned(),
            entries: entries.iter().map(|&(p, c)| (p.to_owned(), c)).collect(),
        }
    }

    fn ids(v: &[String]) -> Vec<&str> {
        v.iter().map(String::as_str).collect()
    }

    #[test]
    fn colour_options_of_one_skin_form_one_variant_group() {
        let files: Vec<FileIndex> = ["base", "crimson", "cherry", "skyblue"]
            .iter()
            .map(|c| {
                index(
                    c,
                    &[
                        ("models/heroes/celeste/legs.vmdl_c", 1),
                        ("materials/celeste/legs.vtex_c", 2),
                    ],
                )
            })
            .collect();
        let g = group(&files);
        assert_eq!(g.variant_groups.len(), 1);
        assert_eq!(
            ids(&g.variant_groups[0]),
            ["base", "crimson", "cherry", "skyblue"]
        );
        assert!(g.standalone.is_empty());
    }

    #[test]
    fn files_that_change_different_things_are_standalone() {
        let g = group(&[
            index("skin", &[("models/heroes/haze/body.vmdl_c", 1)]),
            index("sounds", &[("sounds/heroes/haze/ult.vsnd_c", 2)]),
        ]);
        assert!(g.variant_groups.is_empty());
        assert_eq!(ids(&g.standalone), ["skin", "sounds"]);
    }

    #[test]
    fn files_in_the_package_root_do_not_make_files_variants() {
        // Mod authors put README.txt / credits in the root of every VPK; the game doesn't care about them.
        let g = group(&[
            index(
                "skin",
                &[("README.txt", 0), ("models/heroes/haze/body.vmdl_c", 1)],
            ),
            index(
                "sounds",
                &[("README.txt", 0), ("sounds/heroes/haze/ult.vsnd_c", 2)],
            ),
        ]);
        assert!(g.variant_groups.is_empty());
    }

    #[test]
    fn paths_match_without_case() {
        let g = group(&[
            index("a", &[("Models/Heroes/Haze/Body.vmdl_c", 1)]),
            index("b", &[("models/heroes/haze/body.vmdl_c", 2)]),
        ]);
        assert_eq!(g.variant_groups.len(), 1);
    }

    #[test]
    fn overlaps_chain_into_one_group() {
        let g = group(&[
            index("a", &[("x/1", 1)]),
            index("b", &[("x/1", 1), ("x/2", 1)]),
            index("c", &[("x/2", 1)]),
            index("d", &[("y/1", 1)]),
        ]);
        assert_eq!(g.variant_groups.len(), 1);
        assert_eq!(ids(&g.variant_groups[0]), ["a", "b", "c"]);
        assert_eq!(ids(&g.standalone), ["d"]);
    }

    #[test]
    fn two_independent_variant_groups_stay_apart() {
        let g = group(&[
            index("red", &[("models/body", 1)]),
            index("blue", &[("models/body", 2)]),
            index("voice-a", &[("sounds/vo", 3)]),
            index("voice-b", &[("sounds/vo", 4)]),
        ]);
        assert_eq!(g.variant_groups.len(), 2);
        assert_eq!(ids(&g.variant_groups[0]), ["red", "blue"]);
        assert_eq!(ids(&g.variant_groups[1]), ["voice-a", "voice-b"]);
    }

    #[test]
    fn the_higher_file_wins_a_conflict() {
        let c = conflicts(&[
            index(
                "yamato-redesign",
                &[
                    ("models/heroes/yamato/body.vmdl_c", 10),
                    ("ui/icon.vtex_c", 1),
                ],
            ),
            index(
                "yamato-remodel",
                &[("models/heroes/yamato/body.vmdl_c", 20)],
            ),
        ]);
        assert_eq!(
            c,
            [Conflict {
                winner: "yamato-redesign".into(),
                loser: "yamato-remodel".into(),
                paths: vec!["models/heroes/yamato/body.vmdl_c".into()],
            }]
        );
    }

    #[test]
    fn identical_files_are_not_conflicts() {
        assert!(
            conflicts(&[
                index("a", &[("x/shared", 5)]),
                index("b", &[("x/shared", 5)])
            ])
            .is_empty()
        );
    }

    #[test]
    fn root_files_are_not_conflicts() {
        assert!(
            conflicts(&[
                index("a", &[("README.txt", 1)]),
                index("b", &[("README.txt", 2)])
            ])
            .is_empty()
        );
    }

    #[test]
    fn the_winner_is_reported_against_each_loser() {
        let c = conflicts(&[
            index("a", &[("x/p", 1)]),
            index("b", &[("x/p", 2)]),
            index("c", &[("x/p", 3)]),
        ]);
        let pairs: Vec<(&str, &str)> = c
            .iter()
            .map(|c| (c.winner.as_str(), c.loser.as_str()))
            .collect();
        assert_eq!(pairs, [("a", "b"), ("a", "c")]);
    }

    #[test]
    fn reordering_flips_the_winner() {
        let c = conflicts(&[index("b", &[("x/p", 2)]), index("a", &[("x/p", 1)])]);
        assert_eq!((c[0].winner.as_str(), c[0].loser.as_str()), ("b", "a"));
    }

    #[test]
    fn conflicting_paths_are_sorted_and_grouped_per_pair() {
        let c = conflicts(&[
            index("a", &[("z/2", 1), ("z/1", 1), ("q/3", 1)]),
            index("b", &[("z/1", 9), ("z/2", 9), ("q/3", 1)]),
        ]);
        assert_eq!(c.len(), 1);
        assert_eq!(c[0].paths, ["z/1", "z/2"]);
    }
}
