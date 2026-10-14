//! HTTP transport: the domain allowlist checked on every redirect hop, the honest User-Agent, retries and the
//! network log. Nothing here writes files; hop records go to a [`log::NetLog`] sink.

pub mod allowlist;
pub mod client;
pub mod clock;
pub mod log;
pub mod transport;
pub mod url;

pub use allowlist::{Allowlist, Denied, HostSource, Initiator, Purpose};
pub use client::{ByteRange, Client, NetError, Request, Response, RetryPolicy, USER_AGENT};
pub use url::{Host, HttpsUrl};
