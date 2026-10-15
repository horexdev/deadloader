//! A minimal arena DOM for html5ever: just enough tree to walk once and throw away.
//!
//! The parser is fed in small chunks and stopped once the tree grows past [`MAX_NODES`] or [`MAX_DEPTH`]: HTML
//! parsing can multiply input (misnested formatting tags are re-created for every paragraph), so input size alone
//! does not bound memory or time.

use std::borrow::Cow;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

use html5ever::interface::{ElementFlags, NodeOrText, QuirksMode, TreeSink};
use html5ever::tendril::{StrTendril, TendrilSink};
use html5ever::{
    Attribute, LocalName, ParseOpts, QualName, local_name, ns, parse_fragment as html5ever_fragment,
};

/// More nodes than any real description has (a 16 KB one has a few thousand).
pub const MAX_NODES: usize = 100_000;
/// Deeper than any real description nests; the parser's own work per tag grows with depth.
pub const MAX_DEPTH: usize = 512;
/// Input is fed in pieces this small so that a limit stops the parser soon after it is reached.
const CHUNK_BYTES: usize = 32;

pub type NodeId = usize;

#[derive(Debug)]
pub enum Data {
    Document,
    Element {
        name: Rc<QualName>,
        attrs: Vec<Attribute>,
    },
    Text(String),
    /// Comments, processing instructions, template contents: never shown.
    Other,
}

#[derive(Debug)]
pub struct Node {
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
    pub data: Data,
    depth: usize,
}

#[derive(Debug)]
pub struct Dom {
    pub nodes: Vec<Node>,
    /// The fragment's content lives under this node.
    pub root: Option<NodeId>,
    /// Parsing stopped early at [`MAX_NODES`] or [`MAX_DEPTH`].
    pub limited: bool,
}

/// Parses `html` as the content of a `<body>`, with scripting disabled (so `<noscript>` content is markup, as the
/// sanitiser expects).
pub fn parse_fragment(html: &str) -> Dom {
    let context = QualName::new(None, ns!(html), local_name!("body"));
    let mut parser = html5ever_fragment(
        Sink::default(),
        ParseOpts::default(),
        context,
        Vec::new(),
        false,
    );
    let mut rest = html;
    while !rest.is_empty() && !parser.tokenizer.sink.sink.limited.get() {
        let mut end = CHUNK_BYTES.min(rest.len());
        while !rest.is_char_boundary(end) {
            end += 1;
        }
        parser.process(StrTendril::from_slice(&rest[..end]));
        rest = &rest[end..];
    }
    parser.finish()
}

#[derive(Clone, Debug)]
pub struct Handle {
    id: NodeId,
    name: Option<Rc<QualName>>,
}

#[derive(Debug)]
struct Sink {
    nodes: RefCell<Vec<Node>>,
    /// The template contents created for a template element, by element id.
    templates: RefCell<Vec<(NodeId, NodeId)>>,
    limited: Cell<bool>,
    /// What `elem_name` answers for a non-element, which the parser promises never to ask about.
    no_name: QualName,
}

impl Default for Sink {
    fn default() -> Self {
        Self {
            nodes: RefCell::new(vec![Node {
                parent: None,
                children: Vec::new(),
                data: Data::Document,
                depth: 0,
            }]),
            templates: RefCell::new(Vec::new()),
            limited: Cell::new(false),
            no_name: QualName::new(None, ns!(), LocalName::from("")),
        }
    }
}

impl Sink {
    fn new_node(&self, data: Data) -> NodeId {
        let mut nodes = self.nodes.borrow_mut();
        nodes.push(Node {
            parent: None,
            children: Vec::new(),
            data,
            depth: 0,
        });
        if nodes.len() > MAX_NODES {
            self.limited.set(true);
        }
        nodes.len() - 1
    }

    fn detach(nodes: &mut [Node], id: NodeId) {
        if let Some(parent) = nodes[id].parent.take() {
            nodes[parent].children.retain(|&c| c != id);
        }
    }

    fn adopt(&self, nodes: &mut [Node], parent: NodeId, child: NodeId) {
        nodes[child].parent = Some(parent);
        nodes[child].depth = nodes[parent].depth + 1;
        if nodes[child].depth > MAX_DEPTH {
            self.limited.set(true);
        }
    }

    /// Inserts `child` into `parent` at `index` (or last), merging adjacent text as the parser requires.
    fn insert(&self, parent: NodeId, index: Option<usize>, child: NodeOrText<Handle>) {
        let at = index.unwrap_or_else(|| self.nodes.borrow()[parent].children.len());
        match child {
            NodeOrText::AppendText(text) => {
                let mut nodes = self.nodes.borrow_mut();
                let before = at.checked_sub(1).map(|i| nodes[parent].children[i]);
                if let Some(prev) = before
                    && let Data::Text(existing) = &mut nodes[prev].data
                {
                    existing.push_str(&text);
                    return;
                }
                drop(nodes);
                let id = self.new_node(Data::Text(text.to_string()));
                let mut nodes = self.nodes.borrow_mut();
                nodes[parent].children.insert(at, id);
                self.adopt(&mut nodes, parent, id);
            }
            NodeOrText::AppendNode(handle) => {
                let mut nodes = self.nodes.borrow_mut();
                let id = handle.id;
                let mut at = at;
                // Only a node moved by the parser has a parent here; a new one is appended in O(1).
                if nodes[id].parent == Some(parent)
                    && nodes[parent]
                        .children
                        .iter()
                        .position(|&c| c == id)
                        .is_some_and(|old| old < at)
                {
                    at -= 1;
                }
                if nodes[id].parent.is_some() {
                    Self::detach(&mut nodes, id);
                }
                nodes[parent].children.insert(at, id);
                self.adopt(&mut nodes, parent, id);
            }
        }
    }
}

impl TreeSink for Sink {
    type Handle = Handle;
    type Output = Dom;
    type ElemName<'a> = &'a QualName;

    fn finish(self) -> Dom {
        let nodes = self.nodes.into_inner();
        let root = nodes[0]
            .children
            .iter()
            .copied()
            .find(|&c| matches!(nodes[c].data, Data::Element { .. }));
        Dom {
            nodes,
            root,
            limited: self.limited.get(),
        }
    }

    fn parse_error(&self, _msg: Cow<'static, str>) {}

    fn get_document(&self) -> Handle {
        Handle { id: 0, name: None }
    }

    fn elem_name<'a>(&'a self, target: &'a Handle) -> &'a QualName {
        target.name.as_deref().unwrap_or(&self.no_name)
    }

    fn create_element(&self, name: QualName, attrs: Vec<Attribute>, flags: ElementFlags) -> Handle {
        let name = Rc::new(name);
        let id = self.new_node(Data::Element {
            name: Rc::clone(&name),
            attrs,
        });
        if flags.template {
            let contents = self.new_node(Data::Other);
            self.templates.borrow_mut().push((id, contents));
        }
        Handle {
            id,
            name: Some(name),
        }
    }

    fn create_comment(&self, _text: StrTendril) -> Handle {
        Handle {
            id: self.new_node(Data::Other),
            name: None,
        }
    }

    fn create_pi(&self, _target: StrTendril, _data: StrTendril) -> Handle {
        Handle {
            id: self.new_node(Data::Other),
            name: None,
        }
    }

    fn append(&self, parent: &Handle, child: NodeOrText<Handle>) {
        self.insert(parent.id, None, child);
    }

    fn append_based_on_parent_node(
        &self,
        element: &Handle,
        prev_element: &Handle,
        child: NodeOrText<Handle>,
    ) {
        if self.nodes.borrow()[element.id].parent.is_some() {
            self.append_before_sibling(element, child);
        } else {
            self.append(prev_element, child);
        }
    }

    fn append_doctype_to_document(
        &self,
        _name: StrTendril,
        _public_id: StrTendril,
        _system_id: StrTendril,
    ) {
    }

    fn get_template_contents(&self, target: &Handle) -> Handle {
        let contents = self
            .templates
            .borrow()
            .iter()
            .find(|(element, _)| *element == target.id)
            .map(|&(_, contents)| contents);
        // The parser only asks about template elements; anything else gets a detached node of its own.
        let id = contents.unwrap_or_else(|| self.new_node(Data::Other));
        Handle { id, name: None }
    }

    fn same_node(&self, x: &Handle, y: &Handle) -> bool {
        x.id == y.id
    }

    fn set_quirks_mode(&self, _mode: QuirksMode) {}

    fn append_before_sibling(&self, sibling: &Handle, new_node: NodeOrText<Handle>) {
        let place = {
            let nodes = self.nodes.borrow();
            nodes[sibling.id].parent.and_then(|parent| {
                let index = nodes[parent]
                    .children
                    .iter()
                    .position(|&c| c == sibling.id)?;
                Some((parent, index))
            })
        };
        if let Some((parent, index)) = place {
            self.insert(parent, Some(index), new_node);
        }
    }

    fn add_attrs_if_missing(&self, target: &Handle, attrs: Vec<Attribute>) {
        let mut nodes = self.nodes.borrow_mut();
        if let Data::Element {
            attrs: existing, ..
        } = &mut nodes[target.id].data
        {
            for attr in attrs {
                if !existing.iter().any(|e| e.name == attr.name) {
                    existing.push(attr);
                }
            }
        }
    }

    fn remove_from_parent(&self, target: &Handle) {
        Self::detach(&mut self.nodes.borrow_mut(), target.id);
    }

    fn reparent_children(&self, node: &Handle, new_parent: &Handle) {
        let mut nodes = self.nodes.borrow_mut();
        let children = std::mem::take(&mut nodes[node.id].children);
        for &child in &children {
            self.adopt(&mut nodes, new_parent.id, child);
        }
        nodes[new_parent.id].children.extend(children);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The tree as nested tag names and quoted text, e.g. `p(b("x"))"y"`.
    fn shape(dom: &Dom) -> String {
        fn walk(dom: &Dom, id: NodeId, out: &mut String) {
            for &child in &dom.nodes[id].children {
                assert_eq!(dom.nodes[child].parent, Some(id), "parent link of {child}");
                match &dom.nodes[child].data {
                    Data::Element { name, .. } => {
                        out.push_str(&name.local);
                        out.push('(');
                        walk(dom, child, out);
                        out.push(')');
                    }
                    Data::Text(text) => {
                        out.push('"');
                        out.push_str(text);
                        out.push('"');
                    }
                    Data::Other => out.push('#'),
                    Data::Document => out.push('?'),
                }
            }
        }
        let mut out = String::new();
        if let Some(root) = dom.root {
            walk(dom, root, &mut out);
        }
        out
    }

    #[test]
    fn parses_a_fragment_under_one_root() {
        let dom = parse_fragment("<p>a<b>b</b></p>c<!-- x -->");
        assert_eq!(shape(&dom), "p(\"a\"b(\"b\"))\"c\"#");
        assert!(!dom.limited);
    }

    #[test]
    fn text_split_across_chunks_is_one_node() {
        let text = "x".repeat(CHUNK_BYTES * 3 + 1);
        let dom = parse_fragment(&text);
        assert_eq!(shape(&dom), format!("\"{text}\""));
    }

    #[test]
    fn multibyte_text_survives_chunking() {
        let text = "é".repeat(CHUNK_BYTES * 2);
        let dom = parse_fragment(&format!("<p>{text}</p>"));
        assert_eq!(shape(&dom), format!("p(\"{text}\")"));
    }

    #[test]
    fn misnested_formatting_is_repaired() {
        // The adoption agency algorithm: moves and re-creates nodes.
        let dom = parse_fragment("<b>1<p>2</b>3</p>");
        assert_eq!(shape(&dom), "b(\"1\")p(b(\"2\")\"3\")");
    }

    #[test]
    fn table_content_is_foster_parented() {
        let dom = parse_fragment("<table>x<tr><td>y</td></tr></table>");
        assert_eq!(shape(&dom), "\"x\"table(tbody(tr(td(\"y\"))))");
    }

    #[test]
    fn template_content_stays_out_of_the_tree() {
        let dom = parse_fragment("<template><b>t</b></template>v");
        assert_eq!(shape(&dom), "template()\"v\"");
    }

    #[test]
    fn duplicate_attributes_keep_the_first() {
        let dom = parse_fragment("<p class=\"a\" class=\"b\">x</p>");
        let root = dom.root.expect("root");
        let p = dom.nodes[root].children[0];
        let Data::Element { attrs, .. } = &dom.nodes[p].data else {
            panic!("not an element");
        };
        assert_eq!(attrs.len(), 1);
        assert_eq!(&*attrs[0].value, "a");
    }

    #[test]
    fn stops_at_the_depth_limit() {
        let depth = MAX_DEPTH * 4;
        let dom = parse_fragment(&format!("{}x", "<div>".repeat(depth)));
        assert!(dom.limited);
        assert!(!shape(&dom).contains('x'));
    }

    #[test]
    fn stops_at_the_node_limit_when_formatting_multiplies() {
        // Every new paragraph closes the formatting elements, and its text re-creates all of them: 400 x 400 nodes
        // unchecked, from 5 KB of input. Distinct attributes defeat the parser's own three-copies rule.
        let input = format!(
            "<p>{}{}",
            (0..400).map(|i| format!("<b a{i}>")).collect::<String>(),
            "<p>x".repeat(400)
        );
        let dom = parse_fragment(&input);
        assert!(dom.limited);
        assert!(
            dom.nodes.len() < MAX_NODES + 50_000,
            "{} nodes",
            dom.nodes.len()
        );
    }
}
