//! SHA-256 digests as lowercase hex strings in JSON (journal, manifest): readable and diff-friendly.

use serde::{Deserialize, Deserializer, Serializer, de::Error};

pub fn serialize<S: Serializer>(bytes: &[u8; 32], s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(&bytes.iter().map(|b| format!("{b:02x}")).collect::<String>())
}

pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<[u8; 32], D::Error> {
    let text = String::deserialize(d)?;
    let mut out = [0u8; 32];
    if text.len() != 64 || !text.is_ascii() {
        return Err(D::Error::custom("expected 64 hex digits"));
    }
    for (i, byte) in out.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&text[i * 2..i * 2 + 2], 16).map_err(D::Error::custom)?;
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    #[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq)]
    struct Holder(#[serde(with = "super")] [u8; 32]);

    #[test]
    fn round_trips() {
        let h = Holder([0xAB; 32]);
        let json = serde_json::to_string(&h).expect("serialise");
        assert_eq!(json, format!("\"{}\"", "ab".repeat(32)));
        assert_eq!(serde_json::from_str::<Holder>(&json).expect("parse"), h);
    }

    #[test]
    fn rejects_wrong_length_and_non_hex() {
        assert!(serde_json::from_str::<Holder>("\"abcd\"").is_err());
        assert!(serde_json::from_str::<Holder>(&format!("\"{}\"", "zz".repeat(32))).is_err());
    }

    #[test]
    fn rejects_non_ascii_without_panicking() {
        // 64 bytes whose two-byte characters start at odd offsets: slicing in pairs would split one.
        let text = format!("\"a{}a\"", "é".repeat(31));
        assert!(serde_json::from_str::<Holder>(&text).is_err());
    }
}
