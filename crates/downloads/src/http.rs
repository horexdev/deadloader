//! The bits of HTTP a resumable download needs: `Content-Range` and the validators that tell whether the file on the
//! server is still the one we started (RFC 9110 §8.8, §13.1.5, §14.4).

use serde::{Deserialize, Serialize};

/// `Content-Range: bytes <first>-<last>/<complete length or *>`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContentRange {
    pub first: u64,
    pub last: u64,
    pub complete: Option<u64>,
}

pub fn parse_content_range(value: &str) -> Option<ContentRange> {
    let rest = value.trim().strip_prefix("bytes")?.trim_start();
    let (range, complete) = rest.split_once('/')?;
    let (first, last) = range.trim().split_once('-')?;
    let first = first.trim().parse().ok()?;
    let last = last.trim().parse().ok()?;
    if last < first {
        return None;
    }
    let complete = match complete.trim() {
        "*" => None,
        n => {
            let n: u64 = n.parse().ok()?;
            if last >= n {
                return None;
            }
            Some(n)
        }
    };
    Some(ContentRange {
        first,
        last,
        complete,
    })
}

/// What identifies the version of the file a `.part` holds the start of.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Validator {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
}

impl Validator {
    pub fn from_headers(header: impl Fn(&str) -> Option<String>) -> Option<Self> {
        let v = Self {
            etag: header("etag").map(|s| s.trim().to_owned()),
            last_modified: header("last-modified").map(|s| s.trim().to_owned()),
        };
        (v.etag.is_some() || v.last_modified.is_some()).then_some(v)
    }

    /// The `If-Range` value: a strong ETag, else the date. Weak ETags (`W/…`) are not allowed there.
    pub fn if_range(&self) -> Option<&str> {
        self.etag
            .as_deref()
            .filter(|e| !e.starts_with("W/"))
            .or(self.last_modified.as_deref())
    }

    /// Whether `other` (from a new response) clearly names a different version.
    pub fn conflicts_with(&self, other: &Self) -> bool {
        let differs =
            |a: &Option<String>, b: &Option<String>| matches!((a, b), (Some(a), Some(b)) if a != b);
        differs(&self.etag, &other.etag) || differs(&self.last_modified, &other.last_modified)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn content_ranges() {
        assert_eq!(
            parse_content_range("bytes 100-999/1000"),
            Some(ContentRange {
                first: 100,
                last: 999,
                complete: Some(1000)
            })
        );
        assert_eq!(
            parse_content_range("bytes 0-9/*"),
            Some(ContentRange {
                first: 0,
                last: 9,
                complete: None
            })
        );
        for bad in [
            "",
            "bytes */1000",
            "items 0-9/10",
            "bytes 9-0/10",
            "bytes 0-10/10",
            "bytes a-b/c",
        ] {
            assert_eq!(parse_content_range(bad), None, "{bad}");
        }
    }

    fn headers<'a>(pairs: &'a [(&'a str, &'a str)]) -> impl Fn(&str) -> Option<String> + 'a {
        move |name| {
            pairs
                .iter()
                .find(|(k, _)| k.eq_ignore_ascii_case(name))
                .map(|(_, v)| (*v).to_owned())
        }
    }

    #[test]
    fn validators_from_headers() {
        assert_eq!(Validator::from_headers(headers(&[])), None);
        let v = Validator::from_headers(headers(&[
            ("ETag", "\"abc\""),
            ("Last-Modified", "Tue, 13 Oct 2026 10:00:00 GMT"),
        ]))
        .expect("validator");
        assert_eq!(v.if_range(), Some("\"abc\""));
    }

    #[test]
    fn a_weak_etag_falls_back_to_the_date_for_if_range() {
        let v = Validator {
            etag: Some("W/\"abc\"".into()),
            last_modified: Some("date".into()),
        };
        assert_eq!(v.if_range(), Some("date"));
        let weak_only = Validator {
            etag: Some("W/\"abc\"".into()),
            last_modified: None,
        };
        assert_eq!(weak_only.if_range(), None);
    }

    #[test]
    fn conflicts_only_on_a_present_and_different_value() {
        let a = Validator {
            etag: Some("\"1\"".into()),
            last_modified: None,
        };
        let b = Validator {
            etag: Some("\"2\"".into()),
            last_modified: None,
        };
        let date = Validator {
            etag: None,
            last_modified: Some("d".into()),
        };
        assert!(a.conflicts_with(&b));
        assert!(!a.conflicts_with(&a.clone()));
        assert!(!a.conflicts_with(&date), "nothing to compare");
    }
}
