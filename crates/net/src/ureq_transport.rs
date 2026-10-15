//! The real [`Transport`]: `ureq` 3 over native TLS (SChannel on Windows), one blocking GET per call.
//!
//! Configured so the client cannot do anything behind the request loop's back: no redirects, no proxy unless the user
//! sets one (ureq would otherwise read `HTTPS_PROXY` from the environment), https only, statuses returned as
//! responses rather than errors, and our User-Agent even on proxy CONNECT.

use crate::allowlist::Purpose;
use crate::client::USER_AGENT;
use crate::transport::{HopRequest, HopResponse, Transport, TransportError, TransportErrorKind};
use std::time::Duration;
use ureq::tls::{RootCerts, TlsConfig, TlsProvider};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransportConfig {
    pub connect_timeout: Duration,
    /// Waiting for the response headers; for everything except downloads also the whole body.
    pub read_timeout: Duration,
    /// A user-configured proxy (`http://host:port`, `socks5://…` is not compiled in). `None` = direct.
    pub proxy: Option<String>,
}

impl Default for TransportConfig {
    fn default() -> Self {
        Self {
            connect_timeout: Duration::from_secs(10),
            read_timeout: Duration::from_secs(30),
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
            agent: ureq::Agent::new_with_config(agent_config),
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

/// ureq 3 has no per-read idle timeout, only a total body budget. Small responses get one; downloads can legitimately
/// take many minutes, so their stall detection belongs to the download worker.
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
