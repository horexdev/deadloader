//! Checks shared by the rule tests and the XSS corpus: what the sanitiser may ever write.

use deadloader_html::Sanitized;

const TAGS: &[&str] = &[
    "p",
    "br",
    "hr",
    "b",
    "strong",
    "i",
    "em",
    "u",
    "s",
    "del",
    "code",
    "pre",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "ul",
    "ol",
    "li",
    "a",
    "blockquote",
    "table",
    "thead",
    "tbody",
    "tr",
    "th",
    "td",
    "span",
    "div",
];
const CLASSES: &[&str] = &["dl-green", "dl-red", "dl-spoiler", "dl-image", "dl-youtube"];
const NUMERIC: &[&str] = &[
    "data-dl-link",
    "data-dl-image",
    "start",
    "colspan",
    "rowspan",
];

/// Fails unless every tag in `out.html` is in the output vocabulary, every attribute is one we write, every value
/// is a number or our own class, and every referenced URL is absolute http(s).
pub fn assert_safe(input: &str, out: &Sanitized) {
    let html = &out.html;
    let bytes = html.as_bytes();
    let mut i = 0;
    while let Some(off) = html[i..].find('<') {
        let start = i + off;
        let end = start
            + html[start..]
                .find('>')
                .unwrap_or_else(|| panic!("unterminated tag in {html:?} (input {input:?})"));
        let tag = &html[start + 1..end];
        let tag = tag.strip_prefix('/').unwrap_or(tag);
        let (name, mut rest) = tag.split_once(' ').unwrap_or((tag, ""));
        assert!(
            TAGS.contains(&name),
            "tag {name:?} in {html:?} (input {input:?})"
        );
        while !rest.is_empty() {
            let (key, after) = rest
                .split_once("=\"")
                .unwrap_or_else(|| panic!("attribute {rest:?} in {html:?} (input {input:?})"));
            let (value, after) = after
                .split_once('"')
                .unwrap_or_else(|| panic!("unquoted {rest:?} in {html:?}"));
            match key {
                "class" => assert!(
                    value.split(' ').all(|c| CLASSES.contains(&c)),
                    "class {value:?} in {html:?} (input {input:?})"
                ),
                k if NUMERIC.contains(&k) => assert!(
                    !value.is_empty() && value.bytes().all(|b| b.is_ascii_digit() || b == b'-'),
                    "value {value:?} of {k} in {html:?}"
                ),
                other => panic!("attribute {other:?} in {html:?} (input {input:?})"),
            }
            rest = after.strip_prefix(' ').unwrap_or(after);
            assert!(
                after.is_empty() || after.starts_with(' '),
                "junk after {key} in {html:?}"
            );
        }
        i = end + 1;
    }
    assert!(!bytes.contains(&0), "NUL in output of {input:?}");
    for url in out
        .links
        .iter()
        .map(|l| &l.url)
        .chain(out.images.iter().map(|i| &i.url))
    {
        assert!(
            url.starts_with("https://") || url.starts_with("http://"),
            "url {url:?} from input {input:?}"
        );
    }
    for host in out
        .links
        .iter()
        .map(|l| &l.host)
        .chain(out.images.iter().map(|i| &i.host))
    {
        assert!(
            !host.is_empty()
                && host.bytes().all(|b| b.is_ascii_lowercase()
                    || b.is_ascii_digit()
                    || b == b'.'
                    || b == b'-'),
            "host {host:?} from input {input:?}"
        );
    }
    let link_refs = html.matches("data-dl-link=\"").count();
    let image_refs = html.matches("data-dl-image=\"").count();
    assert_eq!(link_refs, out.links.len(), "link refs in {html:?}");
    assert_eq!(image_refs, out.images.len(), "image refs in {html:?}");
}
