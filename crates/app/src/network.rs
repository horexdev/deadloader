//! The one network client the catalog, downloads and update checks share: built from the settings, changed with them,
//! and logged hop by hop into `network-*.jsonl` (ADR 0007, 0009).

use deadloader_core::log::{self as core_log, LogId, NetworkEvent};
use deadloader_core::settings::{AppUpdateCheck, Proxy, Settings};
use deadloader_net::log::{HopOutcome, HopRecord, NetLog};
use deadloader_net::transport::{
    HopRequest, HopResponse, Transport, TransportError, TransportErrorKind,
};
use deadloader_net::{
    Client, Host, HostSource, Initiator, Purpose, TransportConfig, UreqTransport,
};
use deadloader_platform_windows::proxy::SystemProxy;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex, MutexGuard, PoisonError, RwLock};
use std::time::Duration;

/// Where network lines go: the core's logger, or a recorder in tests.
pub trait NetworkLog: Send + Sync {
    fn request_id(&self) -> LogId;
    fn network(&self, msg: &str, event: &NetworkEvent);
}

impl<C, D> NetworkLog for core_log::Logger<C, D>
where
    C: core_log::Clock + Send + Sync,
    D: core_log::LogDir + Send + Sync,
{
    fn request_id(&self) -> LogId {
        core_log::Logger::request_id(self)
    }

    fn network(&self, msg: &str, event: &NetworkEvent) {
        // A failed log write has nowhere else to go; the request itself goes on.
        let _ = core_log::Logger::network(self, msg, event);
    }
}

/// Request ids remembered at most: hops of one request follow each other closely, so a few hundred cover every
/// request in flight.
const KNOWN_REQUESTS: usize = 256;

/// `net`'s hop records as `network` lines.
pub struct HopLog {
    log: Arc<dyn NetworkLog>,
    ids: Mutex<RequestIds>,
}

#[derive(Default)]
struct RequestIds {
    by_request: HashMap<u64, LogId>,
    order: VecDeque<u64>,
}

impl HopLog {
    pub fn new(log: Arc<dyn NetworkLog>) -> Self {
        Self {
            log,
            ids: Mutex::new(RequestIds::default()),
        }
    }

    /// The log's id for `net`'s request, the same for every hop and retry of it.
    fn id_of(&self, request: u64) -> LogId {
        let mut ids = self.ids.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some(id) = ids.by_request.get(&request) {
            return id.clone();
        }
        let id = self.log.request_id();
        ids.by_request.insert(request, id.clone());
        ids.order.push_back(request);
        while ids.order.len() > KNOWN_REQUESTS {
            if let Some(old) = ids.order.pop_front() {
                ids.by_request.remove(&old);
            }
        }
        id
    }
}

impl NetLog for HopLog {
    fn record(&self, record: &HopRecord) {
        let (msg, status, bytes, error, blocked) = match &record.outcome {
            HopOutcome::Response {
                status,
                content_length,
                location,
                retry_in,
            } => {
                let msg = if retry_in.is_some() {
                    "response; retrying"
                } else if location.is_some() {
                    "redirect"
                } else {
                    "response"
                };
                (msg, Some(*status), *content_length, None, None)
            }
            HopOutcome::Blocked(reason) => (
                "request blocked",
                None,
                None,
                None,
                Some(reason.as_str().to_owned()),
            ),
            HopOutcome::Failed(error) => ("request failed", None, None, Some(error.clone()), None),
        };
        let event = NetworkEvent {
            req: self.id_of(record.request_id),
            hop: record.hop,
            attempt: record.attempt,
            method: "GET".to_owned(),
            url: record.url.clone(),
            host_source: record.host_source.map(host_source),
            initiator: initiator(record.initiator),
            purpose: purpose(record.purpose),
            status,
            bytes,
            ms: u64::try_from(record.elapsed.as_millis()).unwrap_or(u64::MAX),
            range: record.range.clone(),
            error,
            blocked,
        };
        self.log.network(msg, &event);
    }
}

fn host_source(source: HostSource) -> core_log::HostSource {
    match source {
        HostSource::BuiltIn => core_log::HostSource::Builtin,
        HostSource::GitHub => core_log::HostSource::Github,
        HostSource::User => core_log::HostSource::User,
        HostSource::UserImage => core_log::HostSource::UserImage,
    }
}

fn initiator(initiator: Initiator) -> core_log::Initiator {
    match initiator {
        Initiator::Auto => core_log::Initiator::Auto,
        Initiator::User => core_log::Initiator::User,
    }
}

fn purpose(purpose: Purpose) -> core_log::Purpose {
    match purpose {
        Purpose::Catalog => core_log::Purpose::Catalog,
        Purpose::Thumbnail => core_log::Purpose::Thumbnail,
        Purpose::DescriptionImage => core_log::Purpose::DescriptionImage,
        Purpose::Download => core_log::Purpose::Download,
        Purpose::UpdateCheck => core_log::Purpose::UpdateCheck,
    }
}

/// A transport that can be replaced while the client is shared: a new proxy or timeout applies to the next request.
#[derive(Clone)]
struct Switchable(Arc<RwLock<Arc<dyn Transport>>>);

impl Switchable {
    fn new(transport: Arc<dyn Transport>) -> Self {
        Self(Arc::new(RwLock::new(transport)))
    }

    fn replace(&self, transport: Arc<dyn Transport>) {
        *self.0.write().unwrap_or_else(PoisonError::into_inner) = transport;
    }
}

impl Transport for Switchable {
    fn send(&self, request: &HopRequest<'_>) -> Result<HopResponse, TransportError> {
        let current = Arc::clone(&self.0.read().unwrap_or_else(PoisonError::into_inner));
        current.send(request)
    }
}

/// Refuses every request with one reason, without touching the network: what a proxy setting deadloader can't follow
/// turns into, so nothing goes out directly behind the user's back.
struct Refusing(String);

impl Transport for Refusing {
    fn send(&self, _: &HopRequest<'_>) -> Result<HopResponse, TransportError> {
        Err(TransportError::new(
            TransportErrorKind::Other,
            self.0.clone(),
        ))
    }
}

/// What the settings ask of the transport.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Wanted {
    Ureq(TransportConfig),
    /// A proxy deadloader can't follow: every request is refused with this reason.
    Refuse(String),
}

/// Windows' proxy settings, read when the network is built or its settings change.
pub type SystemProxyLookup = Box<dyn Fn() -> std::io::Result<SystemProxy> + Send + Sync>;

fn wanted(settings: &Settings, system: &dyn Fn() -> std::io::Result<SystemProxy>) -> Wanted {
    let network = &settings.network;
    let proxy = match &network.proxy {
        Proxy::None => None,
        Proxy::Manual { address } => Some(address.clone()),
        // Decision 182: what Windows has, or nothing at all rather than a direct connection behind the user's back.
        Proxy::System => match system() {
            Ok(SystemProxy::Direct) => None,
            Ok(SystemProxy::Http(address)) => Some(address),
            Ok(SystemProxy::Unsupported(reason)) => {
                return Wanted::Refuse(format!(
                    "{reason}; no request is sent directly instead — choose \"No proxy\" or enter the proxy by \
                     hand in Settings → Network"
                ));
            }
            Err(e) => {
                return Wanted::Refuse(format!(
                    "Windows' proxy settings could not be read ({e}); no request is sent directly instead"
                ));
            }
        },
    };
    Wanted::Ureq(TransportConfig {
        connect_timeout: Duration::from_secs(u64::from(network.connect_timeout_s)),
        read_timeout: Duration::from_secs(u64::from(network.read_timeout_s)),
        proxy,
        ..TransportConfig::default()
    })
}

fn build(wanted: &Wanted) -> Arc<dyn Transport> {
    match wanted {
        Wanted::Ureq(config) => match UreqTransport::new(config) {
            Ok(transport) => Arc::new(transport),
            Err(e) => Arc::new(Refusing(format!(
                "the proxy in the network settings can't be used ({e}); no request is sent until it is fixed"
            ))),
        },
        Wanted::Refuse(reason) => Arc::new(Refusing(reason.clone())),
    }
}

/// The shared client and what it was built from.
pub struct Network {
    client: Arc<Client>,
    transport: Switchable,
    wanted: Mutex<Wanted>,
    system_proxy: SystemProxyLookup,
}

impl Network {
    pub fn new(settings: &Settings, log: Arc<dyn NetworkLog>) -> Self {
        Self::with_system_proxy(
            settings,
            log,
            Box::new(deadloader_platform_windows::proxy::system_proxy),
        )
    }

    pub fn with_system_proxy(
        settings: &Settings,
        log: Arc<dyn NetworkLog>,
        system_proxy: SystemProxyLookup,
    ) -> Self {
        let wanted = wanted(settings, &system_proxy);
        let transport = Switchable::new(build(&wanted));
        let client = Arc::new(Client::new(
            transport.clone(),
            HopLog::new(log),
            deadloader_net::clock::SystemClock,
            deadloader_net::Allowlist::new(),
        ));
        let network = Self {
            client,
            transport,
            wanted: Mutex::new(wanted),
            system_proxy,
        };
        network.configure(settings);
        network
    }

    pub fn client(&self) -> &Arc<Client> {
        &self.client
    }

    /// After a settings change: proxy and timeouts for the next request, local mode, retries, the user's extra hosts
    /// and the GitHub host of the update check. The description-image hosts belong to the catalog.
    pub fn apply(&self, settings: &Settings) {
        let wanted = wanted(settings, &self.system_proxy);
        let mut current = self.lock_wanted();
        if *current != wanted {
            self.transport.replace(build(&wanted));
            *current = wanted;
        }
        drop(current);
        self.configure(settings);
    }

    fn configure(&self, settings: &Settings) {
        self.client.set_local_mode(settings.is_local_mode());
        self.client.set_max_retries(settings.network.retries);
        let extra: Vec<Host> = settings
            .advanced
            .extra_allowed_domains
            .iter()
            .filter_map(|h| Host::parse(h).ok())
            .collect();
        let update_check = settings.app_update_check() == AppUpdateCheck::On;
        self.client.update_allowlist(|allowlist| {
            let stale: Vec<Host> = allowlist
                .user_hosts()
                .filter(|h| !extra.contains(h))
                .cloned()
                .collect();
            for host in &stale {
                allowlist.remove_user_host(host);
            }
            for host in &extra {
                allowlist.add_user_host(host.clone());
            }
            allowlist.set_update_check(update_check);
        });
    }

    fn lock_wanted(&self) -> MutexGuard<'_, Wanted> {
        self.wanted.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use deadloader_net::log::BlockReason;
    use deadloader_net::{Denied, NetError, Request};
    use std::sync::atomic::{AtomicU64, Ordering};

    /// Records network lines; ids count up from 1.
    #[derive(Default)]
    struct Lines {
        next: AtomicU64,
        lines: Mutex<Vec<(String, NetworkEvent)>>,
    }

    impl NetworkLog for Lines {
        fn request_id(&self) -> LogId {
            let n = self.next.fetch_add(1, Ordering::Relaxed) + 1;
            serde_json::from_value(serde_json::json!(format!("req-{n}"))).expect("id")
        }

        fn network(&self, msg: &str, event: &NetworkEvent) {
            self.lines
                .lock()
                .expect("lock")
                .push((msg.to_owned(), event.clone()));
        }
    }

    impl Lines {
        fn take(&self) -> Vec<(String, NetworkEvent)> {
            std::mem::take(&mut *self.lines.lock().expect("lock"))
        }
    }

    fn record(request_id: u64, hop: u32, outcome: HopOutcome) -> HopRecord {
        HopRecord {
            request_id,
            hop,
            attempt: 1,
            purpose: Purpose::Download,
            initiator: Initiator::User,
            url: "https://gamebanana.com/dl/1".to_owned(),
            host: Some("gamebanana.com".to_owned()),
            host_source: Some(HostSource::BuiltIn),
            range: None,
            outcome,
            elapsed: Duration::from_millis(120),
        }
    }

    fn response(status: u16, location: Option<&str>) -> HopOutcome {
        HopOutcome::Response {
            status,
            content_length: Some(10),
            location: location.map(str::to_owned),
            retry_in: None,
        }
    }

    #[test]
    fn hops_of_one_request_share_its_log_id() {
        let lines = Arc::new(Lines::default());
        let hops = HopLog::new(Arc::clone(&lines) as Arc<dyn NetworkLog>);
        hops.record(&record(
            7,
            0,
            response(302, Some("https://files.gamebanana.com/x")),
        ));
        hops.record(&record(8, 0, response(200, None)));
        hops.record(&record(7, 1, response(200, None)));
        let got = lines.take();
        assert_eq!(got[0].0, "redirect");
        assert_eq!(got[0].1.req, got[2].1.req);
        assert_ne!(got[0].1.req, got[1].1.req);
        assert_eq!(got[2].1.hop, 1);
        assert_eq!(got[2].1.status, Some(200));
        assert_eq!(got[2].1.bytes, Some(10));
        assert_eq!(got[2].1.ms, 120);
        assert_eq!(got[2].1.host_source, Some(core_log::HostSource::Builtin));
        assert_eq!(got[2].1.purpose, core_log::Purpose::Download);
        assert_eq!(got[2].1.initiator, core_log::Initiator::User);
    }

    #[test]
    fn refusals_failures_and_retries_say_what_happened() {
        let lines = Arc::new(Lines::default());
        let hops = HopLog::new(Arc::clone(&lines) as Arc<dyn NetworkLog>);
        let mut blocked = record(
            1,
            0,
            HopOutcome::Blocked(BlockReason::Host(Denied::NotListed)),
        );
        blocked.host_source = None;
        hops.record(&blocked);
        hops.record(&record(2, 0, HopOutcome::Failed("timed out".to_owned())));
        let mut retried = record(
            3,
            0,
            HopOutcome::Response {
                status: 503,
                content_length: None,
                location: None,
                retry_in: Some(Duration::from_secs(1)),
            },
        );
        retried.attempt = 2;
        hops.record(&retried);
        let got = lines.take();
        assert_eq!(got[0].0, "request blocked");
        assert_eq!(got[0].1.blocked.as_deref(), Some("host-not-listed"));
        assert_eq!(got[0].1.host_source, None);
        assert_eq!(got[1].0, "request failed");
        assert_eq!(got[1].1.error.as_deref(), Some("timed out"));
        assert_eq!(got[2].0, "response; retrying");
        assert_eq!(got[2].1.attempt, 2);
    }

    #[test]
    fn only_the_newest_request_ids_are_remembered() {
        let lines = Arc::new(Lines::default());
        let hops = HopLog::new(Arc::clone(&lines) as Arc<dyn NetworkLog>);
        for id in 0..=KNOWN_REQUESTS as u64 {
            hops.record(&record(id, 0, response(200, None)));
        }
        let ids = hops.ids.lock().expect("lock");
        assert_eq!(ids.by_request.len(), KNOWN_REQUESTS);
        assert!(!ids.by_request.contains_key(&0));
    }

    fn network(settings: &Settings) -> (Network, Arc<Lines>) {
        with_windows_proxy(settings, SystemProxy::Direct)
    }

    /// A network whose "system proxy" is `windows`, whatever this PC has.
    fn with_windows_proxy(settings: &Settings, windows: SystemProxy) -> (Network, Arc<Lines>) {
        let lines = Arc::new(Lines::default());
        (
            Network::with_system_proxy(
                settings,
                Arc::clone(&lines) as Arc<dyn NetworkLog>,
                Box::new(move || Ok(windows.clone())),
            ),
            lines,
        )
    }

    fn online() -> Settings {
        let mut settings = Settings::default();
        settings.catalog.enabled = true;
        settings
    }

    fn catalog_request() -> Request {
        Request::new(
            Purpose::Catalog,
            Initiator::Auto,
            "https://gamebanana.com/apiv11/Mod/Index",
        )
    }

    #[test]
    fn local_mode_follows_the_settings_and_is_logged() {
        let (net, lines) = network(&Settings::default());
        assert!(matches!(
            net.client().get(&catalog_request()),
            Err(NetError::LocalMode)
        ));
        assert_eq!(lines.take()[0].1.blocked.as_deref(), Some("local-mode"));
    }

    #[test]
    fn the_users_hosts_and_the_update_check_follow_the_settings() {
        let mut settings = online();
        settings.advanced.extra_allowed_domains = vec!["mirror.example.org".to_owned()];
        settings.set_app_update_check(true).expect("catalog on");
        let (net, _) = network(&settings);
        let listed = |net: &Network| -> Vec<String> {
            net.client()
                .allowlist()
                .user_hosts()
                .map(|h| h.as_str().to_owned())
                .collect()
        };
        assert_eq!(listed(&net), ["mirror.example.org"]);
        assert!(net.client().allowlist().update_check());

        settings.advanced.extra_allowed_domains = vec!["other.example.org".to_owned()];
        settings.set_app_update_check(false).expect("off");
        net.apply(&settings);
        assert_eq!(listed(&net), ["other.example.org"]);
        assert!(!net.client().allowlist().update_check());
    }

    fn system(windows: SystemProxy) -> Wanted {
        let mut settings = online();
        settings.network.proxy = Proxy::System;
        wanted(&settings, &move || Ok(windows.clone()))
    }

    #[test]
    fn the_system_proxy_is_the_one_set_in_windows() {
        let Wanted::Ureq(config) = system(SystemProxy::Http("http://proxy.corp:8080".to_owned()))
        else {
            panic!("ureq");
        };
        assert_eq!(config.proxy.as_deref(), Some("http://proxy.corp:8080"));
        let Wanted::Ureq(config) = system(SystemProxy::Direct) else {
            panic!("ureq");
        };
        assert_eq!(config.proxy, None);
    }

    #[test]
    fn a_windows_proxy_deadloader_cant_follow_refuses_requests_instead_of_going_direct() {
        let mut settings = online();
        settings.network.proxy = Proxy::System;
        let (net, lines) = with_windows_proxy(
            &settings,
            SystemProxy::Unsupported("Windows uses a proxy setup script".to_owned()),
        );
        let err = net.client().get(&catalog_request()).expect_err("refused");
        assert!(err.to_string().contains("setup script"), "{err}");
        assert_eq!(lines.take()[0].0, "request failed");

        // Back to a direct connection on the next request (not sent here: it would reach the internet).
        settings.network.proxy = Proxy::None;
        net.apply(&settings);
        assert!(matches!(*net.lock_wanted(), Wanted::Ureq(_)));
    }

    #[test]
    fn unreadable_windows_settings_refuse_requests_too() {
        let mut settings = online();
        settings.network.proxy = Proxy::System;
        let wanted = wanted(&settings, &|| Err(std::io::Error::other("access denied")));
        assert!(matches!(wanted, Wanted::Refuse(reason) if reason.contains("access denied")));
    }

    #[test]
    fn timeouts_and_a_manual_proxy_come_from_the_settings() {
        let mut settings = online();
        settings.network.connect_timeout_s = 7;
        settings.network.read_timeout_s = 45;
        settings.network.proxy = Proxy::Manual {
            address: "http://127.0.0.1:3128".to_owned(),
        };
        let Wanted::Ureq(config) = wanted(&settings, &|| Ok(SystemProxy::Direct)) else {
            panic!("ureq");
        };
        assert_eq!(config.connect_timeout, Duration::from_secs(7));
        assert_eq!(config.read_timeout, Duration::from_secs(45));
        assert_eq!(config.proxy.as_deref(), Some("http://127.0.0.1:3128"));
        assert_eq!(config.idle_timeout, Duration::from_secs(60));
    }
}
