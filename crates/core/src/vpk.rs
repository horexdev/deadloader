//! Reading the directory (file index) of Valve VPK packages, versions 1 and 2.
//!
//! Only the header and the directory tree are read — never file data — so this is cheap even for large packages and
//! is what conflict, part and variant detection work from: the path and CRC32 of every file inside.

use std::io::{self, Read};

pub const SIGNATURE: u32 = 0x55AA_1234;
/// Archive index meaning "the data follows the tree inside the `_dir.vpk` itself" (all single-file mods).
pub const DIR_ARCHIVE: u16 = 0x7FFF;
/// Far above any real mod's directory; stops a forged header from making us read or allocate gigabytes.
pub const MAX_TREE_SIZE: u32 = 64 * 1024 * 1024;

const ENTRY_TERMINATOR: u16 = 0xFFFF;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Index {
    pub version: u32,
    pub entries: Vec<Entry>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    /// `dir/name.ext` with forward slashes, as the game sees it.
    pub path: String,
    pub crc32: u32,
    pub preload_len: u16,
    pub archive_index: u16,
    pub offset: u32,
    pub length: u32,
}

impl Entry {
    pub fn in_dir_file(&self) -> bool {
        self.archive_index == DIR_ARCHIVE
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VpkError {
    NotAVpk,
    UnsupportedVersion(u32),
    /// The file ends before the directory it declares.
    Truncated,
    TreeTooLarge,
    /// The directory is complete but its structure is broken.
    Corrupt,
    InvalidName,
    Io(io::ErrorKind),
}

impl std::fmt::Display for VpkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotAVpk => f.write_str("not a VPK file"),
            Self::UnsupportedVersion(v) => write!(f, "VPK version {v} is not supported"),
            Self::Truncated => f.write_str("the VPK file is cut off"),
            Self::TreeTooLarge => f.write_str("the VPK directory is implausibly large"),
            Self::Corrupt => f.write_str("the VPK directory is damaged"),
            Self::InvalidName => f.write_str("the VPK contains a file name that is not valid text"),
            Self::Io(kind) => write!(f, "reading the VPK failed: {kind}"),
        }
    }
}

impl std::error::Error for VpkError {}

fn io_error(e: io::Error) -> VpkError {
    match e.kind() {
        io::ErrorKind::UnexpectedEof => VpkError::Truncated,
        kind => VpkError::Io(kind),
    }
}

/// Reads the header and directory tree from the start of a `_dir.vpk`.
pub fn read_index(r: &mut impl Read) -> Result<Index, VpkError> {
    let mut sig = [0u8; 4];
    r.read_exact(&mut sig).map_err(|e| match e.kind() {
        io::ErrorKind::UnexpectedEof => VpkError::NotAVpk,
        kind => VpkError::Io(kind),
    })?;
    if u32::from_le_bytes(sig) != SIGNATURE {
        return Err(VpkError::NotAVpk);
    }
    let version = read_u32(r)?;
    let tree_size = read_u32(r)?;
    match version {
        1 => {}
        2 => {
            let mut rest = [0u8; 16];
            r.read_exact(&mut rest).map_err(io_error)?;
        }
        v => return Err(VpkError::UnsupportedVersion(v)),
    }
    if tree_size > MAX_TREE_SIZE {
        return Err(VpkError::TreeTooLarge);
    }
    let mut tree = Vec::new();
    r.take(u64::from(tree_size))
        .read_to_end(&mut tree)
        .map_err(io_error)?;
    if tree.len() != tree_size as usize {
        return Err(VpkError::Truncated);
    }
    Ok(Index {
        version,
        entries: parse_tree(&tree)?,
    })
}

fn read_u32(r: &mut impl Read) -> Result<u32, VpkError> {
    let mut b = [0u8; 4];
    r.read_exact(&mut b).map_err(io_error)?;
    Ok(u32::from_le_bytes(b))
}

fn parse_tree(tree: &[u8]) -> Result<Vec<Entry>, VpkError> {
    let mut c = Cursor { data: tree, pos: 0 };
    let mut entries = Vec::new();
    loop {
        let ext = c.cstr()?;
        if ext.is_empty() {
            return Ok(entries);
        }
        loop {
            let dir = c.cstr()?;
            if dir.is_empty() {
                break;
            }
            loop {
                let name = c.cstr()?;
                if name.is_empty() {
                    break;
                }
                let crc32 = c.u32()?;
                let preload_len = c.u16()?;
                let archive_index = c.u16()?;
                let offset = c.u32()?;
                let length = c.u32()?;
                if c.u16()? != ENTRY_TERMINATOR {
                    return Err(VpkError::Corrupt);
                }
                c.skip(usize::from(preload_len))?;
                entries.push(Entry {
                    path: join(dir, name, ext),
                    crc32,
                    preload_len,
                    archive_index,
                    offset,
                    length,
                });
            }
        }
    }
}

/// A single space stands for "no directory" / "no extension".
fn join(dir: &str, name: &str, ext: &str) -> String {
    let mut path = String::new();
    if dir != " " {
        path.push_str(dir);
        path.push('/');
    }
    path.push_str(name);
    if ext != " " {
        path.push('.');
        path.push_str(ext);
    }
    path
}

struct Cursor<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Cursor<'a> {
    fn take(&mut self, n: usize) -> Result<&'a [u8], VpkError> {
        let end = self.pos.checked_add(n).ok_or(VpkError::Corrupt)?;
        let bytes = self.data.get(self.pos..end).ok_or(VpkError::Corrupt)?;
        self.pos = end;
        Ok(bytes)
    }

    fn skip(&mut self, n: usize) -> Result<(), VpkError> {
        self.take(n).map(|_| ())
    }

    fn u16(&mut self) -> Result<u16, VpkError> {
        let b = self.take(2)?;
        Ok(u16::from_le_bytes([b[0], b[1]]))
    }

    fn u32(&mut self) -> Result<u32, VpkError> {
        let b = self.take(4)?;
        Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }

    fn cstr(&mut self) -> Result<&'a str, VpkError> {
        let rest = self.data.get(self.pos..).ok_or(VpkError::Corrupt)?;
        let len = rest.iter().position(|&b| b == 0).ok_or(VpkError::Corrupt)?;
        let bytes = self.take(len + 1)?;
        std::str::from_utf8(&bytes[..len]).map_err(|_| VpkError::InvalidName)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct File<'a> {
        ext: &'a str,
        dir: &'a str,
        name: &'a str,
        crc: u32,
        preload: &'a [u8],
        archive: u16,
        offset: u32,
        length: u32,
    }

    fn file<'a>(ext: &'a str, dir: &'a str, name: &'a str, crc: u32) -> File<'a> {
        File {
            ext,
            dir,
            name,
            crc,
            preload: &[],
            archive: DIR_ARCHIVE,
            offset: 0,
            length: 10,
        }
    }

    fn cstr(out: &mut Vec<u8>, s: &str) {
        out.extend_from_slice(s.as_bytes());
        out.push(0);
    }

    /// Builds the tree the way Valve's tools do: grouped by extension, then by directory.
    fn tree(files: &[File<'_>]) -> Vec<u8> {
        let mut out = Vec::new();
        let mut i = 0;
        while i < files.len() {
            let ext = files[i].ext;
            cstr(&mut out, ext);
            while i < files.len() && files[i].ext == ext {
                let dir = files[i].dir;
                cstr(&mut out, dir);
                while i < files.len() && files[i].ext == ext && files[i].dir == dir {
                    let f = &files[i];
                    cstr(&mut out, f.name);
                    out.extend_from_slice(&f.crc.to_le_bytes());
                    out.extend_from_slice(&(f.preload.len() as u16).to_le_bytes());
                    out.extend_from_slice(&f.archive.to_le_bytes());
                    out.extend_from_slice(&f.offset.to_le_bytes());
                    out.extend_from_slice(&f.length.to_le_bytes());
                    out.extend_from_slice(&0xFFFFu16.to_le_bytes());
                    out.extend_from_slice(f.preload);
                    i += 1;
                }
                out.push(0);
            }
            out.push(0);
        }
        out.push(0);
        out
    }

    fn vpk(version: u32, tree: &[u8]) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&SIGNATURE.to_le_bytes());
        out.extend_from_slice(&version.to_le_bytes());
        out.extend_from_slice(&(tree.len() as u32).to_le_bytes());
        if version == 2 {
            out.extend_from_slice(&[0u8; 16]); // data, archive md5, other md5, signature sizes
        }
        out.extend_from_slice(tree);
        out
    }

    fn paths(index: &Index) -> Vec<&str> {
        index.entries.iter().map(|e| e.path.as_str()).collect()
    }

    #[test]
    fn reads_full_paths_and_checksums() {
        let bytes = vpk(
            2,
            &tree(&[
                file("vmdl_c", "models/heroes/haze", "haze", 0x1111_1111),
                file(
                    "vtex_c",
                    "materials/models/heroes/haze",
                    "body_color",
                    0x2222_2222,
                ),
                file(
                    "vtex_c",
                    "materials/models/heroes/haze",
                    "body_normal",
                    0x3333_3333,
                ),
            ]),
        );
        let index = read_index(&mut bytes.as_slice()).expect("valid vpk");
        assert_eq!(index.version, 2);
        assert_eq!(
            paths(&index),
            [
                "models/heroes/haze/haze.vmdl_c",
                "materials/models/heroes/haze/body_color.vtex_c",
                "materials/models/heroes/haze/body_normal.vtex_c",
            ]
        );
        assert_eq!(index.entries[1].crc32, 0x2222_2222);
    }

    #[test]
    fn a_single_space_means_no_directory_or_no_extension() {
        let bytes = vpk(
            2,
            &tree(&[file(" ", " ", "README", 1), file("txt", " ", "notes", 2)]),
        );
        let index = read_index(&mut bytes.as_slice()).expect("valid vpk");
        assert_eq!(paths(&index), ["README", "notes.txt"]);
    }

    #[test]
    fn skips_preload_data_between_entries() {
        let mut first = file("vsnd_c", "sounds/haze", "ult", 7);
        first.preload = b"preload bytes";
        let bytes = vpk(2, &tree(&[first, file("vsnd_c", "sounds/haze", "dash", 8)]));
        let index = read_index(&mut bytes.as_slice()).expect("valid vpk");
        assert_eq!(
            paths(&index),
            ["sounds/haze/ult.vsnd_c", "sounds/haze/dash.vsnd_c"]
        );
        assert_eq!(index.entries[0].preload_len, 13);
    }

    #[test]
    fn records_where_the_data_lives() {
        let mut chunked = file("vpcf_c", "particles", "fx", 3);
        chunked.archive = 5;
        chunked.offset = 1024;
        chunked.length = 2048;
        let bytes = vpk(2, &tree(&[file("vpcf_c", "particles", "a", 1), chunked]));
        let index = read_index(&mut bytes.as_slice()).expect("valid vpk");
        assert!(index.entries[0].in_dir_file());
        assert!(!index.entries[1].in_dir_file());
        assert_eq!(
            (index.entries[1].offset, index.entries[1].length),
            (1024, 2048)
        );
    }

    #[test]
    fn reads_version_1_packages() {
        let bytes = vpk(1, &tree(&[file("txt", "scripts", "a", 9)]));
        let index = read_index(&mut bytes.as_slice()).expect("valid vpk");
        assert_eq!(index.version, 1);
        assert_eq!(paths(&index), ["scripts/a.txt"]);
    }

    #[test]
    fn an_empty_package_has_no_entries() {
        let bytes = vpk(2, &tree(&[]));
        assert!(
            read_index(&mut bytes.as_slice())
                .expect("valid vpk")
                .entries
                .is_empty()
        );
    }

    #[test]
    fn rejects_files_that_are_not_vpks() {
        let err = read_index(&mut &b"PK\x03\x04 a zip archive"[..]).expect_err("not a vpk");
        assert_eq!(err, VpkError::NotAVpk);
    }

    #[test]
    fn rejects_unknown_versions() {
        let bytes = vpk(3, &tree(&[]));
        assert_eq!(
            read_index(&mut bytes.as_slice()).expect_err("v3"),
            VpkError::UnsupportedVersion(3)
        );
    }

    #[test]
    fn a_truncated_tree_is_an_error() {
        let mut bytes = vpk(2, &tree(&[file("txt", "a", "b", 1)]));
        bytes.truncate(bytes.len() - 5);
        assert_eq!(
            read_index(&mut bytes.as_slice()).expect_err("truncated"),
            VpkError::Truncated
        );
    }

    #[test]
    fn a_tree_that_ends_early_inside_itself_is_corrupt() {
        // The declared tree is complete on disk, but its content stops in the middle of an entry.
        let full = tree(&[file("txt", "a", "b", 1)]);
        let cut = &full[..full.len() - 8];
        let bytes = vpk(2, cut);
        assert_eq!(
            read_index(&mut bytes.as_slice()).expect_err("corrupt"),
            VpkError::Corrupt
        );
    }

    #[test]
    fn a_missing_entry_terminator_is_corrupt() {
        let mut t = tree(&[file("txt", "a", "b", 1)]);
        let pos = t
            .windows(2)
            .position(|w| w == [0xFF, 0xFF])
            .expect("terminator");
        t[pos] = 0x00;
        let bytes = vpk(2, &t);
        assert_eq!(
            read_index(&mut bytes.as_slice()).expect_err("corrupt"),
            VpkError::Corrupt
        );
    }

    #[test]
    fn a_huge_declared_tree_is_rejected_before_reading_it() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&SIGNATURE.to_le_bytes());
        bytes.extend_from_slice(&2u32.to_le_bytes());
        bytes.extend_from_slice(&u32::MAX.to_le_bytes());
        bytes.extend_from_slice(&[0u8; 16]);
        assert_eq!(
            read_index(&mut bytes.as_slice()).expect_err("too large"),
            VpkError::TreeTooLarge
        );
    }

    #[test]
    fn names_must_be_utf8() {
        let mut t = tree(&[file("txt", "a", "bad", 1)]);
        let pos = t.windows(3).position(|w| w == b"bad").expect("name");
        t[pos] = 0xC3; // start of a two-byte sequence followed by an ASCII byte
        let bytes = vpk(2, &t);
        assert_eq!(
            read_index(&mut bytes.as_slice()).expect_err("bad name"),
            VpkError::InvalidName
        );
    }
}
