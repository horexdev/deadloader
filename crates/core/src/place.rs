//! Putting a library file into the game folder: a hard link when the filesystem allows it, otherwise a verified copy.
//!
//! Whether to copy is decided per operation from the error the link attempt returns — not by guessing volumes — so
//! mount points, junctions and `subst` drives behave correctly (ADR 0004).

use crate::fs::FileOps;
use std::io;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Placement {
    /// No extra disk space used.
    Linked,
    Copied {
        bytes: u64,
    },
}

#[derive(Debug)]
pub enum PlaceError {
    /// The copy differs from what the manifest recorded; it has been removed again.
    HashMismatch {
        expected: [u8; 32],
        actual: [u8; 32],
    },
    Io(io::Error),
}

impl std::fmt::Display for PlaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HashMismatch { .. } => {
                f.write_str("the copied file does not match the library's checksum")
            }
            Self::Io(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for PlaceError {}

/// Creates `dest` from `source`: hard link if possible, else a copy checked against `expected_sha256`.
pub fn place(
    fs: &impl FileOps,
    source: &Path,
    dest: &Path,
    expected_sha256: &[u8; 32],
) -> Result<Placement, PlaceError> {
    match fs.hard_link(source, dest) {
        Ok(()) => return Ok(Placement::Linked),
        Err(e) if needs_copy(&e) => {}
        Err(e) => return Err(PlaceError::Io(e)),
    }
    let (bytes, actual) = fs.copy_verified(source, dest).map_err(PlaceError::Io)?;
    if &actual != expected_sha256 {
        // Best effort: a failed cleanup leaves a temp name the intent journal removes on the next start.
        let _ = fs.remove_file(dest);
        return Err(PlaceError::HashMismatch {
            expected: *expected_sha256,
            actual,
        });
    }
    Ok(Placement::Copied { bytes })
}

/// Link failures that mean "this filesystem/volume pair can't hard-link", not "something is wrong".
pub fn needs_copy(err: &io::Error) -> bool {
    if matches!(
        err.kind(),
        io::ErrorKind::CrossesDevices | io::ErrorKind::Unsupported
    ) {
        return true;
    }
    // ERROR_INVALID_FUNCTION (FAT32/exFAT), ERROR_NOT_SAME_DEVICE, ERROR_NOT_SUPPORTED (some network shares).
    cfg!(windows) && matches!(err.raw_os_error(), Some(1 | 17 | 50))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fs::fake::MemFs;
    use std::path::Path;

    fn sha(bytes: &[u8]) -> [u8; 32] {
        crate::fs::sha256(bytes)
    }

    #[test]
    fn same_volume_gets_a_hard_link() {
        let fs = MemFs::default()
            .volume("D:", true)
            .with(r"D:\lib\a.vpk", b"mod data");
        let placed = place(
            &fs,
            Path::new(r"D:\lib\a.vpk"),
            Path::new(r"D:\game\addons\~tmp"),
            &sha(b"mod data"),
        );
        assert_eq!(placed.expect("placed"), Placement::Linked);
        assert!(fs.same_file(r"D:\lib\a.vpk", r"D:\game\addons\~tmp"));
    }

    #[test]
    fn another_volume_gets_a_verified_copy() {
        let fs = MemFs::default()
            .volume("C:", true)
            .volume("D:", true)
            .with(r"C:\lib\a.vpk", b"mod data");
        let placed = place(
            &fs,
            Path::new(r"C:\lib\a.vpk"),
            Path::new(r"D:\game\addons\~tmp"),
            &sha(b"mod data"),
        );
        assert_eq!(placed.expect("placed"), Placement::Copied { bytes: 8 });
        assert_eq!(
            fs.read(r"D:\game\addons\~tmp").as_deref(),
            Some(&b"mod data"[..])
        );
        assert!(!fs.same_file(r"C:\lib\a.vpk", r"D:\game\addons\~tmp"));
    }

    #[test]
    fn a_filesystem_without_hard_links_gets_a_copy() {
        let fs = MemFs::default()
            .volume("E:", false)
            .with(r"E:\lib\a.vpk", b"x");
        let placed = place(
            &fs,
            Path::new(r"E:\lib\a.vpk"),
            Path::new(r"E:\game\~tmp"),
            &sha(b"x"),
        );
        assert_eq!(placed.expect("placed"), Placement::Copied { bytes: 1 });
    }

    #[test]
    fn a_copy_that_does_not_match_the_manifest_is_removed() {
        let fs = MemFs::default()
            .volume("C:", true)
            .volume("D:", true)
            .with(r"C:\lib\a.vpk", b"changed on disk");
        let err = place(
            &fs,
            Path::new(r"C:\lib\a.vpk"),
            Path::new(r"D:\game\~tmp"),
            &sha(b"original"),
        )
        .expect_err("hash mismatch");
        assert!(matches!(err, PlaceError::HashMismatch { .. }), "{err:?}");
        assert_eq!(
            fs.read(r"D:\game\~tmp"),
            None,
            "the bad copy must not stay in the game folder"
        );
    }

    #[test]
    fn other_link_errors_are_reported_without_copying() {
        let fs = MemFs::default()
            .volume("D:", true)
            .with(r"D:\lib\a.vpk", b"x")
            .deny_writes_under(r"D:\game");
        let err = place(
            &fs,
            Path::new(r"D:\lib\a.vpk"),
            Path::new(r"D:\game\~tmp"),
            &sha(b"x"),
        )
        .expect_err("access denied");
        match err {
            PlaceError::Io(e) => assert_eq!(e.kind(), io::ErrorKind::PermissionDenied),
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn link_errors_that_mean_use_a_copy() {
        assert!(needs_copy(&io::Error::from(io::ErrorKind::CrossesDevices)));
        assert!(needs_copy(&io::Error::from(io::ErrorKind::Unsupported)));
        assert!(!needs_copy(&io::Error::from(
            io::ErrorKind::PermissionDenied
        )));
        assert!(!needs_copy(&io::Error::from(io::ErrorKind::AlreadyExists)));
    }

    #[cfg(windows)]
    #[test]
    fn windows_error_codes_that_mean_use_a_copy() {
        // ERROR_INVALID_FUNCTION, ERROR_NOT_SAME_DEVICE, ERROR_NOT_SUPPORTED
        for code in [1, 17, 50] {
            assert!(needs_copy(&io::Error::from_raw_os_error(code)), "{code}");
        }
        assert!(
            !needs_copy(&io::Error::from_raw_os_error(5)),
            "ERROR_ACCESS_DENIED"
        );
    }

    #[test]
    fn works_on_the_real_disk() {
        let dir = std::env::temp_dir().join(format!("deadloader-place-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("temp dir");
        let src = dir.join("a.vpk");
        let dst = dir.join("linked");
        std::fs::write(&src, b"real bytes").expect("write");
        let placed = place(&crate::fs::Disk, &src, &dst, &sha(b"real bytes")).expect("placed");
        assert_eq!(placed, Placement::Linked);
        assert_eq!(std::fs::read(&dst).expect("read"), b"real bytes");

        let copied = dir.join("copied");
        let (bytes, hash) = crate::fs::Disk.copy_verified(&src, &copied).expect("copy");
        assert_eq!((bytes, hash), (10, sha(b"real bytes")));
        std::fs::remove_dir_all(&dir).expect("cleanup");
    }
}
