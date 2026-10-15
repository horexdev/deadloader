//! HTTP transport: the domain allowlist checked on every redirect hop, the honest User-Agent, retries and the
//! network log. Nothing here writes files; hop records go to a [`log::NetLog`] sink.

pub mod allowlist;
pub mod client;
pub mod clock;
mod idle;
pub mod log;
pub mod transport;
pub mod ureq_transport;
pub mod url;

pub use allowlist::{Allowlist, Denied, HostSource, Initiator, Purpose};
pub use client::{ByteRange, Client, NetError, Request, Response, RetryPolicy, USER_AGENT};
pub use ureq_transport::{TransportConfig, UreqTransport};
pub use url::{Host, HttpsUrl};
