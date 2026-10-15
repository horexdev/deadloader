//! Handing an address to Windows: Deadlock through Steam (Play, F4) and web pages in the user's browser (D1,
//! decision 31). Only `https://` and Steam's run link are passed on; anything else is refused before Windows sees it.

#![allow(unsafe_code)]

use std::io;
use windows_sys::Win32::UI::Shell::ShellExecuteW;
use windows_sys::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;

/// Steam's link that starts Deadlock (app 1422450).
pub const DEADLOCK_RUN_URL: &str = "steam://rungameid/1422450";

/// Whether an address may be opened: a web page over https, or Steam's link that starts Deadlock.
pub fn may_open(url: &str) -> bool {
    let url = url.trim();
    (url.len() > "https://".len()
        && url
            .get(..8)
            .is_some_and(|scheme| scheme.eq_ignore_ascii_case("https://"))
        && !url.contains(char::is_whitespace))
        || url == DEADLOCK_RUN_URL
}

/// Opens `url` with the program Windows uses for it.
pub fn open_url(url: &str) -> io::Result<()> {
    if !may_open(url) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("only https pages and Steam's Deadlock link are opened, not {url}"),
        ));
    }
    let wide = |s: &str| s.encode_utf16().chain(Some(0)).collect::<Vec<u16>>();
    let (verb, target) = (wide("open"), wide(url.trim()));
    // SAFETY: both strings are NUL-terminated UTF-16 that outlive the call; the window, parameters and directory are
    // optional and passed as null.
    let result = unsafe {
        ShellExecuteW(
            std::ptr::null_mut(),
            verb.as_ptr(),
            target.as_ptr(),
            std::ptr::null(),
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

/// Shows a folder in File Explorer ("Open folder"). Only an existing folder is passed on, with the `explore` verb, so a
/// file is never run.
pub fn open_folder(path: &std::path::Path) -> io::Result<()> {
    if !std::fs::metadata(path)?.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} is not a folder", path.display()),
        ));
    }
    let wide = |s: &std::ffi::OsStr| {
        use std::os::windows::ffi::OsStrExt;
        s.encode_wide().chain(Some(0)).collect::<Vec<u16>>()
    };
    let (verb, target) = (wide("explore".as_ref()), wide(path.as_os_str()));
    // SAFETY: both strings are NUL-terminated UTF-16 that outlive the call; the window, parameters and directory are
    // optional and passed as null.
    let result = unsafe {
        ShellExecuteW(
            std::ptr::null_mut(),
            verb.as_ptr(),
            target.as_ptr(),
            std::ptr::null(),
            std::ptr::null(),
            SW_SHOWNORMAL,
        )
    };
    if result as isize <= 32 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_folders_are_shown_in_explorer() {
        let file =
            std::env::temp_dir().join(format!("deadloader-shell-{}.txt", std::process::id()));
        std::fs::write(&file, b"x").expect("file");
        assert_eq!(
            open_folder(&file).expect_err("a file").kind(),
            io::ErrorKind::InvalidInput
        );
        let _ = std::fs::remove_file(&file);
        assert_eq!(
            open_folder(std::path::Path::new(r"Z:\no\such\folder\here"))
                .expect_err("missing")
                .kind(),
            io::ErrorKind::NotFound
        );
    }

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
    fn a_refused_address_never_reaches_windows() {
        let err = open_url("file:///C:/Windows/System32/calc.exe").expect_err("refused");
        assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
    }
}
