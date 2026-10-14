//! The only URLs the network layer accepts: `https://<host>[:443]/<path>?<query>`.
//!
//! A deliberately small parser instead of a general URL crate: every hop of a redirect chain is checked against the
//! allowlist, so what counts as "the host" must be unambiguous. Anything a browser would have to guess about —
//! userinfo (`https://gamebanana.com@evil.net`), IP literals, other ports, other schemes, non-ASCII host names — is
//! refused rather than interpreted.

use std::fmt;

/// A DNS host name normalised for comparison: ASCII lowercase, no trailing dot.
///
/// Internationalised names must be given in their ASCII (punycode, `xn--…`) form; comparing raw Unicode would let
/// look-alike characters match.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Host(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HostError {
    Empty,
    /// Unicode host names are accepted only in punycode form.
    NonAscii,
    /// IPv4/IPv6 literals bypass the point of a host allowlist.
    IpLiteral,
    InvalidCharacter(char),
    /// An empty label (`a..b`), a label longer than 63 characters, or one starting/ending with `-`.
    InvalidLabel,
    TooLong,
}

impl fmt::Display for HostError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("the host name is empty"),
            Self::NonAscii => {
                f.write_str("the host name is not ASCII; enter it in punycode form (xn--…)")
            }
            Self::IpLiteral => f.write_str("IP addresses are not accepted, only host names"),
            Self::InvalidCharacter(c) => write!(f, "the host name contains `{c}`"),
            Self::InvalidLabel => f.write_str("the host name has an empty or malformed part"),
            Self::TooLong => f.write_str("the host name is longer than 253 characters"),
        }
    }
}

impl std::error::Error for HostError {}

impl Host {
    pub fn parse(input: &str) -> Result<Self, HostError> {
        let trimmed = input.strip_suffix('.').unwrap_or(input);
        if trimmed.is_empty() {
            return Err(HostError::Empty);
        }
        if !trimmed.is_ascii() {
            return Err(HostError::NonAscii);
        }
        if trimmed.starts_with('[') {
            return Err(HostError::IpLiteral);
        }
        if let Some(c) = trimmed
            .chars()
            .find(|c| !(c.is_ascii_alphanumeric() || *c == '-' || *c == '.'))
        {
            return Err(HostError::InvalidCharacter(c));
        }
        if trimmed.len() > 253 {
            return Err(HostError::TooLong);
        }
        let lower = trimmed.to_ascii_lowercase();
        for label in lower.split('.') {
            if label.is_empty()
                || label.len() > 63
                || label.starts_with('-')
                || label.ends_with('-')
            {
                return Err(HostError::InvalidLabel);
            }
        }
        if looks_like_ipv4(&lower) {
            return Err(HostError::IpLiteral);
        }
        Ok(Self(lower))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Host {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Browsers treat a host whose last label is numeric (`127.1`, `0x7f.1`, `2130706433`) as an IPv4 address.
fn looks_like_ipv4(host: &str) -> bool {
    let last = host.rsplit('.').next().unwrap_or(host);
    last.bytes().all(|b| b.is_ascii_digit())
        || last
            .strip_prefix("0x")
            .is_some_and(|hex| hex.bytes().all(|b| b.is_ascii_hexdigit()))
}

/// An absolute https URL on the default port. The fragment, if any, is dropped (it is never sent).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpsUrl {
    host: Host,
    /// Path plus optional query; always starts with `/`, dot segments removed, percent-encoded where needed.
    path_and_query: String,
    full: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UrlError {
    /// Any scheme other than https, including a downgrade to `http`.
    NotHttps {
        scheme: String,
    },
    /// `https://user@host` — the part before `@` is easily mistaken for the host.
    UserInfo,
    /// A port other than 443.
    Port(String),
    Host(HostError),
    /// Not a URL we can interpret, e.g. `https:foo`, an empty string or control characters.
    Malformed,
}

impl fmt::Display for UrlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotHttps { scheme } => {
                write!(f, "the address uses `{scheme}:` instead of https")
            }
            Self::UserInfo => f.write_str("the address contains a user name before the host"),
            Self::Port(p) => write!(f, "the address uses port {p} instead of the https default"),
            Self::Host(e) => write!(f, "{e}"),
            Self::Malformed => f.write_str("the address is malformed"),
        }
    }
}

impl std::error::Error for UrlError {}

impl From<HostError> for UrlError {
    fn from(e: HostError) -> Self {
        Self::Host(e)
    }
}

impl HttpsUrl {
    pub fn parse(input: &str) -> Result<Self, UrlError> {
        let input = without_fragment(input.trim_matches(is_ascii_ws));
        check_characters(input)?;
        let Some((scheme, rest)) = split_scheme(input) else {
            return Err(UrlError::Malformed);
        };
        if !scheme.eq_ignore_ascii_case("https") {
            return Err(UrlError::NotHttps {
                scheme: scheme.to_ascii_lowercase(),
            });
        }
        let Some(rest) = rest.strip_prefix("//") else {
            return Err(UrlError::Malformed);
        };
        let authority_end = rest.find(['/', '?']).unwrap_or(rest.len());
        let (authority, tail) = rest.split_at(authority_end);
        let host = parse_authority(authority)?;
        Ok(Self::build(host, &absolute_path(tail)))
    }

    /// Resolves a `Location` header value against this URL (RFC 3986 §5.2): absolute, scheme-relative (`//host/…`),
    /// absolute-path, query-only and relative-path references.
    pub fn join(&self, reference: &str) -> Result<Self, UrlError> {
        let reference = reference.trim_matches(is_ascii_ws);
        if reference.is_empty() {
            return Err(UrlError::Malformed);
        }
        let reference = without_fragment(reference);
        check_characters(reference)?;
        if split_scheme(reference).is_some() {
            return Self::parse(reference);
        }
        if reference.starts_with("//") {
            return Self::parse(&format!("https:{reference}"));
        }
        let (base_path, _) = split_query(&self.path_and_query);
        let merged = if reference.is_empty() {
            self.path_and_query.clone()
        } else if reference.starts_with('/') {
            reference.to_owned()
        } else if reference.starts_with('?') {
            format!("{base_path}{reference}")
        } else {
            let dir = &base_path[..base_path.rfind('/').map_or(0, |i| i + 1)];
            format!("{dir}{reference}")
        };
        Ok(Self::build(self.host.clone(), &absolute_path(&merged)))
    }

    pub fn host(&self) -> &Host {
        &self.host
    }

    pub fn path_and_query(&self) -> &str {
        &self.path_and_query
    }

    pub fn as_str(&self) -> &str {
        &self.full
    }

    fn build(host: Host, path_and_query: &str) -> Self {
        let path_and_query = encode(path_and_query);
        let full = format!("https://{host}{path_and_query}");
        Self {
            host,
            path_and_query,
            full,
        }
    }
}

impl fmt::Display for HttpsUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.full)
    }
}

fn is_ascii_ws(c: char) -> bool {
    c.is_ascii_whitespace()
}

fn without_fragment(s: &str) -> &str {
    s.split_once('#').map_or(s, |(before, _)| before)
}

/// Control characters (CR/LF in particular) never belong in a URL; spaces and non-ASCII are percent-encoded later.
fn check_characters(s: &str) -> Result<(), UrlError> {
    if s.chars().any(|c| c.is_control()) {
        return Err(UrlError::Malformed);
    }
    Ok(())
}

/// `scheme ":" rest` when the reference starts with a syntactically valid scheme (RFC 3986 §3.1).
fn split_scheme(s: &str) -> Option<(&str, &str)> {
    let colon = s.find(':')?;
    let scheme = &s[..colon];
    let mut chars = scheme.chars();
    let first = chars.next()?;
    if !first.is_ascii_alphabetic()
        || !chars.all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '-' | '.'))
    {
        return None;
    }
    Some((scheme, &s[colon + 1..]))
}

fn parse_authority(authority: &str) -> Result<Host, UrlError> {
    if authority.contains('@') {
        return Err(UrlError::UserInfo);
    }
    if authority.starts_with('[') {
        return Err(UrlError::Host(HostError::IpLiteral));
    }
    let (host, port) = match authority.rsplit_once(':') {
        Some((host, port)) => (host, port),
        None => (authority, ""),
    };
    if !(port.is_empty() || port == "443") {
        return Err(UrlError::Port(port.to_owned()));
    }
    Ok(Host::parse(host)?)
}

fn split_query(path_and_query: &str) -> (&str, &str) {
    match path_and_query.find('?') {
        Some(i) => path_and_query.split_at(i),
        None => (path_and_query, ""),
    }
}

/// Ensures a leading `/` and removes `.`/`..` segments from the path part (RFC 3986 §5.2.4).
fn absolute_path(path_and_query: &str) -> String {
    let (path, query) = split_query(path_and_query);
    let mut out: Vec<&str> = Vec::new();
    let segments: Vec<&str> = path.split('/').skip(1).collect();
    let last = segments.len().saturating_sub(1);
    for (i, segment) in segments.iter().enumerate() {
        match *segment {
            "." | ".." => {
                if *segment == ".." {
                    out.pop();
                }
                if i == last {
                    out.push("");
                }
            }
            s => out.push(s),
        }
    }
    let mut result = String::with_capacity(path_and_query.len() + 1);
    for segment in &out {
        result.push('/');
        result.push_str(segment);
    }
    if result.is_empty() {
        result.push('/');
    }
    result.push_str(query);
    result
}

/// Percent-encodes bytes that may not appear raw in a request target; existing `%XX` escapes are kept.
fn encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        let allowed = b.is_ascii_alphanumeric()
            || matches!(
                b,
                b'-' | b'.'
                    | b'_'
                    | b'~'
                    | b'!'
                    | b'$'
                    | b'&'
                    | b'\''
                    | b'('
                    | b')'
                    | b'*'
                    | b'+'
                    | b','
                    | b';'
                    | b'='
                    | b':'
                    | b'@'
                    | b'/'
                    | b'?'
                    | b'%'
            );
        if allowed {
            out.push(char::from(b));
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn url(s: &str) -> HttpsUrl {
        HttpsUrl::parse(s).unwrap_or_else(|e| panic!("{s}: {e}"))
    }

    #[test]
    fn hosts_are_lowercased_and_lose_the_trailing_dot() {
        let host = Host::parse("Files.GameBanana.com.").expect("valid");
        assert_eq!(host.as_str(), "files.gamebanana.com");
    }

    #[test]
    fn punycode_hosts_are_accepted_as_ascii() {
        let host = Host::parse("XN--80AK6AA92E.com").expect("valid");
        assert_eq!(host.as_str(), "xn--80ak6aa92e.com");
    }

    #[test]
    fn unusable_hosts_are_rejected() {
        let cases = [
            ("", HostError::Empty),
            (".", HostError::Empty),
            ("gamebаnana.com", HostError::NonAscii), // Cyrillic `а`
            ("127.0.0.1", HostError::IpLiteral),
            ("127.1", HostError::IpLiteral),
            ("2130706433", HostError::IpLiteral),
            ("0x7f.1", HostError::IpLiteral),
            ("[::1]", HostError::IpLiteral),
            ("game banana.com", HostError::InvalidCharacter(' ')),
            ("evil.net/gamebanana.com", HostError::InvalidCharacter('/')),
            ("under_score.com", HostError::InvalidCharacter('_')),
            ("a..b.com", HostError::InvalidLabel),
            ("-a.com", HostError::InvalidLabel),
            ("a-.com", HostError::InvalidLabel),
        ];
        for (input, expected) in cases {
            assert_eq!(Host::parse(input), Err(expected), "{input:?}");
        }
        assert_eq!(
            Host::parse(&format!("{}.com", "a".repeat(64))),
            Err(HostError::InvalidLabel)
        );
        let long = ["a".repeat(63).as_str(); 5].join(".");
        assert_eq!(Host::parse(&long), Err(HostError::TooLong));
    }

    #[test]
    fn parses_an_https_url() {
        let u = url("https://GameBanana.com/apiv11/Mod/Index?_nPage=2#top");
        assert_eq!(u.host().as_str(), "gamebanana.com");
        assert_eq!(u.path_and_query(), "/apiv11/Mod/Index?_nPage=2");
        assert_eq!(
            u.as_str(),
            "https://gamebanana.com/apiv11/Mod/Index?_nPage=2"
        );
    }

    #[test]
    fn an_empty_path_becomes_a_slash() {
        assert_eq!(
            url("https://gamebanana.com").as_str(),
            "https://gamebanana.com/"
        );
        assert_eq!(url("https://gamebanana.com?a=1").path_and_query(), "/?a=1");
    }

    #[test]
    fn the_default_port_is_accepted_and_dropped() {
        assert_eq!(
            url("https://gamebanana.com:443/x").as_str(),
            "https://gamebanana.com/x"
        );
        assert_eq!(
            url("https://gamebanana.com:/x").as_str(),
            "https://gamebanana.com/x"
        );
    }

    #[test]
    fn rejects_urls_that_are_not_plain_https() {
        let cases = [
            (
                "http://gamebanana.com/",
                UrlError::NotHttps {
                    scheme: "http".into(),
                },
            ),
            (
                "HTTP://gamebanana.com/",
                UrlError::NotHttps {
                    scheme: "http".into(),
                },
            ),
            (
                "ftp://gamebanana.com/",
                UrlError::NotHttps {
                    scheme: "ftp".into(),
                },
            ),
            (
                "javascript:alert(1)",
                UrlError::NotHttps {
                    scheme: "javascript".into(),
                },
            ),
            ("https://gamebanana.com@evil.net/", UrlError::UserInfo),
            ("https://user:pw@gamebanana.com/", UrlError::UserInfo),
            (
                "https://gamebanana.com:8443/",
                UrlError::Port("8443".into()),
            ),
            ("https://[::1]/", UrlError::Host(HostError::IpLiteral)),
            ("https://10.0.0.1/", UrlError::Host(HostError::IpLiteral)),
            ("https:gamebanana.com/", UrlError::Malformed),
            ("https:\\\\evil.net/", UrlError::Malformed),
            ("gamebanana.com/x", UrlError::Malformed),
            (
                "https://gamebanana.com/\r\nX-Injected: 1",
                UrlError::Malformed,
            ),
        ];
        for (input, expected) in cases {
            assert_eq!(HttpsUrl::parse(input), Err(expected), "{input:?}");
        }
    }

    #[test]
    fn dot_segments_are_removed() {
        assert_eq!(url("https://a.com/x/./y/../z").path_and_query(), "/x/z");
        assert_eq!(url("https://a.com/../../etc").path_and_query(), "/etc");
        assert_eq!(url("https://a.com/x/y/..").path_and_query(), "/x/");
    }

    #[test]
    fn unsafe_path_bytes_are_percent_encoded() {
        assert_eq!(
            url("https://files.gamebanana.com/mods/my mod ä.zip").path_and_query(),
            "/mods/my%20mod%20%C3%A4.zip"
        );
        assert_eq!(url("https://a.com/a%20b").path_and_query(), "/a%20b");
        assert_eq!(url("https://a.com/a\\b").path_and_query(), "/a%5Cb");
    }

    #[test]
    fn joins_an_absolute_location() {
        let base = url("https://gamebanana.com/dl/1234");
        let next = base
            .join("https://files.gamebanana.com/mods/skin.zip")
            .expect("absolute");
        assert_eq!(next.as_str(), "https://files.gamebanana.com/mods/skin.zip");
    }

    #[test]
    fn joins_relative_locations() {
        let base = url("https://files.gamebanana.com/mods/sub/skin.zip?x=1");
        let cases = [
            (
                "/mods/other.zip",
                "https://files.gamebanana.com/mods/other.zip",
            ),
            (
                "other.zip",
                "https://files.gamebanana.com/mods/sub/other.zip",
            ),
            ("../top.zip", "https://files.gamebanana.com/mods/top.zip"),
            ("?y=2", "https://files.gamebanana.com/mods/sub/skin.zip?y=2"),
            (
                "#frag",
                "https://files.gamebanana.com/mods/sub/skin.zip?x=1",
            ),
            (
                "//filecache07.gamebanana.com/mods/skin.zip",
                "https://filecache07.gamebanana.com/mods/skin.zip",
            ),
        ];
        for (location, expected) in cases {
            let joined = base
                .join(location)
                .unwrap_or_else(|e| panic!("{location}: {e}"));
            assert_eq!(joined.as_str(), expected, "{location}");
        }
    }

    #[test]
    fn a_relative_location_cannot_escape_the_host() {
        let base = url("https://gamebanana.com/dl/1");
        let joined = base.join("\\\\evil.net/x").expect("relative path");
        assert_eq!(joined.host().as_str(), "gamebanana.com");
    }

    #[test]
    fn join_reports_downgrades_and_garbage() {
        let base = url("https://gamebanana.com/dl/1");
        assert_eq!(
            base.join("http://files.gamebanana.com/mods/a.zip"),
            Err(UrlError::NotHttps {
                scheme: "http".into()
            })
        );
        assert_eq!(base.join("   "), Err(UrlError::Malformed));
        assert_eq!(
            base.join("https://gamebanana.com.evil.net@x.com/"),
            Err(UrlError::UserInfo)
        );
    }
}
