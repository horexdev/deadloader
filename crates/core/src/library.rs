//! The library model: what deadloader stores (`manifest.json`) and what a profile asks for (`profiles\*.json`).
//!
//! Both are pretty-printed JSON with a schema number, because users may read and edit them (decisions 36, 50, 156):
//! a newer schema is refused rather than misread, every load is validated, and an invalid manifest is never written.
//! Library paths are stored relative to `deadloader-data\` with forward slashes and must stay inside it.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::io;
use std::path::Path;

pub const SCHEMA: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Manifest {
    pub schema: u32,
    pub mods: Vec<Mod>,
    pub installations: Vec<InstallationState>,
}

impl Default for Manifest {
    fn default() -> Self {
        Self {
            schema: SCHEMA,
            mods: Vec::new(),
            installations: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Mod {
    pub id: String,
    pub name: String,
    pub author: Option<String>,
    pub source: Source,
    pub files: Vec<ModFile>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Section {
    Mod,
    Sound,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum Source {
    #[serde(rename = "gamebanana")]
    GameBanana { section: Section, mod_id: u64 },
    #[serde(rename = "local")]
    Local { imported_from: Option<String> },
}

/// One extracted `.vpk` in the library.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModFile {
    pub id: String,
    pub remote_file_id: Option<u64>,
    /// The download or imported archive it came from (display only).
    pub archive_name: String,
    /// Its path inside that archive (display only, never used as a filesystem path).
    pub inner_path: String,
    /// Relative to `deadloader-data\`, forward slashes.
    pub library_path: String,
    pub len: u64,
    #[serde(with = "crate::hex")]
    pub sha256: [u8; 32],
    pub version: Option<String>,
}

/// What we placed in one game installation's addons folder.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallationState {
    pub game_root: String,
    pub applied_profile: Option<String>,
    pub applied: Vec<AppliedRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppliedRecord {
    pub slot: u8,
    pub file: String,
    pub len: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub game_root: Option<String>,
    /// Priority order: the first entry wins.
    pub entries: Vec<ProfileEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileEntry {
    pub mod_id: String,
    /// The chosen file of each variant group of the mod (files that change the same game files; one per group).
    pub variants: Vec<String>,
    /// Add-on files enabled together with the mod.
    pub add_ons: Vec<String>,
    /// A disabled mod keeps its place in the order and its choices, but is not put into the game.
    #[serde(default = "enabled_by_default")]
    pub enabled: bool,
}

fn enabled_by_default() -> bool {
    true
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModelError {
    NewerSchema {
        found: u32,
        supported: u32,
    },
    Json {
        line: usize,
        column: usize,
        message: String,
    },
    DuplicateId(String),
    UnsafePath(String),
    UnknownFile(String),
    InvalidName(String),
    Io(String),
}

impl std::fmt::Display for ModelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NewerSchema { found, supported } => {
                write!(
                    f,
                    "made by a newer deadloader (format {found}; this version reads up to {supported})"
                )
            }
            Self::Json {
                line,
                column,
                message,
            } => write!(
                f,
                "not valid JSON at line {line}, column {column}: {message}"
            ),
            Self::DuplicateId(id) => write!(f, "the id \"{id}\" is used twice"),
            Self::UnsafePath(p) => write!(
                f,
                "the path \"{p}\" points outside deadloader's data folder"
            ),
            Self::UnknownFile(id) => {
                write!(f, "refers to a file that is not in the library: \"{id}\"")
            }
            Self::InvalidName(n) => write!(f, "\"{n}\" can't be used as a profile name"),
            Self::Io(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for ModelError {}

impl From<serde_json::Error> for ModelError {
    fn from(e: serde_json::Error) -> Self {
        Self::Json {
            line: e.line(),
            column: e.column(),
            message: e.to_string(),
        }
    }
}

impl From<io::Error> for ModelError {
    fn from(e: io::Error) -> Self {
        Self::Io(e.to_string())
    }
}

impl Manifest {
    pub fn validate(&self) -> Result<(), ModelError> {
        let mut mod_ids = HashSet::new();
        let mut file_ids = HashSet::new();
        for m in &self.mods {
            if !mod_ids.insert(m.id.as_str()) {
                return Err(ModelError::DuplicateId(m.id.clone()));
            }
            for f in &m.files {
                if !file_ids.insert(f.id.as_str()) {
                    return Err(ModelError::DuplicateId(f.id.clone()));
                }
                if !is_safe_relative(&f.library_path) {
                    return Err(ModelError::UnsafePath(f.library_path.clone()));
                }
            }
        }
        for install in &self.installations {
            if let Some(r) = install
                .applied
                .iter()
                .find(|r| !file_ids.contains(r.file.as_str()))
            {
                return Err(ModelError::UnknownFile(r.file.clone()));
            }
        }
        Ok(())
    }

    /// A library file and the mod it belongs to.
    pub fn file(&self, id: &str) -> Option<(&Mod, &ModFile)> {
        self.mods
            .iter()
            .find_map(|m| m.files.iter().find(|f| f.id == id).map(|f| (m, f)))
    }
}

/// Relative, no `..`, no drive, root, UNC or stream syntax.
fn is_safe_relative(path: &str) -> bool {
    if path.is_empty() || path.starts_with(['/', '\\']) || path.contains(':') {
        return false;
    }
    path.split(['/', '\\'])
        .all(|c| !c.is_empty() && c != "." && c != "..")
}

#[derive(Deserialize)]
struct Head {
    schema: u32,
}

pub fn manifest_from_json(text: &str) -> Result<Manifest, ModelError> {
    let head: Head = serde_json::from_str(text)?;
    if head.schema > SCHEMA {
        return Err(ModelError::NewerSchema {
            found: head.schema,
            supported: SCHEMA,
        });
    }
    let manifest: Manifest = serde_json::from_str(text)?;
    manifest.validate()?;
    Ok(manifest)
}

pub fn to_json(manifest: &Manifest) -> Result<String, ModelError> {
    Ok(serde_json::to_string_pretty(manifest)?)
}

#[derive(Serialize, Deserialize)]
struct ProfileFile {
    schema: u32,
    #[serde(flatten)]
    profile: Profile,
}

pub fn profile_to_json(profile: &Profile) -> Result<String, ModelError> {
    Ok(serde_json::to_string_pretty(&ProfileFile {
        schema: SCHEMA,
        profile: profile.clone(),
    })?)
}

pub fn profile_from_json(text: &str) -> Result<Profile, ModelError> {
    let head: Head = serde_json::from_str(text)?;
    if head.schema > SCHEMA {
        return Err(ModelError::NewerSchema {
            found: head.schema,
            supported: SCHEMA,
        });
    }
    Ok(serde_json::from_str::<ProfileFile>(text)?.profile)
}

/// `<name>.json`, refusing names Windows can't store or treats specially.
pub fn profile_file_name(name: &str) -> Result<String, ModelError> {
    const RESERVED: [&str; 4] = ["CON", "PRN", "AUX", "NUL"];
    let invalid = || ModelError::InvalidName(name.to_owned());
    let trimmed = name.trim();
    if trimmed.is_empty() || name.ends_with(['.', ' ']) || name == ".." {
        return Err(invalid());
    }
    if name
        .chars()
        .any(|c| c.is_control() || "<>:\"/\\|?*".contains(c))
    {
        return Err(invalid());
    }
    let upper = trimmed.to_ascii_uppercase();
    let device = RESERVED.contains(&upper.as_str())
        || ((upper.starts_with("COM") || upper.starts_with("LPT"))
            && upper.len() == 4
            && upper.as_bytes()[3].is_ascii_digit());
    if device {
        return Err(invalid());
    }
    Ok(format!("{name}.json"))
}

/// Writes atomically: temp file, flushed, previous version kept as `.bak`, then renamed into place.
pub fn save_manifest(path: &Path, manifest: &Manifest) -> Result<(), ModelError> {
    manifest.validate()?;
    crate::fs::write_atomic(path, &to_json(manifest)?)?;
    Ok(())
}

pub fn load_manifest(path: &Path) -> Result<Manifest, ModelError> {
    match std::fs::read_to_string(path) {
        Ok(text) => manifest_from_json(&text),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(Manifest::default()),
        Err(e) => Err(e.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn file(id: &str, len: u64) -> ModFile {
        ModFile {
            id: id.to_owned(),
            remote_file_id: Some(1_516_534),
            archive_name: "faunceleste_bubblegum.rar".to_owned(),
            inner_path: "faunceleste.vpk".to_owned(),
            library_path: format!("library/{id}.vpk"),
            len,
            sha256: [7; 32],
            version: Some("1".to_owned()),
        }
    }

    fn sample() -> Manifest {
        Manifest {
            mods: vec![Mod {
                id: "gb-655209".to_owned(),
                name: "Faun Celeste".to_owned(),
                author: Some("someone".to_owned()),
                source: Source::GameBanana {
                    section: Section::Mod,
                    mod_id: 655_209,
                },
                files: vec![file("gb-655209-1", 5_975_061), file("gb-655209-2", 3_000)],
            }],
            installations: vec![InstallationState {
                game_root: r"D:\SteamLibrary\steamapps\common\Deadlock".to_owned(),
                applied_profile: Some("Evening".to_owned()),
                applied: vec![AppliedRecord {
                    slot: 1,
                    file: "gb-655209-1".to_owned(),
                    len: 5_975_061,
                }],
            }],
            ..Manifest::default()
        }
    }

    fn temp_dir(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("deadloader-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("temp dir");
        dir
    }

    #[test]
    fn a_manifest_round_trips_through_json() {
        let m = sample();
        let text = to_json(&m).expect("serialise");
        assert_eq!(manifest_from_json(&text).expect("parse"), m);
    }

    #[test]
    fn the_manifest_is_readable_json_with_a_schema_version() {
        let text = to_json(&sample()).expect("serialise");
        assert!(text.starts_with("{\n  \"schema\": 1,"), "{text}");
        assert!(text.contains("\"kind\": \"gamebanana\""), "{text}");
        assert!(text.contains(&"07".repeat(32)), "sha256 as hex");
    }

    #[test]
    fn a_newer_schema_is_refused_instead_of_misread() {
        let err = manifest_from_json(r#"{"schema": 2, "mods": [], "installations": []}"#)
            .expect_err("newer");
        assert_eq!(
            err,
            ModelError::NewerSchema {
                found: 2,
                supported: SCHEMA
            }
        );
    }

    #[test]
    fn broken_json_is_a_clear_error() {
        let err = manifest_from_json("{ \"schema\": 1, ").expect_err("broken");
        assert!(matches!(err, ModelError::Json { line: 1, .. }), "{err:?}");
    }

    #[test]
    fn duplicate_ids_are_rejected() {
        let mut m = sample();
        m.mods[0].files[1].id = "gb-655209-1".to_owned();
        assert_eq!(
            m.validate(),
            Err(ModelError::DuplicateId("gb-655209-1".to_owned()))
        );
    }

    #[test]
    fn library_paths_must_stay_inside_the_library() {
        for bad in [
            "../outside.vpk",
            r"..\outside.vpk",
            r"C:\Windows\x.vpk",
            "/etc/x.vpk",
            r"\\server\share\x.vpk",
            "library/../../x.vpk",
        ] {
            let mut m = sample();
            m.mods[0].files[0].library_path = bad.to_owned();
            assert_eq!(
                m.validate(),
                Err(ModelError::UnsafePath(bad.to_owned())),
                "{bad}"
            );
        }
    }

    #[test]
    fn applied_records_must_name_known_files() {
        let mut m = sample();
        m.installations[0].applied[0].file = "nope".to_owned();
        assert_eq!(
            m.validate(),
            Err(ModelError::UnknownFile("nope".to_owned()))
        );
    }

    #[test]
    fn finds_files_and_their_mods() {
        let m = sample();
        let (owner, f) = m.file("gb-655209-2").expect("present");
        assert_eq!(owner.id, "gb-655209");
        assert_eq!(f.len, 3_000);
        assert!(m.file("missing").is_none());
    }

    #[test]
    fn a_profile_round_trips_and_keeps_its_order() {
        let p = Profile {
            name: "Evening".to_owned(),
            game_root: Some(r"D:\SteamLibrary\steamapps\common\Deadlock".to_owned()),
            entries: vec![
                ProfileEntry {
                    mod_id: "b".to_owned(),
                    variants: vec!["b-2".to_owned()],
                    add_ons: vec![],
                    enabled: false,
                },
                ProfileEntry {
                    mod_id: "a".to_owned(),
                    variants: vec![],
                    add_ons: vec!["a-9".to_owned()],
                    enabled: true,
                },
            ],
        };
        let text = profile_to_json(&p).expect("serialise");
        assert_eq!(profile_from_json(&text).expect("parse"), p);
    }

    #[test]
    fn a_profile_written_before_the_enabled_flag_has_its_mods_on() {
        let text = r#"{"schema": 1, "name": "Old", "game_root": null,
            "entries": [{"mod_id": "a", "variants": [], "add_ons": ["a-1"]}]}"#;
        assert!(profile_from_json(text).expect("parse").entries[0].enabled);
    }

    #[test]
    fn profile_names_become_safe_file_names() {
        assert_eq!(profile_file_name("Evening").as_deref(), Ok("Evening.json"));
        assert_eq!(
            profile_file_name("Sasha & me").as_deref(),
            Ok("Sasha & me.json")
        );
        for bad in [
            "", "  ", "..", "a/b", r"a\b", "CON", "nul", "a:b", "a*b", "a?b", "a\"b", "a<b", "a|b",
            "dot.",
        ] {
            assert!(profile_file_name(bad).is_err(), "{bad:?}");
        }
    }

    #[test]
    fn saving_is_atomic_and_keeps_the_previous_version() {
        let dir = temp_dir("manifest-save");
        let path = dir.join("manifest.json");
        let mut m = sample();
        save_manifest(&path, &m).expect("first save");
        m.mods[0].name = "Renamed".to_owned();
        save_manifest(&path, &m).expect("second save");
        assert_eq!(load_manifest(&path).expect("load").mods[0].name, "Renamed");
        let backup = std::fs::read_to_string(dir.join("manifest.json.bak")).expect("backup");
        assert_eq!(
            manifest_from_json(&backup).expect("parse backup").mods[0].name,
            "Faun Celeste"
        );
        assert!(
            !dir.join("manifest.json.tmp").exists(),
            "no temp file left behind"
        );
        std::fs::remove_dir_all(&dir).expect("cleanup");
    }

    #[test]
    fn an_invalid_manifest_is_never_saved() {
        let dir = temp_dir("manifest-invalid");
        let path = dir.join("manifest.json");
        let mut m = sample();
        m.mods[0].files[0].library_path = "../x.vpk".to_owned();
        assert!(save_manifest(&path, &m).is_err());
        assert!(!path.exists());
        std::fs::remove_dir_all(&dir).expect("cleanup");
    }

    #[test]
    fn a_missing_manifest_loads_as_empty() {
        let dir = temp_dir("manifest-missing");
        assert_eq!(
            load_manifest(&dir.join("manifest.json")).expect("load"),
            Manifest::default()
        );
        std::fs::remove_dir_all(&dir).expect("cleanup");
    }
}
