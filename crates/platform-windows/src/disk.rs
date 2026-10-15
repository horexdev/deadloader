//! Free disk space, for the checks made before a download or an extraction starts (E12).

#![allow(unsafe_code)]

use std::io;
use std::os::windows::ffi::OsStrExt;
use std::path::Path;
use windows_sys::Win32::Storage::FileSystem::GetDiskFreeSpaceExW;

/// Bytes the current user may still write on the volume holding `dir` (disk quotas included). `dir` must exist.
pub fn free_space(dir: &Path) -> io::Result<u64> {
    let mut wide: Vec<u16> = dir.as_os_str().encode_wide().collect();
    if wide.contains(&0) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "the path contains a NUL character",
        ));
    }
    wide.push(0);
    let mut available: u64 = 0;
    // SAFETY: `wide` is a NUL-terminated UTF-16 string that outlives the call; `available` is a valid, writable u64;
    // the total and total-free outputs are optional and passed as null.
    let ok = unsafe {
        GetDiskFreeSpaceExW(
            wide.as_ptr(),
            &mut available,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        )
    };
    if ok == 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(available)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_temp_folder_has_some_free_space() {
        let free = free_space(&std::env::temp_dir()).expect("free space");
        assert!(free > 0);
    }

    #[test]
    fn a_missing_folder_is_an_error() {
        let missing = std::env::temp_dir()
            .join("deadloader-no-such-folder-7c1f")
            .join("x");
        assert!(free_space(&missing).is_err());
    }

    #[test]
    fn a_nul_in_the_path_is_refused() {
        let err = free_space(Path::new("C:\\a\0b")).expect_err("nul");
        assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
    }
}
