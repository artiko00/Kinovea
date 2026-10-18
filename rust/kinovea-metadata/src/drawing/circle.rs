//! Circle by centre and integer radius (`DrawingCircle`).

use super::{Header, KvaDrawing, RefValue};
use crate::kva_io::*;
use crate::measure_label::{measure_label_block, MeasureLabel, MeasureLabelType};
use crate::style::default_style;
use kinovea_core::preferences::dom::Element;
use kinovea_core::xml::{parse_boolean, write_boolean};
use kinovea_geometry::PointF;

#[derive(Debug, Clone, PartialEq)]
pub struct Circle {
    pub header: Header,
    pub center: PointF,
    pub radius: i32,
    pub measure_label_type: MeasureLabelType,
    pub show_center: bool,
    pub filled: bool,
    pub label: MeasureLabel,
}

impl Circle {
    pub fn new(center: PointF, radius: i32) -> Self {
        let mut label = MeasureLabel::default();
        label.set_attach(center);
        Circle { header: Header::new("", default_style("Circle")), center, radius, measure_label_type: MeasureLabelType::None, show_center: false, filled: false, label }
    }
}

impl KvaDrawing for Circle {
    const KVA_NAME: &'static str = "Circle";

    fn header(&self) -> &Header {
        &self.header
    }

    fn header_mut(&mut self) -> &mut Header {
        &mut self.header
    }

    fn read_kva(e: &Element, scale: Scale, _map: TimestampMapper) -> Self {
        let mut d = Circle::new(PointF::EMPTY, 0);
        d.header.read_attributes(e);
        for c in &e.children {
            let t = c.text.trim();
            match c.name.as_str() {
                "Origin" => d.center = read_point(c, scale),
                "Radius" => d.radius = (t.parse::<i32>().unwrap_or(0) as f32 * scale.min()) as i32,
                "ExtraData" => d.measure_label_type = parse_enum(t, MeasureLabelType::None),
                "MeasureLabel" => d.label = MeasureLabel::read_kva(c, scale),
                "ShowCenter" => d.show_center = parse_boolean(t),
                "Filled" => d.filled = parse_boolean(t),
                _ => {
                    read_style_or_fading(c, &mut d.header.style, &mut d.header.fading);
                }
            }
        }
        d.label.set_attach(d.center);
        d
    }

    fn write_kva(&self, filter: Filter, _refs: RefValue) -> Vec<Element> {
        let mut out = Vec::new();
        if filter.core {
            out.push(point_leaf("Origin", self.center));
            out.push(Element::leaf("Radius", &self.radius.to_string()));
            out.push(Element::leaf("ExtraData", self.measure_label_type.name()));
            out.push(measure_label_block(self.label.write_kva()));
            out.push(Element::leaf("ShowCenter", &write_boolean(self.show_center)));
            out.push(Element::leaf("Filled", &write_boolean(self.filled)));
        }
        push_style_and_fading(&mut out, filter, &self.header.style, &self.header.fading);
        out
    }

    fn content_hash(&self) -> i32 {
        self.center.hash_code() ^ self.radius ^ i32::from(self.show_center) << 1 ^ i32::from(self.filled) << 2 ^ self.header.style.content_hash() ^ self.header.fading.content_hash()
    }
}
