//! Why a download stopped, and what the user is told (spec `04-messages`: E1, E5, E6, E7, E12, E16).

use crate::task::{Md5, TaskId};
use deadloader_core::message::{Area, MessageId, ToUserMessage, UserMessage, from_io};
use deadloader_net::NetError;
use deadloader_net::transport::TransportErrorKind;
use std::fmt;
use std::io;
use std::time::Duration;

#[derive(Debug)]
pub enum DownloadError {
    /// The request failed: local mode, the allowlist, a redirect, the connection, or 429/5xx after retries.
    Net(NetError),
    /// A final answer other than the file (404, 403, …).
    Status {
        status: u16,
    },
    /// No byte arrived for this long.
    Stalled {
        after: Duration,
    },
    /// The connection broke or ended before the whole file arrived.
    ConnectionLost {
        detail: String,
    },
    /// The server's answers to a resumed request did not fit together twice in a row.
    Inconsistent {
        detail: String,
    },
    Checksum {
        expected: Md5,
        actual: Md5,
    },
    Size {
        expected: u64,
        actual: u64,
    },
    InsufficientSpace {
        needed: u64,
        available: u64,
        drive: Option<String>,
    },
    /// Writing our files in `deadloader-data\downloads\` failed.
    Disk(io::Error),
    /// The completion hook (extraction, import) failed; its message is shown as is.
    Hook(UserMessage),
    InvalidMd5(String),
    UnknownTask(TaskId),
    /// E.g. pausing a task that is being installed.
    InvalidState {
        id: TaskId,
        state: &'static str,
        action: &'static str,
    },
}

/// How a failed attempt continues.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Recovery {
    /// Retried on its own after a delay (at least `min_wait`).
    Automatic { min_wait: Option<Duration> },
    /// Waits until the app says the network is usable again, or the user retries.
    WhenOnline,
    /// The user has to act.
    User,
}

impl DownloadError {
    pub(crate) fn recovery(&self) -> Recovery {
        match self {
            Self::Net(NetError::LocalMode) => Recovery::WhenOnline,
            Self::Net(NetError::Transport { .. })
            | Self::Stalled { .. }
            | Self::ConnectionLost { .. } => Recovery::Automatic { min_wait: None },
            Self::Net(NetError::Unavailable { retry_after, .. }) => Recovery::Automatic {
                min_wait: *retry_after,
            },
            _ => Recovery::User,
        }
    }
}

impl fmt::Display for DownloadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Net(e) => e.fmt(f),
            Self::Status { status } => {
                write!(f, "the server answered HTTP {status} instead of the file")
            }
            Self::Stalled { after } => write!(f, "no data arrived for {} s", after.as_secs()),
            Self::ConnectionLost { detail } => write!(f, "the connection broke: {detail}"),
            Self::Inconsistent { detail } => {
                write!(
                    f,
                    "the server's answers to a resumed download don't fit together: {detail}"
                )
            }
            Self::Checksum { expected, actual } => write!(
                f,
                "the md5 of the downloaded file is {actual}, GameBanana lists {expected}"
            ),
            Self::Size { expected, actual } => write!(
                f,
                "the downloaded file has {actual} bytes, GameBanana lists {expected}"
            ),
            Self::InsufficientSpace {
                needed, available, ..
            } => write!(f, "{needed} bytes are needed, {available} are free"),
            Self::Disk(e) => write!(f, "a download file could not be written: {e}"),
            Self::Hook(m) => write!(f, "installing the download failed ({:?})", m.id),
            Self::InvalidMd5(text) => write!(f, "`{text}` is not an md5 digest"),
            Self::UnknownTask(id) => write!(f, "there is no download {id}"),
            Self::InvalidState { id, state, action } => {
                write!(f, "download {id} is {state}, so it cannot be {action}")
            }
        }
    }
}

impl std::error::Error for DownloadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Net(e) => Some(e),
            Self::Disk(e) => Some(e),
            _ => None,
        }
    }
}

impl From<NetError> for DownloadError {
    fn from(value: NetError) -> Self {
        Self::Net(value)
    }
}

impl ToUserMessage for DownloadError {
    fn user_message(&self) -> UserMessage {
        let e6 = |reason: &str| {
            UserMessage::new(MessageId::E6)
                .arg("reason", reason)
                .arg("detail", self)
        };
        let e16 = |reason: &str| {
            UserMessage::new(MessageId::E16)
                .arg("reason", reason)
                .arg("detail", self)
        };
        match self {
            Self::Net(net) => match net {
                NetError::LocalMode => e6("local_mode"),
                NetError::Transport { host, error } => e6(match error.kind {
                    TransportErrorKind::Dns => "dns",
                    TransportErrorKind::Connect => "connect",
                    TransportErrorKind::Tls => "tls",
                    TransportErrorKind::Timeout => "timeout",
                    TransportErrorKind::Io | TransportErrorKind::Other => "connection_lost",
                })
                .arg("host", host),
                NetError::Unavailable { host, status, .. } => UserMessage::new(MessageId::E5)
                    .arg("status", status)
                    .arg("host", host)
                    .arg("detail", self),
                NetError::HostNotAllowed {
                    host,
                    redirected_from,
                    ..
                } => {
                    let m = e16("not_listed").arg("host", host);
                    match redirected_from {
                        Some(from) => m.arg("from", from),
                        None => m,
                    }
                }
                NetError::InsecureRedirect { from, location } => {
                    e16("not_https").arg("from", from).arg("location", location)
                }
                NetError::BadRedirect { from, .. } => e16("bad_redirect").arg("from", from),
                NetError::TooManyRedirects { last, .. } => {
                    e16("too_many_redirects").arg("from", last)
                }
                // The caller passed an address that can't be requested: our mistake, not the user's.
                NetError::InvalidUrl { .. } => {
                    UserMessage::new(MessageId::Internal).arg("detail", self)
                }
            },
            Self::Status { status } => e6("http_status").arg("status", status),
            Self::Stalled { .. } => e6("timeout"),
            Self::ConnectionLost { .. } | Self::Inconsistent { .. } => e6("connection_lost"),
            Self::Checksum { expected, actual } => UserMessage::new(MessageId::E7)
                .arg("reason", "checksum")
                .arg("expected", expected)
                .arg("actual", actual),
            Self::Size { expected, actual } => UserMessage::new(MessageId::E7)
                .arg("reason", "size")
                .arg("expected", expected)
                .arg("actual", actual),
            Self::InsufficientSpace {
                needed,
                available,
                drive,
            } => {
                let m = UserMessage::new(MessageId::E12)
                    .arg("needed", needed)
                    .arg("available", available);
                match drive {
                    Some(drive) => m.arg("drive", drive),
                    None => m,
                }
            }
            Self::Disk(e) => from_io(e, Area::Data),
            Self::Hook(m) => m.clone(),
            Self::InvalidMd5(_) | Self::UnknownTask(_) | Self::InvalidState { .. } => {
                UserMessage::new(MessageId::Internal).arg("detail", self)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use deadloader_net::allowlist::{Denied, Purpose};
    use deadloader_net::transport::TransportError;
    use deadloader_net::url::Host;

    fn host(name: &str) -> Host {
        deadloader_net::HttpsUrl::parse(&format!("https://{name}/"))
            .expect("url")
            .host()
            .clone()
    }

    fn arg<'a>(m: &'a UserMessage, key: &str) -> Option<&'a str> {
        m.args.get(key).map(String::as_str)
    }

    fn transport(kind: TransportErrorKind) -> DownloadError {
        DownloadError::Net(NetError::Transport {
            host: host("filecache07.gamebanana.com"),
            error: TransportError::new(kind, "x"),
        })
    }

    #[test]
    fn offline_and_timeouts_are_e6_and_resume_on_their_own() {
        for (kind, reason) in [
            (TransportErrorKind::Dns, "dns"),
            (TransportErrorKind::Connect, "connect"),
            (TransportErrorKind::Timeout, "timeout"),
            (TransportErrorKind::Io, "connection_lost"),
        ] {
            let e = transport(kind);
            let m = e.user_message();
            assert_eq!((m.id, arg(&m, "reason")), (MessageId::E6, Some(reason)));
            assert_eq!(e.recovery(), Recovery::Automatic { min_wait: None });
        }
        for e in [
            DownloadError::Stalled {
                after: Duration::from_secs(30),
            },
            DownloadError::ConnectionLost {
                detail: "reset".into(),
            },
        ] {
            assert_eq!(e.user_message().id, MessageId::E6);
            assert_eq!(e.recovery(), Recovery::Automatic { min_wait: None });
        }
    }

    #[test]
    fn local_mode_waits_for_the_network() {
        let e = DownloadError::Net(NetError::LocalMode);
        assert_eq!(e.recovery(), Recovery::WhenOnline);
        assert_eq!(arg(&e.user_message(), "reason"), Some("local_mode"));
    }

    #[test]
    fn a_busy_server_is_e5_and_honours_retry_after() {
        let e = DownloadError::Net(NetError::Unavailable {
            host: host("gamebanana.com"),
            status: 429,
            attempts: 6,
            retry_after: Some(Duration::from_secs(120)),
        });
        let m = e.user_message();
        assert_eq!((m.id, arg(&m, "status")), (MessageId::E5, Some("429")));
        assert_eq!(
            e.recovery(),
            Recovery::Automatic {
                min_wait: Some(Duration::from_secs(120))
            }
        );
    }

    #[test]
    fn blocked_hosts_are_e16() {
        let blocked = DownloadError::Net(NetError::HostNotAllowed {
            host: host("evil.example"),
            purpose: Purpose::Download,
            redirected_from: Some(host("files.gamebanana.com")),
            denied: Denied::NotListed,
        });
        let m = blocked.user_message();
        assert_eq!(m.id, MessageId::E16);
        assert_eq!(arg(&m, "host"), Some("evil.example"));
        assert_eq!(arg(&m, "from"), Some("files.gamebanana.com"));
        assert_eq!(blocked.recovery(), Recovery::User);
        let insecure = DownloadError::Net(NetError::InsecureRedirect {
            from: host("files.gamebanana.com"),
            location: "http://x/".into(),
        });
        assert_eq!(arg(&insecure.user_message(), "reason"), Some("not_https"));
        let looped = DownloadError::Net(NetError::TooManyRedirects {
            limit: 5,
            last: host("gamebanana.com"),
        });
        assert_eq!(looped.user_message().id, MessageId::E16);
    }

    #[test]
    fn damaged_files_are_e7() {
        let sum = DownloadError::Checksum {
            expected: Md5([0; 16]),
            actual: Md5([1; 16]),
        };
        let m = sum.user_message();
        assert_eq!((m.id, arg(&m, "reason")), (MessageId::E7, Some("checksum")));
        assert_eq!(arg(&m, "expected"), Some("0".repeat(32).as_str()));
        assert_eq!(sum.recovery(), Recovery::User);
        let size = DownloadError::Size {
            expected: 10,
            actual: 9,
        }
        .user_message();
        assert_eq!(
            (size.id, arg(&size, "reason")),
            (MessageId::E7, Some("size"))
        );
    }

    #[test]
    fn disk_problems_are_e12_or_e1() {
        let space = DownloadError::InsufficientSpace {
            needed: 100,
            available: 5,
            drive: Some("D:".into()),
        }
        .user_message();
        assert_eq!(space.id, MessageId::E12);
        assert_eq!(
            (
                arg(&space, "needed"),
                arg(&space, "available"),
                arg(&space, "drive")
            ),
            (Some("100"), Some("5"), Some("D:"))
        );
        let full = DownloadError::Disk(io::Error::from(io::ErrorKind::StorageFull));
        assert_eq!(full.user_message().id, MessageId::E12);
        let denied = DownloadError::Disk(io::Error::from(io::ErrorKind::PermissionDenied));
        assert_eq!(denied.user_message().id, MessageId::E1);
        assert_eq!(denied.recovery(), Recovery::User);
    }

    #[test]
    fn hook_messages_pass_through() {
        let m = UserMessage::new(MessageId::E8).arg("reason", "damaged");
        assert_eq!(DownloadError::Hook(m.clone()).user_message(), m);
    }

    #[test]
    fn caller_mistakes_are_internal() {
        assert_eq!(
            DownloadError::InvalidMd5("x".into()).user_message().id,
            MessageId::Internal
        );
        assert_eq!(
            DownloadError::UnknownTask(TaskId(3)).user_message().id,
            MessageId::Internal
        );
    }
}
