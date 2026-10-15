//! What the catalog reports: the network status shown on every screen, and why a call could not be answered.
//!
//! Local mode, offline and "not found" are states the UI shows in place (spec S2/S3, N7, N10), so they carry no
//! message. Real failures carry a [`UserMessage`] from spec `04-messages` (E4, E5, E16, E1, Internal) whose `detail`
//! and `log_id` point at the log line with the technical details (decision 98).

use deadloader_core::message::UserMessage;
use deadloader_gamebanana::SearchTextError;
use serde::Serialize;
use std::fmt;

/// The catalog's connection to GameBanana, as the global network indicator shows it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Status {
    /// The catalog is off: zero network requests (decision 108, N10).
    LocalMode,
    Online,
    /// GameBanana could not be reached since then; saved pages are shown (N7).
    Offline {
        since_unix_ms: i64,
    },
    /// GameBanana answered 429 or a server error; nothing is asked until then (E5).
    RateLimited {
        until_unix_ms: i64,
        message: UserMessage,
    },
    /// GameBanana answered in a way this version does not understand (E4); the rest of the app works.
    ApiChanged {
        message: UserMessage,
    },
}

/// Why a search text is not sent (GameBanana would refuse it); the search field should prevent it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchProblem {
    TooShort,
    TooLong,
}

impl From<SearchTextError> for SearchProblem {
    fn from(e: SearchTextError) -> Self {
        match e {
            SearchTextError::TooShort => Self::TooShort,
            SearchTextError::TooLong => Self::TooLong,
        }
    }
}

/// Why a listing, the categories or a mod page cannot be shown.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum CatalogError {
    /// The catalog is off and nothing is saved to show.
    LocalMode,
    /// GameBanana can't be reached and nothing is saved to show.
    Offline {
        since_unix_ms: i64,
    },
    RateLimited {
        retry_in_s: u64,
        message: UserMessage,
    },
    ApiChanged {
        message: UserMessage,
    },
    /// A request was refused by the allowlist (a redirect to another host, E16).
    Blocked {
        message: UserMessage,
    },
    /// GameBanana has no such submission (deleted, withheld), or it belongs to another game.
    NotFound {
        other_game: Option<u64>,
    },
    InvalidSearch {
        problem: SearchProblem,
    },
    /// A mistake of deadloader itself.
    Internal {
        message: UserMessage,
    },
}

impl CatalogError {
    /// The error to show, if this is one; the states (local mode, offline, not found, a search too short) are shown
    /// in place instead.
    pub fn message(&self) -> Option<&UserMessage> {
        match self {
            Self::RateLimited { message, .. }
            | Self::ApiChanged { message }
            | Self::Blocked { message }
            | Self::Internal { message } => Some(message),
            Self::LocalMode
            | Self::Offline { .. }
            | Self::NotFound { .. }
            | Self::InvalidSearch { .. } => None,
        }
    }
}

impl fmt::Display for CatalogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LocalMode => f.write_str("the catalog is off (local mode)"),
            Self::Offline { .. } => f.write_str("GameBanana can't be reached"),
            Self::RateLimited { retry_in_s, .. } => {
                write!(f, "GameBanana is busy; retrying in {retry_in_s} s")
            }
            Self::ApiChanged { .. } => f.write_str("GameBanana's API changed"),
            Self::Blocked { .. } => f.write_str("a request was blocked by the allowlist"),
            Self::NotFound { .. } => f.write_str("GameBanana has no such submission"),
            Self::InvalidSearch { problem } => {
                write!(f, "the search text is invalid ({problem:?})")
            }
            Self::Internal { .. } => f.write_str("an internal error"),
        }
    }
}

impl std::error::Error for CatalogError {}

/// Why an image can't be served. The image protocol turns these into HTTP status codes; the UI shows E19 and keeps
/// the placeholder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImageError {
    /// Not an id the catalog issued (or one it has forgotten).
    UnknownId,
    /// A description image the user has not clicked or consented to.
    NotActivated,
    LocalMode,
    /// Offline, and the image is not saved.
    Offline,
    /// GameBanana asked us to slow down.
    RateLimited,
    /// The allowlist refused a hop (a redirect to a host that is not allowed, or not https).
    Blocked,
    /// The host could not be reached.
    Unreachable,
    /// The host answered, but not with the image.
    Upstream {
        status: u16,
    },
    NotAnImage,
    TooLarge,
    /// Reading the cache failed.
    Internal,
}

impl fmt::Display for ImageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::UnknownId => "unknown image id",
            Self::NotActivated => "the image was not requested by the user",
            Self::LocalMode => "the catalog is off (local mode)",
            Self::Offline => "offline, and the image is not saved",
            Self::RateLimited => "GameBanana asked to wait",
            Self::Blocked => "the request was blocked by the allowlist",
            Self::Unreachable => "the host could not be reached",
            Self::Upstream { .. } => "the host did not send the image",
            Self::NotAnImage => "not a JPEG, PNG, GIF or WebP image",
            Self::TooLarge => "the image is too large",
            Self::Internal => "the image cache could not be read",
        };
        f.write_str(text)
    }
}

impl std::error::Error for ImageError {}

/// Why a click or a consent on a description image was not taken.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ConsentError {
    UnknownId,
    /// Consent is for description images; thumbnails and the gallery come from GameBanana.
    NotADescriptionImage,
    /// The host is not allowed yet: show dialog D2.
    NeedsConsent {
        host: String,
    },
    /// "Always allow" could not be saved to the settings; nothing was allowed.
    Save {
        message: UserMessage,
    },
}

impl fmt::Display for ConsentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownId => f.write_str("unknown image id"),
            Self::NotADescriptionImage => f.write_str("not a description image"),
            Self::NeedsConsent { host } => write!(f, "images from {host} need the user's consent"),
            Self::Save { .. } => f.write_str("the allowed host could not be saved"),
        }
    }
}

impl std::error::Error for ConsentError {}
