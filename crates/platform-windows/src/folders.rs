//! The user's Downloads and Desktop folders as Windows knows them (also when moved, e.g. into OneDrive), for the
//! first-run warning of decision 59. Read only.

#![allow(unsafe_code)]

use std::path::PathBuf;
use windows_sys::Win32::System::Com::CoTaskMemFree;
use windows_sys::Win32::UI::Shell::{FOLDERID_Desktop, FOLDERID_Downloads, SHGetKnownFolderPath};
use windows_sys::core::{GUID, PWSTR};

pub fn downloads() -> Option<PathBuf> {
    known(&FOLDERID_Downloads)
}

pub fn desktop() -> Option<PathBuf> {
    known(&FOLDERID_Desktop)
}

fn known(id: &GUID) -> Option<PathBuf> {
    let mut path: PWSTR = std::ptr::null_mut();
    // SAFETY: `id` points at a valid GUID for the call; flags 0 and a null token ask for the current user's folder;
    // `path` is a valid out-pointer. The shell allocates the string with CoTaskMemAlloc, also on failure, and it is
    // freed exactly once below.
    let hr = unsafe { SHGetKnownFolderPath(id, 0, std::ptr::null_mut(), &mut path) };
    let result = if hr >= 0 && !path.is_null() {
        // SAFETY: on success `path` is a NUL-terminated UTF-16 string we own until it is freed below; every unit up to
        // and including the NUL is readable, and the slice ends before it.
        let wide = unsafe {
            let mut len = 0;
            while *path.add(len) != 0 {
                len += 1;
            }
            std::slice::from_raw_parts(path, len)
        };
        Some(PathBuf::from(String::from_utf16_lossy(wide)))
    } else {
        None
    };
    // SAFETY: `path` is null or the shell's CoTaskMemAlloc string; CoTaskMemFree accepts both.
    unsafe { CoTaskMemFree(path as *const core::ffi::c_void) };
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_known_folders_are_absolute_paths() {
        for folder in [downloads(), desktop()].into_iter().flatten() {
            assert!(folder.is_absolute(), "{}", folder.display());
        }
    }
}
