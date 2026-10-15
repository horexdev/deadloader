//! The download queue in the app (F2): a catalog file becomes a task, a verified archive goes into the library and
//! the game, and every step reaches the UI through the push channel.

use crate::backend::Backend;
use crate::events::{Outbox, UiEvent};
use deadloader_core::library::Section;
use deadloader_core::message::{MessageId, UserMessage};
use deadloader_core::session::Download;
use deadloader_core::settings::Settings;
use deadloader_downloads::{
    CompletionHook, Config, Deps, DownloadLog, Downloads, Event, Initiator, NewTask, TaskId,
    TaskInfo,
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
}

/// The mod a file belongs to, as the UI shows it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModRef {
    pub section: Section,
    pub mod_id: u64,
    pub name: String,
    pub author: Option<String>,
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
    task
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

impl CompletionHook for Install {
    fn downloaded(&self, archive: &Path, task: &TaskInfo) -> Result<(), UserMessage> {
        let download = download_of(task)?;
        let backend = self.backend.upgrade().ok_or_else(|| {
            UserMessage::new(MessageId::Internal).arg("detail", "the app is closing")
        })?;
        let installed = backend.install_download(archive, &download)?;
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
        }
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
    fn a_catalog_file_becomes_a_task_and_back() {
        let task = task_for(&haze_file(), &haze());
        assert_eq!(task.url, "https://gamebanana.com/dl/1480331");
        assert_eq!(task.size, Some(73));
        assert_eq!(
            task.md5.as_deref(),
            Some("0123456789abcdef0123456789abcdef")
        );
        assert_eq!(task.initiator, Initiator::User);
        assert_eq!(task.keep_as.as_deref(), Some("Haze V2 - haze_v2.vpk"), "174");
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
        hook.downloaded(&archive, &task).expect("installed");
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
        let err = hook.downloaded(&archive, &task).expect_err("refused");
        assert_ne!(err.id, MessageId::Internal, "{err:?}");
    }
}
