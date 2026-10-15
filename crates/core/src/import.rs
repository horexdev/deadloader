//! Bringing mod files into the library: a `.vpk`, or the `.vpk` files inside a zip/7z archive (decisions 23, 24,
//! 47, 134, 144). Used for imports from disk and, later, for downloads.
//!
//! Files land in `library\<mod>\<batch>\<n>.vpk` under names we choose; every one must be a readable VPK (its
//! directory is what variants and conflicts are computed from), and is made read-only so a tool writing through a
//! hard link in the game folder can't change it (ADR 0004). A failed import leaves no folder and no manifest change.
//! The original file is never touched (decision 134: the library keeps its own copy).

use crate::fs::{Disk, FileOps};
use crate::library::{Manifest, Mod, ModFile, ModelError, Source};
use crate::overlap::{FileIndex, Grouping, group};
use crate::vpk::{VpkError, read_index};
pub use deadloader_archive::{ArchiveError, Hooks, Limits, NoHooks};
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct ImportRequest {
    /// A `.vpk`, `.zip` or `.7z` file.
    pub source: PathBuf,
    /// The library mod the files belong to; created if new. Letters, digits, `-` and `_` only.
    pub mod_id: String,
    pub name: String,
    pub mod_source: Source,
    pub version: Option<String>,
    /// The GameBanana file this archive is (a download), for update detection (164).
    pub remote_file_id: Option<u64>,
    /// The name to record instead of the source's own: a download's name on GameBanana, not the queue's `17.zip`.
    pub archive_name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Imported {
    /// Ids of the files added to the mod.
    pub new_files: Vec<String>,
    /// Variant groups and standalone files over all files of the mod, old and new.
    pub grouping: Grouping,
    /// Names inside the archive that were not mod files (decision 47: shown to the user).
    pub skipped: Vec<String>,
}

#[derive(Debug)]
pub enum ImportError {
    Archive(ArchiveError),
    NoModFiles { skipped: Vec<String> },
    NotAVpk { name: String, error: VpkError },
    InvalidModId(String),
    Manifest(ModelError),
    Io(io::Error),
}

impl std::fmt::Display for ImportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Archive(e) => write!(f, "{e}"),
            Self::NoModFiles { .. } => f.write_str("the file contains no .vpk mod files"),
            Self::NotAVpk { name, error } => {
                write!(f, "{name} is named .vpk but can't be read as one: {error}")
            }
            Self::InvalidModId(id) => write!(f, "invalid mod id {id}"),
            Self::Manifest(e) => write!(f, "{e}"),
            Self::Io(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for ImportError {}

/// Adds the mod files in `req.source` to `manifest` (the caller saves it).
pub fn import(
    data_dir: &Path,
    manifest: &mut Manifest,
    req: &ImportRequest,
    limits: &Limits,
    hooks: &dyn Hooks,
) -> Result<Imported, ImportError> {
    if req.mod_id.is_empty()
        || !req
            .mod_id
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        return Err(ImportError::InvalidModId(req.mod_id.clone()));
    }
    let (batch, out_dir) = free_batch(data_dir, manifest, &req.mod_id);
    std::fs::create_dir_all(&out_dir).map_err(ImportError::Io)?;
    match fill(data_dir, manifest, req, limits, hooks, batch, &out_dir) {
        Ok(imported) => Ok(imported),
        Err(e) => {
            let _ = std::fs::remove_dir_all(&out_dir);
            // Drop the mod folder too if this import created it and it is now empty.
            if let Some(parent) = out_dir.parent() {
                let _ = std::fs::remove_dir(parent);
            }
            Err(e)
        }
    }
}

fn fill(
    data_dir: &Path,
    manifest: &mut Manifest,
    req: &ImportRequest,
    limits: &Limits,
    hooks: &dyn Hooks,
    batch: u32,
    out_dir: &Path,
) -> Result<Imported, ImportError> {
    let source_name = req
        .source
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    let archive_name = req
        .archive_name
        .clone()
        .unwrap_or_else(|| source_name.clone());
    let archive_md5 = crate::fs::md5_file(&req.source).map_err(ImportError::Io)?;
    // A download's own name decides: the queue stores a bare `.vpk` as `<n>.bin`.
    let (vpks, skipped) = if deadloader_archive::is_vpk_name(&archive_name) {
        let output = out_dir.join("0.vpk");
        let (size, sha256) = Disk
            .copy_verified(&req.source, &output)
            .map_err(ImportError::Io)?;
        (
            vec![(archive_name.clone(), output, size, sha256)],
            Vec::new(),
        )
    } else {
        let x = deadloader_archive::extract_vpks(&req.source, out_dir, limits, hooks)
            .map_err(ImportError::Archive)?;
        let vpks = x
            .vpks
            .into_iter()
            .map(|v| (v.archive_path, v.output, v.size, v.sha256))
            .collect();
        (vpks, x.skipped)
    };
    if vpks.is_empty() {
        return Err(ImportError::NoModFiles { skipped });
    }

    let mut files = Vec::new();
    for (n, (inner, output, len, sha256)) in vpks.iter().enumerate() {
        let mut f = std::fs::File::open(output).map_err(ImportError::Io)?;
        read_index(&mut io::BufReader::new(&mut f)).map_err(|error| ImportError::NotAVpk {
            name: inner.clone(),
            error,
        })?;
        files.push(ModFile {
            id: format!("{}-{batch}-{n}", req.mod_id),
            remote_file_id: req.remote_file_id,
            archive_name: archive_name.clone(),
            inner_path: inner.clone(),
            library_path: format!("library/{}/{batch}/{n}.vpk", req.mod_id),
            len: *len,
            sha256: *sha256,
            version: req.version.clone(),
            successor: None,
            archive_md5: Some(archive_md5.clone()),
        });
    }

    let mut updated = manifest.clone();
    let new_files: Vec<String> = files.iter().map(|f| f.id.clone()).collect();
    match updated.mods.iter_mut().find(|m| m.id == req.mod_id) {
        Some(m) => m.files.extend(files),
        None => updated.mods.push(Mod {
            id: req.mod_id.clone(),
            name: req.name.clone(),
            author: None,
            source: req.mod_source.clone(),
            files,
        }),
    }
    updated.validate().map_err(ImportError::Manifest)?;

    for (_, output, _, _) in &vpks {
        let mut perm = std::fs::metadata(output)
            .map_err(ImportError::Io)?
            .permissions();
        perm.set_readonly(true);
        std::fs::set_permissions(output, perm).map_err(ImportError::Io)?;
    }
    let grouping = group_mod(data_dir, &updated, &req.mod_id)?;
    *manifest = updated;
    Ok(Imported {
        new_files,
        grouping,
        skipped,
    })
}

/// The next batch number not used by the manifest nor present on disk.
fn free_batch(data_dir: &Path, manifest: &Manifest, mod_id: &str) -> (u32, PathBuf) {
    let prefix = format!("library/{mod_id}/");
    let used = manifest
        .mods
        .iter()
        .filter(|m| m.id == mod_id)
        .flat_map(|m| &m.files)
        .filter_map(|f| {
            f.library_path
                .strip_prefix(&prefix)?
                .split('/')
                .next()?
                .parse::<u32>()
                .ok()
        })
        .max()
        .unwrap_or(0);
    let mut batch = used + 1;
    loop {
        let dir = data_dir
            .join("library")
            .join(mod_id)
            .join(batch.to_string());
        if !dir.exists() {
            return (batch, dir);
        }
        batch += 1;
    }
}

fn group_mod(data_dir: &Path, manifest: &Manifest, mod_id: &str) -> Result<Grouping, ImportError> {
    let mut indexes = Vec::new();
    for f in manifest
        .mods
        .iter()
        .filter(|m| m.id == mod_id)
        .flat_map(|m| &m.files)
    {
        let path = f
            .library_path
            .split('/')
            .fold(data_dir.to_path_buf(), |p, c| p.join(c));
        let mut file = std::fs::File::open(&path).map_err(ImportError::Io)?;
        let index = read_index(&mut io::BufReader::new(&mut file)).map_err(|error| {
            ImportError::NotAVpk {
                name: f.inner_path.clone(),
                error,
            }
        })?;
        indexes.push(FileIndex {
            id: f.id.clone(),
            entries: index
                .entries
                .into_iter()
                .map(|e| (e.path, e.crc32))
                .collect(),
        });
    }
    Ok(group(&indexes))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::{Manifest, Source};
    use std::io::Write;

    /// A VPK v2 whose directory lists the given `dir/name.ext` paths with their CRCs.
    use crate::testing::vpk;

    fn temp_dir(name: &str) -> PathBuf {
        let dir =
            std::env::temp_dir().join(format!("deadloader-import-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("temp dir");
        dir
    }

    fn zip(path: &Path, entries: &[(&str, &[u8])]) {
        let mut w = zip::ZipWriter::new(std::fs::File::create(path).expect("zip file"));
        let opts = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Stored);
        for (name, bytes) in entries {
            w.start_file(*name, opts).expect("entry");
            w.write_all(bytes).expect("write");
        }
        w.finish().expect("finish");
    }

    fn request(source: PathBuf) -> ImportRequest {
        ImportRequest {
            source,
            mod_id: "local-1".into(),
            name: "My mod".into(),
            mod_source: Source::Local {
                imported_from: Some("x".into()),
            },
            version: None,
            remote_file_id: None,
            archive_name: None,
        }
    }

    fn colour(crc: u32) -> Vec<u8> {
        vpk(&[
            ("models/heroes/haze/body.vmdl_c", crc),
            ("materials/haze/body.vtex_c", crc),
        ])
    }

    #[test]
    fn a_single_vpk_is_copied_into_the_library() {
        let dir = temp_dir("single");
        let data = dir.join("data");
        let src = dir.join("haze.vpk");
        let bytes = colour(1);
        std::fs::write(&src, &bytes).expect("vpk");
        let mut m = Manifest::default();
        let imported = import(
            &data,
            &mut m,
            &request(src.clone()),
            &Limits::default(),
            &NoHooks,
        )
        .expect("import");
        assert_eq!(imported.new_files.len(), 1);
        let (owner, f) = m.file(&imported.new_files[0]).expect("recorded");
        assert_eq!(owner.id, "local-1");
        assert_eq!(f.sha256, crate::fs::sha256(&bytes));
        assert_eq!(f.archive_name, "haze.vpk");
        let stored = data.join(f.library_path.replace('/', "\\"));
        assert_eq!(std::fs::read(&stored).expect("stored"), bytes);
        assert!(
            std::fs::metadata(&stored)
                .expect("meta")
                .permissions()
                .readonly(),
            "library files are read-only"
        );
        assert!(src.exists(), "the original is left where it was");
        cleanup(&dir);
    }

    #[test]
    fn every_file_records_the_md5_of_the_archive_it_came_from() {
        let dir = temp_dir("md5");
        let data = dir.join("data");
        let archive = dir.join("haze-colours.zip");
        zip(
            &archive,
            &[
                ("pink/pak01_dir.vpk", &colour(1)),
                ("blue/pak01_dir.vpk", &colour(2)),
            ],
        );
        let expected = crate::fs::md5_hex(&std::fs::read(&archive).expect("archive"));
        let mut m = Manifest::default();
        let imported = import(
            &data,
            &mut m,
            &request(archive),
            &Limits::default(),
            &NoHooks,
        )
        .expect("import");
        for id in &imported.new_files {
            let (_, f) = m.file(id).expect("recorded");
            // 171: what links a mod imported from disk to its file on GameBanana.
            assert_eq!(f.archive_md5.as_deref(), Some(expected.as_str()));
        }
        cleanup(&dir);
    }

    #[test]
    fn a_download_keeps_its_gamebanana_file_id_and_name() {
        let dir = temp_dir("download");
        let data = dir.join("data");
        // The download queue names kept archives by task number; GameBanana's own name is what updates match.
        let archive = dir.join("17.zip");
        zip(&archive, &[("pak01_dir.vpk", &colour(1))]);
        let mut m = Manifest::default();
        let req = ImportRequest {
            mod_id: "gb-655209".into(),
            mod_source: Source::GameBanana {
                section: crate::library::Section::Mod,
                mod_id: 655_209,
            },
            remote_file_id: Some(1_480_331),
            archive_name: Some("haze_v2_ea17d.zip".into()),
            ..request(archive)
        };
        let imported = import(&data, &mut m, &req, &Limits::default(), &NoHooks).expect("import");
        let (_, f) = m.file(&imported.new_files[0]).expect("recorded");
        assert_eq!(f.remote_file_id, Some(1_480_331));
        assert_eq!(f.archive_name, "haze_v2_ea17d.zip");
        cleanup(&dir);
    }

    #[test]
    fn a_downloaded_bare_vpk_is_recognised_by_its_gamebanana_name() {
        let dir = temp_dir("downloadvpk");
        let data = dir.join("data");
        let queued = dir.join("18.bin");
        std::fs::write(&queued, colour(1)).expect("vpk");
        let mut m = Manifest::default();
        let req = ImportRequest {
            archive_name: Some("haze.vpk".into()),
            ..request(queued)
        };
        let imported = import(&data, &mut m, &req, &Limits::default(), &NoHooks).expect("import");
        assert_eq!(
            m.file(&imported.new_files[0])
                .expect("recorded")
                .1
                .archive_name,
            "haze.vpk"
        );
        cleanup(&dir);
    }

    #[test]
    fn colour_options_in_one_archive_become_one_variant_group() {
        let dir = temp_dir("variants");
        let archive = dir.join("haze-colours.zip");
        zip(
            &archive,
            &[
                ("red/haze.vpk", &colour(1)),
                ("blue/haze.vpk", &colour(2)),
                ("README.txt", b"hi"),
            ],
        );
        let mut m = Manifest::default();
        let imported = import(
            &dir.join("data"),
            &mut m,
            &request(archive),
            &Limits::default(),
            &NoHooks,
        )
        .expect("import");
        assert_eq!(imported.new_files.len(), 2);
        assert_eq!(imported.grouping.variant_groups.len(), 1);
        assert_eq!(imported.skipped, ["README.txt"]);
        let inner: Vec<_> = m.mods[0]
            .files
            .iter()
            .map(|f| f.inner_path.as_str())
            .collect();
        assert_eq!(inner, ["red/haze.vpk", "blue/haze.vpk"]);
        cleanup(&dir);
    }

    #[test]
    fn files_changing_different_things_are_add_ons() {
        let dir = temp_dir("addons");
        let archive = dir.join("pack.zip");
        zip(
            &archive,
            &[
                ("skin.vpk", &vpk(&[("models/a/b.vmdl_c", 1)])),
                ("sound.vpk", &vpk(&[("sounds/a/b.vsnd_c", 2)])),
            ],
        );
        let mut m = Manifest::default();
        let imported = import(
            &dir.join("data"),
            &mut m,
            &request(archive),
            &Limits::default(),
            &NoHooks,
        )
        .expect("import");
        assert!(imported.grouping.variant_groups.is_empty());
        assert_eq!(imported.grouping.standalone.len(), 2);
        cleanup(&dir);
    }

    #[test]
    fn an_archive_without_mod_files_is_refused_and_leaves_nothing() {
        let dir = temp_dir("empty");
        let archive = dir.join("not-a-mod.zip");
        zip(&archive, &[("readme.txt", b"x")]);
        let data = dir.join("data");
        let mut m = Manifest::default();
        let err = import(
            &data,
            &mut m,
            &request(archive),
            &Limits::default(),
            &NoHooks,
        )
        .expect_err("empty");
        assert!(
            matches!(err, ImportError::NoModFiles { ref skipped } if skipped == &["readme.txt"]),
            "{err:?}"
        );
        assert!(m.mods.is_empty());
        assert!(
            !data.join("library").join("local-1").exists(),
            "no folder left behind"
        );
        cleanup(&dir);
    }

    #[test]
    fn a_vpk_that_is_not_one_is_refused_and_cleaned_up() {
        let dir = temp_dir("fake");
        let archive = dir.join("broken.zip");
        zip(
            &archive,
            &[
                ("good.vpk", &colour(1)),
                ("bad.vpk", b"PK not a vpk at all"),
            ],
        );
        let data = dir.join("data");
        let mut m = Manifest::default();
        let err = import(
            &data,
            &mut m,
            &request(archive),
            &Limits::default(),
            &NoHooks,
        )
        .expect_err("bad vpk");
        assert!(
            matches!(err, ImportError::NotAVpk { ref name, .. } if name == "bad.vpk"),
            "{err:?}"
        );
        assert!(m.mods.is_empty());
        assert!(!data.join("library").join("local-1").exists());
        cleanup(&dir);
    }

    #[test]
    fn a_second_import_adds_to_the_same_mod_without_clashing() {
        let dir = temp_dir("second");
        let data = dir.join("data");
        let first = dir.join("red.vpk");
        let second = dir.join("blue.vpk");
        std::fs::write(&first, colour(1)).expect("vpk");
        std::fs::write(&second, colour(2)).expect("vpk");
        let mut m = Manifest::default();
        import(&data, &mut m, &request(first), &Limits::default(), &NoHooks).expect("first");
        let imported = import(
            &data,
            &mut m,
            &request(second),
            &Limits::default(),
            &NoHooks,
        )
        .expect("second");
        assert_eq!(m.mods.len(), 1);
        assert_eq!(m.mods[0].files.len(), 2);
        assert_eq!(
            imported.grouping.variant_groups.len(),
            1,
            "the two colours overlap"
        );
        m.validate().expect("unique ids, safe paths");
        cleanup(&dir);
    }

    #[test]
    fn something_that_is_neither_vpk_nor_archive_is_refused() {
        let dir = temp_dir("other");
        let src = dir.join("notes.txt");
        std::fs::write(&src, b"hello").expect("file");
        let err = import(
            &dir.join("data"),
            &mut Manifest::default(),
            &request(src),
            &Limits::default(),
            &NoHooks,
        )
        .expect_err("not supported");
        assert!(matches!(err, ImportError::Archive(_)), "{err:?}");
        cleanup(&dir);
    }

    fn cleanup(dir: &Path) {
        // Library files are read-only; clear the attribute before deleting the test folder.
        fn walk(p: &Path) {
            if let Ok(entries) = std::fs::read_dir(p) {
                for e in entries.flatten() {
                    let path = e.path();
                    if path.is_dir() {
                        walk(&path);
                    } else if let Ok(meta) = std::fs::metadata(&path) {
                        let mut perm = meta.permissions();
                        #[allow(clippy::permissions_set_readonly_false)]
                        perm.set_readonly(false);
                        let _ = std::fs::set_permissions(&path, perm);
                    }
                }
            }
        }
        walk(dir);
        std::fs::remove_dir_all(dir).expect("cleanup");
    }
}
