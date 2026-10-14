//! Where Steam is installed, from the registry.
//!
//! Steam writes its path for the current user (`HKCU\Software\Valve\Steam\SteamPath`, lower case with forward
//! slashes); the machine-wide key written by the installer is the fallback. Values are read, never written.

use std::io;
use std::path::PathBuf;

const SOURCES: [(&windows_registry::Key, &str, &str); 2] = [
    (
        windows_registry::CURRENT_USER,
        r"Software\Valve\Steam",
        "SteamPath",
    ),
    (
        windows_registry::LOCAL_MACHINE,
        r"SOFTWARE\WOW6432Node\Valve\Steam",
        "InstallPath",
    ),
];

/// The Steam installation folder, or `None` when Steam is not installed.
pub fn steam_root() -> io::Result<Option<PathBuf>> {
    for (hive, key, value) in SOURCES {
        match read_string(hive, key, value) {
            Ok(text) => {
                if let Some(path) = non_empty(&text) {
                    return Ok(Some(path));
                }
            }
            Err(e) if e.kind() == io::ErrorKind::NotFound => {}
            Err(e) => return Err(e),
        }
    }
    Ok(None)
}

fn read_string(hive: &windows_registry::Key, key: &str, value: &str) -> io::Result<String> {
    hive.open(key)
        .and_then(|k| k.get_string(value))
        .map_err(io::Error::from)
}

fn non_empty(raw: &str) -> Option<PathBuf> {
    let path = normalize(raw);
    (!path.as_os_str().is_empty()).then_some(path)
}

fn normalize(raw: &str) -> PathBuf {
    PathBuf::from(raw.trim().replace('/', "\\").trim_end_matches('\\'))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn registry_paths_get_windows_separators() {
        assert_eq!(
            normalize("c:/program files (x86)/steam"),
            PathBuf::from(r"c:\program files (x86)\steam")
        );
    }

    #[test]
    fn already_normal_paths_are_kept() {
        assert_eq!(normalize(r"D:\Steam"), PathBuf::from(r"D:\Steam"));
    }

    #[test]
    fn trailing_separators_and_spaces_are_dropped() {
        assert_eq!(normalize("d:/steam/ "), PathBuf::from(r"d:\steam"));
    }

    #[test]
    fn empty_values_mean_no_path() {
        assert_eq!(non_empty(""), None);
        assert_eq!(non_empty("  "), None);
        assert_eq!(non_empty("c:/steam"), Some(PathBuf::from(r"c:\steam")));
    }

    #[test]
    fn reading_the_real_registry_never_fails_when_steam_is_absent() {
        // On machines without Steam (CI) this is Ok(None); with Steam it is a path.
        let root = steam_root().expect("registry access works");
        if let Some(path) = root {
            assert!(path.is_absolute(), "{path:?}");
        }
    }
}
