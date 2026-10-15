//! The download queue in the app (F2): a catalog file becomes a task, a verified archive goes into the library and
//! the game, and every step reaches the UI through the push channel.

use crate::backend::Backend;
use crate::events::{Outbox, UiEvent};
use deadloader_core::library::Section;
use deadloader_core::message::{MessageId, UserMessage};
use deadloader_core::session::Download;
use deadloader_core::settings::Settings;
use deadloader_downloads::{
    Cancel, CompletionHook, Config, Deps, DownloadLog, Downloads, Event, Initiator, NewTask,
    TaskId, TaskInfo,
};
use deadloader_gamebanana::RemoteFile;
use deadloader_net::Client;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Weak};

/// What a task remembers for the hook, across restarts (`queue.json`).
mod meta {
    pub const SECTION: &str = "section";
    pub const MOD_ID: &str = "mod_id";
    pub const FILE_ID: &str = "file_id";
    pub const MOD_NAME: &str = "mod_name";
    pub const AUTHOR: &str = "author";
    pub const VERSION: &str = "version";
    /// The GameBanana file this download replaces (an update, D5).
    pub const REPLACES: &str = "replaces";
    /// The catalog's id of the mod's picture, which the row shows (`https://dl.localhost/img/<id>`).
    pub const THUMBNAIL: &str = "thumbnail";
}

/// The mod a file belongs to, as the UI shows it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModRef {
    pub section: Section,
    pub mod_id: u64,
    pub name: String,
    pub author: Option<String>,
    /// The catalog's id of the mod's picture (`Catalog::download_thumbnail`); never an address.
    pub thumbnail: Option<String>,
}

/// A task for one file of a GameBanana mod; the URL never leaves the core.
pub fn task_for(file: &RemoteFile, of: &ModRef) -> NewTask {
    let section = match of.section {
        Section::Mod => "mod",
        Section::Sound => "sound",
    };
    let mut task = NewTask::new(&file.download_url, &file.file_name)
        .size(file.size_bytes)
        .initiator(Initiator::User)
        .meta(meta::SECTION, section)
        .meta(meta::MOD_ID, of.mod_id.to_string())
        .meta(meta::FILE_ID, file.id.to_string())
        .meta(meta::MOD_NAME, &of.name)
        // Decision 174: a kept archive is named for people, not by its queue number.
        .keep_as(format!("{} - {}", of.name, file.file_name));
    if let Some(md5) = &file.md5 {
        task = task.md5(md5);
    }
    if let Some(author) = &of.author {
        task = task.meta(meta::AUTHOR, author);
    }
    if let Some(version) = &file.version {
        task = task.meta(meta::VERSION, version);
    }
    if let Some(thumbnail) = &of.thumbnail {
        task = task.meta(meta::THUMBNAIL, thumbnail);
    }
    task
}

/// The mods whose pictures queued downloads show, once each. The catalog forgets its picture ids when deadloader
/// closes; asking it for these again after a start makes the ids in the tasks valid again.
pub fn with_thumbnails(tasks: &[TaskInfo]) -> Vec<(Section, u64)> {
    let mut mods = Vec::new();
    for task in tasks
        .iter()
        .filter(|t| t.meta.contains_key(meta::THUMBNAIL))
    {
        let Ok(download) = download_of(task) else {
            continue;
        };
        let target = (download.section, download.mod_id);
        if !mods.contains(&target) {
            mods.push(target);
        }
    }
    mods
}

/// A mod an imported profile names but the library lacks (D9), with the GameBanana files it used.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub struct MissingMod {
    pub section: Section,
    pub mod_id: u64,
    pub name: String,
    pub file_ids: Vec<u64>,
}

/// A missing mod that couldn't be queued, and why.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct NotStarted {
    pub name: String,
    pub message: UserMessage,
}

/// "Download missing" in D9: a task per listed file. `find` gives a file with its download URL (fetching the mod
/// page when it isn't saved); a file GameBanana no longer lists is E24 for its mod. `thumbnail` gives the mod's
/// picture id, asked after `find`.
pub fn tasks_for_missing(
    mods: &[MissingMod],
    mut find: impl FnMut(Section, u64, u64) -> Result<Option<RemoteFile>, UserMessage>,
    mut thumbnail: impl FnMut(Section, u64) -> Option<String>,
) -> (Vec<NewTask>, Vec<NotStarted>) {
    let mut tasks = Vec::new();
    let mut failed = Vec::new();
    for m in mods {
        let mut of = ModRef {
            section: m.section,
            mod_id: m.mod_id,
            name: m.name.clone(),
            author: None,
            thumbnail: None,
        };
        let mut problem = None;
        for &file_id in &m.file_ids {
            match find(m.section, m.mod_id, file_id) {
                Ok(Some(file)) => {
                    if of.thumbnail.is_none() {
                        of.thumbnail = thumbnail(m.section, m.mod_id);
                    }
                    tasks.push(task_for(&file, &of));
                }
                Ok(None) => {
                    problem = Some(
                        UserMessage::new(MessageId::E24)
                            .arg("detail", format!("file {file_id} of mod {}", m.mod_id)),
                    );
                }
                Err(message) => {
                    problem = Some(message);
                    break;
                }
            }
        }
        if let Some(message) = problem {
            failed.push(NotStarted {
                name: m.name.clone(),
                message,
            });
        }
    }
    (tasks, failed)
}

/// A task for the file that replaces our GameBanana file `replaced` (an update the user picked in D5).
pub fn update_task_for(file: &RemoteFile, of: &ModRef, replaced: u64) -> NewTask {
    task_for(file, of).meta(meta::REPLACES, replaced.to_string())
}

/// The hook's view of a finished task; a task without our values is a mistake of deadloader.
pub fn download_of(task: &TaskInfo) -> Result<Download, UserMessage> {
    let get = |key: &str| task.meta.get(key).cloned();
    let number = |key: &str| get(key).and_then(|v| v.parse::<u64>().ok());
    let broken = || {
        UserMessage::new(MessageId::Internal).arg(
            "detail",
            format!("download {} has no mod or file id", task.id),
        )
    };
    let section = match get(meta::SECTION).as_deref() {
        Some("mod") => Section::Mod,
        Some("sound") => Section::Sound,
        _ => return Err(broken()),
    };
    Ok(Download {
        section,
        mod_id: number(meta::MOD_ID).ok_or_else(broken)?,
        file_id: number(meta::FILE_ID).ok_or_else(broken)?,
        file_name: task.file_name.clone(),
        mod_name: get(meta::MOD_NAME).unwrap_or_else(|| task.file_name.clone()),
        author: get(meta::AUTHOR),
        version: get(meta::VERSION),
    })
}

/// A verified archive: into the library and the game (F2 steps 3–6). A failure stays on the task with Retry (155).
struct Install {
    backend: Weak<Backend>,
    outbox: Arc<Outbox>,
}

/// The user's Cancel reaches the unpacking (between chunks) and the import (until the files go into the library).
struct Cancellable<'a, 'b>(&'a Cancel<'b>);

impl deadloader_core::import::Hooks for Cancellable<'_, '_> {
    fn cancelled(&self) -> bool {
        self.0.requested()
    }
}

impl CompletionHook for Install {
    fn downloaded(
        &self,
        archive: &Path,
        task: &TaskInfo,
        cancel: &Cancel<'_>,
    ) -> Result<(), UserMessage> {
        let hooks = Cancellable(cancel);
        let download = download_of(task)?;
        let backend = self.backend.upgrade().ok_or_else(|| {
            UserMessage::new(MessageId::Internal).arg("detail", "the app is closing")
        })?;
        if let Some(replaced) = task.meta.get(meta::REPLACES) {
            let replaced = replaced.parse::<u64>().map_err(|_| {
                UserMessage::new(MessageId::Internal)
                    .arg("detail", format!("download {} replaces no file", task.id))
            })?;
            let step = backend.install_update_file(archive, &download, replaced, &hooks)?;
            self.outbox.send(UiEvent::UpdateStep {
                task: Some(task.id),
                step,
            });
            return Ok(());
        }
        let installed = backend.install_download(archive, &download, &hooks)?;
        self.outbox.send(UiEvent::Installed {
            task: task.id,
            installed,
        });
        Ok(())
    }
}

/// Opens the queue in `dir`: queued and interrupted downloads go on, waiting ones are retried.
pub fn open(
    dir: PathBuf,
    settings: &Settings,
    client: Arc<Client>,
    log: Arc<dyn DownloadLog>,
    backend: &Arc<Backend>,
    outbox: Arc<Outbox>,
) -> Result<Downloads, UserMessage> {
    let events = Arc::clone(&outbox);
    Downloads::open(
        Config::new(dir, &settings.downloads),
        Deps {
            client,
            clock: Arc::new(deadloader_net::clock::SystemClock),
            log,
            space: Arc::new(|dir: &Path| deadloader_platform_windows::disk::free_space(dir).ok()),
            hook: Arc::new(Install {
                backend: Arc::downgrade(backend),
                outbox,
            }),
            events: Arc::new(move |event: &Event| events.send(UiEvent::Download(event.clone()))),
        },
    )
    .map_err(|e| deadloader_core::message::ToUserMessage::user_message(&e))
}

/// What a settings change means for the queue; leaving local mode retries what waited for the network.
pub fn follow_settings(
    downloads: Arc<Downloads>,
    settings: &Settings,
) -> crate::backend::SettingsListener {
    let local = AtomicBool::new(settings.is_local_mode());
    Box::new(move |settings: &Settings| {
        let d = &settings.downloads;
        downloads.set_parallel(d.parallel);
        downloads.set_speed_limit(d.speed_limit_kib_per_s);
        downloads.set_keep_archives(d.keep_archives);
        let was_local = local.swap(settings.is_local_mode(), Ordering::Relaxed);
        if was_local && !settings.is_local_mode() {
            downloads.retry_waiting();
        }
    })
}

/// The task id of a new download, or the existing one for the same file (a double click doesn't download twice).
pub fn install(
    downloads: &Downloads,
    file: &RemoteFile,
    of: &ModRef,
) -> Result<TaskId, UserMessage> {
    downloads
        .add(task_for(file, of))
        .map_err(|e| deadloader_core::message::ToUserMessage::user_message(&e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::tests::{FakeGame, Fixture};
    use crate::events::tests::Recorder;
    use deadloader_gamebanana::Antivirus;
    use std::collections::BTreeMap;

    fn haze_file() -> RemoteFile {
        RemoteFile {
            id: 1_480_331,
            file_name: "haze_v2.vpk".to_owned(),
            size_bytes: 73,
            date_added: None,
            download_count: 0,
            description: None,
            version: Some("2.0".to_owned()),
            md5: Some("0123456789abcdef0123456789abcdef".to_owned()),
            antivirus: Antivirus::Clean,
            download_url: "https://gamebanana.com/dl/1480331".to_owned(),
        }
    }

    fn haze() -> ModRef {
        ModRef {
            section: Section::Mod,
            mod_id: 655_209,
            name: "Haze V2".to_owned(),
            author: Some("Nyx".to_owned()),
            thumbnail: Some("0123456789abcdef0123456789abcdef".to_owned()),
        }
    }

    #[test]
    fn pictures_of_queued_downloads_are_named_again_once_per_mod_after_a_restart() {
        let mut one = info(&task_for(&haze_file(), &haze()));
        one.id = TaskId(1);
        let mut same_mod = one.clone();
        same_mod.id = TaskId(2);
        let mut sound = info(&task_for(
            &haze_file(),
            &ModRef {
                section: Section::Sound,
                mod_id: 9,
                ..haze()
            },
        ));
        sound.id = TaskId(3);
        let mut without = info(&task_for(
            &haze_file(),
            &ModRef {
                mod_id: 5,
                thumbnail: None,
                ..haze()
            },
        ));
        without.id = TaskId(4);
        assert_eq!(
            with_thumbnails(&[one, same_mod, sound, without]),
            [(Section::Mod, 655_209), (Section::Sound, 9)]
        );
    }

    fn info(new: &NewTask) -> TaskInfo {
        TaskInfo {
            id: TaskId(17),
            url: new.url.clone(),
            file_name: new.file_name.clone(),
            md5: None,
            size: new.size,
            initiator: new.initiator,
            meta: new.meta.clone(),
        }
    }

    #[test]
    fn missing_mods_of_an_imported_profile_become_tasks_and_problems_are_named() {
        let missing = [
            MissingMod {
                section: Section::Mod,
                mod_id: 655_209,
                name: "Haze V2".to_owned(),
                file_ids: vec![1_480_331],
            },
            MissingMod {
                section: Section::Mod,
                mod_id: 1,
                name: "Gone".to_owned(),
                file_ids: vec![9],
            },
            MissingMod {
                section: Section::Mod,
                mod_id: 2,
                name: "Removed file".to_owned(),
                file_ids: vec![8],
            },
        ];
        let mut asked = Vec::new();
        let (tasks, failed) = tasks_for_missing(
            &missing,
            |section, mod_id, file_id| {
                asked.push((section, mod_id, file_id));
                match mod_id {
                    655_209 => Ok(Some(haze_file())),
                    1 => Err(UserMessage::new(MessageId::E24)),
                    _ => Ok(None),
                }
            },
            |_, mod_id| (mod_id == 655_209).then(|| "ab".repeat(16)),
        );
        assert_eq!(asked.len(), 3);
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].meta["mod_name"], "Haze V2");
        assert_eq!(tasks[0].meta["file_id"], "1480331");
        assert_eq!(tasks[0].meta["thumbnail"], "ab".repeat(16));
        let named: Vec<(&str, MessageId)> = failed
            .iter()
            .map(|f| (f.name.as_str(), f.message.id))
            .collect();
        assert_eq!(
            named,
            [("Gone", MessageId::E24), ("Removed file", MessageId::E24)]
        );
    }

    #[test]
    fn a_catalog_file_becomes_a_task_and_back() {
        let task = task_for(&haze_file(), &haze());
        assert_eq!(task.url, "https://gamebanana.com/dl/1480331");
        assert_eq!(task.size, Some(73));
        assert_eq!(
            task.md5.as_deref(),
            Some("0123456789abcdef0123456789abcdef")
        );
        assert_eq!(task.initiator, Initiator::User);
        assert_eq!(
            task.keep_as.as_deref(),
            Some("Haze V2 - haze_v2.vpk"),
            "174"
        );
        assert_eq!(
            task.meta["thumbnail"], "0123456789abcdef0123456789abcdef",
            "the row shows the mod's picture"
        );
        let without = task_for(
            &haze_file(),
            &ModRef {
                thumbnail: None,
                ..haze()
            },
        );
        assert!(!without.meta.contains_key("thumbnail"));
        let download = download_of(&info(&task)).expect("ours");
        assert_eq!(
            download,
            Download {
                section: Section::Mod,
                mod_id: 655_209,
                file_id: 1_480_331,
                file_name: "haze_v2.vpk".to_owned(),
                mod_name: "Haze V2".to_owned(),
                author: Some("Nyx".to_owned()),
                version: Some("2.0".to_owned()),
            }
        );
    }

    #[test]
    fn a_task_without_our_values_is_an_internal_error() {
        let mut task = info(&task_for(&haze_file(), &haze()));
        task.meta = BTreeMap::new();
        assert_eq!(
            download_of(&task).expect_err("not ours").id,
            MessageId::Internal
        );
    }

    #[test]
    fn a_finished_download_is_installed_and_the_ui_hears_it() {
        let f = Fixture::new("hook");
        let backend = Arc::new(f.backend(&FakeGame::default()));
        let outbox = Arc::new(Outbox::new());
        let ui = Recorder::default();
        outbox.connect(Box::new(ui.clone()));
        let hook = Install {
            backend: Arc::downgrade(&backend),
            outbox,
        };
        let task = info(&task_for(&haze_file(), &haze()));
        let archive = f.mod_file("17.bin", "models/haze/body.vmdl_c");
        hook.downloaded(&archive, &task, &Cancel::never())
            .expect("installed");
        let events = ui.take();
        let UiEvent::Installed {
            task: id,
            installed,
        } = &events[0]
        else {
            panic!("{events:?}");
        };
        assert_eq!(*id, TaskId(17));
        assert_eq!(installed.outcome.mod_id, "gb-655209");
        let library = backend.library().expect("library");
        assert_eq!(library.mods[0].name, "Haze V2");
        assert!(library.mods[0].enabled);
    }

    #[test]
    fn a_downloaded_update_replaces_the_old_file_instead_of_being_installed_beside_it() {
        let f = Fixture::new("hookupdate");
        let backend = Arc::new(f.backend(&FakeGame::default()));
        let outbox = Arc::new(Outbox::new());
        let ui = Recorder::default();
        outbox.connect(Box::new(ui.clone()));
        let hook = Install {
            backend: Arc::downgrade(&backend),
            outbox,
        };
        let v1 = info(&task_for(&haze_file(), &haze()));
        hook.downloaded(
            &f.mod_file("1.bin", "models/haze/body.vmdl_c"),
            &v1,
            &Cancel::never(),
        )
        .expect("v1");
        ui.take();

        let mut v2_file = haze_file();
        v2_file.id = 1_480_400;
        let v2 = info(&update_task_for(&v2_file, &haze(), 1_480_331));
        hook.downloaded(
            &f.mod_file("2.bin", "models/haze/body.vmdl_c"),
            &v2,
            &Cancel::never(),
        )
        .expect("v2");
        let events = ui.take();
        let UiEvent::UpdateStep { step, .. } = &events[0] else {
            panic!("{events:?}");
        };
        assert!(
            matches!(step, crate::backend::UpdateStep::Applied { .. }),
            "{step:?}"
        );
        let library = backend.library().expect("library");
        assert_eq!(library.mods.len(), 1);
        assert_eq!(library.mods[0].files.len(), 2, "the old version stays (22)");
    }

    #[test]
    fn an_archive_that_is_not_a_mod_fails_the_task_with_the_reason() {
        let f = Fixture::new("hookfail");
        let backend = Arc::new(f.backend(&FakeGame::default()));
        let hook = Install {
            backend: Arc::downgrade(&backend),
            outbox: Arc::new(Outbox::new()),
        };
        let archive = f.dir.join("17.bin");
        std::fs::write(&archive, b"not a vpk").expect("write");
        let task = info(&task_for(&haze_file(), &haze()));
        let err = hook
            .downloaded(&archive, &task, &Cancel::never())
            .expect_err("refused");
        assert_ne!(err.id, MessageId::Internal, "{err:?}");
    }

    #[test]
    fn a_cancelled_install_leaves_the_library_and_the_game_as_they_were() {
        let f = Fixture::new("hookcancel");
        let game = FakeGame {
            installs: vec![f.game()],
            ..FakeGame::default()
        };
        let backend = Arc::new(f.backend(&game));
        let outbox = Arc::new(Outbox::new());
        let ui = Recorder::default();
        outbox.connect(Box::new(ui.clone()));
        let hook = Install {
            backend: Arc::downgrade(&backend),
            outbox,
        };
        let task = info(&task_for(&haze_file(), &haze()));
        let archive = f.mod_file("17.bin", "models/haze/body.vmdl_c");
        let asked = || true;
        assert!(
            hook.downloaded(&archive, &task, &Cancel::new(&asked))
                .is_err()
        );
        assert!(ui.take().is_empty(), "nothing installed to report");
        assert!(backend.library().expect("library").mods.is_empty());
        assert!(!f.addons().join("pak01_dir.vpk").exists());
        assert!(
            !f.dir
                .join("deadloader-data")
                .join("library")
                .join("gb-655209")
                .exists()
        );
    }
}
