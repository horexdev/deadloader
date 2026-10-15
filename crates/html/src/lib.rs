//! Turns a mod description written by its author (GameBanana HTML) into HTML the webview can show as is.
//!
//! The output is built from a fixed vocabulary rather than filtered from the input: only allowlisted tags, a few
//! numeric attributes, our own class names and index attributes are ever written, and all text is escaped. Nothing
//! in it can make the webview load or open anything:
//! - links become `<a data-dl-link="N">` without `href`; the UI asks the core to open [`Sanitized::links`]`[N]`
//!   (confirmation dialog D1, decision 31);
//! - images become empty `<span class="dl-image" data-dl-image="N"></span>` placeholders; the UI asks the core to
//!   load [`Sanitized::images`]`[N]` after the user's consent (dialog D2, decisions 29 and 122);
//! - YouTube embeds become `<span class="dl-youtube" data-dl-link="N"></span>` cards pointing at a link.
//!
//! User-visible words ("Image from …", "Spoiler") are not produced here: they belong to the UI and its i18n.

mod dom;
mod emit;
mod urls;

/// Inputs longer than this are cut (on a character boundary) before parsing; [`Sanitized::truncated`] says so.
/// Real descriptions are at most ~16 KB (R07).
pub const MAX_INPUT_BYTES: usize = 256 * 1024;

/// Safe HTML plus everything it refers to by index.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Sanitized {
    pub html: String,
    /// Targets of `data-dl-link="N"`, in document order.
    pub links: Vec<Link>,
    /// Targets of `data-dl-image="N"`, in document order.
    pub images: Vec<ImageRef>,
    /// Only the beginning of the input is shown: it was longer than [`MAX_INPUT_BYTES`], or nested or multiplied
    /// into a tree no real description needs (parsing stops there to bound memory and time).
    pub truncated: bool,
}

/// A link the user may ask to open.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Link {
    /// Absolute `http`/`https` URL, normalised (what the browser would be given).
    pub url: String,
    /// Lowercase ASCII host (IDN in punycode), for highlighting in the confirmation dialog.
    pub host: String,
    /// Set for `gamebanana.com/mods/<id>`: the UI can open that mod inside deadloader instead.
    pub internal_mod_id: Option<u64>,
}

/// An image the description shows; nothing is loaded until the user asks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageRef {
    /// Absolute `http`/`https` URL, normalised.
    pub url: String,
    /// Lowercase ASCII host (IDN in punycode), shown in the placeholder and the consent dialog.
    pub host: String,
    /// The author's `alt` text, if any and not blank.
    pub alt: Option<String>,
}

/// Sanitises one description. Never fails: whatever cannot be shown safely is dropped.
pub fn sanitize(html: &str) -> Sanitized {
    let (input, truncated) = clip(html, MAX_INPUT_BYTES);
    let dom = dom::parse_fragment(input);
    let mut out = emit::emit(&dom);
    out.truncated = truncated || dom.limited;
    out
}

/// The longest prefix of `text` of at most `max` bytes that ends on a character boundary.
fn clip(text: &str, max: usize) -> (&str, bool) {
    if text.len() <= max {
        return (text, false);
    }
    let mut end = max;
    while !text.is_char_boundary(end) {
        end -= 1;
    }
    (&text[..end], true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clip_keeps_short_text() {
        assert_eq!(clip("abc", 3), ("abc", false));
    }

    #[test]
    fn clip_cuts_on_a_character_boundary() {
        // "é" is two bytes: a cut after 2 bytes would split it.
        assert_eq!(clip("aéb", 2), ("a", true));
        assert_eq!(clip("aéb", 3), ("aé", true));
    }
}
