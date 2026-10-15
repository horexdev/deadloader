//! Fakes shared by the tests: a clock that only moves when slept on, temp folders, an HTTP server in memory.

use deadloader_net::clock::Clock;
use deadloader_net::log::{HopRecord, NetLog};
use deadloader_net::transport::{
    HopRequest, HopResponse, Transport, TransportError, TransportErrorKind,
};
use std::collections::{HashMap, VecDeque};
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};

/// Time moves only through `sleep` and `advance`; every sleep is recorded. Clones share the same time.
#[derive(Debug, Clone)]
pub struct FakeClock(Arc<ClockInner>);

#[derive(Debug)]
pub struct ClockInner {
    start: Instant,
    offset: Mutex<Duration>,
    sleeps: Mutex<Vec<Duration>>,
}

impl FakeClock {
    pub fn new() -> Self {
        Self(Arc::new(ClockInner {
            start: Instant::now(),
            offset: Mutex::new(Duration::ZERO),
            sleeps: Mutex::new(Vec::new()),
        }))
    }

    pub fn shared(&self) -> Arc<dyn Clock> {
        Arc::new(self.clone())
    }

    pub fn elapsed(&self) -> Duration {
        *self.0.offset.lock().expect("lock")
    }

    pub fn advance(&self, by: Duration) {
        *self.0.offset.lock().expect("lock") += by;
    }

    pub fn sleeps(&self) -> Vec<Duration> {
        self.0.sleeps.lock().expect("lock").clone()
    }
}

impl Clock for FakeClock {
    fn now(&self) -> Instant {
        self.0.start + self.elapsed()
    }

    fn sleep(&self, duration: Duration) {
        self.advance(duration);
        self.0.sleeps.lock().expect("lock").push(duration);
    }
}

/// A fresh folder under the system temp folder, removed on drop.
#[derive(Debug)]
pub struct TempDir(PathBuf);

impl TempDir {
    pub fn new(name: &str) -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let dir = std::env::temp_dir().join(format!(
            "deadloader-downloads-{name}-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("create temp dir");
        Self(dir)
    }

    pub fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// Blocks a response body at a byte until opened (or broken), to hold a download mid-transfer.
#[derive(Debug, Default)]
pub struct Gate {
    state: Mutex<GateState>,
    changed: Condvar,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum GateState {
    #[default]
    Closed,
    Open,
    Broken,
}

impl Gate {
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    pub fn open(&self) {
        *self.state.lock().expect("lock") = GateState::Open;
        self.changed.notify_all();
    }

    /// The held connection fails (like a reset), so an abandoned reader ends.
    pub fn break_off(&self) {
        *self.state.lock().expect("lock") = GateState::Broken;
        self.changed.notify_all();
    }

    pub(crate) fn pass(&self) -> io::Result<()> {
        let mut state = self.state.lock().expect("lock");
        loop {
            match *state {
                GateState::Open => return Ok(()),
                GateState::Broken => return Err(io::Error::from(io::ErrorKind::ConnectionReset)),
                GateState::Closed => state = self.changed.wait(state).expect("lock"),
            }
        }
    }
}

/// What goes wrong with one response.
#[derive(Debug, Clone)]
pub enum Fault {
    /// The body fails with a connection reset after this many bytes.
    CutAfter(usize),
    /// The body waits at this many bytes until the gate opens.
    HoldAt(usize, Arc<Gate>),
    /// The connection takes this long to close once the body is dropped.
    CloseSlowly(Duration),
    /// The whole response is this status, with no body.
    Status(u16),
    /// A redirect to this address.
    Redirect(String),
    /// The transport fails before any response.
    Fail(TransportErrorKind),
}

#[derive(Debug, Clone)]
pub struct Served {
    pub content: Arc<Vec<u8>>,
    pub etag: Option<String>,
    /// Answers `Range` with 206.
    pub ranges: bool,
    /// Answers 200 when `If-Range` names another version.
    pub if_range: bool,
}

impl Served {
    pub fn new(content: Vec<u8>, etag: &str) -> Self {
        Self {
            content: Arc::new(content),
            etag: Some(format!("\"{etag}\"")),
            ranges: true,
            if_range: true,
        }
    }
}

/// One request the server saw.
#[derive(Debug, Clone)]
pub struct Call {
    pub url: String,
    pub headers: Vec<(String, String)>,
}

impl Call {
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(name))
            .map(|(_, v)| v.as_str())
    }
}

/// An HTTP server in memory: files by URL, per-request faults, and a count of bodies open at once.
#[derive(Debug, Default)]
pub struct FakeServer {
    files: Mutex<HashMap<String, Served>>,
    faults: Mutex<HashMap<String, VecDeque<Fault>>>,
    calls: Mutex<Vec<Call>>,
    open_bodies: Arc<AtomicUsize>,
    most_open: Arc<AtomicUsize>,
}

impl FakeServer {
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    pub fn serve(&self, url: &str, served: Served) {
        self.files
            .lock()
            .expect("lock")
            .insert(url.to_owned(), served);
    }

    /// Faults apply to the next requests of `url`, one each, in order.
    pub fn fault(&self, url: &str, fault: Fault) {
        self.faults
            .lock()
            .expect("lock")
            .entry(url.to_owned())
            .or_default()
            .push_back(fault);
    }

    pub fn calls(&self) -> Vec<Call> {
        self.calls.lock().expect("lock").clone()
    }

    pub fn most_open(&self) -> usize {
        self.most_open.load(Ordering::SeqCst)
    }

    fn respond(
        &self,
        url: &str,
        headers: &[(String, String)],
    ) -> Result<HopResponse, TransportError> {
        let call = Call {
            url: url.to_owned(),
            headers: headers.to_vec(),
        };
        self.calls.lock().expect("lock").push(call.clone());
        let fault = self
            .faults
            .lock()
            .expect("lock")
            .get_mut(url)
            .and_then(VecDeque::pop_front);
        let reply = |status: u16, headers: Vec<(String, String)>| HopResponse {
            status,
            headers,
            body: Box::new(io::empty()),
        };
        match &fault {
            Some(Fault::Status(status)) => return Ok(reply(*status, Vec::new())),
            Some(Fault::Redirect(to)) => {
                return Ok(reply(302, vec![("Location".into(), to.clone())]));
            }
            Some(Fault::Fail(kind)) => {
                return Err(TransportError::new(*kind, "scripted failure"));
            }
            _ => {}
        }
        let Some(served) = self.files.lock().expect("lock").get(url).cloned() else {
            return Ok(reply(404, Vec::new()));
        };
        let len = served.content.len();
        let range_start = call
            .header("range")
            .and_then(|r| r.strip_prefix("bytes="))
            .and_then(|r| r.strip_suffix('-'))
            .and_then(|n| n.parse::<usize>().ok());
        let same_version = match (call.header("if-range"), &served.etag) {
            (None, _) => true,
            (Some(asked), Some(etag)) => asked == etag,
            (Some(_), None) => false,
        };
        let partial = served.ranges && (same_version || !served.if_range);
        let mut out = Vec::new();
        if let Some(etag) = &served.etag {
            out.push(("ETag".to_owned(), etag.clone()));
        }
        let (status, start) = match range_start {
            Some(start) if partial && start >= len => return Ok(reply(416, out)),
            Some(start) if partial => {
                out.push((
                    "Content-Range".to_owned(),
                    format!("bytes {start}-{}/{len}", len - 1),
                ));
                (206, start)
            }
            _ => (200, 0),
        };
        out.push(("Content-Length".to_owned(), (len - start).to_string()));
        let open = self.open_bodies.fetch_add(1, Ordering::SeqCst) + 1;
        self.most_open.fetch_max(open, Ordering::SeqCst);
        Ok(HopResponse {
            status,
            headers: out,
            body: Box::new(Body {
                content: served.content,
                pos: start,
                sent: 0,
                fault,
                open: Arc::clone(&self.open_bodies),
            }),
        })
    }
}

#[derive(Debug, Clone)]
pub struct ServerHandle(pub Arc<FakeServer>);

impl Transport for ServerHandle {
    fn send(&self, request: &HopRequest<'_>) -> Result<HopResponse, TransportError> {
        self.0.respond(request.url.as_str(), request.headers)
    }
}

struct Body {
    content: Arc<Vec<u8>>,
    pos: usize,
    /// Bytes of this response sent so far.
    sent: usize,
    fault: Option<Fault>,
    open: Arc<AtomicUsize>,
}

impl Read for Body {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut limit = self.content.len() - self.pos;
        match &self.fault {
            Some(Fault::CutAfter(at)) => {
                if self.sent >= *at {
                    return Err(io::Error::from(io::ErrorKind::ConnectionReset));
                }
                limit = limit.min(at - self.sent);
            }
            Some(Fault::HoldAt(at, gate)) => {
                if self.sent >= *at {
                    gate.pass()?;
                } else {
                    limit = limit.min(at - self.sent);
                }
            }
            _ => {}
        }
        let n = buf.len().min(limit);
        buf[..n].copy_from_slice(&self.content[self.pos..self.pos + n]);
        self.pos += n;
        self.sent += n;
        Ok(n)
    }
}

impl Drop for Body {
    fn drop(&mut self) {
        if let Some(Fault::CloseSlowly(after)) = &self.fault {
            std::thread::sleep(*after);
        }
        self.open.fetch_sub(1, Ordering::SeqCst);
    }
}

/// Every network log record.
#[derive(Debug, Default, Clone)]
pub struct NetRecords(pub Arc<Mutex<Vec<HopRecord>>>);

impl NetLog for NetRecords {
    fn record(&self, record: &HopRecord) {
        self.0.lock().expect("lock").push(record.clone());
    }
}

/// Deterministic, incompressible-looking bytes.
pub fn content(len: usize, seed: u32) -> Vec<u8> {
    let mut x = seed.wrapping_mul(2_654_435_761).wrapping_add(1);
    (0..len)
        .map(|_| {
            x ^= x << 13;
            x ^= x >> 17;
            x ^= x << 5;
            (x >> 24) as u8
        })
        .collect()
}

pub fn md5_hex(bytes: &[u8]) -> String {
    use md5::Digest;
    md5::Md5::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}
