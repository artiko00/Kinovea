//! Text label with optional background, arrow and circle (`DrawingText`, KVA `Label`).
//! The arrow end defaults to 50 px up-left of the position, as the original constructor.

use super::{Header, KvaDrawing, RefValue};
use crate::kva_io::*;
use crate::style::default_style;
use kinovea_core::preferences::dom::Element;
use kinovea_core::xml::{parse_boolean, write_boolean};
use kinovea_geometry::PointF;

#[derive(Debug, Clone, PartialEq)]
pub struct Label {
    pub header: Header,
    pub text: String,
    /// Top-left of the text background.
    pub position: PointF,
    pub background_visible: bool,
    pub arrow_visible: bool,
    pub circle_visible: bool,
    pub arrow_end: PointF,
}

impl Label {
    pub fn new(text: &str, position: PointF) -> Self {
        Label {
            header: Header::new("", default_style("Label")),
            text: text.to_owned(),
            position,
            background_visible: true,
            arrow_visible: false,
            circle_visible: false,
            arrow_end: position.translate(-50.0, -50.0),
        }
    }
}

impl KvaDrawing for Label {
    const KVA_NAME: &'static str = "Label";

    fn header(&self) -> &Header {
        &self.header
    }

    fn header_mut(&mut self) -> &mut Header {
        &mut self.header
    }

    fn read_kva(e: &Element, scale: Scale, _map: TimestampMapper) -> Self {
        let mut d = Label::new("", PointF::EMPTY);
        d.header.read_attributes(e);
        for c in &e.children {
            let t = c.text.trim();
            match c.name.as_str() {
                "Text" => d.text = c.text.clone(),
                "Position" => d.position = read_point(c, scale),
                "BackgroundVisible" => d.background_visible = parse_boolean(t),
                "ArrowVisible" => d.arrow_visible = parse_boolean(t),
                "CircleVisible" => d.circle_visible = parse_boolean(t),
                "ArrowEnd" => d.arrow_end = read_point(c, Scale::IDENTITY),
                _ => {
                    read_style_or_fading(c, &mut d.header.style, &mut d.header.fading);
                }
            }
        }
        d
    }

    fn write_kva(&self, filter: Filter, _refs: RefValue) -> Vec<Element> {
        let mut out = Vec::new();
        if filter.core {
            out.push(Element::leaf("Text", &self.text));
            out.push(point_leaf("Position", self.position));
            out.push(Element::leaf(
                "BackgroundVisible",
                &write_boolean(self.background_visible),
            ));
            out.push(Element::leaf(
                "ArrowVisible",
                &write_boolean(self.arrow_visible),
            ));
            out.push(Element::leaf(
                "CircleVisible",
                &write_boolean(self.circle_visible),
            ));
            out.push(point_leaf("ArrowEnd", self.arrow_end));
        }
        push_style_and_fading(&mut out, filter, &self.header.style, &self.header.fading);
        out
    }

    fn content_hash(&self) -> i32 {
        let flags = i32::from(self.background_visible)
            | i32::from(self.arrow_visible) << 1
            | i32::from(self.circle_visible) << 2;
        self.position.hash_code()
            ^ self.arrow_end.hash_code()
            ^ flags
            ^ (self.text.len() as i32)
            ^ self.header.style.content_hash()
            ^ self.header.fading.content_hash()
    }
}
