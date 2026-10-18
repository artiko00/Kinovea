//! Free quadrilateral drawn as a rectangle (`DrawingRectangle`).

use super::{Header, KvaDrawing, RefValue};
use crate::kva_io::*;
use crate::style::default_style;
use kinovea_core::preferences::dom::Element;
use kinovea_core::xml::{parse_boolean, write_boolean};
use kinovea_geometry::Quadrilateral;

#[derive(Debug, Clone, PartialEq)]
pub struct Rectangle {
    pub header: Header,
    pub quad: Quadrilateral,
    pub filled: bool,
}

pub(crate) const CORNER_NAMES: [&str; 4] = [
    "PointUpperLeft",
    "PointUpperRight",
    "PointLowerRight",
    "PointLowerLeft",
];

impl Rectangle {
    pub fn new(quad: Quadrilateral) -> Self {
        Rectangle {
            header: Header::new("", default_style("Rectangle")),
            quad,
            filled: false,
        }
    }
}

impl KvaDrawing for Rectangle {
    const KVA_NAME: &'static str = "Rectangle";

    fn header(&self) -> &Header {
        &self.header
    }

    fn header_mut(&mut self) -> &mut Header {
        &mut self.header
    }

    fn read_kva(e: &Element, scale: Scale, _map: TimestampMapper) -> Self {
        let mut d = Rectangle::new(Quadrilateral::empty());
        d.header.read_attributes(e);
        for c in &e.children {
            if let Some(i) = CORNER_NAMES.iter().position(|n| *n == c.name) {
                d.quad[i] = read_point(c, scale);
            } else if c.name == "Filled" {
                d.filled = parse_boolean(c.text.trim());
            } else {
                read_style_or_fading(c, &mut d.header.style, &mut d.header.fading);
            }
        }
        d
    }

    fn write_kva(&self, filter: Filter, _refs: RefValue) -> Vec<Element> {
        let mut out = Vec::new();
        if filter.core {
            for (name, p) in CORNER_NAMES.iter().zip(self.quad.vertices) {
                out.push(point_leaf(name, p));
            }
            out.push(Element::leaf("Filled", &write_boolean(self.filled)));
        }
        push_style_and_fading(&mut out, filter, &self.header.style, &self.header.fading);
        out
    }

    fn content_hash(&self) -> i32 {
        self.quad.hash_code()
            ^ i32::from(self.filled)
            ^ self.header.style.content_hash()
            ^ self.header.fading.content_hash()
    }
}
