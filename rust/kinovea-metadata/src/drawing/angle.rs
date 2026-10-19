//! Angle between two legs around an origin (`DrawingAngle`).

use super::{Header, KvaDrawing, RefValue};
use crate::kva_io::*;
use crate::style::default_style;
use kinovea_core::preferences::dom::Element;
use kinovea_core::xml::{parse_boolean, write_boolean};
use kinovea_geometry::PointF;

#[derive(Debug, Clone, PartialEq)]
pub struct Angle {
    pub header: Header,
    pub o: PointF,
    pub a: PointF,
    pub b: PointF,
    pub signed: bool,
    pub counter_clockwise: bool,
    pub supplementary: bool,
    pub show_circle: bool,
}

impl Angle {
    pub fn new(o: PointF, a: PointF, b: PointF) -> Self {
        Angle {
            header: Header::new("", default_style("Angle")),
            o,
            a,
            b,
            signed: true,
            counter_clockwise: true,
            supplementary: false,
            show_circle: false,
        }
    }

    /// Oriented angle in radians with the drawing's options, in pixel space.
    pub fn value(&self) -> f32 {
        let (mut a, mut b) = (self.a, self.b);
        if self.supplementary {
            let c = PointF::new(2.0 * self.o.x - a.x, 2.0 * self.o.y - a.y);
            a = b;
            b = c;
        }
        let angle = if self.counter_clockwise {
            kinovea_geometry::helpers::angle_at(self.o, a, b)
        } else {
            kinovea_geometry::helpers::angle_at(self.o, b, a)
        };
        if !self.signed && angle < 0.0 {
            (std::f64::consts::TAU + f64::from(angle)) as f32
        } else {
            angle
        }
    }
}

impl KvaDrawing for Angle {
    const KVA_NAME: &'static str = "Angle";

    fn header(&self) -> &Header {
        &self.header
    }

    fn header_mut(&mut self) -> &mut Header {
        &mut self.header
    }

    fn read_kva(e: &Element, scale: Scale, _map: TimestampMapper) -> Self {
        let mut d = Angle::new(PointF::EMPTY, PointF::EMPTY, PointF::EMPTY);
        d.header.read_attributes(e);
        for c in &e.children {
            let t = c.text.trim();
            match c.name.as_str() {
                "PointO" => d.o = read_point(c, scale),
                "PointA" => d.a = read_point(c, scale),
                "PointB" => d.b = read_point(c, scale),
                "ReferenceTimestamp" => d.header.reference_timestamp = parse_timestamp(t),
                "Signed" => d.signed = parse_boolean(t),
                "CCW" => d.counter_clockwise = parse_boolean(t),
                "Supplementary" => d.supplementary = parse_boolean(t),
                "ShowCircle" => d.show_circle = parse_boolean(t),
                _ => {
                    read_style_or_fading(c, &mut d.header.style, &mut d.header.fading);
                }
            }
        }
        d
    }

    fn write_kva(&self, filter: Filter, refs: RefValue) -> Vec<Element> {
        let mut out = Vec::new();
        if filter.core {
            out.push(point_leaf("PointO", refs(self.header.id, "o")));
            out.push(point_leaf("PointA", refs(self.header.id, "a")));
            out.push(point_leaf("PointB", refs(self.header.id, "b")));
            out.push(Element::leaf(
                "ReferenceTimestamp",
                &self.header.reference_timestamp.to_string(),
            ));
            out.push(Element::leaf("Signed", &write_boolean(self.signed)));
            out.push(Element::leaf("CCW", &write_boolean(self.counter_clockwise)));
            out.push(Element::leaf(
                "Supplementary",
                &write_boolean(self.supplementary),
            ));
            out.push(Element::leaf(
                "ShowCircle",
                &write_boolean(self.show_circle),
            ));
        }
        push_style_and_fading(&mut out, filter, &self.header.style, &self.header.fading);
        out
    }

    fn trackable_points(&self) -> Vec<(String, PointF)> {
        vec![
            ("o".into(), self.o),
            ("a".into(), self.a),
            ("b".into(), self.b),
        ]
    }

    fn set_trackable_point(&mut self, key: &str, value: PointF) {
        match key {
            "o" => self.o = value,
            "a" => self.a = value,
            "b" => self.b = value,
            _ => {}
        }
    }

    fn content_hash(&self) -> i32 {
        let flags = i32::from(self.signed)
            | i32::from(self.counter_clockwise) << 1
            | i32::from(self.supplementary) << 2
            | i32::from(self.show_circle) << 3;
        self.o.hash_code()
            ^ self.a.hash_code()
            ^ self.b.hash_code()
            ^ flags
            ^ self.header.style.content_hash()
            ^ self.header.fading.content_hash()
    }
}
