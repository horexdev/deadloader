//! The Windows recycle bin, for "delete mods to the recycle bin" (decision 42).
//!
//! Uses the shell's `SHFileOperationW` without any of its own dialogs, except one: a drive without a recycle bin
//! (USB sticks, network shares — common for a portable app) would make the shell delete the file for good, so it is
//! asked to warn first, and the user decides.

#![allow(unsafe_code)]

use std::io;
use std::os::windows::ffi::OsStrExt;
use std::path::Path;
use windows_sys::Win32::UI::Shell::{
    FO_DELETE, FOF_ALLOWUNDO, FOF_NOCONFIRMATION, FOF_NOERRORUI, FOF_SILENT, FOF_WANTNUKEWARNING,
    SHFILEOPSTRUCTW, SHFileOperationW,
};

/// Moves one file to the recycle bin. `Interrupted` if the user declined a permanent delete on a drive without one.
pub fn move_to_recycle_bin(path: &Path) -> io::Result<()> {
    let from = shell_path_list(path)?;
    // The shell reports a missing file with its own codes; say it plainly.
    std::fs::symlink_metadata(path)?;
    let mut op = SHFILEOPSTRUCTW {
        wFunc: FO_DELETE,
        pFrom: from.as_ptr(),
        fFlags: (FOF_ALLOWUNDO
            | FOF_NOCONFIRMATION
            | FOF_SILENT
            | FOF_NOERRORUI
            | FOF_WANTNUKEWARNING) as u16,
        ..Default::default()
    };
    // SAFETY: `op` is a fully initialised SHFILEOPSTRUCTW; `pFrom` points at `from`, a double-NUL-terminated UTF-16
    // list that outlives the call; `pTo`, `hwnd`, `hNameMappings` and `lpszProgressTitle` are null, which FO_DELETE
    // without FOF_SIMPLEPROGRESS or name mappings allows.
    let code = unsafe { SHFileOperationW(&mut op) };
    if code != 0 {
        return Err(io::Error::other(format!(
            "Windows could not move {} to the recycle bin (shell error {code:#x})",
            path.display()
        )));
    }
    if op.fAnyOperationsAborted != 0 {
        return Err(io::Error::new(
            io::ErrorKind::Interrupted,
            "the permanent delete was declined",
        ));
    }
    Ok(())
}

/// The path as the shell wants it: absolute, UTF-16, followed by two NULs (it takes a list).
fn shell_path_list(path: &Path) -> io::Result<Vec<u16>> {
    let text = path.as_os_str().to_string_lossy();
    if text.contains(['*', '?']) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "the path contains wildcard characters",
        ));
    }
    let absolute = std::path::absolute(path)?;
    let mut list: Vec<u16> = absolute.as_os_str().encode_wide().collect();
    if list.contains(&0) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "the path contains a NUL character",
        ));
    }
    list.extend([0, 0]);
    Ok(list)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path, PathBuf};

    fn temp_dir(name: &str) -> PathBuf {
        let dir =
            std::env::temp_dir().join(format!("deadloader-recycle-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("temp dir");
        dir
    }

    #[test]
    fn the_shell_gets_one_absolute_path_ending_in_two_nuls() {
        let list = shell_path_list(Path::new(r"C:\data\library\skin\1\0.vpk")).expect("ok");
        let text = String::from_utf16(&list).expect("utf-16");
        assert_eq!(text, "C:\\data\\library\\skin\\1\\0.vpk\0\0");
    }

    #[test]
    fn relative_paths_are_made_absolute() {
        // The shell resolves relative paths against a process-wide current directory; never hand it one.
        let list = shell_path_list(Path::new(r"library\0.vpk")).expect("ok");
        let text = String::from_utf16(&list).expect("utf-16");
        assert!(
            Path::new(text.trim_end_matches('\0')).is_absolute(),
            "{text}"
        );
    }

    #[test]
    fn wildcards_and_verbatim_paths_are_refused() {
        // The shell expands `*` and `?`, which would delete more than asked, and does not understand `\\?\`.
        for bad in [r"C:\data\*.vpk", r"C:\data\?.vpk", r"\\?\C:\data\0.vpk"] {
            let err = shell_path_list(Path::new(bad)).expect_err(bad);
            assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput, "{bad}");
        }
    }

    #[test]
    fn a_missing_file_is_an_error() {
        let dir = temp_dir("missing");
        let err = move_to_recycle_bin(&dir.join("nothing.vpk")).expect_err("missing");
        assert_eq!(err.kind(), std::io::ErrorKind::NotFound, "{err}");
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// Puts a small file into this machine's recycle bin, so it is not run by default: `cargo test -- --ignored`.
    #[test]
    #[ignore = "writes to the user's recycle bin"]
    fn a_read_only_file_goes_to_the_recycle_bin_without_a_prompt() {
        let dir = temp_dir("real");
        let path = dir.join("deadloader-recycle-test.vpk");
        std::fs::write(&path, b"deadloader recycle bin test").expect("write");
        let mut perm = std::fs::metadata(&path).expect("meta").permissions();
        perm.set_readonly(true);
        std::fs::set_permissions(&path, perm).expect("read-only");
        move_to_recycle_bin(&path).expect("recycled");
        assert!(!path.exists());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
