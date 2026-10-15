//! What the first-run setup step (S1 step 6) and Settings → Game report about this PC: the state of a game folder's
//! `gameinfo.gi`, whether deadloader sits somewhere temporary (decision 59), whether the library and the game share a
//! drive (decision 124), and which folder the user meant when they picked one inside a Deadlock installation.
//!
//! Only reads; nothing here changes a file.

use crate::fs::ReadText;
use crate::gamedir::gameinfo_path;
use crate::gameinfo::{self, State};
use serde::Serialize;
use std::io;
use std::path::{Component, Path, PathBuf, Prefix};

/// `gameinfo.gi` as the setup step and the installations list show it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GameinfoState {
    /// Our line is there: mods load.
    Ours,
    /// As Steam installed it: our line is added when a profile is applied there.
    Vanilla,
    /// Changed by another program (often Deadlock Mod Manager): left alone (19, D10).
    Foreign,
    /// No `game\citadel\gameinfo.gi`: not a Deadlock folder, or the game was moved.
    Missing,
    /// The file is there but can't be read.
    Unreadable,
}

pub fn gameinfo_state(files: &impl ReadText, game_root: &Path) -> GameinfoState {
    match files.read_text(&gameinfo_path(game_root)) {
        Ok(text) => match gameinfo::inspect(&text) {
            State::Ours => GameinfoState::Ours,
            State::Vanilla => GameinfoState::Vanilla,
            State::Foreign(_) => GameinfoState::Foreign,
        },
        Err(e) if e.kind() == io::ErrorKind::NotFound => GameinfoState::Missing,
        Err(_) => GameinfoState::Unreadable,
    }
}

/// Where deadloader.exe is, for the warning of decision 59: data next to the exe in Downloads or on the Desktop is lost
/// the next time the user tidies up.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExeLocation {
    Downloads,
    Desktop,
    /// Anywhere else: nothing to warn about.
    Permanent,
}

/// `exe_dir` inside the user's Downloads or Desktop folder (as Windows reports them; `None` when unknown).
pub fn exe_location(
    exe_dir: &Path,
    downloads: Option<&Path>,
    desktop: Option<&Path>,
) -> ExeLocation {
    if downloads.is_some_and(|d| is_inside(exe_dir, d)) {
        ExeLocation::Downloads
    } else if desktop.is_some_and(|d| is_inside(exe_dir, d)) {
        ExeLocation::Desktop
    } else {
        ExeLocation::Permanent
    }
}

/// `path` is `folder` or inside it, comparing like Windows does (no case, either slash, no trailing separator).
pub fn is_inside(path: &Path, folder: &Path) -> bool {
    let parts = |p: &Path| -> Vec<String> {
        p.components()
            .filter(|c| !matches!(c, Component::CurDir))
            .map(|c| c.as_os_str().to_string_lossy().to_lowercase())
            .collect()
    };
    let (path, folder) = (parts(path), parts(folder));
    !folder.is_empty() && path.len() >= folder.len() && path[..folder.len()] == folder[..]
}

/// The volume a path is on: `D:` or `\\server\share`, upper-cased; `None` for a relative path.
pub fn volume(path: &Path) -> Option<String> {
    match path.components().next()? {
        Component::Prefix(prefix) => Some(match prefix.kind() {
            Prefix::Disk(letter) | Prefix::VerbatimDisk(letter) => {
                format!("{}:", char::from(letter).to_ascii_uppercase())
            }
            Prefix::UNC(server, share) | Prefix::VerbatimUNC(server, share) => format!(
                r"\\{}\{}",
                server.to_string_lossy().to_uppercase(),
                share.to_string_lossy().to_uppercase()
            ),
            _ => prefix.as_os_str().to_string_lossy().to_uppercase(),
        }),
        _ => None,
    }
}

/// Hard links work only within one volume (decision 124): otherwise mods are copied.
pub fn same_volume(a: &Path, b: &Path) -> bool {
    matches!((volume(a), volume(b)), (Some(x), Some(y)) if x == y)
}

/// The Deadlock installation a chosen folder belongs to: the folder itself, or the installation above it when the user
/// picked `game` or `game\citadel` (or something inside them). `None` when no `game\citadel\gameinfo.gi` is found.
pub fn game_root_of(files: &impl ReadText, chosen: &Path) -> Option<PathBuf> {
    let mut candidate = Some(chosen);
    // The installation is at most a few levels above anything a user would pick inside it.
    for _ in 0..4 {
        let dir = candidate?;
        if !matches!(gameinfo_state(files, dir), GameinfoState::Missing) {
            return Some(dir.to_path_buf());
        }
        candidate = dir.parent();
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fs::fake::MemFs;

    const ROOT: &str = r"D:\SteamLibrary\steamapps\common\Deadlock";
    const VANILLA: &str = "\"GameInfo\"\r\n{\r\n\tFileSystem\r\n\t{\r\n\t\tSearchPaths\r\n\t\t{\r\n\t\t\tGame\t\t\t\tcitadel\r\n\t\t\tGame\t\t\t\tcore\r\n\t\t}\r\n\t}\r\n}\r\n";

    fn game(text: &str) -> MemFs {
        MemFs::default().with(
            &format!(r"{ROOT}\game\citadel\gameinfo.gi"),
            text.as_bytes(),
        )
    }

    #[test]
    fn tells_the_state_of_gameinfo() {
        assert_eq!(
            gameinfo_state(&game(VANILLA), Path::new(ROOT)),
            GameinfoState::Vanilla
        );
        let ours = gameinfo::add_addons_path(VANILLA).expect("vanilla");
        assert_eq!(
            gameinfo_state(&game(&ours), Path::new(ROOT)),
            GameinfoState::Ours
        );
        let foreign = VANILLA.replace(
            "\t\t\tGame\t\t\t\tcore\r\n",
            "\t\t\tGame\t\t\t\tcore\r\n\t\t\tMod\tcitadel/addons\r\n",
        );
        assert_eq!(
            gameinfo_state(&game(&foreign), Path::new(ROOT)),
            GameinfoState::Foreign
        );
        assert_eq!(
            gameinfo_state(&MemFs::default(), Path::new(ROOT)),
            GameinfoState::Missing
        );
    }

    #[test]
    fn warns_about_downloads_and_the_desktop() {
        let downloads = Path::new(r"C:\Users\Alex\Downloads");
        let desktop = Path::new(r"C:\Users\Alex\OneDrive\Desktop");
        let at = |dir: &str| exe_location(Path::new(dir), Some(downloads), Some(desktop));
        assert_eq!(
            at(r"C:\Users\Alex\Downloads\deadloader"),
            ExeLocation::Downloads
        );
        assert_eq!(at(r"c:\users\alex\downloads"), ExeLocation::Downloads);
        assert_eq!(
            at(r"C:\Users\Alex\OneDrive\Desktop\tools\deadloader"),
            ExeLocation::Desktop
        );
        assert_eq!(at(r"D:\Games\deadloader"), ExeLocation::Permanent);
        assert_eq!(
            at(r"C:\Users\Alex\Downloads2\deadloader"),
            ExeLocation::Permanent,
            "a folder that only starts with the same letters"
        );
        assert_eq!(
            exe_location(Path::new(r"C:\x"), None, None),
            ExeLocation::Permanent
        );
    }

    #[test]
    fn compares_volumes_like_windows() {
        assert!(same_volume(
            Path::new(r"d:\Games\deadloader\deadloader-data"),
            Path::new(r"D:\SteamLibrary\steamapps\common\Deadlock")
        ));
        assert!(!same_volume(
            Path::new(r"C:\Users\Alex\deadloader"),
            Path::new(r"E:\SteamLibrary\steamapps\common\Deadlock")
        ));
        assert!(same_volume(
            Path::new(r"\\nas\games\deadloader"),
            Path::new(r"\\NAS\Games\Deadlock")
        ));
        assert!(!same_volume(Path::new("relative"), Path::new("relative")));
        assert_eq!(volume(Path::new(r"e:\x")).as_deref(), Some("E:"));
    }

    #[test]
    fn finds_the_installation_above_a_folder_inside_it() {
        let fs = game(VANILLA);
        for chosen in [
            ROOT.to_owned(),
            format!(r"{ROOT}\game"),
            format!(r"{ROOT}\game\citadel"),
            format!(r"{ROOT}\game\citadel\addons"),
        ] {
            assert_eq!(
                game_root_of(&fs, Path::new(&chosen)),
                Some(PathBuf::from(ROOT)),
                "{chosen}"
            );
        }
        assert_eq!(
            game_root_of(&fs, Path::new(r"D:\SteamLibrary\steamapps")),
            None
        );
    }
}
