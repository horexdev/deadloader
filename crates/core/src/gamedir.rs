//! The game's files we touch, on disk: where they are, and making `gameinfo.gi` load mods (ADR 0006).
//!
//! An edit is: back up the current file, write the new text to a temporary file next to it, atomically replace the
//! original. A crash leaves either the old or the new file, never a half-written one. Foreign files are not touched.

use crate::fs::{FileOps, ReadText};
use crate::gameinfo::{self, Foreign, State};
use std::io;
use std::path::{Path, PathBuf};

pub fn citadel_dir(game_root: &Path) -> PathBuf {
    game_root.join("game").join("citadel")
}

pub fn gameinfo_path(game_root: &Path) -> PathBuf {
    citadel_dir(game_root).join("gameinfo.gi")
}

pub fn addons_path(game_root: &Path) -> PathBuf {
    citadel_dir(game_root).join("addons")
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GuardOutcome {
    /// Nothing to do.
    AlreadyOk,
    /// Our line was added; the previous file is in `backup`.
    Added {
        backup: PathBuf,
    },
    /// Our line was removed; the previous file is in `backup`.
    Removed {
        backup: PathBuf,
    },
    /// Changed by something else (decision 19, D10): left as it is.
    Foreign(Vec<Foreign>),
    Missing,
}

/// Makes `gameinfo.gi` list our addons folder if it is vanilla, and repairs 0.1.0's edit (see `gameinfo::upgrade`).
pub fn ensure_mods_load(
    fs: &(impl FileOps + ReadText),
    game_root: &Path,
    backup_dir: &Path,
    backup_name: &str,
) -> io::Result<GuardOutcome> {
    edit(
        fs,
        game_root,
        backup_dir,
        backup_name,
        |text| match gameinfo::inspect(text) {
            State::Ours => Ok(gameinfo::upgrade(text)),
            State::Foreign(reasons) => Err(GuardOutcome::Foreign(reasons)),
            State::Vanilla => Ok(gameinfo::add_addons_path(text).ok()),
        },
    )
    .map(|r| {
        r.map_or(GuardOutcome::AlreadyOk, |backup| GuardOutcome::Added {
            backup,
        })
    })
    .or_else(early)
}

/// Removes our line again (full rollback, decision 20). Vanilla and foreign files are left alone.
pub fn remove_mods_path(
    fs: &(impl FileOps + ReadText),
    game_root: &Path,
    backup_dir: &Path,
    backup_name: &str,
) -> io::Result<GuardOutcome> {
    edit(
        fs,
        game_root,
        backup_dir,
        backup_name,
        |text| match gameinfo::inspect(text) {
            State::Vanilla => Ok(None),
            State::Foreign(reasons) => Err(GuardOutcome::Foreign(reasons)),
            State::Ours => Ok(gameinfo::remove_addons_path(text).ok()),
        },
    )
    .map(|r| {
        r.map_or(GuardOutcome::AlreadyOk, |backup| GuardOutcome::Removed {
            backup,
        })
    })
    .or_else(early)
}

/// Either a real I/O error or an outcome that ends the edit before any write.
enum Stop {
    Io(io::Error),
    Outcome(GuardOutcome),
}

fn early(stop: Stop) -> io::Result<GuardOutcome> {
    match stop {
        Stop::Io(e) => Err(e),
        Stop::Outcome(o) => Ok(o),
    }
}

/// Reads the file, lets `change` decide, and applies a new text with backup + atomic replace.
/// Returns the backup path when the file was changed.
fn edit(
    fs: &(impl FileOps + ReadText),
    game_root: &Path,
    backup_dir: &Path,
    backup_name: &str,
    change: impl FnOnce(&str) -> Result<Option<String>, GuardOutcome>,
) -> Result<Option<PathBuf>, Stop> {
    let path = gameinfo_path(game_root);
    let text = match fs.read_text(&path) {
        Ok(t) => t,
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            return Err(Stop::Outcome(GuardOutcome::Missing));
        }
        Err(e) => return Err(Stop::Io(e)),
    };
    let Some(new_text) = change(&text).map_err(Stop::Outcome)? else {
        return Ok(None);
    };
    fs.create_dir_all(backup_dir).map_err(Stop::Io)?;
    let backup = backup_dir.join(backup_name);
    fs.write_new(&backup, text.as_bytes()).map_err(Stop::Io)?;
    let staged = path.with_extension("gi.dl-tmp");
    // A leftover from an interrupted edit; checked first so a normal run logs no failed delete.
    if fs.exists(&staged).map_err(Stop::Io)? {
        match fs.remove_file(&staged) {
            Ok(()) => {}
            Err(e) if e.kind() == io::ErrorKind::NotFound => {}
            Err(e) => return Err(Stop::Io(e)),
        }
    }
    fs.write_new(&staged, new_text.as_bytes())
        .map_err(Stop::Io)?;
    fs.replace(&staged, &path).map_err(Stop::Io)?;
    Ok(Some(backup))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fs::fake::MemFs;
    use crate::gameinfo::{Foreign, inspect};

    const ROOT: &str = r"D:\SteamLibrary\steamapps\common\Deadlock";
    const GAMEINFO: &str = r"D:\SteamLibrary\steamapps\common\Deadlock\game\citadel\gameinfo.gi";
    const BACKUPS: &str = r"E:\deadloader\deadloader-data\backups\gameinfo";

    const VANILLA: &str = "\"GameInfo\"\r\n{\r\n\tFileSystem\r\n\t{\r\n\t\tSearchPaths\r\n\t\t{\r\n\t\t\tGame\t\t\t\tcitadel\r\n\t\t\tGame\t\t\t\tcore\r\n\t\t}\r\n\t}\r\n}\r\n";

    fn root() -> &'static Path {
        Path::new(ROOT)
    }

    #[test]
    fn knows_where_the_game_files_are() {
        assert_eq!(gameinfo_path(root()), PathBuf::from(GAMEINFO));
        assert_eq!(
            addons_path(root()),
            PathBuf::from(format!(r"{ROOT}\game\citadel\addons"))
        );
    }

    #[test]
    fn a_vanilla_file_gets_our_line_after_a_backup() {
        let fs = MemFs::default().with(GAMEINFO, VANILLA.as_bytes());
        let outcome = ensure_mods_load(&fs, root(), Path::new(BACKUPS), "2026-10-15-build1.gi")
            .expect("guard");
        let backup = PathBuf::from(format!(r"{BACKUPS}\2026-10-15-build1.gi"));
        assert_eq!(
            outcome,
            GuardOutcome::Added {
                backup: backup.clone()
            }
        );
        assert_eq!(
            fs.read(&backup.to_string_lossy()).as_deref(),
            Some(VANILLA.as_bytes())
        );
        let now = String::from_utf8(fs.read(GAMEINFO).expect("file")).expect("text");
        assert_eq!(inspect(&now), crate::gameinfo::State::Ours);
        assert_eq!(
            fs.snapshot().keys().filter(|k| k.ends_with(".tmp")).count(),
            0,
            "no temp file left"
        );
    }

    #[test]
    fn a_file_with_our_line_is_left_alone() {
        let ours = crate::gameinfo::add_addons_path(VANILLA).expect("vanilla");
        let fs = MemFs::default().with(GAMEINFO, ours.as_bytes());
        let before = fs.snapshot();
        assert_eq!(
            ensure_mods_load(&fs, root(), Path::new(BACKUPS), "b.gi").expect("guard"),
            GuardOutcome::AlreadyOk
        );
        assert_eq!(fs.snapshot(), before, "no backup, no write");
    }

    #[test]
    fn the_0_1_0_edit_is_repaired_after_a_backup() {
        // 0.1.0 added the addons line alone, which made the game stop at start; applying again repairs it.
        let old = VANILLA.replace(
            "\t\t\tGame\t\t\t\tcitadel\r\n",
            "\t\t\tGame\t\t\t\tcitadel/addons\r\n\t\t\tGame\t\t\t\tcitadel\r\n",
        );
        let fs = MemFs::default().with(GAMEINFO, old.as_bytes());
        let outcome = ensure_mods_load(&fs, root(), Path::new(BACKUPS), "b.gi").expect("guard");
        let backup = PathBuf::from(format!(r"{BACKUPS}\b.gi"));
        assert_eq!(
            outcome,
            GuardOutcome::Added {
                backup: backup.clone()
            }
        );
        assert_eq!(
            fs.read(&backup.to_string_lossy()).as_deref(),
            Some(old.as_bytes())
        );
        let now = String::from_utf8(fs.read(GAMEINFO).expect("file")).expect("text");
        assert_eq!(
            now,
            crate::gameinfo::add_addons_path(VANILLA).expect("vanilla")
        );
    }

    #[test]
    fn a_foreign_file_is_never_touched() {
        let foreign = VANILLA.replace(
            "\t\t\tGame\t\t\t\tcore\r\n",
            "\t\t\tGame\t\t\t\tcore\r\n\t\t\tGame\tcitadel/addons\r\n",
        );
        let fs = MemFs::default().with(GAMEINFO, foreign.as_bytes());
        let before = fs.snapshot();
        let outcome = ensure_mods_load(&fs, root(), Path::new(BACKUPS), "b.gi").expect("guard");
        assert_eq!(
            outcome,
            GuardOutcome::Foreign(vec![Foreign::AddonsElsewhere])
        );
        assert_eq!(fs.snapshot(), before);
    }

    #[test]
    fn a_missing_file_is_reported() {
        let fs = MemFs::default();
        assert_eq!(
            ensure_mods_load(&fs, root(), Path::new(BACKUPS), "b.gi").expect("guard"),
            GuardOutcome::Missing
        );
    }

    #[test]
    fn a_failed_write_leaves_the_original_in_place() {
        let fs = MemFs::default()
            .with(GAMEINFO, VANILLA.as_bytes())
            .deny_writes_under(r"D:\SteamLibrary\steamapps\common\Deadlock\game");
        assert!(ensure_mods_load(&fs, root(), Path::new(BACKUPS), "b.gi").is_err());
        assert_eq!(fs.read(GAMEINFO).as_deref(), Some(VANILLA.as_bytes()));
    }

    #[test]
    fn a_leftover_temp_file_from_a_crash_does_not_block_the_fix() {
        let fs = MemFs::default()
            .with(GAMEINFO, VANILLA.as_bytes())
            .with(&format!("{GAMEINFO}.dl-tmp"), b"half written");
        assert!(matches!(
            ensure_mods_load(&fs, root(), Path::new(BACKUPS), "b.gi").expect("guard"),
            GuardOutcome::Added { .. }
        ));
    }

    #[test]
    fn rollback_removes_only_our_line() {
        let ours = crate::gameinfo::add_addons_path(VANILLA).expect("vanilla");
        let fs = MemFs::default().with(GAMEINFO, ours.as_bytes());
        let outcome = remove_mods_path(&fs, root(), Path::new(BACKUPS), "before-rollback.gi")
            .expect("rollback");
        assert!(
            matches!(outcome, GuardOutcome::Removed { .. }),
            "{outcome:?}"
        );
        assert_eq!(fs.read(GAMEINFO).as_deref(), Some(VANILLA.as_bytes()));
    }

    #[test]
    fn rollback_of_a_vanilla_file_does_nothing() {
        let fs = MemFs::default().with(GAMEINFO, VANILLA.as_bytes());
        let before = fs.snapshot();
        assert_eq!(
            remove_mods_path(&fs, root(), Path::new(BACKUPS), "b.gi").expect("rollback"),
            GuardOutcome::AlreadyOk
        );
        assert_eq!(fs.snapshot(), before);
    }
}
