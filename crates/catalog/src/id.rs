//! Opaque image ids: what the UI puts into image URLs instead of a remote address.

use deadloader_core::fs::sha256;
use serde::{Serialize, Serializer};
use std::fmt;

/// An image the catalog issued: 128 bits of the SHA-256 of its kind and remote URL, written as 32 lowercase hex
/// digits. The id says nothing about the URL, and only ids the catalog handed out resolve, so the image protocol can
/// never be turned into a proxy for an address of the webview's choosing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ImageId([u8; 16]);

impl ImageId {
    pub(crate) fn of(kind_tag: &str, url: &str) -> Self {
        let mut input = Vec::with_capacity(kind_tag.len() + 1 + url.len());
        input.extend_from_slice(kind_tag.as_bytes());
        input.push(b'\n');
        input.extend_from_slice(url.as_bytes());
        let digest = sha256(&input);
        let mut id = [0; 16];
        id.copy_from_slice(&digest[..16]);
        Self(id)
    }

    /// Exactly 32 lowercase hex digits; anything else is not an id.
    pub fn parse(text: &str) -> Option<Self> {
        let bytes = text.as_bytes();
        if bytes.len() != 32 {
            return None;
        }
        let nibble = |b: u8| match b {
            b'0'..=b'9' => Some(b - b'0'),
            b'a'..=b'f' => Some(b - b'a' + 10),
            _ => None,
        };
        let mut id = [0; 16];
        for (i, pair) in bytes.chunks_exact(2).enumerate() {
            id[i] = (nibble(pair[0])? << 4) | nibble(pair[1])?;
        }
        Some(Self(id))
    }
}

impl fmt::Display for ImageId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.0 {
            write!(f, "{byte:02x}")?;
        }
        Ok(())
    }
}

impl Serialize for ImageId {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.collect_str(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_through_text() {
        let id = ImageId::of("thumb", "https://images.gamebanana.com/a.jpg");
        let text = id.to_string();
        assert_eq!(text.len(), 32);
        assert_eq!(ImageId::parse(&text), Some(id));
        assert_eq!(
            serde_json::to_value(id).expect("json"),
            serde_json::Value::String(text)
        );
    }

    #[test]
    fn the_kind_is_part_of_the_id() {
        let url = "https://images.gamebanana.com/a.jpg";
        assert_ne!(ImageId::of("thumb", url), ImageId::of("description", url));
        assert_eq!(ImageId::of("thumb", url), ImageId::of("thumb", url));
    }

    #[test]
    fn only_lowercase_hex_of_the_right_length_parses() {
        let good = "00ff".repeat(8);
        assert!(ImageId::parse(&good).is_some());
        for bad in [
            String::new(),
            "00FF".repeat(8),
            "0g".repeat(16),
            "00ff".repeat(8) + "0",
            "../".repeat(11),
            "é".repeat(16),
        ] {
            assert_eq!(ImageId::parse(&bad), None, "{bad:?}");
        }
    }
}
