//! Which files of a mod a profile uses: after an install, when the user must pick a variant, switching variants and
//! moving profiles to an updated file (decisions 23, 24, 136, 137, 144; flow F2, dialog D4).
//!
//! The rule everywhere: per variant group (files changing the same game files) at most one file is active, kept in
//! `variants`; files overlapping nothing are add-ons, enabled together, kept in `add_ons`.

use crate::library::{Profile, ProfileEntry};
use crate::overlap::Grouping;

/// The groups the user must choose in (D4): those with two or more of the just-installed files. One new file in a
/// group simply becomes its active variant (144).
pub fn choices_needed(grouping: &Grouping, new_files: &[String]) -> Vec<Vec<String>> {
    grouping
        .variant_groups
        .iter()
        .map(|g| {
            g.iter()
                .filter(|f| new_files.contains(f))
                .cloned()
                .collect::<Vec<_>>()
        })
        .filter(|news| news.len() >= 2)
        .collect()
}

/// Makes `chosen` active for `mod_id` in `profile`: a mod new to the profile goes to the top (136); a mod already in
/// it keeps its place and is switched on.
pub fn install(profile: &mut Profile, mod_id: &str, grouping: &Grouping, chosen: &[String]) {
    let index = match profile.entries.iter().position(|e| e.mod_id == mod_id) {
        Some(i) => i,
        None => {
            profile.entries.insert(
                0,
                ProfileEntry {
                    mod_id: mod_id.to_owned(),
                    variants: Vec::new(),
                    add_ons: Vec::new(),
                    enabled: true,
                },
            );
            0
        }
    };
    let entry = &mut profile.entries[index];
    entry.enabled = true;
    for file in chosen {
        switch_to(entry, grouping, file);
    }
}

/// Activates `file`: in a variant group it takes the place of the group's active file; otherwise it is enabled.
pub fn switch_to(entry: &mut ProfileEntry, grouping: &Grouping, file: &str) {
    match group_of(grouping, file) {
        Some(group) => {
            entry.add_ons.retain(|f| !group.contains(f));
            match entry.variants.iter().position(|f| group.contains(f)) {
                Some(i) => entry.variants[i] = file.to_owned(),
                None => entry.variants.push(file.to_owned()),
            }
        }
        None => {
            if !entry.add_ons.iter().any(|f| f == file) {
                entry.add_ons.push(file.to_owned());
            }
        }
    }
    normalise(entry, grouping);
}

/// Repairs an entry against the mod's grouping: unknown files dropped, one file per group (the first listed),
/// add-ons only where they overlap nothing.
pub fn normalise(entry: &mut ProfileEntry, grouping: &Grouping) {
    let mut active_groups: Vec<usize> = Vec::new();
    let mut variants = Vec::new();
    let mut add_ons: Vec<String> = Vec::new();
    for file in entry.variants.iter().chain(&entry.add_ons) {
        if let Some(g) = grouping
            .variant_groups
            .iter()
            .position(|g| g.contains(file))
        {
            if !active_groups.contains(&g) {
                active_groups.push(g);
                variants.push(file.clone());
            }
        } else if grouping.standalone.contains(file) && !add_ons.contains(file) {
            add_ons.push(file.clone());
        }
    }
    entry.variants = variants;
    entry.add_ons = add_ons;
}

/// After an update (137): every profile that used `old` of `mod_id` uses `new` instead. Returns the indexes of the
/// profiles that changed (the caller saves them).
pub fn replace_file(profiles: &mut [Profile], mod_id: &str, old: &str, new: &str) -> Vec<usize> {
    let mut changed = Vec::new();
    for (i, profile) in profiles.iter_mut().enumerate() {
        let mut touched = false;
        for entry in profile.entries.iter_mut().filter(|e| e.mod_id == mod_id) {
            for f in entry.variants.iter_mut().chain(entry.add_ons.iter_mut()) {
                if f == old {
                    *f = new.to_owned();
                    touched = true;
                }
            }
        }
        if touched {
            changed.push(i);
        }
    }
    changed
}

/// The toggle in the library list: the mod keeps its place and choices. Whether anything changed.
pub fn set_enabled(profile: &mut Profile, mod_id: &str, enabled: bool) -> bool {
    match profile.entries.iter_mut().find(|e| e.mod_id == mod_id) {
        Some(e) if e.enabled != enabled => {
            e.enabled = enabled;
            true
        }
        _ => false,
    }
}

fn group_of<'a>(grouping: &'a Grouping, file: &str) -> Option<&'a Vec<String>> {
    grouping
        .variant_groups
        .iter()
        .find(|g| g.iter().any(|f| f == file))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::{Profile, ProfileEntry};
    use crate::overlap::Grouping;

    fn s(v: &[&str]) -> Vec<String> {
        v.iter().map(|x| x.to_string()).collect()
    }

    fn grouping(groups: &[&[&str]], standalone: &[&str]) -> Grouping {
        Grouping {
            variant_groups: groups.iter().map(|g| s(g)).collect(),
            standalone: s(standalone),
        }
    }

    fn entry(mod_id: &str, variants: &[&str], add_ons: &[&str]) -> ProfileEntry {
        ProfileEntry {
            mod_id: mod_id.into(),
            variants: s(variants),
            add_ons: s(add_ons),
            enabled: true,
        }
    }

    fn profile(entries: Vec<ProfileEntry>) -> Profile {
        Profile {
            name: "Main".into(),
            game_root: None,
            entries,
        }
    }

    #[test]
    fn several_new_files_of_one_group_need_a_choice() {
        // A fresh archive with colour options (D4).
        let g = grouping(&[&["red", "blue", "green"]], &["hat"]);
        assert_eq!(
            choices_needed(&g, &s(&["red", "blue", "green", "hat"])),
            [s(&["red", "blue", "green"])]
        );
    }

    #[test]
    fn one_new_file_in_a_group_needs_no_choice() {
        // The second colour of a mod downloaded later becomes the active variant (144).
        let g = grouping(&[&["red", "blue"]], &[]);
        assert!(choices_needed(&g, &s(&["blue"])).is_empty());
    }

    #[test]
    fn a_choice_is_only_among_the_new_files() {
        let g = grouping(&[&["old", "new1", "new2"]], &[]);
        assert_eq!(
            choices_needed(&g, &s(&["new1", "new2"])),
            [s(&["new1", "new2"])]
        );
    }

    #[test]
    fn a_new_mod_goes_to_the_top_with_its_add_ons() {
        let mut p = profile(vec![entry("other", &[], &["o"])]);
        let g = grouping(&[&["red", "blue"]], &["hat"]);
        install(&mut p, "skin", &g, &s(&["blue", "hat"]));
        assert_eq!(p.entries[0], entry("skin", &["blue"], &["hat"]));
        assert_eq!(p.entries[1].mod_id, "other");
    }

    #[test]
    fn a_new_variant_replaces_the_active_one_and_the_mod_keeps_its_place() {
        let mut p = profile(vec![
            entry("other", &[], &["o"]),
            entry("skin", &["red"], &["hat"]),
        ]);
        let g = grouping(&[&["red", "blue"]], &["hat"]);
        install(&mut p, "skin", &g, &s(&["blue"]));
        assert_eq!(p.entries[1], entry("skin", &["blue"], &["hat"]));
        assert_eq!(p.entries[0].mod_id, "other");
    }

    #[test]
    fn an_add_on_that_now_overlaps_the_new_file_becomes_a_variant() {
        // "hat" was alone; the new "hat2" changes the same files, so they form a group and the new one wins.
        let mut p = profile(vec![entry("skin", &[], &["hat"])]);
        let g = grouping(&[&["hat", "hat2"]], &[]);
        install(&mut p, "skin", &g, &s(&["hat2"]));
        assert_eq!(p.entries[0], entry("skin", &["hat2"], &[]));
    }

    #[test]
    fn switching_picks_one_variant_of_its_group() {
        let mut e = entry("skin", &["red", "big"], &["hat"]);
        let g = grouping(&[&["red", "blue"], &["big", "small"]], &["hat"]);
        switch_to(&mut e, &g, "blue");
        assert_eq!(e, entry("skin", &["blue", "big"], &["hat"]));
    }

    #[test]
    fn switching_to_a_standalone_file_enables_it() {
        let mut e = entry("skin", &["red"], &[]);
        let g = grouping(&[&["red", "blue"]], &["hat"]);
        switch_to(&mut e, &g, "hat");
        assert_eq!(e, entry("skin", &["red"], &["hat"]));
    }

    #[test]
    fn normalising_leaves_at_most_one_file_per_group() {
        // A hand-edited profile listing two colours keeps the first.
        let mut e = entry("skin", &["red", "blue"], &["blue", "hat", "gone"]);
        let g = grouping(&[&["red", "blue"]], &["hat"]);
        normalise(&mut e, &g);
        assert_eq!(e, entry("skin", &["red"], &["hat"]));
    }

    #[test]
    fn an_update_moves_every_profile_to_the_new_file() {
        // 137: the new version becomes active wherever the old one was used; the old file stays in the library.
        let mut profiles = [
            profile(vec![entry("skin", &["v1"], &[])]),
            profile(vec![entry("skin", &[], &["v1"])]),
            profile(vec![entry("skin", &["other"], &[])]),
        ];
        let changed = replace_file(&mut profiles, "skin", "v1", "v2");
        assert_eq!(changed, [0, 1]);
        assert_eq!(profiles[0].entries[0], entry("skin", &["v2"], &[]));
        assert_eq!(profiles[1].entries[0], entry("skin", &[], &["v2"]));
        assert_eq!(profiles[2].entries[0], entry("skin", &["other"], &[]));
    }

    #[test]
    fn a_disabled_mod_keeps_its_place_in_the_order() {
        // S4 shows every mod of the profile in order, with a toggle; switching off and on again must not move it.
        let mut p = profile(vec![entry("a", &[], &["a1"]), entry("b", &[], &["b1"])]);
        assert!(set_enabled(&mut p, "a", false));
        assert!(!set_enabled(&mut p, "a", false), "already off");
        assert_eq!(p.entries[0].mod_id, "a");
        assert!(!p.entries[0].enabled);
        assert!(set_enabled(&mut p, "a", true));
        assert_eq!(p.entries[0], entry("a", &[], &["a1"]));
        assert!(!set_enabled(&mut p, "missing", true), "not in the profile");
    }

    #[test]
    fn installing_a_file_of_a_disabled_mod_turns_it_on_where_it_is() {
        let mut p = profile(vec![
            entry("other", &[], &["o"]),
            entry("skin", &["red"], &[]),
        ]);
        p.entries[1].enabled = false;
        let g = grouping(&[&["red", "blue"]], &[]);
        install(&mut p, "skin", &g, &s(&["blue"]));
        assert_eq!(p.entries[1], entry("skin", &["blue"], &[]));
    }
}
