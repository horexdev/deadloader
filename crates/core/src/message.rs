//! What the user is told when something fails (decision 98, spec `04-messages`).
//!
//! The core does not hold texts: a message is an id from the spec (E1…) and named values; the UI turns it into
//! "what happened / why / what to do" in the user's language (decision 106) and adds the log entry id. Values are
//! plain strings so they cross the IPC boundary unchanged; `detail` carries the technical error for the Details view.

use crate::apply::PlanError;
use crate::engine::EngineError;
use crate::import::ImportError;
use crate::journal::TxError;
use crate::library::ModelError;
use crate::place::PlaceError;
use crate::profiles::ProfileError;
use crate::remove::RemoveError;
use crate::settings::SettingsError;
use crate::updates::UpdateError;
use deadloader_archive::{ArchiveError, LimitExceeded};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::io;

/// Ids of spec `04-messages`. `Internal` is a mistake of deadloader itself, never blamed on the user.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageId {
    E1,
    E2,
    E3,
    E4,
    E5,
    E6,
    E7,
    E8,
    E9,
    E10,
    E11,
    E12,
    E13,
    E14,
    E15,
    E16,
    E17,
    E18,
    E19,
    /// More mods enabled than the game has pak slots for.
    E20,
    /// A profile with this name exists.
    E21,
    /// The name can't be used for a profile (Windows file-name rules).
    E22,
    /// One of deadloader's own files (settings, manifest) can't be read.
    E23,
    Internal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserMessage {
    pub id: MessageId,
    pub args: BTreeMap<String, String>,
}

impl UserMessage {
    pub fn new(id: MessageId) -> Self {
        Self {
            id,
            args: BTreeMap::new(),
        }
    }

    pub fn arg(mut self, key: &str, value: impl ToString) -> Self {
        self.args.insert(key.to_owned(), value.to_string());
        self
    }

    fn detail(self, error: &dyn std::fmt::Display) -> Self {
        self.arg("detail", error)
    }
}

pub trait ToUserMessage {
    fn user_message(&self) -> UserMessage;
}

/// Where a file operation failed: it decides what the user can do about it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Area {
    /// `deadloader-data\` next to the exe.
    Data,
    /// The game folder.
    Game,
}

pub fn from_io(e: &io::Error, area: Area) -> UserMessage {
    let os = if cfg!(windows) {
        e.raw_os_error()
    } else {
        None
    };
    if e.kind() == io::ErrorKind::StorageFull || matches!(os, Some(39 | 112)) {
        return UserMessage::new(MessageId::E12).detail(e);
    }
    let reason = if matches!(os, Some(32 | 33)) {
        // Sharing / lock violation: an antivirus, the game or another manager has the file open.
        "in_use"
    } else if matches!(
        e.kind(),
        io::ErrorKind::PermissionDenied | io::ErrorKind::ReadOnlyFilesystem
    ) || matches!(os, Some(5 | 19))
    {
        "access_denied"
    } else {
        "other"
    };
    let id = match area {
        Area::Data => MessageId::E1,
        Area::Game => MessageId::E18,
    };
    UserMessage::new(id).arg("reason", reason).detail(e)
}

/// One of our own JSON files (manifest, profiles folder files, settings) that failed to load.
pub fn data_file(e: &ModelError, file: &str) -> UserMessage {
    let m = UserMessage::new(MessageId::E23).arg("file", file).detail(e);
    match e {
        ModelError::NewerSchema { .. } => m.arg("reason", "newer"),
        ModelError::Json { line, column, .. } => m
            .arg("reason", "damaged")
            .arg("line", line)
            .arg("column", column),
        ModelError::Io(_) => UserMessage::new(MessageId::E1).detail(e),
        _ => m.arg("reason", "invalid"),
    }
}

fn internal(e: &dyn std::fmt::Display) -> UserMessage {
    UserMessage::new(MessageId::Internal).detail(e)
}

impl ToUserMessage for ArchiveError {
    fn user_message(&self) -> UserMessage {
        let unpack = |reason: &str, entry: &Option<String>| {
            let m = UserMessage::new(MessageId::E8)
                .arg("reason", reason)
                .detail(self);
            match entry {
                Some(entry) => m.arg("entry", entry),
                None => m,
            }
        };
        match self {
            Self::NotAnArchive => unpack("not_an_archive", &None),
            Self::Rar => UserMessage::new(MessageId::E11),
            Self::Damaged { entry, .. } => unpack("damaged", entry),
            Self::Encrypted { entry } => unpack("encrypted", entry),
            Self::UnsupportedMethod { entry, .. } => unpack("unsupported", entry),
            Self::Limit(limit) => {
                let m = UserMessage::new(MessageId::E9).detail(self);
                match limit {
                    LimitExceeded::TotalSize { limit, .. } => {
                        m.arg("reason", "too_large").arg("limit", limit)
                    }
                    LimitExceeded::TooManyVpks { limit, found } => m
                        .arg("reason", "too_many_files")
                        .arg("limit", limit)
                        .arg("found", found),
                    LimitExceeded::Ratio { limit, entry } => m
                        .arg("reason", "ratio")
                        .arg("limit", limit)
                        .arg("entry", entry),
                    LimitExceeded::Dictionary { limit, needed } => m
                        .arg("reason", "memory")
                        .arg("limit", limit)
                        .arg("needed", needed),
                }
            }
            Self::InsufficientSpace { needed, available } => UserMessage::new(MessageId::E12)
                .arg("needed", needed)
                .arg("available", available),
            Self::DiskFull => UserMessage::new(MessageId::E12),
            Self::Io(e) => from_io(e, Area::Data),
        }
    }
}

impl ToUserMessage for ImportError {
    fn user_message(&self) -> UserMessage {
        match self {
            Self::Archive(e) => e.user_message(),
            Self::NoModFiles { skipped } => {
                UserMessage::new(MessageId::E10).arg("contents", skipped.join("\n"))
            }
            Self::NotAVpk { name, .. } => UserMessage::new(MessageId::E8)
                .arg("reason", "not_a_vpk")
                .arg("entry", name)
                .detail(self),
            Self::Io(e) => from_io(e, Area::Data),
            Self::InvalidModId(_) | Self::Manifest(_) => internal(self),
        }
    }
}

impl ToUserMessage for RemoveError {
    fn user_message(&self) -> UserMessage {
        match self {
            Self::Io(e) => from_io(e, Area::Data),
            // The UI applies the profile without the files before removing them; reaching this is our bug.
            _ => internal(self),
        }
    }
}

impl ToUserMessage for UpdateError {
    fn user_message(&self) -> UserMessage {
        // D5 builds the mapping from the mod's own files; an invalid one is a mistake of deadloader.
        internal(self)
    }
}

impl ToUserMessage for ProfileError {
    fn user_message(&self) -> UserMessage {
        match self {
            Self::NotAProfile => UserMessage::new(MessageId::E17).arg("reason", "not_a_profile"),
            Self::Model(ModelError::NewerSchema { .. }) => UserMessage::new(MessageId::E17)
                .arg("reason", "newer")
                .detail(self),
            Self::Model(ModelError::Json { .. }) => UserMessage::new(MessageId::E17)
                .arg("reason", "not_a_profile")
                .detail(self),
            Self::Model(ModelError::InvalidName(name)) => {
                UserMessage::new(MessageId::E22).arg("name", name)
            }
            Self::NameTaken(name) => UserMessage::new(MessageId::E21).arg("name", name),
            Self::Io(e) => from_io(e, Area::Data),
            Self::Model(_) | Self::NotFound(_) | Self::LastProfile => internal(self),
        }
    }
}

impl ToUserMessage for EngineError {
    fn user_message(&self) -> UserMessage {
        match self {
            Self::GameRunning => UserMessage::new(MessageId::E13),
            // A profile pointing at something the library lacks (hand-edited, or deleted outside deadloader).
            Self::UnknownMod(_) | Self::NotAFileOfMod { .. } => {
                UserMessage::new(MessageId::E14).detail(self)
            }
            Self::Plan(PlanError::TooManyMods { wanted, free }) => UserMessage::new(MessageId::E20)
                .arg("wanted", wanted)
                .arg("free", free),
            Self::Plan(PlanError::Duplicate(_)) => internal(self),
            Self::Tx(TxError::Journal(e)) => from_io(e, Area::Data),
            Self::Tx(TxError::Step {
                error: PlaceError::HashMismatch { .. },
                ..
            }) => UserMessage::new(MessageId::E14).detail(self),
            Self::Tx(TxError::Step {
                error: PlaceError::Io(e),
                ..
            })
            | Self::Tx(TxError::Cleanup(e))
            | Self::Io(e) => from_io(e, Area::Game),
            Self::Tx(TxError::Invalid) | Self::Meta(_) => internal(self),
        }
    }
}

impl ToUserMessage for SettingsError {
    fn user_message(&self) -> UserMessage {
        let m = UserMessage::new(MessageId::E23)
            .arg("file", "settings.json")
            .detail(self);
        match self {
            Self::NewerSchema { .. } => m.arg("reason", "newer"),
            Self::Json { line, column, .. } => m
                .arg("reason", "damaged")
                .arg("line", line)
                .arg("column", column),
            Self::NotAnObject { .. } | Self::MissingSchema | Self::InvalidSchema => {
                m.arg("reason", "damaged")
            }
            Self::Io(_) => UserMessage::new(MessageId::E1).detail(self),
            // Saving refuses invalid settings; the UI validates before it saves.
            Self::Rejected(_) => internal(self),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::apply::PlanError;
    use crate::engine::EngineError;
    use crate::import::ImportError;
    use crate::journal::TxError;
    use crate::library::ModelError;
    use crate::place::PlaceError;
    use crate::profiles::ProfileError;
    use crate::remove::RemoveError;
    use crate::settings::SettingsError;
    use crate::updates::UpdateError;
    use crate::vpk::VpkError;
    use deadloader_archive::{ArchiveError, LimitExceeded};
    use std::io;

    fn arg<'a>(m: &'a UserMessage, key: &str) -> Option<&'a str> {
        m.args.get(key).map(String::as_str)
    }

    #[test]
    fn a_message_is_an_id_and_named_values_for_the_ui_to_translate() {
        let m = UserMessage::new(MessageId::E12)
            .arg("needed", 10)
            .arg("drive", "D:");
        let json = serde_json::to_value(&m).expect("json");
        assert_eq!(
            json,
            serde_json::json!({"id": "E12", "args": {"needed": "10", "drive": "D:"}})
        );
    }

    #[test]
    fn a_message_reads_back_from_json() {
        // Queued downloads keep their last error across restarts (downloads/queue.json).
        let m = UserMessage::new(MessageId::E7).arg("reason", "checksum");
        let json = serde_json::to_string(&m).expect("json");
        assert_eq!(
            serde_json::from_str::<UserMessage>(&json).expect("parse"),
            m
        );
        assert!(serde_json::from_str::<UserMessage>(r#"{"id":"E99","args":{}}"#).is_err());
    }

    #[test]
    fn file_errors_depend_on_the_folder() {
        let denied = io::Error::from(io::ErrorKind::PermissionDenied);
        assert_eq!(from_io(&denied, Area::Data).id, MessageId::E1);
        let game = from_io(&denied, Area::Game);
        assert_eq!(game.id, MessageId::E18);
        assert_eq!(arg(&game, "reason"), Some("access_denied"));
        assert!(
            arg(&game, "detail").is_some(),
            "details are kept for the log"
        );
    }

    #[test]
    fn a_full_disk_is_said_plainly_wherever_it_happens() {
        for area in [Area::Data, Area::Game] {
            let full = io::Error::from(io::ErrorKind::StorageFull);
            assert_eq!(from_io(&full, area).id, MessageId::E12);
        }
        if cfg!(windows) {
            for code in [39, 112] {
                let e = io::Error::from_raw_os_error(code);
                assert_eq!(from_io(&e, Area::Game).id, MessageId::E12, "{code}");
            }
        }
    }

    #[test]
    fn files_in_use_by_another_program_are_named_as_such() {
        if cfg!(windows) {
            for code in [32, 33] {
                let e = io::Error::from_raw_os_error(code);
                let m = from_io(&e, Area::Game);
                assert_eq!((m.id, arg(&m, "reason")), (MessageId::E18, Some("in_use")));
            }
        }
    }

    #[test]
    fn archive_problems() {
        let cases = [
            (
                ArchiveError::NotAnArchive,
                MessageId::E8,
                Some("not_an_archive"),
            ),
            (ArchiveError::Rar, MessageId::E11, None),
            (
                ArchiveError::Encrypted { entry: None },
                MessageId::E8,
                Some("encrypted"),
            ),
            (
                ArchiveError::Damaged {
                    entry: Some("a.vpk".into()),
                    detail: "crc".into(),
                },
                MessageId::E8,
                Some("damaged"),
            ),
            (
                ArchiveError::UnsupportedMethod {
                    entry: None,
                    method: "ppmd".into(),
                },
                MessageId::E8,
                Some("unsupported"),
            ),
            (
                ArchiveError::Limit(LimitExceeded::TooManyVpks {
                    limit: 256,
                    found: 300,
                }),
                MessageId::E9,
                Some("too_many_files"),
            ),
            (ArchiveError::DiskFull, MessageId::E12, None),
        ];
        for (error, id, reason) in cases {
            let m = error.user_message();
            assert_eq!((m.id, arg(&m, "reason")), (id, reason), "{error:?}");
        }
        let space = ArchiveError::InsufficientSpace {
            needed: 5,
            available: 2,
        }
        .user_message();
        assert_eq!(space.id, MessageId::E12);
        assert_eq!(
            (arg(&space, "needed"), arg(&space, "available")),
            (Some("5"), Some("2"))
        );
    }

    #[test]
    fn import_problems() {
        let none = ImportError::NoModFiles {
            skipped: vec!["README.txt".into(), "preview.png".into()],
        }
        .user_message();
        assert_eq!(none.id, MessageId::E10);
        assert_eq!(arg(&none, "contents"), Some("README.txt\npreview.png"));

        let fake = ImportError::NotAVpk {
            name: "x.vpk".into(),
            error: VpkError::NotAVpk,
        }
        .user_message();
        assert_eq!(
            (fake.id, arg(&fake, "reason"), arg(&fake, "entry")),
            (MessageId::E8, Some("not_a_vpk"), Some("x.vpk"))
        );
        assert_eq!(
            ImportError::Archive(ArchiveError::Rar).user_message().id,
            MessageId::E11
        );
    }

    #[test]
    fn apply_problems() {
        assert_eq!(EngineError::GameRunning.user_message().id, MessageId::E13);
        let many = EngineError::Plan(PlanError::TooManyMods {
            wanted: 120,
            free: 99,
        })
        .user_message();
        assert_eq!(many.id, MessageId::E20);
        assert_eq!(
            (arg(&many, "wanted"), arg(&many, "free")),
            (Some("120"), Some("99"))
        );
        let damaged = EngineError::Tx(TxError::Step {
            index: 0,
            error: PlaceError::HashMismatch {
                expected: [0; 32],
                actual: [1; 32],
            },
        })
        .user_message();
        assert_eq!(damaged.id, MessageId::E14);
        let locked = EngineError::Tx(TxError::Step {
            index: 0,
            error: PlaceError::Io(io::Error::from(io::ErrorKind::PermissionDenied)),
        })
        .user_message();
        assert_eq!(locked.id, MessageId::E18);
        let journal = EngineError::Tx(TxError::Journal(io::Error::from(
            io::ErrorKind::PermissionDenied,
        )))
        .user_message();
        assert_eq!(
            journal.id,
            MessageId::E1,
            "the journal lives in our data folder"
        );
        assert_eq!(
            EngineError::UnknownMod("x".into()).user_message().id,
            MessageId::E14
        );
    }

    #[test]
    fn profile_problems() {
        assert_eq!(ProfileError::NotAProfile.user_message().id, MessageId::E17);
        let newer = ProfileError::Model(ModelError::NewerSchema {
            found: 2,
            supported: 1,
        })
        .user_message();
        assert_eq!(
            (newer.id, arg(&newer, "reason")),
            (MessageId::E17, Some("newer"))
        );
        let taken = ProfileError::NameTaken("Main".into()).user_message();
        assert_eq!(
            (taken.id, arg(&taken, "name")),
            (MessageId::E21, Some("Main"))
        );
        assert_eq!(
            ProfileError::Model(ModelError::InvalidName("CON".into()))
                .user_message()
                .id,
            MessageId::E22
        );
    }

    #[test]
    fn our_own_files_that_cannot_be_read() {
        let s = SettingsError::Json {
            line: 3,
            column: 7,
            message: "x".into(),
        }
        .user_message();
        assert_eq!(s.id, MessageId::E23);
        assert_eq!(
            (arg(&s, "file"), arg(&s, "reason"), arg(&s, "line")),
            (Some("settings.json"), Some("damaged"), Some("3"))
        );
        let m = data_file(
            &ModelError::NewerSchema {
                found: 2,
                supported: 1,
            },
            "manifest.json",
        );
        assert_eq!((m.id, arg(&m, "reason")), (MessageId::E23, Some("newer")));
    }

    #[test]
    fn mistakes_of_deadloader_itself_say_so() {
        // Shown as "something went wrong inside deadloader" with the log entry; never blamed on the user.
        assert_eq!(
            RemoveError::SeveralMods.user_message().id,
            MessageId::Internal
        );
        assert_eq!(
            EngineError::Plan(PlanError::Duplicate("x".into()))
                .user_message()
                .id,
            MessageId::Internal
        );
        // D5 only offers files of the mod; a mapping it can't apply is a bug of the dialog, not the user's.
        let wrong = UpdateError::NotInMod("x".into());
        let m = wrong.user_message();
        assert_eq!(m.id, MessageId::Internal);
        assert_eq!(arg(&m, "detail"), Some(wrong.to_string().as_str()));
    }
}
