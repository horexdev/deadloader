//! Dated backups of the files users can edit by hand or reset (decision 156): before "Reset all settings" and before
//! a save from the Advanced editor, the current file is copied to `<name>.bak-YYYYMMDD-HHMM` next to it, and only the
//! newest [`KEEP`] copies stay. The plain `<name>.bak` of every save ([`crate::fs::write_atomic`]) is separate.
//!
//! The caller logs the copy and the deletions it reports.

use std::io;
use std::path::{Path, PathBuf};
use time::OffsetDateTime;

/// How many dated backups of one file are kept.
pub const KEEP: usize = 10;

/// What [`keep`] did.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Kept {
    /// The new backup.
    pub backup: PathBuf,
    /// Older backups deleted to stay at [`KEEP`].
    pub pruned: Vec<PathBuf>,
}

/// Copies `file` to a dated backup next to it (local time of `at`) and prunes the oldest beyond [`KEEP`]. `None` when
/// the file doesn't exist yet (nothing to keep).
pub fn keep(file: &Path, at: OffsetDateTime) -> io::Result<Option<Kept>> {
    if !file.exists() {
        return Ok(None);
    }
    let stamp = format!(
        "{:04}{:02}{:02}-{:02}{:02}",
        at.year(),
        u8::from(at.month()),
        at.day(),
        at.hour(),
        at.minute()
    );
    let base = format!("{}.bak-{stamp}", name_of(file));
    let mut backup = file.with_file_name(&base);
    let mut n = 2;
    while backup.exists() {
        backup = file.with_file_name(format!("{base}-{n}"));
        n += 1;
    }
    std::fs::copy(file, &backup)?;
    let mut pruned = Vec::new();
    let all = list(file)?;
    if all.len() > KEEP {
        for old in &all[..all.len() - KEEP] {
            std::fs::remove_file(old)?;
            pruned.push(old.clone());
        }
    }
    Ok(Some(Kept { backup, pruned }))
}

/// The dated backups of `file`, oldest first.
pub fn list(file: &Path) -> io::Result<Vec<PathBuf>> {
    let Some(dir) = file.parent() else {
        return Ok(Vec::new());
    };
    let prefix = format!("{}.bak-", name_of(file)).to_lowercase();
    let mut found: Vec<(String, PathBuf)> = match std::fs::read_dir(dir) {
        Ok(entries) => entries
            .filter_map(Result::ok)
            .filter_map(|e| {
                let name = e.file_name().to_string_lossy().into_owned();
                let stamp = name.to_lowercase().strip_prefix(&prefix)?.to_owned();
                is_stamp(&stamp).then(|| (sort_key(&stamp), e.path()))
            })
            .collect(),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Vec::new(),
        Err(e) => return Err(e),
    };
    found.sort();
    Ok(found.into_iter().map(|(_, p)| p).collect())
}

fn name_of(file: &Path) -> String {
    file.file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default()
}

/// `YYYYMMDD-HHMM` with an optional `-N` for several in one minute.
fn is_stamp(s: &str) -> bool {
    let (main, n) = match s.get(13..) {
        Some(rest) if !rest.is_empty() => (&s[..13], rest.strip_prefix('-')),
        _ => (s, Some("1")),
    };
    let b = main.as_bytes();
    main.len() == 13
        && b[8] == b'-'
        && b.iter()
            .enumerate()
            .all(|(i, c)| i == 8 || c.is_ascii_digit())
        && n.is_some_and(|n| !n.is_empty() && n.bytes().all(|c| c.is_ascii_digit()))
}

/// Sorts by time, then by the number within one minute.
fn sort_key(stamp: &str) -> String {
    let n: u32 = stamp.get(14..).and_then(|n| n.parse().ok()).unwrap_or(1);
    format!("{}-{n:06}", &stamp[..13])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{remove_all, temp_dir};
    use time::{Date, Month, UtcOffset};

    fn at(minute: u8, second: u8) -> OffsetDateTime {
        Date::from_calendar_date(2026, Month::October, 15)
            .expect("date")
            .with_hms(11, minute, second)
            .expect("time")
            .assume_offset(UtcOffset::from_hms(3, 0, 0).expect("offset"))
    }

    #[test]
    fn keeps_a_dated_copy_next_to_the_file() {
        let dir = temp_dir("backups", "copy");
        let file = dir.join("settings.json");
        std::fs::write(&file, "{\"schema\": 1}").expect("write");
        let kept = keep(&file, at(31, 0)).expect("kept").expect("a copy");
        assert_eq!(kept.backup, dir.join("settings.json.bak-20261015-1131"));
        assert_eq!(
            std::fs::read_to_string(&kept.backup).expect("read"),
            "{\"schema\": 1}"
        );
        // A second one in the same minute gets its own name.
        let again = keep(&file, at(31, 40)).expect("kept").expect("a copy");
        assert_eq!(again.backup, dir.join("settings.json.bak-20261015-1131-2"));
        remove_all(&dir);
    }

    #[test]
    fn a_missing_file_has_nothing_to_keep() {
        let dir = temp_dir("backups", "missing");
        assert_eq!(
            keep(&dir.join("settings.json"), at(31, 0)).expect("ok"),
            None
        );
        remove_all(&dir);
    }

    #[test]
    fn only_the_newest_ten_stay() {
        let dir = temp_dir("backups", "prune");
        let file = dir.join("manifest.json");
        std::fs::write(&file, "{}").expect("write");
        // Unrelated files with similar names are never touched.
        std::fs::write(dir.join("manifest.json.bak"), "x").expect("bak");
        std::fs::write(dir.join("manifest.json.bak-notes"), "x").expect("notes");
        let mut last = None;
        for minute in 0..12u8 {
            last = keep(&file, at(minute, 0)).expect("kept");
        }
        let left = list(&file).expect("list");
        assert_eq!(left.len(), KEEP);
        assert_eq!(
            left[0],
            dir.join("manifest.json.bak-20261015-1102"),
            "the oldest went"
        );
        assert_eq!(last.expect("kept").pruned.len(), 1);
        assert!(dir.join("manifest.json.bak").exists());
        assert!(dir.join("manifest.json.bak-notes").exists());
        remove_all(&dir);
    }
}
