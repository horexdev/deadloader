//! Windows' own proxy settings (Settings → Network → Proxy), for "use the system proxy" (decision 182).
//!
//! Read through WinHTTP. A proxy set by hand is used; a setup script (PAC) is not supported; "detect settings
//! automatically" alone, which Windows turns on by default, means a direct connection. The bypass list is not used:
//! deadloader only talks to public hosts.

#![allow(unsafe_code)]

use std::io;
use windows_sys::Win32::Foundation::GlobalFree;
use windows_sys::Win32::Networking::WinHttp::{
    WINHTTP_CURRENT_USER_IE_PROXY_CONFIG, WinHttpGetIEProxyConfigForCurrentUser,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SystemProxy {
    /// No proxy is set.
    Direct,
    /// `http://host:port`.
    Http(String),
    /// Windows uses something deadloader can't follow; why, in English for the log and the error text.
    Unsupported(String),
}

/// The current user's proxy settings.
pub fn system_proxy() -> io::Result<SystemProxy> {
    let mut config = WINHTTP_CURRENT_USER_IE_PROXY_CONFIG {
        fAutoDetect: 0,
        lpszAutoConfigUrl: std::ptr::null_mut(),
        lpszProxy: std::ptr::null_mut(),
        lpszProxyBypass: std::ptr::null_mut(),
    };
    // SAFETY: `config` is a valid, writable struct of the type the call fills.
    let ok = unsafe { WinHttpGetIEProxyConfigForCurrentUser(&mut config) };
    if ok == 0 {
        return Err(io::Error::last_os_error());
    }
    let script = take(config.lpszAutoConfigUrl);
    let proxy = take(config.lpszProxy);
    // Not used, but WinHTTP allocated it.
    let _ = take(config.lpszProxyBypass);
    Ok(decide(script.as_deref(), proxy.as_deref()))
}

/// Copies a string WinHTTP allocated and frees it.
fn take(ptr: *mut u16) -> Option<String> {
    if ptr.is_null() {
        return None;
    }
    // SAFETY: WinHTTP returns NUL-terminated UTF-16 strings; we read up to the terminator only.
    let text = unsafe {
        let mut len = 0;
        while *ptr.add(len) != 0 {
            len += 1;
        }
        String::from_utf16_lossy(std::slice::from_raw_parts(ptr, len))
    };
    // SAFETY: the string was allocated by WinHTTP with GlobalAlloc and is freed exactly once, after the copy.
    unsafe { GlobalFree(ptr.cast()) };
    Some(text)
}

/// What to do with Windows' settings: the setup script (if any) and the proxy line (if any).
pub fn decide(script: Option<&str>, proxy: Option<&str>) -> SystemProxy {
    let proxy = proxy.map(str::trim).filter(|p| !p.is_empty());
    if let Some(line) = proxy {
        return match pick(line) {
            Some(address) => SystemProxy::Http(address),
            None => SystemProxy::Unsupported(format!(
                "the Windows proxy \"{line}\" has no HTTP or HTTPS entry deadloader can use"
            )),
        };
    }
    if script.is_some_and(|s| !s.trim().is_empty()) {
        return SystemProxy::Unsupported(
            "Windows uses a proxy setup script, which deadloader doesn't support".to_owned(),
        );
    }
    SystemProxy::Direct
}

/// The entry for HTTPS traffic from a proxy line: `host:port`, `http://host:port`, or a per-protocol list like
/// `http=a:80;https=b:443` (the `https` entry first, then `http`, then one without a protocol).
fn pick(line: &str) -> Option<String> {
    let entries: Vec<(Option<String>, &str)> = line
        .split([';', ' '])
        .map(str::trim)
        .filter(|e| !e.is_empty())
        .map(|e| match e.split_once('=') {
            Some((scheme, address)) => (Some(scheme.trim().to_ascii_lowercase()), address.trim()),
            None => (None, e),
        })
        .collect();
    let find = |wanted: Option<&str>| {
        entries
            .iter()
            .find(|(scheme, _)| scheme.as_deref() == wanted)
            .map(|(_, address)| *address)
    };
    let address = find(Some("https"))
        .or_else(|| find(Some("http")))
        .or_else(|| find(None))?;
    let host_port = match address.split_once("://") {
        Some(("http" | "HTTP", rest)) => rest,
        Some(_) => return None,
        None => address,
    };
    let host_port = host_port.trim_end_matches('/');
    (!host_port.is_empty()).then(|| format!("http://{host_port}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn http(address: &str) -> SystemProxy {
        SystemProxy::Http(address.to_owned())
    }

    #[test]
    fn nothing_set_or_only_automatic_detection_is_direct() {
        assert_eq!(decide(None, None), SystemProxy::Direct);
        assert_eq!(decide(Some(""), Some("  ")), SystemProxy::Direct);
    }

    #[test]
    fn a_proxy_set_by_hand_is_used() {
        assert_eq!(
            decide(None, Some("10.0.0.5:3128")),
            http("http://10.0.0.5:3128")
        );
        assert_eq!(
            decide(None, Some("http://proxy.corp:8080/")),
            http("http://proxy.corp:8080")
        );
    }

    #[test]
    fn a_per_protocol_list_uses_the_https_entry_then_http() {
        assert_eq!(
            decide(None, Some("http=a.corp:80;https=b.corp:443;ftp=c.corp:21")),
            http("http://b.corp:443")
        );
        assert_eq!(
            decide(None, Some("http=a.corp:80;ftp=c.corp:21")),
            http("http://a.corp:80")
        );
    }

    #[test]
    fn a_hand_set_proxy_wins_over_a_script() {
        assert_eq!(
            decide(Some("http://wpad/proxy.pac"), Some("a.corp:80")),
            http("http://a.corp:80")
        );
    }

    #[test]
    fn a_setup_script_or_a_socks_only_proxy_is_not_supported() {
        assert!(matches!(
            decide(Some("http://wpad/proxy.pac"), None),
            SystemProxy::Unsupported(reason) if reason.contains("setup script")
        ));
        assert!(matches!(
            decide(None, Some("socks=s.corp:1080")),
            SystemProxy::Unsupported(_)
        ));
        assert!(matches!(
            decide(None, Some("https=socks5://s.corp:1080")),
            SystemProxy::Unsupported(_)
        ));
    }

    /// Reads the real settings; whatever they are, the call must succeed and answer something sensible.
    #[test]
    fn the_settings_of_this_pc_can_be_read() {
        let proxy = system_proxy().expect("WinHTTP answers");
        if let SystemProxy::Http(address) = proxy {
            assert!(address.starts_with("http://"), "{address}");
        }
    }
}
