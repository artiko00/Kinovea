//! Drawings that exist once per analysis: the coordinate system and the test grid.
//! Spotlights and the number sequence live in [`crate::multi`].

use crate::color::Color;
use crate::drawing::{Header, RefValue};
use crate::kva_io::*;
use crate::style::{StyleElements, StyleValue};
use kinovea_core::preferences::dom::Element;
use kinovea_core::xml::{parse_boolean, write_boolean};
use kinovea_geometry::PointF;

/// World origin and axes (`DrawingCoordinateSystem`).
#[derive(Debug, Clone, PartialEq)]
pub struct CoordinateSystem {
    pub header: Header,
    pub origin: PointF,
    pub visible: bool,
    pub show_grid: bool,
    pub show_graduations: bool,
}

impl Default for CoordinateSystem {
    fn default() -> Self {
        let style = StyleElements::new(vec![("line color", StyleValue::Color(Color::rgb(255, 0, 0)))]);
        CoordinateSystem { header: Header::new("", style), origin: PointF::EMPTY, visible: false, show_grid: true, show_graduations: true }
    }
}

impl CoordinateSystem {
    pub const KVA_NAME: &'static str = "CoordinateSystem";

    pub fn read_kva(&mut self, e: &Element) {
        if let Some(id) = e.attribute("id").and_then(crate::ids::parse_id) {
            self.header.id = id;
        }
        for c in &e.children {
            let t = c.text.trim();
            match c.name.as_str() {
                "Position" => self.origin = read_point(c, Scale::IDENTITY),
                "ReferenceTimestamp" => self.header.reference_timestamp = parse_timestamp(t),
                "Visible" => self.visible = parse_boolean(t),
                "ShowGrid" => self.show_grid = parse_boolean(t),
                "ShowGraduations" => self.show_graduations = parse_boolean(t),
                "DrawingStyle" => self.header.style.import_kva(c),
                _ => {}
            }
        }
    }

    /// The position is the registered reference value (`PointF.Empty` when unregistered).
    pub fn write_kva(&self, filter: Filter, refs: RefValue) -> Vec<Element> {
        let mut out = Vec::new();
        if filter.core {
            out.push(point_leaf("Position", refs(self.header.id, "0")));
            out.push(Element::leaf("ReferenceTimestamp", &self.header.reference_timestamp.to_string()));
            out.push(Element::leaf("Visible", &write_boolean(self.visible)));
            out.push(Element::leaf("ShowGrid", &write_boolean(self.show_grid)));
            out.push(Element::leaf("ShowGraduations", &write_boolean(self.show_graduations)));
        }
        if filter.style {
            out.push(style_block(&self.header.style));
        }
        out
    }

    pub fn trackable_points(&self) -> Vec<(String, PointF)> {
        vec![("0".into(), self.origin)]
    }

    pub fn content_hash(&self) -> i32 {
        i32::from(self.visible) ^ self.header.style.content_hash() ^ i32::from(self.show_grid) << 1 ^ i32::from(self.show_graduations) << 2
    }
}

/// Framing helper lines (`DrawingTestGrid`).
#[derive(Debug, Clone, PartialEq)]
pub struct TestGrid {
    pub header: Header,
    pub visible: bool,
    pub show_horizontal_axis: bool,
    pub show_vertical_axis: bool,
    pub show_framing: bool,
    pub show_thirds: bool,
}

impl Default for TestGrid {
    fn default() -> Self {
        let style = StyleElements::new(vec![("color", StyleValue::Color(Color::rgb(255, 0, 0)))]);
        TestGrid { header: Header::new("", style), visible: false, show_horizontal_axis: true, show_vertical_axis: true, show_framing: true, show_thirds: false }
    }
}

impl TestGrid {
    pub const KVA_NAME: &'static str = "TestGrid";

    pub fn read_kva(&mut self, e: &Element) {
        if let Some(id) = e.attribute("id").and_then(crate::ids::parse_id) {
            self.header.id = id;
        }
        for c in &e.children {
            let t = c.text.trim();
            match c.name.as_str() {
                "Visible" => self.visible = parse_boolean(t),
                "ShowHorizontalAxis" => self.show_horizontal_axis = parse_boolean(t),
                "ShowVerticalAxis" => self.show_vertical_axis = parse_boolean(t),
                "ShowFraming" => self.show_framing = parse_boolean(t),
                "ShowThirds" => self.show_thirds = parse_boolean(t),
                "DrawingStyle" => self.header.style.import_kva(c),
                _ => {}
            }
        }
    }

    pub fn write_kva(&self, filter: Filter) -> Vec<Element> {
        let mut out = Vec::new();
        if filter.core {
            for (name, v) in [("Visible", self.visible), ("ShowHorizontalAxis", self.show_horizontal_axis), ("ShowVerticalAxis", self.show_vertical_axis), ("ShowFraming", self.show_framing), ("ShowThirds", self.show_thirds)] {
                out.push(Element::leaf(name, &write_boolean(v)));
            }
        }
        if filter.style {
            out.push(style_block(&self.header.style));
        }
        out
    }

    pub fn content_hash(&self) -> i32 {
        let flags = i32::from(self.visible) | i32::from(self.show_horizontal_axis) << 1 | i32::from(self.show_vertical_axis) << 2 | i32::from(self.show_framing) << 3 | i32::from(self.show_thirds) << 4;
        flags ^ self.header.style.content_hash()
    }
}

/// `<Name id="…" name="…">children</Name>` for singletons.
pub fn singleton_element(name: &str, header: &Header, children: Vec<Element>) -> Element {
    let mut e = Element::new(name);
    e.attributes.push(("id".into(), crate::ids::id_text(header.id)));
    e.attributes.push(("name".into(), header.name.clone()));
    e.children = children;
    e
}
