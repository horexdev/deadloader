//! `deadloader-data\downloads\`: the queue file and the files of each task (ADR 0003).
//!
//! - `queue.json` — every unfinished task, in order, written with [`deadloader_core::fs::write_atomic`] (a flushed
//!   temp file renamed over the old one, the previous version kept as `.bak`), so a crash leaves either the old or the
//!   new queue, never a mix.
//! - `<id>.part` — the start of a download. Its trustworthy length is the task's `checkpoint`: the bytes up to it
//!   were flushed to disk before the checkpoint was saved. Anything after it may be garbage after a power cut and is
//!   cut off before resuming.
//! - `<id>.<zip|7z|rar|bin>` — a verified download, handed to the completion hook; kept afterwards only with
//!   `keep_archives` (decision 45).

use crate::http::Validator;
use crate::log::{DownloadLog, fields, file_op};
use crate::task::{Initiator, Md5, TaskId, TaskInfo};
use deadloader_core::log::{FileOp, Level};
use deadloader_core::message::UserMessage;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::io;
use std::path::{Path, PathBuf};

pub const QUEUE_FILE: &str = "queue.json";
const SCHEMA: u32 = 1;

/// The persisted part of a task's state. Running states are saved as `queued`: after a restart they continue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Saved {
    Queued,
    Paused,
    Waiting,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskRecord {
    pub id: TaskId,
    pub url: String,
    pub file_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub md5: Option<Md5>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
    #[serde(default)]
    pub initiator: Initiator,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub meta: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keep_as: Option<String>,
    pub state: Saved,
    /// Why the task waits or failed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<UserMessage>,
    /// Bytes of `<id>.part` known to be on disk.
    #[serde(default)]
    pub checkpoint: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validator: Option<Validator>,
}

impl TaskRecord {
    pub fn info(&self) -> TaskInfo {
        TaskInfo {
            id: self.id,
            url: self.url.clone(),
            file_name: self.file_name.clone(),
            md5: self.md5,
            size: self.size,
            initiator: self.initiator,
            meta: self.meta.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueueFile {
    pub schema: u32,
    pub next_id: u64,
    pub tasks: Vec<TaskRecord>,
}

impl Default for QueueFile {
    fn default() -> Self {
        Self {
            schema: SCHEMA,
            next_id: 1,
            tasks: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Loaded {
    pub queue: QueueFile,
    /// `false` when neither `queue.json` nor its backup could be read: then nothing on disk may be swept.
    pub trusted: bool,
}

pub struct Store<'a> {
    pub dir: &'a Path,
    pub log: &'a dyn DownloadLog,
}

impl Store<'_> {
    pub fn part_path(&self, id: TaskId) -> PathBuf {
        self.dir.join(format!("{id}.part"))
    }

    pub fn archive_path(&self, id: TaskId, file_name: &str) -> PathBuf {
        self.dir
            .join(format!("{id}.{}", archive_extension(file_name)))
    }

    /// Where a kept archive goes: `wanted` as a safe name that no file in the folder has yet.
    pub fn kept_path(&self, wanted: &str, file_name: &str) -> PathBuf {
        let name = kept_name(wanted, archive_extension(file_name), |name| {
            !self.dir.join(name).exists()
        });
        self.dir.join(name)
    }

    /// Reads `queue.json`, falling back to `queue.json.bak` if it is damaged. A missing file is an empty queue.
    pub fn load(&self) -> Loaded {
        let path = self.dir.join(QUEUE_FILE);
        match read_queue(&path) {
            Ok(Some(queue)) => {
                return Loaded {
                    queue,
                    trusted: true,
                };
            }
            Ok(None) => {
                return Loaded {
                    queue: QueueFile::default(),
                    trusted: true,
                };
            }
            Err(problem) => {
                self.log.app(
                    Level::Warn,
                    "the download queue could not be read; trying its backup",
                    fields([
                        ("file", path.display().to_string().into()),
                        ("error", problem.into()),
                    ]),
                );
            }
        }
        let backup = self.dir.join(format!("{QUEUE_FILE}.bak"));
        match read_queue(&backup) {
            Ok(Some(queue)) => Loaded {
                queue,
                trusted: true,
            },
            other => {
                self.log.app(
                    Level::Error,
                    "the download queue and its backup could not be read; starting with an empty queue",
                    fields([(
                        "error",
                        match other {
                            Err(e) => e.into(),
                            _ => "no backup".into(),
                        },
                    )]),
                );
                Loaded {
                    queue: QueueFile::default(),
                    trusted: false,
                }
            }
        }
    }

    pub fn save(&self, queue: &QueueFile) -> io::Result<()> {
        let path = self.dir.join(QUEUE_FILE);
        let text = serde_json::to_string_pretty(queue).map_err(io::Error::other)?;
        let bytes = text.len() as u64;
        file_op(
            self.log,
            FileOp::Write,
            "saved the download queue",
            None,
            Some(&path),
            Some(bytes),
            deadloader_core::fs::write_atomic(&path, &text),
        )
    }

    /// Deletes `<n>.part` files no task owns (left by a task removed while its file was locked, or a lost queue).
    pub fn sweep_orphans(&self, owned: &BTreeSet<TaskId>) {
        let Ok(entries) = std::fs::read_dir(self.dir) else {
            return;
        };
        for entry in entries.flatten() {
            let name = entry.file_name();
            let Some(id) = name.to_str().and_then(part_id) else {
                continue;
            };
            if owned.contains(&id) {
                continue;
            }
            let path = entry.path();
            let _ = file_op(
                self.log,
                FileOp::Delete,
                "deleted a download part no task owns",
                Some(&path),
                None,
                entry.metadata().ok().map(|m| m.len()),
                std::fs::remove_file(&path),
            );
        }
    }

    /// The highest task number used by a file in the folder, so a lost queue never makes ids repeat.
    pub fn highest_id_on_disk(&self) -> u64 {
        std::fs::read_dir(self.dir)
            .into_iter()
            .flatten()
            .flatten()
            .filter_map(|e| {
                e.file_name()
                    .to_str()
                    .and_then(|n| n.split_once('.'))
                    .and_then(|(stem, _)| stem.parse::<u64>().ok())
            })
            .max()
            .unwrap_or(0)
    }
}

fn read_queue(path: &Path) -> Result<Option<QueueFile>, String> {
    let text = match std::fs::read_to_string(path) {
        Ok(text) => text,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(e.to_string()),
    };
    let queue: QueueFile = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    if queue.schema != SCHEMA {
        return Err(format!(
            "schema {} is not supported (this version reads {SCHEMA})",
            queue.schema
        ));
    }
    Ok(Some(queue))
}

fn part_id(name: &str) -> Option<TaskId> {
    let stem = name.strip_suffix(".part")?;
    (!stem.is_empty() && stem.bytes().all(|b| b.is_ascii_digit()))
        .then(|| stem.parse().ok().map(TaskId))
        .flatten()
}

/// One of the archive formats deadloader reads, from the listed name; anything else is `bin` (the format is detected
/// from the bytes when unpacking anyway).
/// Characters kept of a kept archive's name before the extension; well inside Windows' path limits.
const KEPT_NAME_CHARS: usize = 150;

/// A Windows file name for a kept archive: forbidden and control characters become `_`, trailing dots and spaces go,
/// reserved device names get a `_` in front, and the extension is always ours. `free` says whether a name is unused;
/// a taken one gets ` (2)`, ` (3)`, ….
pub fn kept_name(wanted: &str, ext: &str, free: impl Fn(&str) -> bool) -> String {
    let dotted = format!(".{ext}");
    let stem = if wanted.len() > dotted.len()
        && wanted[wanted.len() - dotted.len()..].eq_ignore_ascii_case(&dotted)
    {
        &wanted[..wanted.len() - dotted.len()]
    } else {
        wanted
    };
    let cleaned: String = stem
        .chars()
        .map(|c| {
            if c.is_control() || r#"<>:"/\|?*"#.contains(c) {
                '_'
            } else {
                c
            }
        })
        .take(KEPT_NAME_CHARS)
        .collect();
    let mut stem = cleaned.trim_end_matches(['.', ' ']).trim_start().to_owned();
    if stem.is_empty() {
        stem = "download".to_owned();
    }
    let device = stem
        .split('.')
        .next()
        .unwrap_or_default()
        .to_ascii_uppercase();
    let reserved = matches!(device.as_str(), "CON" | "PRN" | "AUX" | "NUL")
        || ((device.starts_with("COM") || device.starts_with("LPT"))
            && device.len() == 4
            && device.as_bytes()[3].is_ascii_digit());
    if reserved {
        stem.insert(0, '_');
    }
    let first = format!("{stem}.{ext}");
    if free(&first) {
        return first;
    }
    (2..)
        .map(|n| format!("{stem} ({n}).{ext}"))
        .find(|name| free(name))
        .expect("a free name")
}

pub fn archive_extension(file_name: &str) -> &'static str {
    let ext = Path::new(file_name)
        .extension()
        .and_then(|e| e.to_str())
        .map(str::to_ascii_lowercase);
    match ext.as_deref() {
        Some("zip") => "zip",
        Some("7z") => "7z",
        Some("rar") => "rar",
        _ => "bin",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::log::mem::MemLog;
    use crate::test_support::TempDir;

    fn record(id: u64) -> TaskRecord {
        TaskRecord {
            id: TaskId(id),
            url: format!("https://gamebanana.com/dl/{id}"),
            file_name: "skin.zip".into(),
            md5: Md5::parse("d41d8cd98f00b204e9800998ecf8427e"),
            size: Some(10),
            initiator: Initiator::User,
            meta: BTreeMap::from([("mod".into(), "123".into())]),
            keep_as: None,
            state: Saved::Queued,
            message: None,
            checkpoint: 4,
            validator: Some(Validator {
                etag: Some("\"e\"".into()),
                last_modified: None,
            }),
        }
    }

    #[test]
    fn the_queue_round_trips_and_every_save_is_logged() {
        let tmp = TempDir::new("store-roundtrip");
        let log = MemLog::default();
        let store = Store {
            dir: tmp.path(),
            log: &log,
        };
        let queue = QueueFile {
            schema: 1,
            next_id: 3,
            tasks: vec![record(1), record(2)],
        };
        store.save(&queue).expect("save");
        store.save(&queue).expect("save again");
        let loaded = store.load();
        assert!(loaded.trusted);
        assert_eq!(loaded.queue, queue);
        let files = log.files();
        assert_eq!(files.len(), 2);
        assert_eq!(files[0].1.op, FileOp::Write);
    }

    #[test]
    fn a_missing_queue_is_empty_and_trusted() {
        let tmp = TempDir::new("store-missing");
        let log = MemLog::default();
        let loaded = Store {
            dir: tmp.path(),
            log: &log,
        }
        .load();
        assert!(loaded.trusted);
        assert!(loaded.queue.tasks.is_empty());
        assert_eq!(loaded.queue.next_id, 1);
    }

    #[test]
    fn a_damaged_queue_falls_back_to_the_backup() {
        let tmp = TempDir::new("store-damaged");
        let log = MemLog::default();
        let store = Store {
            dir: tmp.path(),
            log: &log,
        };
        let one = QueueFile {
            tasks: vec![record(1)],
            next_id: 2,
            ..QueueFile::default()
        };
        store.save(&one).expect("save");
        store.save(&one).expect("save makes a backup");
        // A crash mid-edit by hand, or a disk error: the file is cut short.
        std::fs::write(tmp.path().join(QUEUE_FILE), "{\"schema\":1,\"next").expect("damage");
        let loaded = store.load();
        assert!(loaded.trusted);
        assert_eq!(loaded.queue, one);
        assert!(log.app_msgs()[0].contains("backup"));
    }

    #[test]
    fn with_nothing_readable_the_folder_is_not_trusted() {
        let tmp = TempDir::new("store-untrusted");
        std::fs::write(tmp.path().join(QUEUE_FILE), "garbage").expect("write");
        let log = MemLog::default();
        let loaded = Store {
            dir: tmp.path(),
            log: &log,
        }
        .load();
        assert!(!loaded.trusted);
        assert!(loaded.queue.tasks.is_empty());
    }

    #[test]
    fn a_newer_schema_is_not_read() {
        let tmp = TempDir::new("store-newer");
        std::fs::write(
            tmp.path().join(QUEUE_FILE),
            r#"{"schema":2,"next_id":1,"tasks":[]}"#,
        )
        .expect("write");
        let log = MemLog::default();
        assert!(
            !Store {
                dir: tmp.path(),
                log: &log
            }
            .load()
            .trusted
        );
    }

    #[test]
    fn orphan_parts_are_swept_and_nothing_else() {
        let tmp = TempDir::new("store-sweep");
        for name in ["1.part", "2.part", "3.zip", "notes.part", "x.txt"] {
            std::fs::write(tmp.path().join(name), b"x").expect("write");
        }
        let log = MemLog::default();
        let store = Store {
            dir: tmp.path(),
            log: &log,
        };
        store.sweep_orphans(&BTreeSet::from([TaskId(1)]));
        let mut left: Vec<String> = std::fs::read_dir(tmp.path())
            .expect("list")
            .map(|e| e.expect("entry").file_name().to_string_lossy().into_owned())
            .collect();
        left.sort();
        assert_eq!(left, ["1.part", "3.zip", "notes.part", "x.txt"]);
        let files = log.files();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].1.op, FileOp::Delete);
        assert_eq!(store.highest_id_on_disk(), 3);
    }

    #[test]
    fn kept_names_are_safe_windows_file_names() {
        let free = |_: &str| true;
        assert_eq!(
            kept_name("Haze V2 - haze.zip", "zip", free),
            "Haze V2 - haze.zip"
        );
        // The extension is ours, whatever the name says.
        assert_eq!(
            kept_name("Haze V2 - haze.exe", "zip", free),
            "Haze V2 - haze.exe.zip"
        );
        assert_eq!(
            kept_name(r#"a<b>c:d"e/f\g|h?i*j.zip"#, "zip", free),
            "a_b_c_d_e_f_g_h_i_j.zip"
        );
        assert_eq!(kept_name("trailing. . ", "rar", free), "trailing.rar");
        assert_eq!(kept_name("CON", "zip", free), "_CON.zip");
        assert_eq!(kept_name("", "zip", free), "download.zip");
        assert_eq!(kept_name("x\u{7}y", "7z", free), "x_y.7z");
        let long = "a".repeat(400);
        assert!(kept_name(&long, "zip", free).chars().count() <= 150 + 4);
    }

    #[test]
    fn a_taken_kept_name_gets_a_number() {
        let taken = ["Haze.zip", "Haze (2).zip"];
        assert_eq!(
            kept_name("Haze.zip", "zip", |n| !taken.contains(&n)),
            "Haze (3).zip"
        );
    }

    #[test]
    fn archive_names_are_ours() {
        assert_eq!(archive_extension("Skin V2.ZIP"), "zip");
        assert_eq!(archive_extension("a.7z"), "7z");
        assert_eq!(archive_extension("a.rar"), "rar");
        assert_eq!(archive_extension("../../evil.exe"), "bin");
        assert_eq!(archive_extension("noext"), "bin");
        let log = MemLog::default();
        let store = Store {
            dir: Path::new("D:\\data\\downloads"),
            log: &log,
        };
        assert_eq!(
            store.archive_path(TaskId(7), "x.rar"),
            Path::new("D:\\data\\downloads\\7.rar")
        );
        assert_eq!(
            store.part_path(TaskId(7)),
            Path::new("D:\\data\\downloads\\7.part")
        );
    }
}
