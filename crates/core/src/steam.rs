//! Finding Deadlock installations from Steam's own library files.
//!
//! Libraries come from `steamapps\libraryfolders.vdf`; an installation is a library whose
//! `steamapps\appmanifest_1422450.acf` exists and says the files are complete. The `apps` map in the library file is
//! not trusted: Steam updates it lazily. Only the fields listed in [`AppManifest`] are read — never `LastOwner` or
//! other account data.

use crate::fs::ReadText;
use crate::vdf;
use std::fmt;
use std::io;
use std::path::{Path, PathBuf};

pub const DEADLOCK_APP_ID: u32 = 1_422_450;

const FULLY_INSTALLED: u32 = 4;
/// Update running, update started, uninstalling: the files are changing right now.
const IN_FLUX: u32 = 256 | 1024 | 2048;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Library {
    pub path: PathBuf,
    pub app_ids: Vec<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppManifest {
    pub app_id: u32,
    pub install_dir: String,
    /// Changes with every game update.
    pub build_id: Option<u64>,
    /// Installed and not being updated or removed right now.
    pub fully_installed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Installation {
    pub library: PathBuf,
    pub game_root: PathBuf,
    pub build_id: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SteamError {
    Format(vdf::Error),
    Missing(&'static str),
    Invalid(&'static str),
}

impl fmt::Display for SteamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Format(e) => write!(f, "not a valid Steam file: {e}"),
            Self::Missing(key) => write!(f, "the entry \"{key}\" is missing"),
            Self::Invalid(key) => write!(f, "the entry \"{key}\" has an invalid value"),
        }
    }
}

impl std::error::Error for SteamError {}

impl From<vdf::Error> for SteamError {
    fn from(e: vdf::Error) -> Self {
        Self::Format(e)
    }
}

/// A Steam file that exists but could not be used; discovery goes on without it.
#[derive(Debug)]
pub struct Problem {
    pub path: PathBuf,
    pub error: ProblemKind,
}

#[derive(Debug)]
pub enum ProblemKind {
    Io(io::Error),
    Steam(SteamError),
}

#[derive(Debug, Default)]
pub struct Discovery {
    pub installations: Vec<Installation>,
    pub problems: Vec<Problem>,
}

pub fn parse_library_folders(text: &str) -> Result<Vec<Library>, SteamError> {
    let doc = vdf::parse(text)?;
    let root = doc
        .section("libraryfolders")
        .ok_or(SteamError::Missing("libraryfolders"))?;
    let mut libraries = Vec::new();
    for (_, entry) in root.entries() {
        let vdf::Value::Section(lib) = entry else {
            continue;
        };
        let Some(path) = lib.value("path") else {
            continue;
        };
        let app_ids = lib
            .section("apps")
            .map(|apps| {
                apps.entries()
                    .iter()
                    .filter_map(|(id, _)| id.parse().ok())
                    .collect()
            })
            .unwrap_or_default();
        libraries.push(Library {
            path: PathBuf::from(path),
            app_ids,
        });
    }
    Ok(libraries)
}

pub fn parse_app_manifest(text: &str) -> Result<AppManifest, SteamError> {
    let doc = vdf::parse(text)?;
    let state = doc
        .section("AppState")
        .ok_or(SteamError::Missing("AppState"))?;
    let app_id = state
        .value("appid")
        .ok_or(SteamError::Missing("appid"))?
        .parse()
        .map_err(|_| SteamError::Invalid("appid"))?;
    let install_dir = state
        .value("installdir")
        .ok_or(SteamError::Missing("installdir"))?
        .to_owned();
    let build_id = state.value("buildid").and_then(|b| b.parse().ok());
    let flags: u32 = state
        .value("StateFlags")
        .and_then(|f| f.parse().ok())
        .unwrap_or(0);
    Ok(AppManifest {
        app_id,
        install_dir,
        build_id,
        fully_installed: flags & FULLY_INSTALLED != 0 && flags & IN_FLUX == 0,
    })
}

/// Looks for Deadlock in every library known to the Steam installation at `steam_root`.
pub fn find_installations(steam_root: &Path, files: &impl ReadText) -> Discovery {
    let mut found = Discovery::default();
    let mut libraries = Vec::new();

    let library_file = steam_root.join("steamapps").join("libraryfolders.vdf");
    match files.read_text(&library_file) {
        Ok(text) => match parse_library_folders(&text) {
            Ok(libs) => libraries.extend(libs.into_iter().map(|l| l.path)),
            Err(e) => found.problems.push(Problem {
                path: library_file,
                error: ProblemKind::Steam(e),
            }),
        },
        Err(e) if e.kind() == io::ErrorKind::NotFound => {}
        Err(e) => found.problems.push(Problem {
            path: library_file,
            error: ProblemKind::Io(e),
        }),
    }
    if !libraries.iter().any(|l| same_path(l, steam_root)) {
        libraries.push(steam_root.to_path_buf());
    }

    let manifest_name = format!("appmanifest_{DEADLOCK_APP_ID}.acf");
    for library in libraries {
        let manifest_file = library.join("steamapps").join(&manifest_name);
        let text = match files.read_text(&manifest_file) {
            Ok(text) => text,
            Err(e) if e.kind() == io::ErrorKind::NotFound => continue,
            Err(e) => {
                found.problems.push(Problem {
                    path: manifest_file,
                    error: ProblemKind::Io(e),
                });
                continue;
            }
        };
        match parse_app_manifest(&text) {
            Ok(m) if m.app_id == DEADLOCK_APP_ID && m.fully_installed => {
                found.installations.push(Installation {
                    game_root: library
                        .join("steamapps")
                        .join("common")
                        .join(&m.install_dir),
                    library,
                    build_id: m.build_id,
                });
            }
            Ok(_) => {}
            Err(e) => found.problems.push(Problem {
                path: manifest_file,
                error: ProblemKind::Steam(e),
            }),
        }
    }
    found
}

/// Windows path equality: case-insensitive, either slash, trailing separators ignored.
fn same_path(a: &Path, b: &Path) -> bool {
    let norm = |p: &Path| {
        p.to_string_lossy()
            .replace('/', "\\")
            .trim_end_matches('\\')
            .to_ascii_lowercase()
    };
    norm(a) == norm(b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::io;
    use std::path::{Path, PathBuf};

    /// In-memory files keyed by path; anything else is "not found".
    #[derive(Debug, Default)]
    struct FakeFiles(HashMap<PathBuf, String>);

    impl FakeFiles {
        fn with(mut self, path: &str, text: &str) -> Self {
            self.0.insert(PathBuf::from(path), text.to_owned());
            self
        }
    }

    impl ReadText for FakeFiles {
        fn read_text(&self, path: &Path) -> io::Result<String> {
            self.0
                .get(path)
                .cloned()
                .ok_or_else(|| io::Error::from(io::ErrorKind::NotFound))
        }
    }

    const LIBRARIES: &str = r#""libraryfolders"
{
	"0"
	{
		"path"		"C:\\Program Files (x86)\\Steam"
		"apps"		{ "228980"		"348323003" }
	}
	"1"
	{
		"path"		"D:\\SteamLibrary"
		"apps"		{ "1422450"		"39077793133" }
	}
}"#;

    fn manifest(state_flags: &str, build_id: &str) -> String {
        format!(
            r#""AppState"
{{
	"appid"		"1422450"
	"name"		"Deadlock"
	"StateFlags"		"{state_flags}"
	"installdir"		"Deadlock"
	"buildid"		"{build_id}"
	"LastOwner"		"76561190000000000"
}}"#
        )
    }

    #[test]
    fn lists_libraries_in_file_order() {
        let libs = parse_library_folders(LIBRARIES).expect("valid file");
        let paths: Vec<_> = libs.iter().map(|l| l.path.clone()).collect();
        assert_eq!(
            paths,
            [
                PathBuf::from(r"C:\Program Files (x86)\Steam"),
                PathBuf::from(r"D:\SteamLibrary")
            ]
        );
        assert_eq!(libs[1].app_ids, [DEADLOCK_APP_ID]);
    }

    #[test]
    fn library_file_without_root_section_is_an_error() {
        let err = parse_library_folders(r#""other" { }"#).expect_err("wrong root");
        assert_eq!(err, SteamError::Missing("libraryfolders"));
    }

    #[test]
    fn reads_the_fields_we_need_from_an_app_manifest() {
        let m = parse_app_manifest(&manifest("4", "25939744")).expect("valid manifest");
        assert_eq!(m.app_id, DEADLOCK_APP_ID);
        assert_eq!(m.install_dir, "Deadlock");
        assert_eq!(m.build_id, Some(25_939_744));
        assert!(m.fully_installed);
    }

    #[test]
    fn state_flags_decide_whether_the_files_are_usable() {
        let usable = |flags: &str| {
            parse_app_manifest(&manifest(flags, "1"))
                .expect("valid manifest")
                .fully_installed
        };
        assert!(usable("4"), "fully installed");
        assert!(
            usable("6"),
            "installed, an update is queued but not running"
        );
        assert!(!usable("2"), "update required, not installed");
        assert!(
            !usable("1028"),
            "installed, update started: files are changing"
        );
        assert!(!usable("260"), "installed, update running");
        assert!(!usable("2052"), "installed, being uninstalled");
        assert!(!usable("garbage"), "unreadable flags are not trusted");
    }

    #[test]
    fn manifest_without_install_dir_is_an_error() {
        let err =
            parse_app_manifest(r#""AppState" { "appid" "1422450" }"#).expect_err("no installdir");
        assert_eq!(err, SteamError::Missing("installdir"));
    }

    #[test]
    fn manifest_with_a_non_numeric_app_id_is_an_error() {
        let err = parse_app_manifest(r#""AppState" { "appid" "x" "installdir" "D" }"#)
            .expect_err("bad appid");
        assert_eq!(err, SteamError::Invalid("appid"));
    }

    #[test]
    fn finds_the_game_in_a_secondary_library() {
        let files = FakeFiles::default()
            .with(r"C:\Steam\steamapps\libraryfolders.vdf", LIBRARIES)
            .with(
                r"D:\SteamLibrary\steamapps\appmanifest_1422450.acf",
                &manifest("4", "42"),
            );
        let found = find_installations(Path::new(r"C:\Steam"), &files);
        assert_eq!(
            found.installations,
            [Installation {
                library: PathBuf::from(r"D:\SteamLibrary"),
                game_root: PathBuf::from(r"D:\SteamLibrary\steamapps\common\Deadlock"),
                build_id: Some(42),
            }]
        );
        assert!(found.problems.is_empty());
    }

    #[test]
    fn trusts_manifests_not_the_apps_map() {
        // The apps map lists the game in D:, but only C: really has its manifest.
        let files = FakeFiles::default()
            .with(r"C:\Steam\steamapps\libraryfolders.vdf", LIBRARIES)
            .with(
                r"C:\Program Files (x86)\Steam\steamapps\appmanifest_1422450.acf",
                &manifest("4", "7"),
            );
        let found = find_installations(Path::new(r"C:\Steam"), &files);
        assert_eq!(found.installations.len(), 1);
        assert_eq!(
            found.installations[0].library,
            PathBuf::from(r"C:\Program Files (x86)\Steam")
        );
    }

    #[test]
    fn skips_installations_that_are_not_fully_installed() {
        let files = FakeFiles::default()
            .with(r"C:\Steam\steamapps\libraryfolders.vdf", LIBRARIES)
            .with(
                r"D:\SteamLibrary\steamapps\appmanifest_1422450.acf",
                &manifest("1026", "1"),
            );
        let found = find_installations(Path::new(r"C:\Steam"), &files);
        assert!(found.installations.is_empty());
    }

    #[test]
    fn falls_back_to_the_steam_root_when_the_library_file_is_missing() {
        let files = FakeFiles::default().with(
            r"C:\Steam\steamapps\appmanifest_1422450.acf",
            &manifest("4", "9"),
        );
        let found = find_installations(Path::new(r"C:\Steam"), &files);
        assert_eq!(found.installations.len(), 1);
        assert_eq!(found.installations[0].library, PathBuf::from(r"C:\Steam"));
    }

    #[test]
    fn does_not_list_the_steam_root_twice() {
        // Registry paths come in lower case with forward slashes; the library file has the canonical spelling.
        let libs = r#""libraryfolders" { "0" { "path" "C:\\Steam" } }"#;
        let files = FakeFiles::default()
            .with(r"c:/steam\steamapps\libraryfolders.vdf", libs)
            .with(
                r"C:\Steam\steamapps\appmanifest_1422450.acf",
                &manifest("4", "1"),
            );
        let found = find_installations(Path::new("c:/steam"), &files);
        assert_eq!(found.installations.len(), 1);
    }

    #[test]
    fn reports_a_damaged_manifest_and_keeps_going() {
        let files = FakeFiles::default()
            .with(r"C:\Steam\steamapps\libraryfolders.vdf", LIBRARIES)
            .with(
                r"C:\Program Files (x86)\Steam\steamapps\appmanifest_1422450.acf",
                "\"AppState\" {",
            )
            .with(
                r"D:\SteamLibrary\steamapps\appmanifest_1422450.acf",
                &manifest("4", "3"),
            );
        let found = find_installations(Path::new(r"C:\Steam"), &files);
        assert_eq!(found.installations.len(), 1);
        assert_eq!(found.problems.len(), 1);
        assert_eq!(
            found.problems[0].path,
            PathBuf::from(r"C:\Program Files (x86)\Steam\steamapps\appmanifest_1422450.acf")
        );
    }

    #[test]
    fn reports_a_damaged_library_file() {
        let files = FakeFiles::default().with(r"C:\Steam\steamapps\libraryfolders.vdf", "{");
        let found = find_installations(Path::new(r"C:\Steam"), &files);
        assert!(found.installations.is_empty());
        assert_eq!(
            found.problems[0].path,
            PathBuf::from(r"C:\Steam\steamapps\libraryfolders.vdf")
        );
    }
}
