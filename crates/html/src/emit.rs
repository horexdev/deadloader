//! Writes the safe HTML from the parsed tree, collecting the links and images it refers to.
//!
//! The walk is iterative: a hostile description must not be able to exhaust the stack.

use html5ever::{Attribute, LocalName, local_name, ns};

use crate::dom::{Data, Dom, NodeId};
use crate::{ImageRef, Sanitized, urls};

/// Kept as they are (with only the attributes [`write_attrs`] allows).
const KEPT: &[&str] = &[
    "p",
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

/// Dropped together with everything inside. Other unknown elements are dropped keeping their content.
const REMOVED: &[&str] = &[
    "script", "style", "iframe", "frame", "frameset", "object", "embed", "applet", "param", "svg",
    "math", "form", "input", "button", "textarea", "select", "option", "optgroup", "datalist",
    "template", "noscript", "noembed", "noframes", "title", "audio", "video", "canvas", "picture",
    "source", "track", "map", "area", "portal",
];

enum Step {
    Enter(NodeId),
    Close(&'static str),
    CloseLink,
}

pub fn emit(dom: &Dom) -> Sanitized {
    let mut writer = Writer {
        dom,
        out: Sanitized::default(),
        stack: Vec::new(),
        in_link: false,
    };
    if let Some(root) = dom.root {
        writer.push_children(root);
    }
    while let Some(step) = writer.stack.pop() {
        match step {
            Step::Enter(id) => writer.enter(id),
            Step::Close(tag) => writer.close(tag),
            Step::CloseLink => {
                writer.close("a");
                writer.in_link = false;
            }
        }
    }
    writer.out
}

struct Writer<'a> {
    dom: &'a Dom,
    out: Sanitized,
    stack: Vec<Step>,
    /// Inside an emitted link: a nested one would be split apart by the webview's parser, so it is not emitted.
    in_link: bool,
}

impl Writer<'_> {
    fn push_children(&mut self, id: NodeId) {
        let children = &self.dom.nodes[id].children;
        self.stack
            .extend(children.iter().rev().map(|&c| Step::Enter(c)));
    }

    fn enter(&mut self, id: NodeId) {
        match &self.dom.nodes[id].data {
            Data::Text(text) => escape_text(&mut self.out.html, text),
            Data::Element { name, attrs } => {
                if name.ns != ns!(html) {
                    return; // SVG and MathML, with their content.
                }
                self.element(id, &name.local, attrs);
            }
            Data::Document | Data::Other => {}
        }
    }

    fn element(&mut self, id: NodeId, tag: &LocalName, attrs: &[Attribute]) {
        match &**tag {
            "a" => self.link(id, attrs),
            "img" => self.image(attrs),
            "iframe" => self.embed(attrs),
            "br" => self.out.html.push_str("<br>"),
            "hr" => self.out.html.push_str("<hr>"),
            name if REMOVED.contains(&name) => {}
            name => match KEPT.iter().find(|&&kept| kept == name) {
                Some(&kept) => self.kept(id, kept, attrs),
                None => self.push_children(id),
            },
        }
    }

    fn kept(&mut self, id: NodeId, tag: &'static str, attrs: &[Attribute]) {
        let html = &mut self.out.html;
        html.push('<');
        html.push_str(tag);
        write_class(html, attrs);
        match tag {
            "ol" => {
                if let Some(start) = attr(attrs, local_name!("start"))
                    .and_then(|v| integer(v, i64::from(i32::MIN), i64::from(i32::MAX)))
                {
                    html.push_str(&format!(" start=\"{start}\""));
                }
            }
            "td" | "th" => {
                if let Some(n) =
                    attr(attrs, local_name!("colspan")).and_then(|v| integer(v, 1, 1000))
                {
                    html.push_str(&format!(" colspan=\"{n}\""));
                }
                if let Some(n) =
                    attr(attrs, local_name!("rowspan")).and_then(|v| integer(v, 1, 65534))
                {
                    html.push_str(&format!(" rowspan=\"{n}\""));
                }
            }
            _ => {}
        }
        html.push('>');
        if tag == "pre" && self.starts_with_newline(id) {
            // The parser drops one newline right after <pre>; keep the author's.
            self.out.html.push('\n');
        }
        self.stack.push(Step::Close(tag));
        self.push_children(id);
    }

    fn link(&mut self, id: NodeId, attrs: &[Attribute]) {
        let target = attr(attrs, local_name!("href")).and_then(urls::link);
        match target {
            Some(link) if !self.in_link => {
                let html = &mut self.out.html;
                html.push_str("<a");
                write_class(html, attrs);
                html.push_str(&format!(" data-dl-link=\"{}\">", self.out.links.len()));
                self.out.links.push(link);
                self.in_link = true;
                self.stack.push(Step::CloseLink);
                self.push_children(id);
            }
            _ => self.push_children(id),
        }
    }

    fn image(&mut self, attrs: &[Attribute]) {
        let Some((host, url)) = attr(attrs, local_name!("src")).and_then(urls::image) else {
            return;
        };
        let alt = attr(attrs, local_name!("alt"))
            .map(str::trim)
            .filter(|alt| !alt.is_empty())
            .map(str::to_owned);
        self.out.html.push_str(&format!(
            "<span class=\"dl-image\" data-dl-image=\"{}\"></span>",
            self.out.images.len()
        ));
        self.out.images.push(ImageRef { url, host, alt });
    }

    fn embed(&mut self, attrs: &[Attribute]) {
        let Some(link) = attr(attrs, local_name!("src")).and_then(urls::youtube) else {
            return;
        };
        self.out.html.push_str(&format!(
            "<span class=\"dl-youtube\" data-dl-link=\"{}\"></span>",
            self.out.links.len()
        ));
        self.out.links.push(link);
    }

    fn close(&mut self, tag: &str) {
        self.out.html.push_str("</");
        self.out.html.push_str(tag);
        self.out.html.push('>');
    }

    fn starts_with_newline(&self, id: NodeId) -> bool {
        self.dom.nodes[id].children.first().is_some_and(
            |&c| matches!(&self.dom.nodes[c].data, Data::Text(t) if t.starts_with('\n')),
        )
    }
}

/// The value of a plain (no-namespace) attribute.
fn attr(attrs: &[Attribute], name: LocalName) -> Option<&str> {
    attrs
        .iter()
        .find(|a| a.name.ns == ns!() && a.name.local == name)
        .map(|a| &*a.value)
}

/// GameBanana's own classes, as ours, in a fixed order; everything else is dropped.
fn write_class(html: &mut String, attrs: &[Attribute]) {
    let Some(value) = attr(attrs, local_name!("class")) else {
        return;
    };
    let has = |class: &str| value.split_ascii_whitespace().any(|c| c == class);
    let ours: Vec<&str> = [
        ("GreenColor", "dl-green"),
        ("RedColor", "dl-red"),
        ("Spoiler", "dl-spoiler"),
    ]
    .iter()
    .filter(|(theirs, _)| has(theirs))
    .map(|&(_, ours)| ours)
    .collect();
    if !ours.is_empty() {
        html.push_str(" class=\"");
        html.push_str(&ours.join(" "));
        html.push('"');
    }
}

/// A plain decimal integer (optional minus sign) within `min..=max`.
fn integer(value: &str, min: i64, max: i64) -> Option<i64> {
    let value = value.trim_ascii();
    let digits = value.strip_prefix('-').unwrap_or(value);
    if digits.is_empty() || digits.len() > 10 || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    value.parse().ok().filter(|n| (min..=max).contains(n))
}

/// Text content: markup characters escaped, NUL replaced (the parser already does; this does not rely on it).
fn escape_text(html: &mut String, text: &str) {
    for c in text.chars() {
        match c {
            '&' => html.push_str("&amp;"),
            '<' => html.push_str("&lt;"),
            '>' => html.push_str("&gt;"),
            '\0' => html.push('\u{FFFD}'),
            c => html.push(c),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integers() {
        assert_eq!(integer(" 12 ", 1, 100), Some(12));
        assert_eq!(integer("-3", -5, 5), Some(-3));
        for bad in ["", "-", "+1", "1.5", "1e3", "0x10", "12345678901", "101"] {
            assert_eq!(integer(bad, -5, 100), None, "{bad:?}");
        }
    }

    #[test]
    fn text_escaping() {
        let mut html = String::new();
        escape_text(&mut html, "<a href=\"x\">&amp;\0</a>");
        assert_eq!(html, "&lt;a href=\"x\"&gt;&amp;amp;\u{FFFD}&lt;/a&gt;");
    }
}
