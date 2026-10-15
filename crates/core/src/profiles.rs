//! Profiles on disk (`profiles\<name>.json`) and their export/import as a shareable file (S5, F14, decisions 35–37).
//!
//! The file name is the profile's name (a user renaming the file in Explorer renamed the profile); names are unique
//! without case, like Windows file names. An export names mods by their source and files by content (SHA-256),
//! GameBanana file id and archive path, so it can be imported on another PC whose library uses other ids; it holds no
//! paths of the exporting machine.

use crate::fs::write_atomic;
use crate::library::{
    Manifest, Mod, ModFile, ModelError, Profile, ProfileEntry, Source, profile_file_name,
    profile_from_json, profile_to_json,
};
use serde::{Deserialize, Serialize};
use std::io;
use std::path::{Path, PathBuf};

pub const DEFAULT_NAME: &str = "Default";
pub const EXPORT_FORMAT: u32 = 1;

#[derive(Debug)]
pub enum ProfileError {
    Model(ModelError),
    NameTaken(String),
    NotFound(String),
    LastProfile,
    NotAProfile,
    Io(io::Error),
}

impl std::fmt::Display for ProfileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Model(e) => write!(f, "{e}"),
            Self::NameTaken(n) => write!(f, "a profile named \"{n}\" already exists"),
            Self::NotFound(n) => write!(f, "there is no profile named \"{n}\""),
            Self::LastProfile => f.write_str("the last profile can't be deleted"),
            Self::NotAProfile => f.write_str("the file is not a deadloader profile"),
            Self::Io(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for ProfileError {}

impl From<ModelError> for ProfileError {
    fn from(e: ModelError) -> Self {
        Self::Model(e)
    }
}

impl From<io::Error> for ProfileError {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

#[derive(Debug, Default)]
pub struct Listed {
    /// Sorted by name, without case.
    pub profiles: Vec<Profile>,
    /// Profile files that could not be read: file name and why.
    pub broken: Vec<(String, ModelError)>,
}

pub fn list(dir: &Path) -> io::Result<Listed> {
    let mut listed = Listed::default();
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(listed),
        Err(e) => return Err(e),
    };
    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name().to_string_lossy().into_owned();
        let Some(stem) = profile_stem(&file_name) else {
            continue;
        };
        if !entry.file_type()?.is_file() {
            continue;
        }
        let read = std::fs::read_to_string(entry.path())
            .map_err(ModelError::from)
            .and_then(|text| profile_from_json(&text));
        match read {
            Ok(mut profile) => {
                profile.name = stem.to_owned();
                listed.profiles.push(profile);
            }
            Err(e) => listed.broken.push((file_name, e)),
        }
    }
    listed.profiles.sort_by_key(|p| p.name.to_lowercase());
    Ok(listed)
}

pub fn load(dir: &Path, name: &str) -> Result<Profile, ProfileError> {
    let path = existing(dir, name)?.ok_or_else(|| ProfileError::NotFound(name.to_owned()))?;
    let mut profile = profile_from_json(&std::fs::read_to_string(&path)?)?;
    profile.name = path
        .file_name()
        .and_then(|n| profile_stem(&n.to_string_lossy()).map(str::to_owned))
        .unwrap_or_else(|| name.to_owned());
    Ok(profile)
}

/// Writes `profile` under its name, replacing a profile of that name.
pub fn save(dir: &Path, profile: &Profile) -> Result<(), ProfileError> {
    let path = match existing(dir, &profile.name)? {
        Some(path) => path,
        None => dir.join(profile_file_name(&profile.name)?),
    };
    std::fs::create_dir_all(dir)?;
    write_atomic(&path, &profile_to_json(profile)?)?;
    Ok(())
}

/// Writes a new profile; refuses a name already in use.
pub fn create(dir: &Path, profile: &Profile) -> Result<(), ProfileError> {
    let file_name = profile_file_name(&profile.name)?;
    if existing(dir, &profile.name)?.is_some() {
        return Err(ProfileError::NameTaken(profile.name.clone()));
    }
    std::fs::create_dir_all(dir)?;
    write_atomic(&dir.join(file_name), &profile_to_json(profile)?)?;
    Ok(())
}

/// Creates the empty "Default" profile when there is none (first start); returns it if it was created.
pub fn ensure_default(dir: &Path) -> Result<Option<Profile>, ProfileError> {
    if !list(dir)?.profiles.is_empty() {
        return Ok(None);
    }
    let profile = Profile {
        name: DEFAULT_NAME.to_owned(),
        game_root: None,
        entries: Vec::new(),
    };
    create(dir, &profile)?;
    Ok(Some(profile))
}

/// Renames a profile and the record of where it is applied; `save_manifest` persists the manifest if it changed.
pub fn rename(
    dir: &Path,
    manifest: &mut Manifest,
    old: &str,
    new: &str,
    mut save_manifest: impl FnMut(&Manifest) -> io::Result<()>,
) -> Result<(), ProfileError> {
    let from = existing(dir, old)?.ok_or_else(|| ProfileError::NotFound(old.to_owned()))?;
    let to = dir.join(profile_file_name(new)?);
    if let Some(other) = existing(dir, new)?
        && other != from
    {
        return Err(ProfileError::NameTaken(new.to_owned()));
    }
    let mut profile = load(dir, old)?;
    let mut updated = manifest.clone();
    let mut changed = false;
    for install in &mut updated.installations {
        if install
            .applied_profile
            .as_deref()
            .is_some_and(|p| same_name(p, old))
        {
            install.applied_profile = Some(new.to_owned());
            changed = true;
        }
    }
    if let Some(key) = updated
        .last_applied
        .keys()
        .find(|k| same_name(k, old))
        .cloned()
        && let Some(when) = updated.last_applied.remove(&key)
    {
        updated.last_applied.insert(new.to_owned(), when);
        changed = true;
    }
    if changed {
        save_manifest(&updated)?;
    }
    if let Err(e) = std::fs::rename(&from, &to) {
        if changed {
            let _ = save_manifest(manifest);
        }
        return Err(e.into());
    }
    *manifest = updated;
    // The file name is what counts; the name inside is for people reading the file.
    profile.name = new.to_owned();
    let _ = write_atomic(&to, &profile_to_json(&profile)?);
    Ok(())
}

pub fn duplicate(dir: &Path, name: &str, new_name: &str) -> Result<Profile, ProfileError> {
    let mut copy = load(dir, name)?;
    copy.name = new_name.to_owned();
    create(dir, &copy)?;
    Ok(copy)
}

/// Deletes a profile (the UI confirms first if it is applied). Where it was applied, the game keeps its files and the
/// manifest still knows them; only the name is cleared.
pub fn delete(
    dir: &Path,
    manifest: &mut Manifest,
    name: &str,
    mut save_manifest: impl FnMut(&Manifest) -> io::Result<()>,
) -> Result<(), ProfileError> {
    let path = existing(dir, name)?.ok_or_else(|| ProfileError::NotFound(name.to_owned()))?;
    if list(dir)?.profiles.len() <= 1 {
        return Err(ProfileError::LastProfile);
    }
    let stem = load(dir, name)?.name;
    let mut updated = manifest.clone();
    let mut changed = false;
    for install in &mut updated.installations {
        if install
            .applied_profile
            .as_deref()
            .is_some_and(|p| same_name(p, &stem))
        {
            install.applied_profile = None;
            changed = true;
        }
    }
    let before = updated.last_applied.len();
    updated.last_applied.retain(|k, _| !same_name(k, &stem));
    changed |= updated.last_applied.len() != before;
    if changed {
        save_manifest(&updated)?;
        *manifest = updated;
    }
    std::fs::remove_file(&path)?;
    let mut bak = path.into_os_string();
    bak.push(".bak");
    let _ = std::fs::remove_file(PathBuf::from(bak));
    Ok(())
}

/// `base`, or `base (2)`, `base (3)`… — the first one no profile uses.
pub fn free_name(existing: &[Profile], base: &str) -> String {
    let taken = |n: &str| existing.iter().any(|p| same_name(&p.name, n));
    if !taken(base) {
        return base.to_owned();
    }
    (2..)
        .map(|n| format!("{base} ({n})"))
        .find(|n| !taken(n))
        .expect("a free name")
}

/// Profile names compare like Windows file names: without case, in any script.
fn same_name(a: &str, b: &str) -> bool {
    a.to_lowercase() == b.to_lowercase()
}

/// The stem of `<name>.json`; not of `.json.bak` / `.json.tmp`.
fn profile_stem(file_name: &str) -> Option<&str> {
    let stem = file_name.get(..file_name.len().checked_sub(5)?)?;
    file_name[stem.len()..]
        .eq_ignore_ascii_case(".json")
        .then_some(stem)
        .filter(|s| !s.is_empty())
}

/// The file of the profile named `name`, matched without case.
fn existing(dir: &Path, name: &str) -> io::Result<Option<PathBuf>> {
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(e),
    };
    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name().to_string_lossy().into_owned();
        if profile_stem(&file_name).is_some_and(|s| same_name(s, name)) {
            return Ok(Some(entry.path()));
        }
    }
    Ok(None)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Role {
    Variant,
    AddOn,
}

#[derive(Serialize, Deserialize)]
struct Exported {
    deadloader_profile: u32,
    name: String,
    /// Priority order: the first wins.
    mods: Vec<ExportedMod>,
}

#[derive(Serialize, Deserialize)]
struct ExportedMod {
    name: String,
    source: Source,
    #[serde(default = "yes")]
    enabled: bool,
    files: Vec<ExportedFile>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct ExportedFile {
    role: Role,
    archive: String,
    inner_path: String,
    remote_file_id: Option<u64>,
    version: Option<String>,
    #[serde(with = "crate::hex")]
    sha256: [u8; 32],
}

/// A mod or files of an imported profile this library does not have (D9).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Missing {
    pub name: String,
    pub source: Source,
    /// The whole mod is missing, not just some of its files.
    pub whole_mod: bool,
    pub files: Vec<MissingFile>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MissingFile {
    pub archive: String,
    pub inner_path: String,
    pub remote_file_id: Option<u64>,
    pub version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportedProfile {
    /// With what this library has; not saved yet (the caller picks a free name).
    pub profile: Profile,
    pub missing: Vec<Missing>,
    /// How many mods the file lists (D9: "lists 8 mods; 5 are already in your library").
    pub listed: usize,
}

/// The shareable JSON of `profile` (F14).
pub fn export(profile: &Profile, manifest: &Manifest) -> Result<String, ProfileError> {
    let mut mods = Vec::new();
    for entry in &profile.entries {
        let m = manifest
            .mods
            .iter()
            .find(|m| m.id == entry.mod_id)
            .ok_or_else(|| ModelError::UnknownFile(entry.mod_id.clone()))?;
        let mut files = Vec::new();
        for (role, ids) in [
            (Role::Variant, &entry.variants),
            (Role::AddOn, &entry.add_ons),
        ] {
            for id in ids {
                let f = m
                    .files
                    .iter()
                    .find(|f| &f.id == id)
                    .ok_or_else(|| ModelError::UnknownFile(id.clone()))?;
                files.push(ExportedFile {
                    role,
                    archive: f.archive_name.clone(),
                    inner_path: f.inner_path.clone(),
                    remote_file_id: f.remote_file_id,
                    version: f.version.clone(),
                    sha256: f.sha256,
                });
            }
        }
        let source = match &m.source {
            // Where it was imported from is a path on this machine.
            Source::Local { .. } => Source::Local {
                imported_from: None,
            },
            other => other.clone(),
        };
        mods.push(ExportedMod {
            name: m.name.clone(),
            source,
            enabled: entry.enabled,
            files,
        });
    }
    let exported = Exported {
        deadloader_profile: EXPORT_FORMAT,
        name: profile.name.clone(),
        mods,
    };
    Ok(serde_json::to_string_pretty(&exported).map_err(ModelError::from)?)
}

/// Reads an exported profile and maps it onto `manifest`'s library.
pub fn import(text: &str, manifest: &Manifest) -> Result<ImportedProfile, ProfileError> {
    let value: serde_json::Value =
        serde_json::from_str(text).map_err(|_| ProfileError::NotAProfile)?;
    let format = value
        .get("deadloader_profile")
        .and_then(serde_json::Value::as_u64)
        .ok_or(ProfileError::NotAProfile)?;
    if format > u64::from(EXPORT_FORMAT) {
        return Err(ModelError::NewerSchema {
            found: u32::try_from(format).unwrap_or(u32::MAX),
            supported: EXPORT_FORMAT,
        }
        .into());
    }
    let exported: Exported = serde_json::from_value(value).map_err(ModelError::from)?;
    let mut profile = Profile {
        name: exported.name,
        game_root: None,
        entries: Vec::new(),
    };
    let mut missing = Vec::new();
    for em in &exported.mods {
        let Some(m) = find_mod(manifest, em) else {
            missing.push(Missing {
                name: em.name.clone(),
                source: em.source.clone(),
                whole_mod: true,
                files: em.files.iter().map(missing_file).collect(),
            });
            continue;
        };
        let mut entry = ProfileEntry {
            mod_id: m.id.clone(),
            variants: Vec::new(),
            add_ons: Vec::new(),
            enabled: em.enabled,
        };
        let mut lost = Vec::new();
        for ef in &em.files {
            match find_file(m, ef) {
                Some(f) if ef.role == Role::Variant => entry.variants.push(f.id.clone()),
                Some(f) => entry.add_ons.push(f.id.clone()),
                None => lost.push(missing_file(ef)),
            }
        }
        if !lost.is_empty() {
            missing.push(Missing {
                name: em.name.clone(),
                source: em.source.clone(),
                whole_mod: false,
                files: lost,
            });
        }
        profile.entries.push(entry);
    }
    Ok(ImportedProfile {
        profile,
        missing,
        listed: exported.mods.len(),
    })
}

/// By GameBanana source, else by holding one of its files, else a local mod of the same name.
fn find_mod<'a>(manifest: &'a Manifest, em: &ExportedMod) -> Option<&'a Mod> {
    let by_source = || match &em.source {
        Source::GameBanana { .. } => manifest.mods.iter().find(|m| m.source == em.source),
        Source::Local { .. } => None,
    };
    let by_content = || {
        manifest.mods.iter().find(|m| {
            m.files
                .iter()
                .any(|f| em.files.iter().any(|ef| ef.sha256 == f.sha256))
        })
    };
    let by_name = || {
        manifest
            .mods
            .iter()
            .find(|m| matches!(m.source, Source::Local { .. }) && m.name == em.name)
    };
    by_source().or_else(by_content).or_else(by_name)
}

/// By content, else by GameBanana file id, else by archive and path inside it.
fn find_file<'a>(m: &'a Mod, ef: &ExportedFile) -> Option<&'a ModFile> {
    m.files
        .iter()
        .find(|f| f.sha256 == ef.sha256)
        .or_else(|| {
            ef.remote_file_id
                .and_then(|id| m.files.iter().find(|f| f.remote_file_id == Some(id)))
        })
        .or_else(|| {
            m.files
                .iter()
                .find(|f| f.archive_name == ef.archive && f.inner_path == ef.inner_path)
        })
}

fn yes() -> bool {
    true
}

fn missing_file(ef: &ExportedFile) -> MissingFile {
    MissingFile {
        archive: ef.archive.clone(),
        inner_path: ef.inner_path.clone(),
        remote_file_id: ef.remote_file_id,
        version: ef.version.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::{
        AppliedRecord, InstallationState, Manifest, Mod, ModFile, Profile, ProfileEntry, Section,
        Source,
    };
    use std::io;
    use std::path::PathBuf;

    fn temp_dir(name: &str) -> PathBuf {
        let dir =
            std::env::temp_dir().join(format!("deadloader-profiles-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("temp dir");
        dir
    }

    fn profile(name: &str) -> Profile {
        Profile {
            name: name.into(),
            game_root: None,
            entries: vec![],
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

    fn names(listed: &Listed) -> Vec<&str> {
        listed.profiles.iter().map(|p| p.name.as_str()).collect()
    }

    fn applied_to(manifest: &mut Manifest, root: &str, profile: &str) {
        manifest.installations.push(InstallationState {
            game_root: root.into(),
            applied_profile: Some(profile.into()),
            applied: vec![],
        });
    }

    #[test]
    fn a_missing_folder_lists_no_profiles() {
        let dir = temp_dir("none").join("profiles");
        let listed = list(&dir).expect("listed");
        assert!(listed.profiles.is_empty() && listed.broken.is_empty());
    }

    #[test]
    fn profiles_are_listed_by_name_and_broken_files_reported() {
        let dir = temp_dir("list");
        create(&dir, &profile("zed")).expect("zed");
        create(&dir, &profile("Alpha")).expect("alpha");
        std::fs::write(dir.join("broken.json"), "{ not json").expect("write");
        std::fs::write(dir.join("notes.txt"), "ignored").expect("write");
        let listed = list(&dir).expect("listed");
        assert_eq!(names(&listed), ["Alpha", "zed"]);
        assert_eq!(listed.broken.len(), 1);
        assert_eq!(listed.broken[0].0, "broken.json");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_file_name_is_the_profile_name() {
        // A user who renames the file in Explorer renamed the profile.
        let dir = temp_dir("stem");
        create(&dir, &profile("Old")).expect("created");
        std::fs::rename(dir.join("Old.json"), dir.join("New.json")).expect("renamed");
        assert_eq!(names(&list(&dir).expect("listed")), ["New"]);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn names_are_unique_without_case() {
        let dir = temp_dir("unique");
        create(&dir, &profile("Main")).expect("created");
        assert!(matches!(
            create(&dir, &profile("MAIN")),
            Err(ProfileError::NameTaken(n)) if n == "MAIN"
        ));
        assert!(matches!(
            create(&dir, &profile("bad/name")),
            Err(ProfileError::Model(_))
        ));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn saving_replaces_the_profile_and_keeps_a_backup() {
        let dir = temp_dir("save");
        create(&dir, &profile("Main")).expect("created");
        let mut p = profile("Main");
        p.entries.push(entry("skin", &["skin-1-0"], &[]));
        save(&dir, &p).expect("saved");
        assert_eq!(list(&dir).expect("listed").profiles, [p]);
        assert!(dir.join("Main.json.bak").exists());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_default_profile_exists_from_the_first_start() {
        let dir = temp_dir("default");
        assert_eq!(
            ensure_default(&dir).expect("created"),
            Some(profile(DEFAULT_NAME))
        );
        assert_eq!(ensure_default(&dir).expect("exists"), None);
        assert_eq!(names(&list(&dir).expect("listed")), [DEFAULT_NAME]);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn renaming_follows_the_applied_profile_in_the_manifest() {
        let dir = temp_dir("rename");
        create(&dir, &profile("Main")).expect("created");
        let mut m = Manifest::default();
        applied_to(&mut m, r"D:\Games\Deadlock", "Main");
        applied_to(&mut m, r"E:\Deadlock", "Other");
        let mut saved = 0;
        rename(&dir, &mut m, "Main", "Mine", |_| {
            saved += 1;
            Ok(())
        })
        .expect("renamed");
        assert_eq!(saved, 1);
        assert_eq!(names(&list(&dir).expect("listed")), ["Mine"]);
        assert_eq!(m.installations[0].applied_profile.as_deref(), Some("Mine"));
        assert_eq!(m.installations[1].applied_profile.as_deref(), Some("Other"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn renaming_only_the_case_works() {
        let dir = temp_dir("recase");
        create(&dir, &profile("main")).expect("created");
        rename(&dir, &mut Manifest::default(), "main", "Main", |_| Ok(())).expect("renamed");
        assert_eq!(names(&list(&dir).expect("listed")), ["Main"]);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn renaming_onto_another_profile_or_a_missing_one_is_refused() {
        let dir = temp_dir("renamebad");
        create(&dir, &profile("A")).expect("a");
        create(&dir, &profile("B")).expect("b");
        let mut m = Manifest::default();
        assert!(matches!(
            rename(&dir, &mut m, "A", "b", |_| Ok(())),
            Err(ProfileError::NameTaken(_))
        ));
        assert!(matches!(
            rename(&dir, &mut m, "C", "D", |_| Ok(())),
            Err(ProfileError::NotFound(n)) if n == "C"
        ));
        assert_eq!(names(&list(&dir).expect("listed")), ["A", "B"]);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_rename_whose_manifest_cannot_be_saved_changes_nothing() {
        let dir = temp_dir("renamefail");
        create(&dir, &profile("Main")).expect("created");
        let mut m = Manifest::default();
        applied_to(&mut m, r"D:\Games\Deadlock", "Main");
        let before = m.clone();
        let err = rename(&dir, &mut m, "Main", "Mine", |_| {
            Err(io::Error::other("disk full"))
        })
        .expect_err("failed");
        assert!(matches!(err, ProfileError::Io(_)));
        assert_eq!(m, before);
        assert_eq!(names(&list(&dir).expect("listed")), ["Main"]);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn duplicating_copies_the_entries_under_a_new_name() {
        let dir = temp_dir("dup");
        let mut p = profile("Main");
        p.game_root = Some(r"D:\Games\Deadlock".into());
        p.entries.push(entry("skin", &["skin-1-0"], &["skin-1-1"]));
        create(&dir, &p).expect("created");
        let copy = duplicate(&dir, "Main", "Main copy").expect("duplicated");
        assert_eq!(copy.name, "Main copy");
        assert_eq!(copy.entries, p.entries);
        assert_eq!(copy.game_root, p.game_root);
        assert_eq!(names(&list(&dir).expect("listed")), ["Main", "Main copy"]);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn deleting_the_applied_profile_leaves_its_files_known_but_unnamed() {
        // The UI confirms first (S5). The game folder keeps its files; deadloader still knows them.
        let dir = temp_dir("delete");
        create(&dir, &profile("Main")).expect("created");
        create(&dir, &profile("Other")).expect("created");
        let mut m = Manifest::default();
        applied_to(&mut m, r"D:\Games\Deadlock", "Main");
        m.installations[0].applied.push(AppliedRecord {
            slot: 1,
            file: "x".into(),
            len: 1,
        });
        m.mods.push(Mod {
            id: "x".into(),
            name: "x".into(),
            author: None,
            source: Source::Local {
                imported_from: None,
            },
            files: vec![file("x", "x", Some(1), [1; 32])],
        });
        delete(&dir, &mut m, "main", |_| Ok(())).expect("deleted");
        assert_eq!(names(&list(&dir).expect("listed")), ["Other"]);
        assert_eq!(m.installations[0].applied_profile, None);
        assert_eq!(m.installations[0].applied.len(), 1);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_last_profile_cannot_be_deleted() {
        let dir = temp_dir("last");
        create(&dir, &profile("Main")).expect("created");
        assert!(matches!(
            delete(&dir, &mut Manifest::default(), "Main", |_| Ok(())),
            Err(ProfileError::LastProfile)
        ));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn names_in_other_scripts_compare_without_case_too() {
        let dir = temp_dir("cyrillic");
        create(&dir, &profile("Основной")).expect("created");
        assert!(matches!(
            create(&dir, &profile("ОСНОВНОЙ")),
            Err(ProfileError::NameTaken(_))
        ));
        let mut m = Manifest::default();
        applied_to(&mut m, r"D:\Games\Deadlock", "основной");
        rename(&dir, &mut m, "основной", "Друзья", |_| Ok(())).expect("renamed");
        assert_eq!(
            m.installations[0].applied_profile.as_deref(),
            Some("Друзья")
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_free_name_is_numbered_like_windows_does() {
        let existing = [profile("Main"), profile("main (2)")];
        assert_eq!(free_name(&existing, "Main"), "Main (3)");
        assert_eq!(free_name(&existing, "New"), "New");
    }

    fn file(mod_id: &str, id: &str, remote: Option<u64>, sha: [u8; 32]) -> ModFile {
        ModFile {
            id: id.into(),
            remote_file_id: remote,
            archive_name: format!("{mod_id}.zip"),
            inner_path: format!("{id}.vpk"),
            library_path: format!("library/{mod_id}/1/{id}.vpk"),
            len: 1,
            sha256: sha,
            version: Some("1.2".into()),
            successor: None,
            archive_md5: None,
        }
    }

    fn gb(id: &str, gb_id: u64, files: Vec<ModFile>) -> Mod {
        Mod {
            id: id.into(),
            name: format!("{id} name"),
            author: None,
            source: Source::GameBanana {
                section: Section::Mod,
                mod_id: gb_id,
            },
            files,
        }
    }

    fn local(id: &str, files: Vec<ModFile>) -> Mod {
        Mod {
            id: id.into(),
            name: format!("{id} name"),
            author: None,
            source: Source::Local {
                imported_from: None,
            },
            files,
        }
    }

    /// The library on the machine that exports.
    fn library_a() -> Manifest {
        Manifest {
            mods: vec![
                gb(
                    "skin",
                    100,
                    vec![
                        file("skin", "red", Some(1), [1; 32]),
                        file("skin", "blue", Some(2), [2; 32]),
                        file("skin", "hat", Some(3), [3; 32]),
                    ],
                ),
                local("mine", vec![file("mine", "m", None, [9; 32])]),
                gb("sounds", 200, vec![file("sounds", "s", Some(7), [7; 32])]),
            ],
            ..Manifest::default()
        }
    }

    fn shared_profile() -> Profile {
        Profile {
            name: "Tournament".into(),
            game_root: Some(r"D:\Games\Deadlock".into()),
            entries: vec![
                entry("sounds", &[], &["s"]),
                entry("skin", &["blue"], &["hat"]),
                entry("mine", &[], &["m"]),
            ],
        }
    }

    #[test]
    fn an_export_is_readable_and_holds_no_machine_paths() {
        let text = export(&shared_profile(), &library_a()).expect("exported");
        let json: serde_json::Value = serde_json::from_str(&text).expect("json");
        assert_eq!(json["deadloader_profile"], 1);
        assert_eq!(json["name"], "Tournament");
        assert_eq!(json["mods"][0]["name"], "sounds name");
        assert_eq!(json["mods"][1]["source"]["mod_id"], 100);
        assert_eq!(json["mods"][1]["files"][0]["role"], "variant");
        assert_eq!(json["mods"][1]["files"][0]["version"], "1.2");
        assert_eq!(json["mods"][1]["files"][1]["role"], "add_on");
        assert!(!text.contains("Games"), "no game path: {text}");
        assert!(!text.contains("library/"), "no library path: {text}");
    }

    #[test]
    fn exporting_a_profile_with_unknown_files_is_refused() {
        let mut p = shared_profile();
        p.entries[1].variants = vec!["gone".into()];
        assert!(matches!(
            export(&p, &library_a()),
            Err(ProfileError::Model(_))
        ));
    }

    #[test]
    fn disabled_mods_travel_with_the_export() {
        let mut p = shared_profile();
        p.entries[0].enabled = false;
        let text = export(&p, &library_a()).expect("exported");
        let imported = import(&text, &library_a()).expect("imported");
        assert!(!imported.profile.entries[0].enabled);
        assert!(imported.profile.entries[1].enabled);
    }

    #[test]
    fn an_import_on_the_same_machine_gives_the_same_profile() {
        let text = export(&shared_profile(), &library_a()).expect("exported");
        let imported = import(&text, &library_a()).expect("imported");
        assert!(imported.missing.is_empty(), "{:?}", imported.missing);
        let mut expected = shared_profile();
        expected.game_root = None;
        assert_eq!(imported.profile, expected);
    }

    #[test]
    fn an_import_maps_files_by_content_and_gamebanana_ids_to_another_library() {
        // Another PC: the same mods under other local ids; one file found by its GameBanana file id only
        // (different bytes after a re-pack), the local mod by its content.
        let library_b = Manifest {
            mods: vec![
                gb(
                    "gb-100",
                    100,
                    vec![
                        file("gb-100", "b-blue", Some(2), [2; 32]),
                        file("gb-100", "b-hat", Some(3), [33; 32]),
                    ],
                ),
                local("copied", vec![file("copied", "c", None, [9; 32])]),
                gb("gb-200", 200, vec![file("gb-200", "b-s", Some(7), [7; 32])]),
            ],
            ..Manifest::default()
        };
        let text = export(&shared_profile(), &library_a()).expect("exported");
        let imported = import(&text, &library_b).expect("imported");
        assert!(imported.missing.is_empty(), "{:?}", imported.missing);
        assert_eq!(
            imported.profile.entries,
            [
                entry("gb-200", &[], &["b-s"]),
                entry("gb-100", &["b-blue"], &["b-hat"]),
                entry("copied", &[], &["c"]),
            ]
        );
    }

    #[test]
    fn missing_mods_and_files_are_listed_and_left_out() {
        let library_c = Manifest {
            mods: vec![gb(
                "gb-100",
                100,
                vec![file("gb-100", "b-blue", Some(2), [2; 32])],
            )],
            ..Manifest::default()
        };
        let text = export(&shared_profile(), &library_a()).expect("exported");
        let imported = import(&text, &library_c).expect("imported");
        assert_eq!(
            imported.profile.entries,
            [entry("gb-100", &["b-blue"], &[])]
        );
        let missing: Vec<(&str, bool, usize)> = imported
            .missing
            .iter()
            .map(|m| (m.name.as_str(), m.whole_mod, m.files.len()))
            .collect();
        assert_eq!(
            missing,
            [
                ("sounds name", true, 1),
                ("skin name", false, 1),
                ("mine name", true, 1)
            ]
        );
        assert_eq!(
            imported.missing[0].source,
            Source::GameBanana {
                section: Section::Mod,
                mod_id: 200
            }
        );
    }

    #[test]
    fn files_that_are_not_profiles_are_refused_plainly() {
        for text in ["{}", r#"{"schema": 1, "mods": []}"#, "[]"] {
            assert!(
                matches!(import(text, &library_a()), Err(ProfileError::NotAProfile)),
                "{text}"
            );
        }
        assert!(matches!(
            import("not json", &library_a()),
            Err(ProfileError::NotAProfile)
        ));
        let newer = r#"{"deadloader_profile": 99, "name": "x", "mods": []}"#;
        assert!(matches!(
            import(newer, &library_a()),
            Err(ProfileError::Model(ModelError::NewerSchema {
                found: 99,
                ..
            }))
        ));
    }
}
