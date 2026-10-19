//! Segment with a fractional time marker along it (`DrawingTimeSegment`).

use super::{Header, KvaDrawing, RefValue};
use crate::kva_io::*;
use crate::measure_label::{measure_label_block, MeasureLabel};
use crate::style::default_style;
use kinovea_core::preferences::dom::Element;
use kinovea_core::xml::{parse_float, write_float};
use kinovea_geometry::PointF;

#[derive(Debug, Clone, PartialEq)]
pub struct TimeSegment {
    pub header: Header,
    pub a: PointF,
    pub b: PointF,
    pub fraction: f32,
    pub label: MeasureLabel,
}

impl TimeSegment {
    pub fn new(a: PointF, b: PointF) -> Self {
        let mut d = TimeSegment {
            header: Header::new("", default_style("TimeSegment")),
            a,
            b,
            fraction: 0.5,
            label: MeasureLabel::default(),
        };
        d.label.set_attach(d.time_point());
        d
    }

    /// Point at `fraction` along the segment.
    pub fn time_point(&self) -> PointF {
        PointF::new(
            self.a.x + (self.b.x - self.a.x) * self.fraction,
            self.a.y + (self.b.y - self.a.y) * self.fraction,
        )
    }
}

impl KvaDrawing for TimeSegment {
    const KVA_NAME: &'static str = "TimeSegment";

    fn header(&self) -> &Header {
        &self.header
    }

    fn header_mut(&mut self) -> &mut Header {
        &mut self.header
    }

    fn read_kva(e: &Element, scale: Scale, _map: TimestampMapper) -> Self {
        let mut d = TimeSegment::new(PointF::EMPTY, PointF::EMPTY);
        d.header.read_attributes(e);
        for c in &e.children {
            match c.name.as_str() {
                "Start" => d.a = read_point(c, scale),
                "End" => d.b = read_point(c, scale),
                "Fraction" => d.fraction = parse_float(&c.text),
                "MeasureLabel" => d.label = MeasureLabel::read_kva(c, scale),
                _ => {
                    read_style_or_fading(c, &mut d.header.style, &mut d.header.fading);
                }
            }
        }
        d.label.set_attach(d.time_point());
        d
    }

    fn write_kva(&self, filter: Filter, _refs: RefValue) -> Vec<Element> {
        let mut out = Vec::new();
        if filter.core {
            out.push(point_leaf("Start", self.a));
            out.push(point_leaf("End", self.b));
            out.push(Element::leaf("Fraction", &write_float(self.fraction)));
            out.push(measure_label_block(self.label.write_kva()));
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
        self.label.set_attach(self.time_point());
    }

    fn content_hash(&self) -> i32 {
        self.label.content_hash()
            ^ (self.fraction.to_bits() as i32)
            ^ self.header.style.content_hash()
            ^ self.header.fading.content_hash()
    }
}
