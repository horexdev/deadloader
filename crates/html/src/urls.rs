//! Which URLs a description may refer to, and what they become.
//!
//! Only absolute `http`/`https` URLs to public domain names survive; protocol-relative ones become `https`.
//! Relative paths, other schemes, IP literals and local names are dropped: there is no page they are relative to,
//! and a click must never reach the user's own network.

use url::{Host, Url};

use crate::Link;

/// Longer URLs are refused: real ones are far shorter, and the confirmation dialog must be able to show them.
const MAX_URL_BYTES: usize = 2048;
const MAX_HOST_BYTES: usize = 253;
const GAMEBANANA_HOSTS: &[&str] = &["gamebanana.com", "www.gamebanana.com"];
const YOUTUBE_HOSTS: &[&str] = &[
    "youtube.com",
    "www.youtube.com",
    "m.youtube.com",
    "youtube-nocookie.com",
    "www.youtube-nocookie.com",
];

/// A link target from `<a href>`. GameBanana's link filter is unwrapped so the dialog shows the real destination.
pub fn link(raw: &str) -> Option<Link> {
    let mut url = absolute(raw)?;
    if is_gamebanana(&url) && url.path() == "/linkfilter" {
        let target = url
            .query_pairs()
            .find(|(key, _)| key == "url")
            .map(|(_, value)| value.into_owned())?;
        url = absolute(&target)?;
    }
    let internal_mod_id = mod_id(&url);
    Some(Link {
        host: host(&url),
        url: url.into(),
        internal_mod_id,
    })
}

/// An image URL from `<img src>`, with its host.
pub fn image(raw: &str) -> Option<(String, String)> {
    let url = absolute(raw)?;
    Some((host(&url), url.into()))
}

/// The watch page of a YouTube embed from `<iframe src>`; any other embed gives nothing.
pub fn youtube(raw: &str) -> Option<Link> {
    let url = absolute(raw)?;
    if !YOUTUBE_HOSTS.contains(&url.host_str()?) {
        return None;
    }
    let id = match segments(&url)?.as_slice() {
        ["embed", id] => *id,
        _ => return None,
    };
    let valid = !id.is_empty()
        && id.len() <= 64
        && id
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_');
    if !valid {
        return None;
    }
    let url = Url::parse(&format!("https://www.youtube.com/watch?v={id}")).ok()?;
    Some(Link {
        host: host(&url),
        url: url.into(),
        internal_mod_id: None,
    })
}

/// Parses and checks one attribute value; the result is normalised (lowercase punycode host, percent-encoded),
/// without credentials.
fn absolute(raw: &str) -> Option<Url> {
    let raw = raw.trim_matches(|c: char| c.is_ascii_whitespace() || c.is_ascii_control());
    if raw.len() > MAX_URL_BYTES {
        return None;
    }
    let mut url = match Url::parse(raw) {
        Ok(url) => url,
        Err(url::ParseError::RelativeUrlWithoutBase) if is_protocol_relative(raw) => {
            Url::parse(&format!("https:{raw}")).ok()?
        }
        Err(_) => return None,
    };
    if !matches!(url.scheme(), "http" | "https") {
        return None;
    }
    match url.host()? {
        Host::Domain(domain) if is_public_domain(domain) => {}
        _ => return None,
    }
    // "https://gamebanana.com@evil.example/" must not look like GameBanana anywhere it is shown.
    url.set_username("").ok()?;
    url.set_password(None).ok()?;
    (url.as_str().len() <= MAX_URL_BYTES).then_some(url)
}

/// `//host/path`, also spelled with backslashes as browsers accept it; tabs and newlines are ignored, as the URL
/// parser ignores them.
fn is_protocol_relative(raw: &str) -> bool {
    let mut chars = raw.chars().filter(|c| !matches!(c, '\t' | '\n' | '\r'));
    matches!(
        (chars.next(), chars.next()),
        (Some('/' | '\\'), Some('/' | '\\'))
    )
}

/// A dotted DNS name that is not a local one. The URL parser has already lowercased it and converted IDN to
/// punycode.
fn is_public_domain(domain: &str) -> bool {
    let well_formed = domain.len() <= MAX_HOST_BYTES
        && domain.contains('.')
        && !domain.starts_with('.')
        && !domain.ends_with('.')
        && domain
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'.' || b == b'-');
    let local = [".localhost", ".local", ".internal", ".home.arpa"]
        .iter()
        .any(|suffix| domain.ends_with(suffix));
    well_formed && !local
}

fn host(url: &Url) -> String {
    url.host_str().unwrap_or_default().to_owned()
}

fn is_gamebanana(url: &Url) -> bool {
    url.host_str()
        .is_some_and(|host| GAMEBANANA_HOSTS.contains(&host))
}

/// Path segments without a trailing empty one (`/mods/1/` is `["mods", "1"]`).
fn segments(url: &Url) -> Option<Vec<&str>> {
    let mut segments: Vec<&str> = url.path_segments()?.collect();
    if segments.last() == Some(&"") {
        segments.pop();
    }
    Some(segments)
}

/// `gamebanana.com/mods/<id>`.
fn mod_id(url: &Url) -> Option<u64> {
    if !is_gamebanana(url) {
        return None;
    }
    match segments(url)?.as_slice() {
        ["mods", id] if id.bytes().all(|b| b.is_ascii_digit()) => {
            id.parse().ok().filter(|&id| id > 0)
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn protocol_relative_spellings() {
        assert!(is_protocol_relative("//x.com"));
        assert!(is_protocol_relative("\\\\x.com"));
        assert!(is_protocol_relative("/\t/x.com"));
        assert!(!is_protocol_relative("/x.com"));
        assert!(!is_protocol_relative("x.com"));
    }

    #[test]
    fn backslashes_resolve_like_a_browser() {
        assert_eq!(
            link("/\\evil.example/a").map(|l| l.url),
            Some("https://evil.example/a".to_owned())
        );
        assert_eq!(
            link("https:\\\\evil.example\\a").map(|l| l.url),
            Some("https://evil.example/a".to_owned())
        );
    }

    #[test]
    fn hidden_scheme_characters_do_not_help() {
        for raw in [
            "java\tscript:alert(1)",
            "java\nscript:alert(1)",
            "\u{1}javascript:alert(1)",
            " JAVASCRIPT:alert(1)",
            "javascript\u{0}:alert(1)",
        ] {
            assert_eq!(link(raw), None, "{raw:?}");
        }
    }

    #[test]
    fn public_domains() {
        assert!(is_public_domain("i.imgur.com"));
        assert!(is_public_domain("xn--bcher-kva.example"));
        for domain in [
            "localhost",
            "a.localhost",
            "nas.local",
            "x.internal",
            "router.home.arpa",
            "intranet",
            "a_b.example",
            "example.com.",
            ".example.com",
        ] {
            assert!(!is_public_domain(domain), "{domain}");
        }
    }

    #[test]
    fn mod_ids_are_plain_positive_numbers() {
        let id = |raw: &str| link(raw).and_then(|l| l.internal_mod_id);
        assert_eq!(id("https://gamebanana.com/mods/12"), Some(12));
        assert_eq!(id("http://gamebanana.com/mods/12?x=1#f"), Some(12));
        assert_eq!(id("https://gamebanana.com/mods/+12"), None);
        assert_eq!(
            id("https://gamebanana.com/mods/99999999999999999999999"),
            None
        );
        assert_eq!(id("https://gamebanana.com/Mods/12"), None);
        assert_eq!(id("https://gamebanana.com/mods/12/extra"), None);
    }

    #[test]
    fn link_filter_is_unwrapped_once() {
        let inner =
            "https%3A%2F%2Fgamebanana.com%2Flinkfilter%3Furl%3Dhttps%253A%252F%252Fa.example";
        let link = link(&format!("https://gamebanana.com/linkfilter?url={inner}")).expect("link");
        assert_eq!(
            link.url,
            "https://gamebanana.com/linkfilter?url=https%3A%2F%2Fa.example"
        );
        assert_eq!(super::link("https://gamebanana.com/linkfilter"), None);
    }
}
