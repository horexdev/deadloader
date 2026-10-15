//! Handing an address to Windows: Deadlock through Steam (Play, F4), Steam's check of the game files (D10, decision
//! 132) and web pages in the user's browser (D1, decision 31). Only `https://` and those two Steam links are passed
//! on; anything else is refused before Windows sees it. Also: showing a file in File Explorer and opening a text file
//! in Notepad, for paths the core chose (never one from the UI).

#![allow(unsafe_code)]

use std::io;
use std::path::{Component, Path};
use windows_sys::Win32::UI::Shell::ShellExecuteW;
use windows_sys::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;

/// Steam's link that starts Deadlock (app 1422450).
pub const DEADLOCK_RUN_URL: &str = "steam://rungameid/1422450";

/// Steam's link that checks Deadlock's files and restores the originals (D10).
pub const DEADLOCK_VALIDATE_URL: &str = "steam://validate/1422450";

/// Whether an address may be opened: a web page over https, or Steam's links that start Deadlock or check its files.
pub fn may_open(url: &str) -> bool {
    let url = url.trim();
    (url.len() > "https://".len()
        && url
            .get(..8)
            .is_some_and(|scheme| scheme.eq_ignore_ascii_case("https://"))
        && !url.contains(char::is_whitespace))
        || url == DEADLOCK_RUN_URL
        || url == DEADLOCK_VALIDATE_URL
}

/// Opens `url` with the program Windows uses for it.
pub fn open_url(url: &str) -> io::Result<()> {
    if !may_open(url) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("only https pages and Steam's Deadlock links are opened, not {url}"),
        ));
    }
    execute(url.trim(), None)
}

/// The program [`reveal`] starts.
pub const EXPLORER: &str = "explorer.exe";
/// The program [`open_in_notepad`] starts.
pub const NOTEPAD: &str = "notepad.exe";

/// File Explorer's command line that opens the folder of `path` with `path` selected.
pub fn explorer_arguments(path: &Path) -> io::Result<String> {
    plain(path).map(|p| format!("/select,\"{p}\""))
}

/// Notepad's command line for `path`.
pub fn notepad_arguments(path: &Path) -> io::Result<String> {
    plain(path).map(|p| format!("\"{p}\""))
}

/// Shows `path` selected in its folder in File Explorer ("Open folder").
pub fn reveal(path: &Path) -> io::Result<()> {
    let arguments = explorer_arguments(path)?;
    execute(EXPLORER, Some(&arguments))
}

/// Opens a text file in Notepad ("Open file" for `gameinfo.gi`, which has no program of its own).
pub fn open_in_notepad(path: &Path) -> io::Result<()> {
    let arguments = notepad_arguments(path)?;
    execute(NOTEPAD, Some(&arguments))
}

/// An absolute path without `..` or quotes, as text.
fn plain(path: &Path) -> io::Result<String> {
    let text = path.to_string_lossy();
    let ok = path.is_absolute()
        && !text.contains('"')
        && path
            .components()
            .all(|c| !matches!(c, Component::ParentDir));
    if !ok {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("not an absolute, plain path: {text}"),
        ));
    }
    Ok(text.into_owned())
}

fn execute(target: &str, parameters: Option<&str>) -> io::Result<()> {
    let wide = |s: &str| s.encode_utf16().chain(Some(0)).collect::<Vec<u16>>();
    let (verb, target) = (wide("open"), wide(target));
    let parameters = parameters.map(wide);
    // SAFETY: every string is NUL-terminated UTF-16 that outlives the call; the window and directory are optional and
    // passed as null, and so are the parameters when there are none.
    let result = unsafe {
        ShellExecuteW(
            std::ptr::null_mut(),
            verb.as_ptr(),
            target.as_ptr(),
            parameters.as_ref().map_or(std::ptr::null(), |p| p.as_ptr()),
            std::ptr::null(),
            SW_SHOWNORMAL,
        )
    };
    // ShellExecute reports success with a value above 32.
    if result as isize <= 32 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_web_pages_and_the_deadlock_link_are_opened() {
        assert!(may_open("https://gamebanana.com/mods/655209"));
        assert!(may_open("HTTPS://example.org/a?b=c"));
        assert!(may_open(DEADLOCK_RUN_URL));
        for refused in [
            "",
            "https://",
            "http://example.org",
            "file:///C:/Windows/System32/calc.exe",
            "C:\\Windows\\notepad.exe",
            "steam://run/570",
            "steam://rungameid/1422450 -console",
            "https://example.org/a b",
            "javascript:alert(1)",
            "ms-settings:privacy",
        ] {
            assert!(!may_open(refused), "{refused}");
        }
    }

    #[test]
    fn steams_verify_link_for_deadlock_is_opened_too() {
        assert!(may_open(DEADLOCK_VALIDATE_URL));
        assert_eq!(DEADLOCK_VALIDATE_URL, "steam://validate/1422450");
        for refused in [
            "steam://validate/570",
            "steam://validate/1422450/",
            "steam://validate/1422450 -x",
        ] {
            assert!(!may_open(refused), "{refused}");
        }
    }

    #[test]
    fn explorer_selects_the_item_it_is_given() {
        assert_eq!(
            explorer_arguments(Path::new(
                r"D:\Games\Deadlock\game\citadel\addons\pak01_dir.vpk"
            ))
            .expect("absolute"),
            r#"/select,"D:\Games\Deadlock\game\citadel\addons\pak01_dir.vpk""#
        );
    }

    #[test]
    fn only_absolute_plain_paths_are_shown() {
        for refused in [r"addons\pak01_dir.vpk", r#"D:\a"b"#, "", r"D:\a\..\b"] {
            assert!(explorer_arguments(Path::new(refused)).is_err(), "{refused}");
            assert!(notepad_arguments(Path::new(refused)).is_err(), "{refused}");
        }
    }

    #[test]
    fn notepad_gets_the_file_quoted() {
        assert_eq!(
            notepad_arguments(Path::new(r"D:\Games\Deadlock\game\citadel\gameinfo.gi"))
                .expect("absolute"),
            r#""D:\Games\Deadlock\game\citadel\gameinfo.gi""#
        );
    }

    #[test]
    fn a_refused_address_never_reaches_windows() {
        let err = open_url("file:///C:/Windows/System32/calc.exe").expect_err("refused");
        assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
    }
}
