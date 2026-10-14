//! What the network layer reports: one [`HopRecord`] per hop attempt, including hops that were refused before any
//! byte left the machine. Writing records to `network-*.jsonl` and showing them in the UI is the job of the sink.

use crate::allowlist::{Denied, HostSource, Initiator, Purpose};
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HopRecord {
    /// Groups the hops of one logical request.
    pub request_id: u64,
    /// 0 = the requested URL, 1.. = redirect targets.
    pub hop: u32,
    /// 1 = first try of this hop, 2.. = retries after 429/5xx.
    pub attempt: u32,
    pub purpose: Purpose,
    pub initiator: Initiator,
    /// The URL as sent, or the raw value that could not be turned into an acceptable URL.
    pub url: String,
    pub host: Option<String>,
    /// Which allowlist entry let the hop through; `None` when it was refused.
    pub host_source: Option<HostSource>,
    /// The `Range` header value, if any.
    pub range: Option<String>,
    pub outcome: HopOutcome,
    /// Time until the response headers arrived; zero for refused hops.
    pub elapsed: Duration,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HopOutcome {
    Response {
        status: u16,
        content_length: Option<u64>,
        location: Option<String>,
        /// Set when this response will be retried after the given wait.
        retry_in: Option<Duration>,
    },
    /// Refused without contacting the host.
    Blocked(BlockReason),
    /// The transport failed (DNS, connect, TLS, timeout…).
    Failed(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockReason {
    LocalMode,
    Host(Denied),
    NotHttps,
    InvalidUrl,
    TooManyRedirects,
}

impl BlockReason {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::LocalMode => "local-mode",
            Self::Host(Denied::NotListed) => "host-not-listed",
            Self::Host(Denied::WrongPurpose(_)) => "host-wrong-purpose",
            Self::NotHttps => "not-https",
            Self::InvalidUrl => "invalid-url",
            Self::TooManyRedirects => "too-many-redirects",
        }
    }
}

pub trait NetLog: Send + Sync {
    fn record(&self, record: &HopRecord);
}
