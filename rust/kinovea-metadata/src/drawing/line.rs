//! Line segment with an optional measurement label (`DrawingLine`).

use super::{Header, KvaDrawing, RefValue};
use crate::kva_io::*;
use crate::measure_label::{measure_label_block, MeasureLabel, MeasureLabelType};
use crate::style::default_style;
use kinovea_core::preferences::dom::Element;
use kinovea_geometry::helpers::middle_point;
use kinovea_geometry::PointF;

#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    pub header: Header,
    pub a: PointF,
    pub b: PointF,
    pub measure_label_type: MeasureLabelType,
    pub label: MeasureLabel,
}

impl Line {
    pub fn new(a: PointF, b: PointF) -> Self {
        let label = MeasureLabel::new(middle_point(a, b));
        Line {
            header: Header::new("", default_style("Line")),
            a,
            b,
            measure_label_type: MeasureLabelType::None,
            label,
        }
    }

    /// Length in pixels.
    pub fn length(&self) -> f32 {
        kinovea_geometry::helpers::distance(self.a, self.b)
    }
}

impl KvaDrawing for Line {
    const KVA_NAME: &'static str = "Line";

    fn header(&self) -> &Header {
        &self.header
    }

    fn header_mut(&mut self) -> &mut Header {
        &mut self.header
    }

    fn read_kva(e: &Element, scale: Scale, _map: TimestampMapper) -> Self {
        let mut d = Line::new(PointF::EMPTY, PointF::new(10.0, 0.0));
        d.header.read_attributes(e);
        for c in &e.children {
            match c.name.as_str() {
                "Start" => d.a = read_point(c, scale),
                "End" => d.b = read_point(c, scale),
                "ReferenceTimestamp" => d.header.reference_timestamp = parse_timestamp(&c.text),
                "ExtraData" => d.measure_label_type = parse_enum(&c.text, MeasureLabelType::None),
                "MeasureLabel" => d.label = MeasureLabel::read_kva(c, scale),
                _ => {
                    read_style_or_fading(c, &mut d.header.style, &mut d.header.fading);
                }
            }
        }
        d.label.set_attach(middle_point(d.a, d.b));
        d
    }

    fn write_kva(&self, filter: Filter, refs: RefValue) -> Vec<Element> {
        let mut out = Vec::new();
        if filter.core {
            let a = refs(self.header.id, "a");
            let b = refs(self.header.id, "b");
            out.push(point_leaf("Start", a));
            out.push(point_leaf("End", b));
            out.push(Element::leaf(
                "ReferenceTimestamp",
                &self.header.reference_timestamp.to_string(),
            ));
            out.push(Element::leaf("ExtraData", self.measure_label_type.name()));
            out.push(measure_label_block(
                self.label.write_kva_attached(middle_point(a, b)),
            ));
        }
        push_style_and_fading(&mut out, filter, &self.header.style, &self.header.fading);
        out
    }

    fn trackable_points(&self) -> Vec<(String, PointF)> {
        vec![("a".into(), self.a), ("b".into(), self.b)]
    }

    fn set_trackable_point(&mut self, key: &str, value: PointF) {
        match key {
            "a" => self.a = value,
            "b" => self.b = value,
            _ => {}
        }
        self.label.set_attach(middle_point(self.a, self.b));
    }

    fn content_hash(&self) -> i32 {
        self.a.hash_code()
            ^ self.b.hash_code()
            ^ self.header.style.content_hash()
            ^ self.header.fading.content_hash()
            ^ (self.measure_label_type as i32)
    }
}
