//! One HTTP hop: send a GET, get back the status, headers and a body stream. Nothing else.
//!
//! Implementations must not follow redirects, retry or add hosts of their own: the redirect loop, the allowlist and
//! retries live in [`crate::client`], where they are testable with a fake transport and visible in the log.

use crate::allowlist::Purpose;
use crate::url::HttpsUrl;
use std::fmt;
use std::io::Read;

pub type Body = Box<dyn Read + Send>;

#[derive(Debug)]
pub struct HopRequest<'a> {
    pub url: &'a HttpsUrl,
    /// Lets the transport pick per-purpose limits (e.g. a total body deadline for small API responses).
    pub purpose: Purpose,
    /// Every header to send, including `User-Agent` and `Range`; the transport adds none of its own that matter.
    pub headers: &'a [(String, String)],
}

pub struct HopResponse {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: Body,
}

impl fmt::Debug for HopResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HopResponse")
            .field("status", &self.status)
            .field("headers", &self.headers)
            .finish_non_exhaustive()
    }
}

impl HopResponse {
    /// First header with this name, compared case-insensitively.
    pub fn header(&self, name: &str) -> Option<&str> {
        find_header(&self.headers, name)
    }
}

pub(crate) fn find_header<'a>(headers: &'a [(String, String)], name: &str) -> Option<&'a str> {
    headers
        .iter()
        .find(|(k, _)| k.eq_ignore_ascii_case(name))
        .map(|(_, v)| v.as_str())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportErrorKind {
    Dns,
    Connect,
    Tls,
    Timeout,
    Io,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransportError {
    pub kind: TransportErrorKind,
    pub message: String,
}

impl TransportError {
    pub fn new(kind: TransportErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }
}

impl fmt::Display for TransportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let what = match self.kind {
            TransportErrorKind::Dns => "the host name could not be resolved",
            TransportErrorKind::Connect => "the connection could not be established",
            TransportErrorKind::Tls => "the secure connection failed",
            TransportErrorKind::Timeout => "the server did not answer in time",
            TransportErrorKind::Io => "the connection broke",
            TransportErrorKind::Other => "the request failed",
        };
        write!(f, "{what} ({})", self.message)
    }
}

impl std::error::Error for TransportError {}

pub trait Transport: Send + Sync {
    fn send(&self, request: &HopRequest<'_>) -> Result<HopResponse, TransportError>;
}
