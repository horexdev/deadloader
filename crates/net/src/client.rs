//! The request loop: local mode, allowlist, https-only, redirects, retries and the log record of every hop.
//!
//! The transport never follows redirects. We do, one hop at a time, so every host a request touches is checked
//! against the allowlist for the request's purpose and shows up in the network log — the GameBanana download chain
//! `gamebanana.com/dl/{id}` → `files.gamebanana.com` → `filecacheNN.gamebanana.com` included (ADR 0007).

use crate::allowlist::{Allowlist, Denied, HostSource, Initiator, Purpose};
use crate::clock::Clock;
use crate::log::{BlockReason, HopOutcome, HopRecord, NetLog};
use crate::transport::{Body, HopRequest, Transport, TransportError, find_header};
use crate::url::{Host, HttpsUrl, UrlError};
use std::fmt;
use std::io::{self, Read};
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::{PoisonError, RwLock};
use std::time::Duration;

/// Sent on every request so GameBanana's administrators can see who we are and reach us (decision 56).
pub const USER_AGENT: &str = concat!(
    "deadloader/",
    env!("CARGO_PKG_VERSION"),
    " (+https://github.com/horexdev/deadloader)"
);

/// Redirects followed per request; one more is refused.
pub const MAX_REDIRECTS: u32 = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetryPolicy {
    /// Retries per request after 429/5xx (on top of the first attempt).
    pub max_retries: u32,
    /// First backoff delay; doubled on every further retry.
    pub base_delay: Duration,
    /// Upper bound for one wait. A `Retry-After` longer than this ends the request instead of blocking a worker.
    pub max_delay: Duration,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 5,
            base_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(60),
        }
    }
}

/// Inclusive byte range for `Range: bytes=start-[end]` (resuming downloads).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ByteRange {
    pub start: u64,
    pub end: Option<u64>,
}

impl ByteRange {
    fn header_value(self) -> String {
        match self.end {
            Some(end) => format!("bytes={}-{end}", self.start),
            None => format!("bytes={}-", self.start),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    url: String,
    purpose: Purpose,
    initiator: Initiator,
    range: Option<ByteRange>,
    headers: Vec<(String, String)>,
    consented_image_host: Option<Host>,
}

impl Request {
    pub fn new(purpose: Purpose, initiator: Initiator, url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            purpose,
            initiator,
            range: None,
            headers: Vec::new(),
            consented_image_host: None,
        }
    }

    /// The user consented to load this one description image from `host` without allowing the host for good
    /// (dialog D2 with "always allow" unchecked, decision 122). The host is allowed for this request only, only for
    /// [`Purpose::DescriptionImage`], and only that exact host: a redirect elsewhere is still checked against the
    /// allowlist. It is logged as `user-image`.
    pub fn consented_image_host(mut self, host: Host) -> Self {
        self.consented_image_host = Some(host);
        self
    }

    pub fn range(mut self, range: ByteRange) -> Self {
        self.range = Some(range);
        self
    }

    /// An extra header (e.g. `Accept`). `User-Agent`, `Host` and `Range` are ours and are ignored here.
    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        let name = name.into();
        if !["user-agent", "host", "range"]
            .iter()
            .any(|reserved| name.eq_ignore_ascii_case(reserved))
        {
            self.headers.push((name, value.into()));
        }
        self
    }
}

pub struct Response {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    /// Where the response finally came from, after redirects.
    pub url: HttpsUrl,
    pub host_source: HostSource,
    pub request_id: u64,
    body: Body,
}

impl fmt::Debug for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Response")
            .field("status", &self.status)
            .field("url", &self.url.as_str())
            .field("host_source", &self.host_source)
            .field("request_id", &self.request_id)
            .finish_non_exhaustive()
    }
}

#[derive(Debug)]
pub enum BodyError {
    TooLarge { limit: u64 },
    Io(io::Error),
}

impl fmt::Display for BodyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooLarge { limit } => write!(f, "the response is larger than {limit} bytes"),
            Self::Io(e) => write!(f, "reading the response failed: {e}"),
        }
    }
}

impl std::error::Error for BodyError {}

impl Response {
    pub fn header(&self, name: &str) -> Option<&str> {
        find_header(&self.headers, name)
    }

    /// The body as a stream, for downloads.
    pub fn into_body(self) -> Body {
        self.body
    }

    /// The whole body, refusing anything over `limit` bytes (API responses, images).
    pub fn read_body(self, limit: u64) -> Result<Vec<u8>, BodyError> {
        let mut out = Vec::new();
        self.body
            .take(limit.saturating_add(1))
            .read_to_end(&mut out)
            .map_err(BodyError::Io)?;
        if out.len() as u64 > limit {
            return Err(BodyError::TooLarge { limit });
        }
        Ok(out)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetError {
    /// The user chose local mode (decision 108): nothing is sent.
    LocalMode,
    /// The requested address itself is unusable.
    InvalidUrl {
        url: String,
        reason: UrlError,
    },
    HostNotAllowed {
        host: Host,
        purpose: Purpose,
        /// The host whose redirect led here; `None` for the requested URL itself.
        redirected_from: Option<Host>,
        denied: Denied,
    },
    /// A redirect to `http:` or another non-https scheme.
    InsecureRedirect {
        from: Host,
        location: String,
    },
    /// A redirect without a `Location`, or with one we cannot interpret.
    BadRedirect {
        from: Host,
        location: Option<String>,
        reason: Option<UrlError>,
    },
    TooManyRedirects {
        limit: u32,
        last: Host,
    },
    Transport {
        host: Host,
        error: TransportError,
    },
    /// Still 429/5xx after the allowed retries, or the server asked to wait longer than we block for.
    Unavailable {
        host: Host,
        status: u16,
        attempts: u32,
        retry_after: Option<Duration>,
    },
}

impl fmt::Display for NetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LocalMode => f.write_str(
                "deadloader is in local mode, so it makes no network requests; \
                 turn the catalog on in Settings to go online",
            ),
            Self::InvalidUrl { url, reason } => write!(f, "`{url}` cannot be requested: {reason}"),
            Self::HostNotAllowed {
                host,
                purpose,
                redirected_from: Some(from),
                denied,
            } => write!(
                f,
                "`{from}` redirected the {purpose} to `{host}`, which {denied}"
            ),
            Self::HostNotAllowed {
                host,
                purpose,
                redirected_from: None,
                denied,
            } => write!(f, "`{host}` {denied}, so the {purpose} was not requested"),
            Self::InsecureRedirect { from, location } => write!(
                f,
                "`{from}` redirected to `{location}`, which is not an https address; \
                 deadloader follows only https"
            ),
            Self::BadRedirect {
                from,
                location: None,
                ..
            } => write!(f, "`{from}` answered with a redirect but gave no address"),
            Self::BadRedirect {
                from,
                location: Some(location),
                reason,
            } => {
                write!(f, "`{from}` redirected to an unusable address `{location}`")?;
                match reason {
                    Some(reason) => write!(f, ": {reason}"),
                    None => Ok(()),
                }
            }
            Self::TooManyRedirects { limit, last } => write!(
                f,
                "the request was redirected more than {limit} times (last host `{last}`); stopped to avoid a loop"
            ),
            Self::Transport { host, error } => write!(f, "`{host}`: {error}"),
            Self::Unavailable {
                host,
                status,
                attempts,
                retry_after,
            } => {
                write!(
                    f,
                    "`{host}` answered HTTP {status} after {attempts} attempt(s)"
                )?;
                match retry_after {
                    Some(wait) => write!(
                        f,
                        " and asked to wait {} s; try again later",
                        wait.as_secs()
                    ),
                    None => f.write_str("; try again later"),
                }
            }
        }
    }
}

impl std::error::Error for NetError {}

/// Shared by all network worker threads; settings changes (local mode, allowlist) apply to the next hop.
pub struct Client {
    transport: Box<dyn Transport>,
    log: Box<dyn NetLog>,
    clock: Box<dyn Clock>,
    allowlist: RwLock<Allowlist>,
    local_mode: AtomicBool,
    retry: RetryPolicy,
    /// The retry count of `retry`, changeable while the client is shared (the settings' "retries").
    max_retries: AtomicU32,
    next_id: AtomicU64,
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Client")
            .field("local_mode", &self.local_mode)
            .field("retry", &self.retry)
            .finish_non_exhaustive()
    }
}

impl Client {
    pub fn new(
        transport: impl Transport + 'static,
        log: impl NetLog + 'static,
        clock: impl Clock + 'static,
        allowlist: Allowlist,
    ) -> Self {
        Self {
            transport: Box::new(transport),
            log: Box::new(log),
            clock: Box::new(clock),
            allowlist: RwLock::new(allowlist),
            local_mode: AtomicBool::new(false),
            retry: RetryPolicy::default(),
            max_retries: AtomicU32::new(RetryPolicy::default().max_retries),
            next_id: AtomicU64::new(1),
        }
    }

    pub fn with_retry_policy(mut self, retry: RetryPolicy) -> Self {
        self.max_retries = AtomicU32::new(retry.max_retries);
        self.retry = retry;
        self
    }

    /// Retries per request after 429/5xx from now on; requests already waiting keep their count.
    pub fn set_max_retries(&self, retries: u32) {
        self.max_retries.store(retries, Ordering::SeqCst);
    }

    /// Local mode (decision 108): while on, every request fails with [`NetError::LocalMode`] before reaching the
    /// transport. Checked before every hop, so turning it on also stops requests already in flight at their next hop.
    pub fn set_local_mode(&self, on: bool) {
        self.local_mode.store(on, Ordering::SeqCst);
    }

    pub fn local_mode(&self) -> bool {
        self.local_mode.load(Ordering::SeqCst)
    }

    pub fn allowlist(&self) -> Allowlist {
        self.allowlist
            .read()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }

    pub fn update_allowlist(&self, change: impl FnOnce(&mut Allowlist)) {
        change(
            &mut self
                .allowlist
                .write()
                .unwrap_or_else(PoisonError::into_inner),
        );
    }

    /// Sends a GET and follows redirects, checking local mode, https and the allowlist before every hop.
    ///
    /// Any final status (2xx, 4xx, a non-redirect 3xx such as 304) comes back as a [`Response`]; 429/5xx are retried
    /// per the [`RetryPolicy`] and become [`NetError::Unavailable`] when retries run out.
    pub fn get(&self, request: &Request) -> Result<Response, NetError> {
        let mut trace = Trace {
            log: self.log.as_ref(),
            request,
            id: self.next_id.fetch_add(1, Ordering::Relaxed),
            range: request.range.map(ByteRange::header_value),
            hop: 0,
            attempt: 1,
        };
        if self.local_mode() {
            trace.blocked(&request.url, None, BlockReason::LocalMode);
            return Err(NetError::LocalMode);
        }
        let mut url = match HttpsUrl::parse(&request.url) {
            Ok(url) => url,
            Err(reason) => {
                trace.blocked(&request.url, None, block_reason(&reason));
                return Err(NetError::InvalidUrl {
                    url: request.url.clone(),
                    reason,
                });
            }
        };
        let headers = self.headers(request, trace.range.as_deref());
        let mut retries = 0;
        let mut redirected_from: Option<Host> = None;
        loop {
            if self.local_mode() {
                trace.blocked(url.as_str(), Some(url.host()), BlockReason::LocalMode);
                return Err(NetError::LocalMode);
            }
            let checked = self
                .allowlist
                .read()
                .unwrap_or_else(PoisonError::into_inner)
                .check(url.host(), request.purpose)
                .or_else(|denied| match &request.consented_image_host {
                    Some(host)
                        if denied == Denied::NotListed
                            && request.purpose == Purpose::DescriptionImage
                            && host == url.host() =>
                    {
                        Ok(HostSource::UserImage)
                    }
                    _ => Err(denied),
                });
            let source = match checked {
                Ok(source) => source,
                Err(denied) => {
                    trace.blocked(url.as_str(), Some(url.host()), BlockReason::Host(denied));
                    return Err(NetError::HostNotAllowed {
                        host: url.host().clone(),
                        purpose: request.purpose,
                        redirected_from,
                        denied,
                    });
                }
            };

            let started = self.clock.now();
            let sent = self.transport.send(&HopRequest {
                url: &url,
                purpose: request.purpose,
                headers: &headers,
            });
            let elapsed = self.clock.now().saturating_duration_since(started);
            let response = match sent {
                Ok(response) => response,
                Err(error) => {
                    let outcome = HopOutcome::Failed(error.to_string());
                    trace.emit(
                        url.as_str(),
                        Some(url.host()),
                        Some(source),
                        outcome,
                        elapsed,
                    );
                    return Err(NetError::Transport {
                        host: url.host().clone(),
                        error,
                    });
                }
            };
            let status = response.status;
            let content_length = response
                .header("content-length")
                .and_then(|v| v.trim().parse().ok());
            let outcome =
                |location: Option<String>, retry_in: Option<Duration>| HopOutcome::Response {
                    status,
                    content_length,
                    location,
                    retry_in,
                };

            if is_retryable(status) {
                let retry_after = response.header("retry-after").and_then(parse_retry_after);
                let wait = if retries < self.max_retries.load(Ordering::SeqCst) {
                    match retry_after {
                        Some(wait) if wait <= self.retry.max_delay => Some(wait),
                        Some(_) => None,
                        None => Some(backoff(&self.retry, retries)),
                    }
                } else {
                    None
                };
                trace.emit(
                    url.as_str(),
                    Some(url.host()),
                    Some(source),
                    outcome(None, wait),
                    elapsed,
                );
                let Some(wait) = wait else {
                    return Err(NetError::Unavailable {
                        host: url.host().clone(),
                        status,
                        attempts: trace.attempt,
                        retry_after,
                    });
                };
                drop(response);
                self.clock.sleep(wait);
                retries += 1;
                trace.attempt += 1;
                continue;
            }

            if is_redirect(status) {
                let location = response.header("location").map(str::to_owned);
                trace.emit(
                    url.as_str(),
                    Some(url.host()),
                    Some(source),
                    outcome(location.clone(), None),
                    elapsed,
                );
                drop(response);
                let from = url.host().clone();
                let Some(location) = location else {
                    return Err(NetError::BadRedirect {
                        from,
                        location: None,
                        reason: None,
                    });
                };
                trace.hop += 1;
                trace.attempt = 1;
                let next = match url.join(&location) {
                    Ok(next) => next,
                    Err(reason) => {
                        trace.blocked(&location, None, block_reason(&reason));
                        return Err(match reason {
                            UrlError::NotHttps { .. } => {
                                NetError::InsecureRedirect { from, location }
                            }
                            reason => NetError::BadRedirect {
                                from,
                                location: Some(location),
                                reason: Some(reason),
                            },
                        });
                    }
                };
                if trace.hop > MAX_REDIRECTS {
                    trace.blocked(
                        next.as_str(),
                        Some(next.host()),
                        BlockReason::TooManyRedirects,
                    );
                    return Err(NetError::TooManyRedirects {
                        limit: MAX_REDIRECTS,
                        last: from,
                    });
                }
                redirected_from = Some(from);
                url = next;
                continue;
            }

            trace.emit(
                url.as_str(),
                Some(url.host()),
                Some(source),
                outcome(None, None),
                elapsed,
            );
            return Ok(Response {
                status,
                headers: response.headers,
                url,
                host_source: source,
                request_id: trace.id,
                body: response.body,
            });
        }
    }

    fn headers(&self, request: &Request, range: Option<&str>) -> Vec<(String, String)> {
        let mut headers = vec![("User-Agent".to_owned(), USER_AGENT.to_owned())];
        if let Some(range) = range {
            headers.push(("Range".to_owned(), range.to_owned()));
        }
        headers.extend(request.headers.iter().cloned());
        headers
    }
}

/// Builds the log records of one request; `hop`/`attempt` advance as the loop moves on.
struct Trace<'a> {
    log: &'a dyn NetLog,
    request: &'a Request,
    id: u64,
    range: Option<String>,
    hop: u32,
    attempt: u32,
}

impl Trace<'_> {
    fn emit(
        &self,
        url: &str,
        host: Option<&Host>,
        host_source: Option<HostSource>,
        outcome: HopOutcome,
        elapsed: Duration,
    ) {
        self.log.record(&HopRecord {
            request_id: self.id,
            hop: self.hop,
            attempt: self.attempt,
            purpose: self.request.purpose,
            initiator: self.request.initiator,
            url: url.to_owned(),
            host: host.map(|h| h.as_str().to_owned()),
            host_source,
            range: self.range.clone(),
            outcome,
            elapsed,
        });
    }

    fn blocked(&self, url: &str, host: Option<&Host>, reason: BlockReason) {
        self.emit(url, host, None, HopOutcome::Blocked(reason), Duration::ZERO);
    }
}

fn block_reason(error: &UrlError) -> BlockReason {
    match error {
        UrlError::NotHttps { .. } => BlockReason::NotHttps,
        _ => BlockReason::InvalidUrl,
    }
}

fn is_retryable(status: u16) -> bool {
    status == 429 || (500..=599).contains(&status)
}

fn is_redirect(status: u16) -> bool {
    matches!(status, 301 | 302 | 303 | 307 | 308)
}

/// Only the delta-seconds form; an HTTP-date falls back to our own backoff rather than trusting the local clock.
fn parse_retry_after(value: &str) -> Option<Duration> {
    value.trim().parse::<u64>().ok().map(Duration::from_secs)
}

/// `base_delay × 2^retry`, capped at `max_delay`.
fn backoff(policy: &RetryPolicy, retry: u32) -> Duration {
    1u32.checked_shl(retry)
        .and_then(|factor| policy.base_delay.checked_mul(factor))
        .map_or(policy.max_delay, |delay| delay.min(policy.max_delay))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transport::{HopRequest, HopResponse, TransportErrorKind};
    use std::collections::{HashMap, VecDeque};
    use std::sync::{Arc, Mutex};
    use std::time::Instant;

    struct Reply {
        status: u16,
        headers: Vec<(String, String)>,
        body: Vec<u8>,
    }

    /// URL and headers of one transport call.
    type Call = (String, Vec<(String, String)>);

    #[derive(Default)]
    struct FakeTransport {
        replies: Mutex<HashMap<String, VecDeque<Result<Reply, TransportError>>>>,
        calls: Mutex<Vec<Call>>,
    }

    impl FakeTransport {
        fn on(
            self: &Arc<Self>,
            url: &str,
            status: u16,
            headers: &[(&str, &str)],
            body: &[u8],
        ) -> Arc<Self> {
            let reply = Reply {
                status,
                headers: headers
                    .iter()
                    .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
                    .collect(),
                body: body.to_vec(),
            };
            self.push(url, Ok(reply));
            Arc::clone(self)
        }

        fn redirect(self: &Arc<Self>, url: &str, location: &str) -> Arc<Self> {
            self.on(url, 302, &[("Location", location)], b"")
        }

        fn fail(self: &Arc<Self>, url: &str, error: TransportError) -> Arc<Self> {
            self.push(url, Err(error));
            Arc::clone(self)
        }

        fn push(&self, url: &str, reply: Result<Reply, TransportError>) {
            let mut replies = self.replies.lock().expect("lock");
            replies.entry(url.to_owned()).or_default().push_back(reply);
        }

        fn urls(&self) -> Vec<String> {
            self.calls
                .lock()
                .expect("lock")
                .iter()
                .map(|(u, _)| u.clone())
                .collect()
        }

        fn headers_of_call(&self, i: usize) -> Vec<(String, String)> {
            self.calls.lock().expect("lock")[i].1.clone()
        }
    }

    impl Transport for Arc<FakeTransport> {
        fn send(&self, request: &HopRequest<'_>) -> Result<HopResponse, TransportError> {
            let url = request.url.as_str().to_owned();
            self.calls
                .lock()
                .expect("lock")
                .push((url.clone(), request.headers.to_vec()));
            let next = self
                .replies
                .lock()
                .expect("lock")
                .get_mut(&url)
                .and_then(VecDeque::pop_front);
            match next {
                Some(Ok(reply)) => Ok(HopResponse {
                    status: reply.status,
                    headers: reply.headers,
                    body: Box::new(io::Cursor::new(reply.body)),
                }),
                Some(Err(e)) => Err(e),
                None => Err(TransportError::new(
                    TransportErrorKind::Connect,
                    format!("no reply scripted for {url}"),
                )),
            }
        }
    }

    #[derive(Default)]
    struct MemLog(Mutex<Vec<HopRecord>>);

    impl MemLog {
        fn records(&self) -> Vec<HopRecord> {
            self.0.lock().expect("lock").clone()
        }
    }

    impl NetLog for Arc<MemLog> {
        fn record(&self, record: &HopRecord) {
            self.0.lock().expect("lock").push(record.clone());
        }
    }

    struct FakeClock {
        start: Instant,
        offset: Mutex<Duration>,
        slept: Mutex<Vec<Duration>>,
    }

    impl FakeClock {
        fn new() -> Arc<Self> {
            Arc::new(Self {
                start: Instant::now(),
                offset: Mutex::new(Duration::ZERO),
                slept: Mutex::new(Vec::new()),
            })
        }

        fn slept(&self) -> Vec<Duration> {
            self.slept.lock().expect("lock").clone()
        }
    }

    impl Clock for Arc<FakeClock> {
        fn now(&self) -> Instant {
            self.start + *self.offset.lock().expect("lock")
        }

        fn sleep(&self, duration: Duration) {
            *self.offset.lock().expect("lock") += duration;
            self.slept.lock().expect("lock").push(duration);
        }
    }

    struct Harness {
        transport: Arc<FakeTransport>,
        log: Arc<MemLog>,
        clock: Arc<FakeClock>,
        client: Client,
    }

    fn harness(transport: Arc<FakeTransport>) -> Harness {
        harness_with(transport, Allowlist::new())
    }

    fn harness_with(transport: Arc<FakeTransport>, allowlist: Allowlist) -> Harness {
        let log = Arc::new(MemLog::default());
        let clock = FakeClock::new();
        let client = Client::new(
            Arc::clone(&transport),
            Arc::clone(&log),
            Arc::clone(&clock),
            allowlist,
        );
        Harness {
            transport,
            log,
            clock,
            client,
        }
    }

    fn fake() -> Arc<FakeTransport> {
        Arc::new(FakeTransport::default())
    }

    fn host(s: &str) -> Host {
        Host::parse(s).unwrap_or_else(|e| panic!("{s}: {e}"))
    }

    fn download(url: &str) -> Request {
        Request::new(Purpose::Download, Initiator::User, url)
    }

    const DL: &str = "https://gamebanana.com/dl/1234";
    const FILES: &str = "https://files.gamebanana.com/mods/skin.zip";
    const CACHE: &str = "https://filecache07.gamebanana.com/mods/skin.zip";

    fn gamebanana_chain() -> Arc<FakeTransport> {
        fake().redirect(DL, FILES).redirect(FILES, CACHE).on(
            CACHE,
            200,
            &[("Content-Length", "4")],
            b"PK\x03\x04",
        )
    }

    #[test]
    fn follows_the_gamebanana_download_chain() {
        let h = harness(gamebanana_chain());
        let response = h.client.get(&download(DL)).expect("downloaded");
        assert_eq!(response.status, 200);
        assert_eq!(response.url.as_str(), CACHE);
        assert_eq!(response.host_source, HostSource::BuiltIn);
        assert_eq!(response.read_body(1024).expect("body"), b"PK\x03\x04");
        assert_eq!(h.transport.urls(), [DL, FILES, CACHE]);

        let records = h.log.records();
        assert_eq!(records.len(), 3);
        let hops: Vec<_> = records
            .iter()
            .map(|r| (r.hop, r.url.as_str(), r.host.as_deref()))
            .collect();
        assert_eq!(
            hops,
            [
                (0, DL, Some("gamebanana.com")),
                (1, FILES, Some("files.gamebanana.com")),
                (2, CACHE, Some("filecache07.gamebanana.com")),
            ]
        );
        assert!(
            records
                .iter()
                .all(|r| r.host_source == Some(HostSource::BuiltIn))
        );
        assert!(
            records
                .iter()
                .all(|r| r.purpose == Purpose::Download && r.initiator == Initiator::User)
        );
        assert_eq!(
            records[0].outcome,
            HopOutcome::Response {
                status: 302,
                content_length: None,
                location: Some(FILES.to_owned()),
                retry_in: None,
            }
        );
        assert_eq!(
            records[2].outcome,
            HopOutcome::Response {
                status: 200,
                content_length: Some(4),
                location: None,
                retry_in: None,
            }
        );
    }

    #[test]
    fn every_hop_carries_the_honest_user_agent() {
        let h = harness(gamebanana_chain());
        h.client
            .get(&download(DL).header("User-Agent", "Mozilla/5.0"))
            .expect("downloaded");
        assert_eq!(
            USER_AGENT,
            format!(
                "deadloader/{} (+https://github.com/horexdev/deadloader)",
                env!("CARGO_PKG_VERSION")
            )
        );
        for i in 0..3 {
            let uas: Vec<_> = h
                .transport
                .headers_of_call(i)
                .into_iter()
                .filter(|(k, _)| k.eq_ignore_ascii_case("user-agent"))
                .map(|(_, v)| v)
                .collect();
            assert_eq!(uas, [USER_AGENT], "hop {i}");
        }
    }

    #[test]
    fn the_range_header_is_sent_on_every_hop_and_logged() {
        let h = harness(gamebanana_chain());
        h.client
            .get(&download(DL).range(ByteRange {
                start: 1024,
                end: None,
            }))
            .expect("downloaded");
        for i in 0..3 {
            let headers = h.transport.headers_of_call(i);
            assert!(
                headers.contains(&("Range".to_owned(), "bytes=1024-".to_owned())),
                "hop {i}: {headers:?}"
            );
        }
        assert!(
            h.log
                .records()
                .iter()
                .all(|r| r.range.as_deref() == Some("bytes=1024-"))
        );
        assert_eq!(
            ByteRange {
                start: 0,
                end: Some(99)
            }
            .header_value(),
            "bytes=0-99"
        );
    }

    #[test]
    fn extra_headers_are_passed_through() {
        let h = harness(fake().on("https://gamebanana.com/apiv11/x", 200, &[], b"{}"));
        h.client
            .get(
                &Request::new(
                    Purpose::Catalog,
                    Initiator::Auto,
                    "https://gamebanana.com/apiv11/x",
                )
                .header("Accept", "application/json"),
            )
            .expect("ok");
        assert!(
            h.transport
                .headers_of_call(0)
                .contains(&("Accept".to_owned(), "application/json".to_owned()))
        );
    }

    #[test]
    fn a_relative_location_is_resolved_against_the_current_hop() {
        let h = harness(fake().redirect(DL, "/mods/skin.zip").on(
            "https://gamebanana.com/mods/skin.zip",
            200,
            &[],
            b"x",
        ));
        let response = h.client.get(&download(DL)).expect("ok");
        assert_eq!(
            response.url.as_str(),
            "https://gamebanana.com/mods/skin.zip"
        );
    }

    #[test]
    fn a_redirect_to_a_foreign_host_is_refused_before_contacting_it() {
        let h = harness(
            fake()
                .redirect(DL, FILES)
                .redirect(FILES, "https://cdn.evil.net/skin.zip"),
        );
        let err = h.client.get(&download(DL)).expect_err("foreign host");
        assert_eq!(
            err,
            NetError::HostNotAllowed {
                host: host("cdn.evil.net"),
                purpose: Purpose::Download,
                redirected_from: Some(host("files.gamebanana.com")),
                denied: Denied::NotListed,
            }
        );
        assert_eq!(
            err.to_string(),
            "`files.gamebanana.com` redirected the download to `cdn.evil.net`, which is not on the allowlist"
        );
        assert_eq!(
            h.transport.urls(),
            [DL, FILES],
            "the foreign host is never contacted"
        );

        let last = h.log.records().pop().expect("record of the refused hop");
        assert_eq!(last.hop, 2);
        assert_eq!(last.url, "https://cdn.evil.net/skin.zip");
        assert_eq!(last.host.as_deref(), Some("cdn.evil.net"));
        assert_eq!(last.host_source, None);
        assert_eq!(
            last.outcome,
            HopOutcome::Blocked(BlockReason::Host(Denied::NotListed))
        );
    }

    #[test]
    fn look_alike_redirect_targets_are_refused() {
        for target in [
            "https://gamebanana.com.evil.net/x",
            "https://evilgamebanana.com/x",
            "https://filecache.gamebanana.com/x",
            "https://filecache1234.gamebanana.com/x",
        ] {
            let h = harness(fake().redirect(DL, target));
            let err = h.client.get(&download(DL)).expect_err(target);
            assert!(
                matches!(err, NetError::HostNotAllowed { .. }),
                "{target}: {err:?}"
            );
            assert_eq!(h.transport.urls(), [DL], "{target}");
        }
    }

    #[test]
    fn a_downgrade_to_http_is_refused() {
        let h = harness(fake().redirect(DL, "http://files.gamebanana.com/mods/skin.zip"));
        let err = h.client.get(&download(DL)).expect_err("downgrade");
        assert_eq!(
            err,
            NetError::InsecureRedirect {
                from: host("gamebanana.com"),
                location: "http://files.gamebanana.com/mods/skin.zip".to_owned(),
            }
        );
        assert_eq!(h.transport.urls(), [DL]);
        let last = h.log.records().pop().expect("record");
        assert_eq!(last.hop, 1);
        assert_eq!(last.url, "http://files.gamebanana.com/mods/skin.zip");
        assert_eq!(last.outcome, HopOutcome::Blocked(BlockReason::NotHttps));
    }

    #[test]
    fn a_malformed_or_missing_location_is_an_error() {
        let h = harness(fake().redirect(DL, "https://user@files.gamebanana.com/x"));
        let err = h.client.get(&download(DL)).expect_err("userinfo");
        assert_eq!(
            err,
            NetError::BadRedirect {
                from: host("gamebanana.com"),
                location: Some("https://user@files.gamebanana.com/x".to_owned()),
                reason: Some(UrlError::UserInfo),
            }
        );
        assert_eq!(
            h.log.records().pop().expect("record").outcome,
            HopOutcome::Blocked(BlockReason::InvalidUrl)
        );

        let h = harness(fake().on(DL, 302, &[], b""));
        let err = h.client.get(&download(DL)).expect_err("no location");
        assert_eq!(
            err,
            NetError::BadRedirect {
                from: host("gamebanana.com"),
                location: None,
                reason: None,
            }
        );
    }

    fn chain_of(redirects: u32) -> Arc<FakeTransport> {
        let t = fake();
        for i in 0..redirects {
            t.redirect(
                &format!("https://gamebanana.com/r/{i}"),
                &format!("/r/{}", i + 1),
            );
        }
        t.on(
            &format!("https://gamebanana.com/r/{redirects}"),
            200,
            &[],
            b"done",
        )
    }

    #[test]
    fn five_redirects_are_followed() {
        let h = harness(chain_of(MAX_REDIRECTS));
        let response = h
            .client
            .get(&download("https://gamebanana.com/r/0"))
            .expect("within the limit");
        assert_eq!(response.status, 200);
        assert_eq!(h.transport.urls().len(), 6);
    }

    #[test]
    fn the_sixth_redirect_is_refused() {
        let h = harness(chain_of(MAX_REDIRECTS + 1));
        let err = h
            .client
            .get(&download("https://gamebanana.com/r/0"))
            .expect_err("over the limit");
        assert_eq!(
            err,
            NetError::TooManyRedirects {
                limit: MAX_REDIRECTS,
                last: host("gamebanana.com"),
            }
        );
        assert_eq!(
            h.transport.urls().len(),
            6,
            "the seventh URL is never requested"
        );
        let last = h.log.records().pop().expect("record");
        assert_eq!(last.hop, 6);
        assert_eq!(last.url, "https://gamebanana.com/r/6");
        assert_eq!(
            last.outcome,
            HopOutcome::Blocked(BlockReason::TooManyRedirects)
        );
    }

    #[test]
    fn a_request_to_an_unlisted_host_sends_nothing() {
        let h = harness(fake());
        let err = h
            .client
            .get(&Request::new(
                Purpose::Catalog,
                Initiator::Auto,
                "https://evil.net/api",
            ))
            .expect_err("unlisted");
        assert_eq!(
            err,
            NetError::HostNotAllowed {
                host: host("evil.net"),
                purpose: Purpose::Catalog,
                redirected_from: None,
                denied: Denied::NotListed,
            }
        );
        assert_eq!(
            err.to_string(),
            "`evil.net` is not on the allowlist, so the catalog request was not requested"
        );
        assert!(h.transport.urls().is_empty());
        let records = h.log.records();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].hop, 0);
        assert_eq!(
            records[0].outcome,
            HopOutcome::Blocked(BlockReason::Host(Denied::NotListed))
        );
    }

    #[test]
    fn an_invalid_url_sends_nothing() {
        let h = harness(fake());
        let err = h
            .client
            .get(&download("http://gamebanana.com/dl/1"))
            .expect_err("http");
        assert_eq!(
            err,
            NetError::InvalidUrl {
                url: "http://gamebanana.com/dl/1".to_owned(),
                reason: UrlError::NotHttps {
                    scheme: "http".to_owned()
                },
            }
        );
        assert!(h.transport.urls().is_empty());
        assert_eq!(
            h.log.records()[0].outcome,
            HopOutcome::Blocked(BlockReason::NotHttps)
        );
    }

    #[test]
    fn a_host_consented_for_one_image_is_allowed_for_that_request_only() {
        let img = "https://i.imgur.com/a.png";
        let other = "https://cdn.example.org/b.png";
        let h = harness(fake().on(img, 200, &[], b"png").redirect(img, other));
        let once = || {
            Request::new(Purpose::DescriptionImage, Initiator::User, img)
                .consented_image_host(host("i.imgur.com"))
        };

        let response = h.client.get(&once()).expect("consented for this request");
        assert_eq!(response.host_source, HostSource::UserImage);
        assert_eq!(h.log.records()[0].host_source, Some(HostSource::UserImage));

        // The same image without the consent, and the consent on a request of another purpose, are refused.
        let plain = Request::new(Purpose::DescriptionImage, Initiator::User, img);
        assert!(matches!(
            h.client.get(&plain),
            Err(NetError::HostNotAllowed {
                denied: Denied::NotListed,
                ..
            })
        ));
        let download = download(img).consented_image_host(host("i.imgur.com"));
        assert!(matches!(
            h.client.get(&download),
            Err(NetError::HostNotAllowed {
                purpose: Purpose::Download,
                ..
            })
        ));
        assert_eq!(h.transport.urls(), [img]);

        // The consent covers its host, not a redirect target elsewhere.
        let err = h.client.get(&once()).expect_err("redirect to another host");
        assert!(
            matches!(&err, NetError::HostNotAllowed { host: blocked, .. } if blocked.as_str() == "cdn.example.org"),
            "{err:?}"
        );
        // The client's allowlist is unchanged.
        assert_eq!(h.client.allowlist().image_hosts().count(), 0);
    }

    #[test]
    fn image_hosts_serve_description_images_only() {
        let mut allowlist = Allowlist::new();
        allowlist.add_image_host(host("i.imgur.com"));
        let img = "https://i.imgur.com/a.png";
        let h = harness_with(
            fake().on(img, 200, &[], b"png").redirect(DL, img),
            allowlist,
        );

        let response = h
            .client
            .get(&Request::new(
                Purpose::DescriptionImage,
                Initiator::User,
                img,
            ))
            .expect("consented image host");
        assert_eq!(response.host_source, HostSource::UserImage);
        assert_eq!(h.log.records()[0].host_source, Some(HostSource::UserImage));

        let err = h.client.get(&download(img)).expect_err("not for downloads");
        assert_eq!(
            err,
            NetError::HostNotAllowed {
                host: host("i.imgur.com"),
                purpose: Purpose::Download,
                redirected_from: None,
                denied: Denied::WrongPurpose(HostSource::UserImage),
            }
        );

        let err = h
            .client
            .get(&download(DL))
            .expect_err("not as a download redirect target");
        assert_eq!(
            err.to_string(),
            "`gamebanana.com` redirected the download to `i.imgur.com`, which is allowed only for description images"
        );
        assert_eq!(
            h.transport.urls(),
            [img, DL],
            "the image host is contacted only for the image"
        );
    }

    #[test]
    fn user_hosts_are_logged_as_user() {
        let mut allowlist = Allowlist::new();
        allowlist.add_user_host(host("mirror.example.org"));
        let h = harness_with(
            fake()
                .redirect(DL, "https://mirror.example.org/skin.zip")
                .on("https://mirror.example.org/skin.zip", 200, &[], b"x"),
            allowlist,
        );
        let response = h.client.get(&download(DL)).expect("user host");
        assert_eq!(response.host_source, HostSource::User);
        let sources: Vec<_> = h.log.records().iter().map(|r| r.host_source).collect();
        assert_eq!(sources, [Some(HostSource::BuiltIn), Some(HostSource::User)]);
    }

    #[test]
    fn local_mode_refuses_everything_without_touching_the_transport() {
        let h = harness(gamebanana_chain());
        h.client.set_local_mode(true);
        assert!(h.client.local_mode());
        for purpose in [
            Purpose::Catalog,
            Purpose::Thumbnail,
            Purpose::DescriptionImage,
            Purpose::Download,
            Purpose::UpdateCheck,
        ] {
            let err = h
                .client
                .get(&Request::new(purpose, Initiator::User, DL))
                .expect_err("local mode");
            assert_eq!(err, NetError::LocalMode);
        }
        assert!(h.transport.urls().is_empty());
        assert!(
            h.log
                .records()
                .iter()
                .all(|r| r.outcome == HopOutcome::Blocked(BlockReason::LocalMode))
        );

        h.client.set_local_mode(false);
        h.client.get(&download(DL)).expect("online again");
    }

    #[test]
    fn the_update_check_reaches_github_only_when_enabled() {
        let api = "https://api.github.com/repos/horexdev/deadloader/releases/latest";
        let h = harness(fake().on(api, 200, &[], b"{}"));
        let check = Request::new(Purpose::UpdateCheck, Initiator::Auto, api);
        assert!(matches!(
            h.client.get(&check),
            Err(NetError::HostNotAllowed { .. })
        ));
        assert!(h.transport.urls().is_empty());

        h.client
            .update_allowlist(|list| list.set_update_check(true));
        assert!(h.client.allowlist().update_check());
        let response = h.client.get(&check).expect("enabled");
        assert_eq!(response.host_source, HostSource::GitHub);
    }

    #[test]
    fn server_errors_are_retried_with_exponential_backoff() {
        let url = "https://gamebanana.com/apiv11/x";
        let t = fake();
        for status in [500, 502, 503, 504, 429] {
            t.on(url, status, &[], b"");
        }
        t.on(url, 200, &[], b"ok");
        let h = harness(t);
        let response = h
            .client
            .get(&Request::new(Purpose::Catalog, Initiator::Auto, url))
            .expect("recovers");
        assert_eq!(response.status, 200);
        assert_eq!(h.clock.slept(), [1, 2, 4, 8, 16].map(Duration::from_secs));

        let records = h.log.records();
        assert_eq!(
            records.iter().map(|r| r.attempt).collect::<Vec<_>>(),
            [1, 2, 3, 4, 5, 6]
        );
        assert!(records.iter().all(|r| r.hop == 0));
        assert!(matches!(
            records[0].outcome,
            HopOutcome::Response { status: 500, retry_in: Some(d), .. } if d == Duration::from_secs(1)
        ));
        assert!(matches!(
            records[5].outcome,
            HopOutcome::Response {
                status: 200,
                retry_in: None,
                ..
            }
        ));
    }

    #[test]
    fn retry_after_seconds_is_respected() {
        let url = "https://gamebanana.com/apiv11/x";
        let h = harness(
            fake()
                .on(url, 429, &[("Retry-After", "7")], b"")
                .on(url, 200, &[], b""),
        );
        h.client
            .get(&Request::new(Purpose::Catalog, Initiator::Auto, url))
            .expect("ok");
        assert_eq!(h.clock.slept(), [Duration::from_secs(7)]);
    }

    #[test]
    fn a_retry_after_date_falls_back_to_backoff() {
        let url = "https://gamebanana.com/apiv11/x";
        let h = harness(
            fake()
                .on(
                    url,
                    503,
                    &[("Retry-After", "Wed, 21 Oct 2026 07:28:00 GMT")],
                    b"",
                )
                .on(url, 200, &[], b""),
        );
        h.client
            .get(&Request::new(Purpose::Catalog, Initiator::Auto, url))
            .expect("ok");
        assert_eq!(h.clock.slept(), [Duration::from_secs(1)]);
    }

    #[test]
    fn a_retry_after_beyond_the_limit_ends_the_request_without_waiting() {
        let url = "https://gamebanana.com/apiv11/x";
        let h = harness(fake().on(url, 429, &[("Retry-After", "600")], b""));
        let err = h
            .client
            .get(&Request::new(Purpose::Catalog, Initiator::Auto, url))
            .expect_err("too long to wait");
        assert_eq!(
            err,
            NetError::Unavailable {
                host: host("gamebanana.com"),
                status: 429,
                attempts: 1,
                retry_after: Some(Duration::from_secs(600)),
            }
        );
        assert!(h.clock.slept().is_empty());
    }

    #[test]
    fn retries_give_up_after_the_configured_count() {
        let url = "https://gamebanana.com/apiv11/x";
        let t = fake();
        for _ in 0..10 {
            t.on(url, 503, &[], b"");
        }
        let h = harness(t);
        let err = h
            .client
            .get(&Request::new(Purpose::Catalog, Initiator::Auto, url))
            .expect_err("still failing");
        assert_eq!(
            err,
            NetError::Unavailable {
                host: host("gamebanana.com"),
                status: 503,
                attempts: 6,
                retry_after: None,
            }
        );
        assert_eq!(h.transport.urls().len(), 6);
        assert_eq!(h.clock.slept().len(), 5);
    }

    #[test]
    fn the_retry_policy_is_configurable() {
        let url = "https://gamebanana.com/apiv11/x";
        let t = fake()
            .on(url, 500, &[], b"")
            .on(url, 500, &[], b"")
            .on(url, 200, &[], b"");
        let log = Arc::new(MemLog::default());
        let clock = FakeClock::new();
        let client = Client::new(Arc::clone(&t), log, Arc::clone(&clock), Allowlist::new())
            .with_retry_policy(RetryPolicy {
                max_retries: 1,
                base_delay: Duration::from_millis(250),
                max_delay: Duration::from_secs(1),
            });
        let err = client
            .get(&Request::new(Purpose::Catalog, Initiator::Auto, url))
            .expect_err("one retry only");
        assert!(
            matches!(err, NetError::Unavailable { attempts: 2, .. }),
            "{err:?}"
        );
        assert_eq!(clock.slept(), [Duration::from_millis(250)]);
    }

    #[test]
    fn the_retry_count_can_change_while_the_client_is_shared() {
        let url = "https://gamebanana.com/apiv11/x";
        let t = fake();
        for _ in 0..10 {
            t.on(url, 503, &[], b"");
        }
        let h = harness(t);
        h.client.set_max_retries(1);
        let err = h
            .client
            .get(&Request::new(Purpose::Catalog, Initiator::Auto, url))
            .expect_err("still failing");
        assert!(
            matches!(err, NetError::Unavailable { attempts: 2, .. }),
            "{err:?}"
        );
    }

    #[test]
    fn backoff_is_capped_at_the_maximum_delay() {
        let policy = RetryPolicy {
            max_retries: 10,
            base_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(5),
        };
        let delays: Vec<_> = (0..5).map(|n| backoff(&policy, n)).collect();
        assert_eq!(delays, [1, 2, 4, 5, 5].map(Duration::from_secs));
        assert_eq!(backoff(&policy, 200), Duration::from_secs(5), "no overflow");
    }

    #[test]
    fn client_errors_are_returned_as_responses_without_retrying() {
        let url = "https://gamebanana.com/apiv11/Mod/0";
        let h = harness(fake().on(url, 404, &[], b"not found"));
        let response = h
            .client
            .get(&Request::new(Purpose::Catalog, Initiator::Auto, url))
            .expect("a response");
        assert_eq!(response.status, 404);
        assert!(h.clock.slept().is_empty());
    }

    #[test]
    fn transport_failures_are_reported_and_logged() {
        let h = harness(fake().redirect(DL, FILES).fail(
            FILES,
            TransportError::new(TransportErrorKind::Tls, "bad certificate"),
        ));
        let err = h.client.get(&download(DL)).expect_err("tls");
        assert_eq!(
            err,
            NetError::Transport {
                host: host("files.gamebanana.com"),
                error: TransportError::new(TransportErrorKind::Tls, "bad certificate"),
            }
        );
        let last = h.log.records().pop().expect("record");
        assert_eq!(last.hop, 1);
        assert_eq!(last.host_source, Some(HostSource::BuiltIn));
        assert!(
            matches!(last.outcome, HopOutcome::Failed(ref m) if m.contains("bad certificate")),
            "{:?}",
            last.outcome
        );
    }

    #[test]
    fn each_request_gets_its_own_id() {
        let h = harness(
            gamebanana_chain()
                .redirect(DL, FILES)
                .redirect(FILES, CACHE)
                .on(CACHE, 200, &[], b""),
        );
        let first = h.client.get(&download(DL)).expect("first").request_id;
        let second = h.client.get(&download(DL)).expect("second").request_id;
        assert_ne!(first, second);
        let ids: Vec<_> = h.log.records().iter().map(|r| r.request_id).collect();
        assert_eq!(ids, [first, first, first, second, second, second]);
    }

    #[test]
    fn read_body_enforces_its_limit() {
        let url = "https://gamebanana.com/apiv11/x";
        let h = harness(
            fake()
                .on(url, 200, &[], b"0123456789")
                .on(url, 200, &[], b"0123456789"),
        );
        let request = Request::new(Purpose::Catalog, Initiator::Auto, url);
        assert_eq!(
            h.client
                .get(&request)
                .expect("ok")
                .read_body(10)
                .expect("fits"),
            b"0123456789"
        );
        let err = h
            .client
            .get(&request)
            .expect("ok")
            .read_body(9)
            .expect_err("too large");
        assert!(matches!(err, BodyError::TooLarge { limit: 9 }), "{err:?}");
    }
}
