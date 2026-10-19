//! Minimal XML element tree used to read and write `Preferences.xml` with the same shape
//! as the .NET `XmlWriter` (two-space indentation, `<Empty />` for empty elements).

use quick_xml::events::Event;
use quick_xml::Reader;
use std::fmt::Write as _;

/// An XML element: name, attributes, text content and child elements.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Element {
    pub name: String,
    pub attributes: Vec<(String, String)>,
    pub text: String,
    pub children: Vec<Element>,
    /// Written as `<X></X>` even when empty; `<X />` otherwise (an XSLT `value-of` does this).
    pub end_tag: bool,
}

/// Error raised when the XML text is malformed.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("invalid xml: {0}")]
pub struct XmlError(pub String);

impl Element {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            ..Self::default()
        }
    }

    /// A leaf element with text content (`<Name>text</Name>`).
    pub fn leaf(name: &str, text: &str) -> Self {
        Self {
            name: name.to_owned(),
            text: text.to_owned(),
            ..Self::default()
        }
    }

    pub fn push(&mut self, child: Element) -> &mut Self {
        self.children.push(child);
        self
    }

    pub fn push_leaf(&mut self, name: &str, text: &str) -> &mut Self {
        self.push(Element::leaf(name, text))
    }

    pub fn child(&self, name: &str) -> Option<&Element> {
        self.children.iter().find(|c| c.name == name)
    }

    pub fn child_text(&self, name: &str) -> Option<&str> {
        self.child(name).map(|c| c.text.as_str())
    }

    pub fn attribute(&self, name: &str) -> Option<&str> {
        let found = self.attributes.iter().find(|(k, _)| k == name);
        found.map(|(_, v)| v.as_str())
    }

    /// Parses a document and returns its root element.
    pub fn parse(xml: &str) -> Result<Element, XmlError> {
        let mut reader = Reader::from_str(xml);
        let mut stack: Vec<Element> = Vec::new();
        let err = |e: quick_xml::Error| XmlError(e.to_string());
        loop {
            match reader.read_event().map_err(err)? {
                Event::Start(start) => stack.push(element_from_start(&start)?),
                Event::Empty(start) => {
                    let element = element_from_start(&start)?;
                    match stack.last_mut() {
                        Some(parent) => parent.children.push(element),
                        None => return Ok(element),
                    }
                }
                Event::Text(text) => {
                    // Whitespace between elements is layout, not content (`IgnoreWhitespace`).
                    // XML line-ending normalization, as `XmlReader` does.
                    let text = text.unescape().map_err(err)?.replace("\r\n", "\n").replace('\r', "\n");
                    if let Some(current) = stack.last_mut().filter(|_| !text.trim().is_empty()) {
                        current.text.push_str(&text);
                    }
                }
                Event::End(_) => {
                    let element = stack
                        .pop()
                        .ok_or_else(|| XmlError("unbalanced end tag".into()))?;
                    match stack.last_mut() {
                        Some(parent) => parent.children.push(element),
                        None => return Ok(element),
                    }
                }
                Event::Eof => return Err(XmlError("no root element".into())),
                _ => {}
            }
        }
    }

    /// Serializes the element as a full document in the .NET `XmlWriter` style: declaration,
    /// two-space indentation and no newline after the root end tag.
    pub fn to_document(&self) -> String {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n");
        self.write(&mut out, 0);
        out.pop();
        out
    }

    fn write(&self, out: &mut String, depth: usize) {
        let indent = "  ".repeat(depth);
        let _ = write!(out, "{indent}<{}", self.name);
        for (k, v) in &self.attributes {
            let _ = write!(out, " {k}=\"{}\"", escape(v, true));
        }
        if self.children.is_empty() && self.text.is_empty() && !self.end_tag {
            out.push_str(" />\n");
        } else if self.children.is_empty() {
            let _ = writeln!(out, ">{}</{}>", escape(&self.text, false), self.name);
        } else {
            out.push_str(">\n");
            for child in &self.children {
                child.write(out, depth + 1);
            }
            let _ = writeln!(out, "{indent}</{}>", self.name);
        }
    }
}

fn element_from_start(start: &quick_xml::events::BytesStart<'_>) -> Result<Element, XmlError> {
    let name = String::from_utf8_lossy(start.name().as_ref()).into_owned();
    let mut element = Element::new(&name);
    for attr in start.attributes() {
        let attr = attr.map_err(|e| XmlError(e.to_string()))?;
        let key = String::from_utf8_lossy(attr.key.as_ref()).into_owned();
        let value = attr.unescape_value().map_err(|e| XmlError(e.to_string()))?;
        let value = value.into_owned();
        element.attributes.push((key, value));
    }
    Ok(element)
}

fn escape(text: &str, attribute: bool) -> String {
    let mut s = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '&' => s.push_str("&amp;"),
            '<' => s.push_str("&lt;"),
            '>' => s.push_str("&gt;"),
            '"' if attribute => s.push_str("&quot;"),
            _ => s.push(c),
        }
    }
    s
}
