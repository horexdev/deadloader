//! The real [`Transport`]: `ureq` 3 over native TLS (SChannel on Windows), one blocking GET per call.
//!
//! Configured so the client cannot do anything behind the request loop's back: no redirects, no proxy unless the user
//! sets one (ureq would otherwise read `HTTPS_PROXY` from the environment), https only, statuses returned as
//! responses rather than errors, and our User-Agent even on proxy CONNECT.

use crate::allowlist::Purpose;
use crate::client::USER_AGENT;
use crate::idle::IdleTimeout;
use crate::transport::{HopRequest, HopResponse, Transport, TransportError, TransportErrorKind};
use std::time::Duration;
use ureq::tls::{RootCerts, TlsConfig, TlsProvider};
use ureq::unversioned::resolver::DefaultResolver;
use ureq::unversioned::transport::{Connector, DefaultConnector};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransportConfig {
    pub connect_timeout: Duration,
    /// Waiting for the response headers; for everything except downloads also the whole body.
    pub read_timeout: Duration,
    /// The longest a single wait for data may last, downloads included: a connection that goes silent without
    /// closing fails after this instead of blocking its thread.
    pub idle_timeout: Duration,
    /// A user-configured proxy (`http://host:port`, `socks5://…` is not compiled in). `None` = direct.
    pub proxy: Option<String>,
}

impl Default for TransportConfig {
    fn default() -> Self {
        Self {
            connect_timeout: Duration::from_secs(10),
            read_timeout: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(60),
            proxy: None,
        }
    }
}

#[derive(Debug)]
pub struct UreqTransport {
    agent: ureq::Agent,
    read_timeout: Duration,
}

impl UreqTransport {
    pub fn new(config: &TransportConfig) -> Result<Self, TransportError> {
        let proxy = match &config.proxy {
            Some(url) => Some(ureq::Proxy::new(url).map_err(|e| {
                TransportError::new(TransportErrorKind::Other, format!("proxy `{url}`: {e}"))
            })?),
            None => None,
        };
        let tls = TlsConfig::builder()
            .provider(TlsProvider::NativeTls)
            // For native-tls this means "the operating system's trust store".
            .root_certs(RootCerts::PlatformVerifier)
            .build();
        let agent_config = ureq::Agent::config_builder()
            .max_redirects(0)
            .https_only(true)
            .http_status_as_error(false)
            .proxy(proxy)
            .user_agent(USER_AGENT)
            .tls_config(tls)
            .timeout_connect(Some(config.connect_timeout))
            .timeout_recv_response(Some(config.read_timeout))
            .build();
        Ok(Self {
            agent: ureq::Agent::with_parts(
                agent_config,
                connector(config.idle_timeout),
                DefaultResolver::default(),
            ),
            read_timeout: config.read_timeout,
        })
    }
}

impl Transport for UreqTransport {
    fn send(&self, request: &HopRequest<'_>) -> Result<HopResponse, TransportError> {
        let mut builder = self.agent.get(request.url.as_str());
        for (name, value) in request.headers {
            builder = builder.header(name.as_str(), value.as_str());
        }
        let response = builder
            .config()
            .timeout_recv_body(body_deadline(request.purpose, self.read_timeout))
            .build()
            .call()
            .map_err(map_error)?;
        let status = response.status().as_u16();
        let headers = response
            .headers()
            .iter()
            .map(|(name, value)| {
                (
                    name.as_str().to_owned(),
                    String::from_utf8_lossy(value.as_bytes()).into_owned(),
                )
            })
            .collect();
        Ok(HopResponse {
            status,
            headers,
            body: Box::new(response.into_body().into_reader()),
        })
    }
}

/// ureq's own connection chain (TCP, proxy, TLS) with the idle limit on top.
fn connector(idle: Duration) -> impl Connector {
    DefaultConnector::default().chain(IdleTimeout(idle))
}

/// Small responses get a total body budget. Downloads can legitimately take many minutes, so they have none: the idle
/// limit ends a dead connection, and slow progress is the download worker's call.
fn body_deadline(purpose: Purpose, read_timeout: Duration) -> Option<Duration> {
    (purpose != Purpose::Download).then_some(read_timeout)
}

fn map_error(error: ureq::Error) -> TransportError {
    let kind = match &error {
        ureq::Error::HostNotFound => TransportErrorKind::Dns,
        ureq::Error::ConnectionFailed | ureq::Error::ConnectProxyFailed(_) => {
            TransportErrorKind::Connect
        }
        ureq::Error::Timeout(_) => TransportErrorKind::Timeout,
        ureq::Error::Tls(_) | ureq::Error::NativeTls(_) | ureq::Error::Der(_) => {
            TransportErrorKind::Tls
        }
        ureq::Error::Io(e) if e.kind() == std::io::ErrorKind::TimedOut => {
            TransportErrorKind::Timeout
        }
        ureq::Error::Io(_) => TransportErrorKind::Io,
        _ => TransportErrorKind::Other,
    };
    TransportError::new(kind, error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn builds_with_the_default_configuration() {
        let transport = UreqTransport::new(&TransportConfig::default()).expect("transport");
        let config = transport.agent.config();
        assert_eq!(config.max_redirects(), 0);
        assert!(config.https_only());
        assert!(!config.http_status_as_error());
        assert!(
            config.proxy().is_none(),
            "no proxy unless the user sets one"
        );
        assert!(
            matches!(config.user_agent(), ureq::config::AutoHeaderValue::Provided(ua) if ua.as_str() == USER_AGENT),
            "{:?}",
            config.user_agent()
        );
        assert_eq!(config.timeouts().connect, Some(Duration::from_secs(10)));
        assert_eq!(
            config.timeouts().recv_response,
            Some(Duration::from_secs(30))
        );
        assert_eq!(config.tls_config().provider(), TlsProvider::NativeTls);
        assert!(matches!(
            config.tls_config().root_certs(),
            RootCerts::PlatformVerifier
        ));
        assert_eq!(transport.read_timeout, Duration::from_secs(30));
    }

    /// ureq panics at the first https request if the configured TLS provider is not compiled in. A loopback peer
    /// that is not a TLS server must produce an ordinary error instead. Nothing leaves the machine.
    #[test]
    fn the_native_tls_provider_is_compiled_in() {
        use std::io::Write;
        use std::net::TcpListener;

        let listener = TcpListener::bind("127.0.0.1:0").expect("bind loopback");
        let port = listener.local_addr().expect("addr").port();
        let peer = std::thread::spawn(move || {
            if let Ok((mut stream, _)) = listener.accept() {
                let _ = stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n");
            }
        });
        let transport = UreqTransport::new(&TransportConfig::default()).expect("transport");
        let result = transport
            .agent
            .get(&format!("https://127.0.0.1:{port}/"))
            .call();
        assert!(
            result.is_err(),
            "a plain-text peer cannot pass a TLS handshake"
        );
        let _ = peer.join();
    }

    #[test]
    fn the_idle_limit_defaults_to_a_minute() {
        assert_eq!(
            TransportConfig::default().idle_timeout,
            Duration::from_secs(60)
        );
    }

    /// A peer that sends the headers and part of the body, then goes silent without closing: the read must fail with a
    /// timeout instead of blocking. Plain HTTP on loopback; nothing leaves the machine.
    #[test]
    fn a_silent_connection_times_out_while_reading_the_body() {
        use std::io::{Read, Write};
        use std::net::TcpListener;
        use std::time::Instant;

        let listener = TcpListener::bind("127.0.0.1:0").expect("bind loopback");
        let port = listener.local_addr().expect("addr").port();
        let (done_tx, done_rx) = std::sync::mpsc::channel::<()>();
        let peer = std::thread::spawn(move || {
            if let Ok((mut stream, _)) = listener.accept() {
                let mut request = [0; 1024];
                let _ = stream.read(&mut request);
                let _ = stream
                    .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 1000000\r\n\r\npartial body");
                // Silent until the test is over.
                let _ = done_rx.recv();
            }
        });
        let config = ureq::Agent::config_builder()
            .http_status_as_error(false)
            .build();
        let agent = ureq::Agent::with_parts(
            config,
            connector(Duration::from_millis(300)),
            DefaultResolver::default(),
        );
        let started = Instant::now();
        let response = agent
            .get(&format!("http://127.0.0.1:{port}/"))
            .call()
            .expect("headers");
        let mut body = Vec::new();
        let read = response.into_body().into_reader().read_to_end(&mut body);
        let elapsed = started.elapsed();
        let _ = done_tx.send(());
        let _ = peer.join();
        let err = read.expect_err("the peer went silent");
        // ureq hands a body timeout over as an io error that wraps its own `Timeout`.
        let inner = err.get_ref().and_then(|e| e.downcast_ref::<ureq::Error>());
        assert!(matches!(inner, Some(ureq::Error::Timeout(_))), "{err}");
        assert!(elapsed < Duration::from_secs(10), "{elapsed:?}");
    }

    #[test]
    fn a_manual_proxy_is_used() {
        let transport = UreqTransport::new(&TransportConfig {
            proxy: Some("http://127.0.0.1:3128".to_owned()),
            ..TransportConfig::default()
        })
        .expect("transport");
        assert!(transport.agent.config().proxy().is_some());
    }

    #[test]
    fn a_malformed_proxy_is_an_error() {
        let err = UreqTransport::new(&TransportConfig {
            proxy: Some("not a proxy".to_owned()),
            ..TransportConfig::default()
        })
        .expect_err("bad proxy");
        assert_eq!(err.kind, TransportErrorKind::Other);
        assert!(err.message.contains("proxy"), "{err}");
    }

    #[test]
    fn ureq_errors_map_to_kinds() {
        let cases = [
            (ureq::Error::HostNotFound, TransportErrorKind::Dns),
            (ureq::Error::ConnectionFailed, TransportErrorKind::Connect),
            (
                ureq::Error::Timeout(ureq::Timeout::Connect),
                TransportErrorKind::Timeout,
            ),
            (ureq::Error::Tls("handshake"), TransportErrorKind::Tls),
            (
                ureq::Error::Io(io::Error::from(io::ErrorKind::ConnectionReset)),
                TransportErrorKind::Io,
            ),
            (
                ureq::Error::Io(io::Error::from(io::ErrorKind::TimedOut)),
                TransportErrorKind::Timeout,
            ),
            (
                ureq::Error::BadUri("x".to_owned()),
                TransportErrorKind::Other,
            ),
        ];
        for (error, kind) in cases {
            let text = error.to_string();
            let mapped = map_error(error);
            assert_eq!(mapped.kind, kind, "{text}");
            assert_eq!(mapped.message, text);
        }
    }

    #[test]
    fn only_downloads_have_no_total_body_deadline() {
        let t = Duration::from_secs(30);
        assert_eq!(body_deadline(Purpose::Download, t), None);
        for purpose in [
            Purpose::Catalog,
            Purpose::Thumbnail,
            Purpose::DescriptionImage,
            Purpose::UpdateCheck,
        ] {
            assert_eq!(body_deadline(purpose, t), Some(t), "{purpose:?}");
        }
    }
}
