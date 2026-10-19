//! Marker on a single point with a measurement label (`DrawingCrossMark`).

use super::{Header, KvaDrawing, RefValue};
use crate::kva_io::*;
use crate::measure_label::{measure_label_block, MeasureLabel, MeasureLabelType};
use crate::style::default_style;
use kinovea_core::preferences::dom::Element;
use kinovea_core::xml::{parse_boolean, write_boolean};
use kinovea_geometry::PointF;

#[derive(Debug, Clone, PartialEq)]
pub struct CrossMark {
    pub header: Header,
    pub center: PointF,
    pub measure_label_type: MeasureLabelType,
    pub show_as_dot: bool,
    pub label: MeasureLabel,
}

impl CrossMark {
    pub fn new(center: PointF) -> Self {
        let label = MeasureLabel::new(center);
        CrossMark {
            header: Header::new("", default_style("CrossMark")),
            center,
            measure_label_type: MeasureLabelType::None,
            show_as_dot: false,
            label,
        }
    }
}

impl KvaDrawing for CrossMark {
    const KVA_NAME: &'static str = "CrossMark";

    fn header(&self) -> &Header {
        &self.header
    }

    fn header_mut(&mut self) -> &mut Header {
        &mut self.header
    }

    fn read_kva(e: &Element, scale: Scale, _map: TimestampMapper) -> Self {
        let mut d = CrossMark::new(PointF::EMPTY);
        d.header.read_attributes(e);
        for c in &e.children {
            let t = c.text.trim();
            match c.name.as_str() {
                "CenterPoint" => d.center = read_point(c, scale),
                "ReferenceTimestamp" => d.header.reference_timestamp = parse_timestamp(t),
                "ExtraData" => d.measure_label_type = parse_enum(t, MeasureLabelType::None),
                "MeasureLabel" => d.label = MeasureLabel::read_kva(c, scale),
                "ShowAsDot" => d.show_as_dot = parse_boolean(t),
                _ => {
                    read_style_or_fading(c, &mut d.header.style, &mut d.header.fading);
                }
            }
        }
        d.label.set_attach(d.center);
        d
    }

    fn write_kva(&self, filter: Filter, refs: RefValue) -> Vec<Element> {
        let mut out = Vec::new();
        if filter.core {
            let p = refs(self.header.id, "0");
            out.push(point_leaf("CenterPoint", p));
            out.push(Element::leaf(
                "ReferenceTimestamp",
                &self.header.reference_timestamp.to_string(),
            ));
            out.push(Element::leaf("ExtraData", self.measure_label_type.name()));
            out.push(Element::leaf("ShowAsDot", &write_boolean(self.show_as_dot)));
            out.push(measure_label_block(self.label.write_kva_attached(p)));
        }
        push_style_and_fading(&mut out, filter, &self.header.style, &self.header.fading);
        out
    }

    fn trackable_points(&self) -> Vec<(String, PointF)> {
        vec![("0".into(), self.center)]
    }

    fn set_trackable_point(&mut self, key: &str, value: PointF) {
        if key == "0" {
            self.center = value;
            self.label.set_attach(value);
        }
    }

    fn content_hash(&self) -> i32 {
        self.center.hash_code()
            ^ i32::from(self.show_as_dot)
            ^ self.header.style.content_hash()
            ^ self.header.fading.content_hash()
            ^ (self.measure_label_type as i32)
    }
}
