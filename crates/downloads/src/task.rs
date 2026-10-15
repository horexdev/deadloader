//! What a download is, as callers and the UI see it.

use deadloader_core::message::UserMessage;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error as _};
use std::collections::BTreeMap;
use std::fmt;

/// Unique per data folder, never reused: also the name of the task's files (`<id>.part`, `<id>.zip`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TaskId(pub u64);

impl fmt::Display for TaskId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// An md5 digest, written as 32 lowercase hex digits.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Md5(pub [u8; 16]);

impl Md5 {
    /// Upper or lower case; surrounding spaces are ignored.
    pub fn parse(text: &str) -> Option<Self> {
        let text = text.trim();
        if text.len() != 32 || !text.is_ascii() {
            return None;
        }
        let mut out = [0u8; 16];
        for (i, byte) in out.iter_mut().enumerate() {
            *byte = u8::from_str_radix(&text[i * 2..i * 2 + 2], 16).ok()?;
        }
        Some(Self(out))
    }
}

impl fmt::Display for Md5 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.iter().try_for_each(|b| write!(f, "{b:02x}"))
    }
}

impl fmt::Debug for Md5 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Md5({self})")
    }
}

impl Serialize for Md5 {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for Md5 {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let text = String::deserialize(d)?;
        Self::parse(&text).ok_or_else(|| D::Error::custom("expected 32 hex digits"))
    }
}

/// Whether the user asked for the download, or deadloader started it (automatic updates); shown in the network log.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Initiator {
    #[default]
    User,
    Auto,
}

impl From<Initiator> for deadloader_net::Initiator {
    fn from(value: Initiator) -> Self {
        match value {
            Initiator::User => Self::User,
            Initiator::Auto => Self::Auto,
        }
    }
}

/// What the caller asks for. The caller knows GameBanana; this crate only knows a URL, a size and a digest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewTask {
    pub url: String,
    /// The name GameBanana shows; used for display and for the kept archive's extension, never as a path.
    pub file_name: String,
    /// GameBanana's `_sMd5Checksum` as hex. Checked when present (decision 46).
    pub md5: Option<String>,
    /// The listed size; checked when present, and used for the free-space check before the first byte.
    pub size: Option<u64>,
    pub initiator: Initiator,
    /// Opaque values for the completion hook (mod id, file id, …), kept across restarts.
    pub meta: BTreeMap<String, String>,
}

impl NewTask {
    pub fn new(url: impl Into<String>, file_name: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            file_name: file_name.into(),
            md5: None,
            size: None,
            initiator: Initiator::User,
            meta: BTreeMap::new(),
        }
    }

    pub fn md5(mut self, hex: impl Into<String>) -> Self {
        self.md5 = Some(hex.into());
        self
    }

    pub fn size(mut self, bytes: u64) -> Self {
        self.size = Some(bytes);
        self
    }

    pub fn initiator(mut self, initiator: Initiator) -> Self {
        self.initiator = initiator;
        self
    }

    pub fn meta(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.meta.insert(key.into(), value.into());
        self
    }
}

/// A task's fixed data, as given to the completion hook.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TaskInfo {
    pub id: TaskId,
    pub url: String,
    pub file_name: String,
    pub md5: Option<Md5>,
    pub size: Option<u64>,
    pub initiator: Initiator,
    pub meta: BTreeMap<String, String>,
}

/// Where a task is. Transitions are reported as [`Event::State`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "state", rename_all = "kebab-case")]
pub enum TaskState {
    /// Waiting for a free download slot.
    Queued,
    Downloading,
    /// Checking the size and md5.
    Verifying,
    /// The completion hook runs (extraction and import, in the app).
    Installing,
    Paused,
    /// A temporary problem (offline, timeout, server busy): retried automatically after `retry_in_s`, or when the
    /// app reports the network is back. `None` = only on request (local mode).
    Waiting {
        message: UserMessage,
        retry_in_s: Option<u64>,
    },
    /// Needs the user: Retry starts it again.
    Failed {
        message: UserMessage,
    },
    /// Installed; the task is removed right after.
    Done,
}

impl TaskState {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Queued => "queued",
            Self::Downloading => "downloading",
            Self::Verifying => "verifying",
            Self::Installing => "installing",
            Self::Paused => "paused",
            Self::Waiting { .. } => "waiting",
            Self::Failed { .. } => "failed",
            Self::Done => "done",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TaskSnapshot {
    #[serde(flatten)]
    pub info: TaskInfo,
    pub state: TaskState,
    /// Bytes of the file on disk so far.
    pub bytes: u64,
    /// The full size, once known (from the listing or the server).
    pub total: Option<u64>,
    /// Smoothed bytes per second; 0 when not downloading.
    pub speed: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "event", rename_all = "kebab-case")]
pub enum Event {
    Added {
        task: TaskSnapshot,
    },
    State {
        id: TaskId,
        state: TaskState,
    },
    /// At most about ten a second per task, plus one when a transfer stops.
    Progress {
        id: TaskId,
        bytes: u64,
        total: Option<u64>,
        speed: u64,
    },
    /// Done or cancelled: the task is gone from the queue.
    Removed {
        id: TaskId,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn md5_hex_in_any_case() {
        let m = Md5::parse(" D41D8CD98F00B204E9800998ECF8427E ").expect("md5");
        assert_eq!(m.to_string(), "d41d8cd98f00b204e9800998ecf8427e");
        assert_eq!(
            serde_json::to_string(&m).expect("json"),
            "\"d41d8cd98f00b204e9800998ecf8427e\""
        );
        let back: Md5 =
            serde_json::from_str("\"d41d8cd98f00b204e9800998ecf8427e\"").expect("parse");
        assert_eq!(back, m);
    }

    #[test]
    fn bad_md5_is_refused() {
        for bad in ["", "abc", &"g".repeat(32), &"é".repeat(16), &"a".repeat(33)] {
            assert_eq!(Md5::parse(bad), None, "{bad}");
        }
    }

    #[test]
    fn states_serialise_with_a_tag_for_the_ui() {
        let json = serde_json::to_value(TaskState::Waiting {
            message: UserMessage::new(deadloader_core::message::MessageId::E6),
            retry_in_s: Some(5),
        })
        .expect("json");
        assert_eq!(json["state"], "waiting");
        assert_eq!(json["retry_in_s"], 5);
        assert_eq!(json["message"]["id"], "E6");
    }
}
