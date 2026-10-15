//! Removing mods or single files (old versions) from the library (decision 42, dialog D6).
//!
//! The manifest is saved before any file is deleted: a crash in between leaves files nobody refers to, which
//! [`sweep`] removes on the next start, instead of a manifest pointing at files that are gone. Files deadloader has put
//! into a game folder are never removed; the profile is applied without them first.

use crate::library::{Manifest, Profile};
use std::collections::HashSet;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Target {
    Mod(String),
    /// Some files of one mod (old versions, unwanted parts).
    Files(Vec<String>),
}

/// What D6 shows before the user confirms.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct RemovalPlan {
    pub mod_id: String,
    pub files: Vec<String>,
    pub bytes: u64,
    /// Whether the mod itself goes (all its files are removed).
    pub removes_mod: bool,
    /// Names of the profiles that use the mod (or, for files, chose one of them).
    pub profiles: Vec<String>,
    /// Game folders that still contain some of the files; removal is refused until they are applied without them.
    pub applied_in: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct Removed {
    pub files: Vec<String>,
    /// Files the manifest no longer refers to but that could not be deleted; [`sweep`] tries again.
    pub leftovers: Vec<PathBuf>,
}

#[derive(Debug)]
pub enum RemoveError {
    UnknownMod(String),
    UnknownFile(String),
    NothingToRemove,
    SeveralMods,
    StillInGame(Vec<String>),
    Io(io::Error),
}

impl std::fmt::Display for RemoveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownMod(id) => write!(f, "the mod \"{id}\" is not in the library"),
            Self::UnknownFile(id) => write!(f, "the file \"{id}\" is not in the library"),
            Self::NothingToRemove => f.write_str("no files were chosen"),
            Self::SeveralMods => f.write_str("the files belong to different mods"),
            Self::StillInGame(roots) => write!(
                f,
                "the files are still in the game folder ({}); apply a profile without them first",
                roots.join(", ")
            ),
            Self::Io(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for RemoveError {}

/// How a library file is deleted: permanently or to the recycle bin (decision 42).
pub trait Delete {
    fn delete(&self, path: &Path) -> io::Result<()>;
}

/// Deletes for good, read-only library files included.
#[derive(Debug, Default, Clone, Copy)]
pub struct Permanently;

impl Delete for Permanently {
    fn delete(&self, path: &Path) -> io::Result<()> {
        let mut perm = std::fs::metadata(path)?.permissions();
        if perm.readonly() {
            #[allow(clippy::permissions_set_readonly_false)]
            // Windows only: clears the read-only attribute.
            perm.set_readonly(false);
            std::fs::set_permissions(path, perm)?;
        }
        std::fs::remove_file(path)
    }
}

pub fn plan(
    manifest: &Manifest,
    target: &Target,
    profiles: &[Profile],
) -> Result<RemovalPlan, RemoveError> {
    let (m, files): (_, Vec<String>) = match target {
        Target::Mod(id) => {
            let m = manifest
                .mods
                .iter()
                .find(|m| &m.id == id)
                .ok_or_else(|| RemoveError::UnknownMod(id.clone()))?;
            (m, m.files.iter().map(|f| f.id.clone()).collect())
        }
        Target::Files(ids) => {
            let first = ids.first().ok_or(RemoveError::NothingToRemove)?;
            let (m, _) = manifest
                .file(first)
                .ok_or_else(|| RemoveError::UnknownFile(first.clone()))?;
            for id in ids {
                match manifest.file(id) {
                    None => return Err(RemoveError::UnknownFile(id.clone())),
                    Some((other, _)) if other.id != m.id => return Err(RemoveError::SeveralMods),
                    Some(_) => {}
                }
            }
            (m, ids.clone())
        }
    };
    let chosen: HashSet<&str> = files.iter().map(String::as_str).collect();
    let removes_mod = m.files.iter().all(|f| chosen.contains(f.id.as_str()));
    let bytes = m
        .files
        .iter()
        .filter(|f| chosen.contains(f.id.as_str()))
        .map(|f| f.len)
        .sum();
    let profiles = profiles
        .iter()
        .filter(|p| {
            p.entries.iter().any(|e| {
                e.mod_id == m.id
                    && (removes_mod
                        || e.variants
                            .iter()
                            .chain(&e.add_ons)
                            .any(|f| chosen.contains(f.as_str())))
            })
        })
        .map(|p| p.name.clone())
        .collect();
    let applied_in = manifest
        .installations
        .iter()
        .filter(|i| i.applied.iter().any(|r| chosen.contains(r.file.as_str())))
        .map(|i| i.game_root.clone())
        .collect();
    Ok(RemovalPlan {
        mod_id: m.id.clone(),
        files,
        bytes,
        removes_mod,
        profiles,
        applied_in,
    })
}

/// Removes the files from `manifest`, persists it with `save`, then deletes them from disk.
pub fn remove(
    data_dir: &Path,
    manifest: &mut Manifest,
    target: &Target,
    deleter: &dyn Delete,
    mut save: impl FnMut(&Manifest) -> io::Result<()>,
) -> Result<Removed, RemoveError> {
    let plan = plan(manifest, target, &[])?;
    if !plan.applied_in.is_empty() {
        return Err(RemoveError::StillInGame(plan.applied_in));
    }
    let chosen: HashSet<&str> = plan.files.iter().map(String::as_str).collect();
    let mut updated = manifest.clone();
    let mut paths = Vec::new();
    for m in updated.mods.iter_mut().filter(|m| m.id == plan.mod_id) {
        for f in m.files.iter().filter(|f| chosen.contains(f.id.as_str())) {
            paths.push(disk_path(data_dir, &f.library_path));
        }
        m.files.retain(|f| !chosen.contains(f.id.as_str()));
    }
    if plan.removes_mod {
        updated.mods.retain(|m| m.id != plan.mod_id);
    }
    save(&updated).map_err(RemoveError::Io)?;
    *manifest = updated;

    let mut leftovers = Vec::new();
    for path in &paths {
        if path.exists() && deleter.delete(path).is_err() {
            leftovers.push(path.clone());
        }
    }
    let library = data_dir.join("library");
    for path in &paths {
        remove_empty_parents(path, &library);
    }
    Ok(Removed {
        files: plan.files,
        leftovers,
    })
}

/// Drops from `profile` the mods and files `manifest` no longer has; whether anything changed.
pub fn forget_missing(profile: &mut Profile, manifest: &Manifest) -> bool {
    let before = profile.entries.clone();
    profile
        .entries
        .retain(|e| manifest.mods.iter().any(|m| m.id == e.mod_id));
    for e in &mut profile.entries {
        let known = |id: &String| manifest.file(id).is_some_and(|(m, _)| m.id == e.mod_id);
        let variants = e.variants.iter().filter(|f| known(f)).cloned().collect();
        let add_ons = e.add_ons.iter().filter(|f| known(f)).cloned().collect();
        e.variants = variants;
        e.add_ons = add_ons;
    }
    profile.entries != before
}

/// Deletes the files under `library\` that `manifest` does not refer to (left by a crash or a failed delete) and
/// returns them. Run at start with a manifest that loaded successfully — never with a default one standing in for a
/// manifest that failed to load, which would make every library file look unknown.
pub fn sweep(
    data_dir: &Path,
    manifest: &Manifest,
    deleter: &dyn Delete,
) -> io::Result<Vec<PathBuf>> {
    let library = data_dir.join("library");
    let known: HashSet<String> = manifest
        .mods
        .iter()
        .flat_map(|m| &m.files)
        .map(|f| f.library_path.replace('\\', "/").to_lowercase())
        .collect();
    let mut found = Vec::new();
    match walk(&library, "library", &mut found) {
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
        other => other?,
    }
    let mut swept = Vec::new();
    for (path, relative) in found {
        if !known.contains(&relative.to_lowercase()) && deleter.delete(&path).is_ok() {
            remove_empty_parents(&path, &library);
            swept.push(path);
        }
    }
    Ok(swept)
}

/// Every file under `dir`, with its path relative to the data folder in manifest form.
fn walk(dir: &Path, relative: &str, out: &mut Vec<(PathBuf, String)>) -> io::Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().into_owned();
        let rel = format!("{relative}/{name}");
        if entry.file_type()?.is_dir() {
            walk(&entry.path(), &rel, out)?;
        } else {
            out.push((entry.path(), rel));
        }
    }
    Ok(())
}

/// Removes the now-empty folders between `path` and `stop` (exclusive).
fn remove_empty_parents(path: &Path, stop: &Path) {
    let mut dir = path.parent();
    while let Some(d) = dir {
        if d == stop || !d.starts_with(stop) || std::fs::remove_dir(d).is_err() {
            break;
        }
        dir = d.parent();
    }
}

fn disk_path(data_dir: &Path, library_path: &str) -> PathBuf {
    library_path
        .split(['/', '\\'])
        .fold(data_dir.to_path_buf(), |p, c| p.join(c))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::{
        AppliedRecord, InstallationState, Manifest, Mod, ModFile, Profile, ProfileEntry, Source,
    };
    use std::cell::RefCell;
    use std::io;
    use std::path::{Path, PathBuf};

    fn temp_dir(name: &str) -> PathBuf {
        let dir =
            std::env::temp_dir().join(format!("deadloader-remove-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("temp dir");
        dir
    }

    /// Library files are read-only; delete them the way the library does before removing the folder.
    fn cleanup(data: &Path) {
        let _ = sweep(data, &Manifest::default(), &Permanently);
        let _ = std::fs::remove_dir_all(data);
    }

    fn file(mod_id: &str, batch: u32, n: u32, len: u64) -> ModFile {
        ModFile {
            id: format!("{mod_id}-{batch}-{n}"),
            remote_file_id: None,
            archive_name: "a.zip".into(),
            inner_path: format!("{n}.vpk"),
            library_path: format!("library/{mod_id}/{batch}/{n}.vpk"),
            len,
            sha256: [0; 32],
            version: None,
        }
    }

    fn mod_with(id: &str, files: Vec<ModFile>) -> Mod {
        Mod {
            id: id.into(),
            name: id.into(),
            author: None,
            source: Source::Local {
                imported_from: None,
            },
            files,
        }
    }

    /// Writes every library file of `manifest` to disk, read-only like real library files.
    fn materialise(data: &Path, manifest: &Manifest) {
        for f in manifest.mods.iter().flat_map(|m| &m.files) {
            let path = disk_path(data, &f.library_path);
            std::fs::create_dir_all(path.parent().expect("parent")).expect("dir");
            std::fs::write(&path, vec![1u8; f.len as usize]).expect("write");
            let mut perm = std::fs::metadata(&path).expect("meta").permissions();
            perm.set_readonly(true);
            std::fs::set_permissions(&path, perm).expect("read-only");
        }
    }

    fn disk_path(data: &Path, library_path: &str) -> PathBuf {
        library_path
            .split('/')
            .fold(data.to_path_buf(), |p, c| p.join(c))
    }

    fn library() -> Manifest {
        Manifest {
            mods: vec![
                mod_with(
                    "skin",
                    vec![
                        file("skin", 1, 0, 10),
                        file("skin", 2, 0, 20),
                        file("skin", 2, 1, 5),
                    ],
                ),
                mod_with("sounds", vec![file("sounds", 1, 0, 7)]),
            ],
            ..Manifest::default()
        }
    }

    fn profile(name: &str, entries: Vec<ProfileEntry>) -> Profile {
        Profile {
            name: name.into(),
            game_root: None,
            entries,
        }
    }

    fn entry(mod_id: &str, variants: &[&str], add_ons: &[&str]) -> ProfileEntry {
        ProfileEntry {
            mod_id: mod_id.into(),
            variants: variants.iter().map(|s| s.to_string()).collect(),
            add_ons: add_ons.iter().map(|s| s.to_string()).collect(),
            enabled: true,
        }
    }

    fn applied(game_root: &str, files: &[&str]) -> InstallationState {
        InstallationState {
            game_root: game_root.into(),
            applied_profile: Some("Main".into()),
            applied: files
                .iter()
                .enumerate()
                .map(|(i, f)| AppliedRecord {
                    slot: i as u8 + 1,
                    file: f.to_string(),
                    len: 1,
                })
                .collect(),
        }
    }

    fn ids(v: &[String]) -> Vec<&str> {
        v.iter().map(String::as_str).collect()
    }

    /// Records what it was asked to delete, and deletes it permanently.
    #[derive(Default)]
    struct Recording(RefCell<Vec<PathBuf>>);

    impl Delete for Recording {
        fn delete(&self, path: &Path) -> io::Result<()> {
            self.0.borrow_mut().push(path.to_path_buf());
            Permanently.delete(path)
        }
    }

    struct Refusing;

    impl Delete for Refusing {
        fn delete(&self, _: &Path) -> io::Result<()> {
            Err(io::Error::from(io::ErrorKind::PermissionDenied))
        }
    }

    #[test]
    fn the_plan_lists_files_size_and_the_profiles_using_the_mod() {
        let m = library();
        let profiles = [
            profile("Main", vec![entry("skin", &["skin-2-0"], &[])]),
            profile("Sounds only", vec![entry("sounds", &[], &["sounds-1-0"])]),
            profile(
                "Both",
                vec![entry("sounds", &[], &[]), entry("skin", &[], &[])],
            ),
        ];
        let plan = plan(&m, &Target::Mod("skin".into()), &profiles).expect("plan");
        assert_eq!(ids(&plan.files), ["skin-1-0", "skin-2-0", "skin-2-1"]);
        assert_eq!(plan.bytes, 35);
        assert_eq!(ids(&plan.profiles), ["Main", "Both"]);
        assert!(plan.applied_in.is_empty());
        assert!(plan.removes_mod);
    }

    #[test]
    fn the_plan_for_old_files_names_only_profiles_that_chose_them() {
        let m = library();
        let profiles = [
            profile("Old", vec![entry("skin", &["skin-1-0"], &[])]),
            profile("New", vec![entry("skin", &["skin-2-0"], &["skin-2-1"])]),
        ];
        let target = Target::Files(vec!["skin-1-0".into()]);
        let plan = plan(&m, &target, &profiles).expect("plan");
        assert_eq!(ids(&plan.files), ["skin-1-0"]);
        assert_eq!(plan.bytes, 10);
        assert_eq!(ids(&plan.profiles), ["Old"]);
        assert!(!plan.removes_mod);
    }

    #[test]
    fn removing_every_file_of_a_mod_removes_the_mod() {
        let m = library();
        let target = Target::Files(vec!["sounds-1-0".into()]);
        assert!(plan(&m, &target, &[]).expect("plan").removes_mod);
    }

    #[test]
    fn the_plan_shows_where_the_files_are_still_in_the_game() {
        let mut m = library();
        m.installations
            .push(applied(r"D:\Games\Deadlock", &["sounds-1-0", "skin-2-0"]));
        let plan = plan(&m, &Target::Mod("skin".into()), &[]).expect("plan");
        assert_eq!(ids(&plan.applied_in), [r"D:\Games\Deadlock"]);
    }

    #[test]
    fn unknown_targets_are_refused() {
        let m = library();
        assert!(matches!(
            plan(&m, &Target::Mod("nope".into()), &[]),
            Err(RemoveError::UnknownMod(id)) if id == "nope"
        ));
        assert!(matches!(
            plan(&m, &Target::Files(vec!["skin-9-9".into()]), &[]),
            Err(RemoveError::UnknownFile(id)) if id == "skin-9-9"
        ));
        assert!(matches!(
            plan(&m, &Target::Files(vec![]), &[]),
            Err(RemoveError::NothingToRemove)
        ));
    }

    #[test]
    fn files_from_two_mods_in_one_request_are_refused() {
        let m = library();
        let target = Target::Files(vec!["skin-1-0".into(), "sounds-1-0".into()]);
        assert!(matches!(
            plan(&m, &target, &[]),
            Err(RemoveError::SeveralMods)
        ));
    }

    #[test]
    fn a_mod_is_removed_from_the_manifest_and_the_disk() {
        let data = temp_dir("mod");
        let mut m = library();
        materialise(&data, &m);
        let deleter = Recording::default();
        let mut saved = Vec::new();
        let removed = remove(&data, &mut m, &Target::Mod("skin".into()), &deleter, |m| {
            saved.push(m.clone());
            Ok(())
        })
        .expect("removed");

        assert_eq!(ids(&removed.files), ["skin-1-0", "skin-2-0", "skin-2-1"]);
        assert!(removed.leftovers.is_empty());
        assert_eq!(m.mods.len(), 1);
        assert_eq!(m.mods[0].id, "sounds");
        assert_eq!(saved, [m.clone()]);
        assert_eq!(deleter.0.borrow().len(), 3);
        assert!(
            !data.join("library").join("skin").exists(),
            "empty folders go too"
        );
        assert!(
            data.join("library")
                .join("sounds")
                .join("1")
                .join("0.vpk")
                .exists()
        );
        cleanup(&data);
    }

    #[test]
    fn old_files_are_removed_and_the_rest_of_the_mod_stays() {
        let data = temp_dir("old");
        let mut m = library();
        materialise(&data, &m);
        let target = Target::Files(vec!["skin-1-0".into()]);
        remove(&data, &mut m, &target, &Permanently, |_| Ok(())).expect("removed");

        let skin = m.mods.iter().find(|x| x.id == "skin").expect("mod stays");
        let left: Vec<&str> = skin.files.iter().map(|f| f.id.as_str()).collect();
        assert_eq!(left, ["skin-2-0", "skin-2-1"]);
        assert!(!data.join("library").join("skin").join("1").exists());
        assert!(
            data.join("library")
                .join("skin")
                .join("2")
                .join("1.vpk")
                .exists()
        );
        cleanup(&data);
    }

    #[test]
    fn files_still_in_the_game_are_not_removed() {
        // The game folder holds hard links to them and the manifest records them; the profile has to be applied
        // without them first, otherwise deadloader would lose track of files it put into the game.
        let data = temp_dir("applied");
        let mut m = library();
        m.installations
            .push(applied(r"D:\Games\Deadlock", &["skin-2-0"]));
        materialise(&data, &m);
        let before = m.clone();
        let err = remove(
            &data,
            &mut m,
            &Target::Mod("skin".into()),
            &Permanently,
            |_| panic!("nothing to save"),
        )
        .expect_err("refused");
        assert!(matches!(err, RemoveError::StillInGame(roots) if roots == [r"D:\Games\Deadlock"]));
        assert_eq!(m, before);
        assert!(
            data.join("library")
                .join("skin")
                .join("2")
                .join("0.vpk")
                .exists()
        );
        cleanup(&data);
    }

    #[test]
    fn nothing_is_deleted_when_the_manifest_cannot_be_saved() {
        let data = temp_dir("nosave");
        let mut m = library();
        materialise(&data, &m);
        let before = m.clone();
        let deleter = Recording::default();
        let err = remove(&data, &mut m, &Target::Mod("skin".into()), &deleter, |_| {
            Err(io::Error::other("disk full"))
        })
        .expect_err("save failed");
        assert!(matches!(err, RemoveError::Io(_)));
        assert_eq!(m, before);
        assert!(deleter.0.borrow().is_empty());
        cleanup(&data);
    }

    #[test]
    fn files_that_cannot_be_deleted_are_reported_as_leftovers() {
        // The manifest no longer knows them; the start-up sweep tries again.
        let data = temp_dir("leftover");
        let mut m = library();
        materialise(&data, &m);
        let removed = remove(
            &data,
            &mut m,
            &Target::Mod("sounds".into()),
            &Refusing,
            |_| Ok(()),
        )
        .expect("removed from the manifest");
        assert_eq!(
            removed.leftovers,
            [disk_path(&data, "library/sounds/1/0.vpk")]
        );
        assert!(m.mods.iter().all(|x| x.id != "sounds"));
        cleanup(&data);
    }

    #[test]
    fn a_file_already_gone_from_disk_is_not_an_error() {
        let data = temp_dir("gone");
        let mut m = library();
        let removed = remove(
            &data,
            &mut m,
            &Target::Mod("sounds".into()),
            &Permanently,
            |_| Ok(()),
        )
        .expect("removed");
        assert!(removed.leftovers.is_empty());
        cleanup(&data);
    }

    #[test]
    fn profiles_forget_removed_mods_and_files() {
        let mut m = library();
        let mut p = profile(
            "Main",
            vec![
                entry("skin", &["skin-1-0"], &["skin-2-1"]),
                entry("sounds", &[], &["sounds-1-0"]),
            ],
        );
        m.mods[0].files.retain(|f| f.id != "skin-1-0");
        assert!(forget_missing(&mut p, &m));
        assert_eq!(p.entries[0], entry("skin", &[], &["skin-2-1"]));

        m.mods.retain(|x| x.id != "sounds");
        assert!(forget_missing(&mut p, &m));
        assert_eq!(p.entries.len(), 1);
        assert!(!forget_missing(&mut p, &m), "nothing left to forget");
    }

    #[test]
    fn the_sweep_deletes_library_files_the_manifest_does_not_know() {
        let data = temp_dir("sweep");
        let m = library();
        materialise(&data, &m);
        // A crashed import (folder 3) and a stray file next to known ones.
        let stray_batch = data.join("library").join("skin").join("3");
        std::fs::create_dir_all(&stray_batch).expect("dir");
        std::fs::write(stray_batch.join("0.vpk"), b"x").expect("write");
        std::fs::write(
            data.join("library").join("skin").join("2").join("9.vpk"),
            b"x",
        )
        .expect("write");
        let unknown_mod = data.join("library").join("ghost").join("1");
        std::fs::create_dir_all(&unknown_mod).expect("dir");
        std::fs::write(unknown_mod.join("0.vpk"), b"x").expect("write");

        let mut swept = sweep(&data, &m, &Permanently).expect("swept");
        swept.sort();
        let mut expected = vec![
            stray_batch.join("0.vpk"),
            data.join("library").join("skin").join("2").join("9.vpk"),
            unknown_mod.join("0.vpk"),
        ];
        expected.sort();
        assert_eq!(swept, expected);
        assert!(!stray_batch.exists());
        assert!(!data.join("library").join("ghost").exists());
        for f in m.mods.iter().flat_map(|x| &x.files) {
            assert!(disk_path(&data, &f.library_path).exists(), "{} kept", f.id);
        }
        cleanup(&data);
    }

    #[test]
    fn the_sweep_matches_paths_without_case() {
        // Windows paths are case-insensitive; a hand-edited manifest may spell a folder differently.
        let data = temp_dir("sweepcase");
        let mut m = library();
        materialise(&data, &m);
        m.mods[1].files[0].library_path = "Library/Sounds/1/0.vpk".into();
        let swept = sweep(&data, &m, &Permanently).expect("swept");
        assert!(swept.is_empty(), "{swept:?}");
        cleanup(&data);
    }

    #[test]
    fn the_sweep_without_a_library_folder_does_nothing() {
        let data = temp_dir("sweepnone");
        assert!(
            sweep(&data, &Manifest::default(), &Permanently)
                .expect("ok")
                .is_empty()
        );
        cleanup(&data);
    }

    #[test]
    fn permanent_deletion_clears_the_read_only_flag() {
        let data = temp_dir("ro");
        let path = data.join("f.vpk");
        std::fs::write(&path, b"x").expect("write");
        let mut perm = std::fs::metadata(&path).expect("meta").permissions();
        perm.set_readonly(true);
        std::fs::set_permissions(&path, perm).expect("ro");
        Permanently.delete(&path).expect("deleted");
        assert!(!path.exists());
        cleanup(&data);
    }
}
