//! Beat counter / cadence drawing (`DrawingCounter`).

use super::chrono::push_label_and_style;
use super::{Header, KvaDrawing, RefValue};
use crate::kva_io::*;
use crate::measure_label::MeasureLabelType;
use crate::style::default_style;
use kinovea_core::preferences::dom::Element;
use kinovea_core::xml::{parse_boolean, write_boolean};
use kinovea_geometry::PointF;

#[derive(Debug, Clone, PartialEq)]
pub struct Counter {
    pub header: Header,
    pub position: PointF,
    pub visible: i64,
    pub invisible: i64,
    pub beats: Vec<i64>,
    pub locked: bool,
    pub zero_based: bool,
    pub half_cadence: bool,
    pub double_cadence: bool,
    pub measure_label_type: MeasureLabelType,
    /// Not read back by the original; always written from the in-memory value.
    pub show_label: bool,
}

impl Counter {
    pub fn new(position: PointF) -> Self {
        Counter {
            header: Header::new("", default_style("Counter")),
            position,
            visible: 0,
            invisible: i64::MAX,
            beats: Vec::new(),
            locked: false,
            zero_based: false,
            half_cadence: false,
            double_cadence: false,
            measure_label_type: MeasureLabelType::None,
            show_label: false,
        }
    }
}

impl KvaDrawing for Counter {
    const KVA_NAME: &'static str = "Counter";

    fn header(&self) -> &Header {
        &self.header
    }

    fn header_mut(&mut self) -> &mut Header {
        &mut self.header
    }

    fn read_kva(e: &Element, scale: Scale, map: TimestampMapper) -> Self {
        let mut d = Counter::new(PointF::EMPTY);
        d.header.read_attributes(e);
        for c in &e.children {
            match c.name.as_str() {
                "Position" => d.position = read_point(c, scale),
                "Values" => {
                    for v in &c.children {
                        let t = v.text.trim();
                        match v.name.as_str() {
                            "Visible" => d.visible = map(parse_timestamp(t)),
                            "Invisible" => d.invisible = read_open_timestamp(t, map),
                            "Beats" => {
                                d.beats = v
                                    .children
                                    .iter()
                                    .filter(|b| b.name == "Beat")
                                    .map(|b| parse_timestamp(b.text.trim()))
                                    .collect()
                            }
                            "Locked" => d.locked = parse_boolean(t),
                            "ZeroBased" => d.zero_based = parse_boolean(t),
                            "HalfCadence" => d.half_cadence = parse_boolean(t),
                            "DoubleCadence" => d.double_cadence = parse_boolean(t),
                            "ExtraData" => {
                                d.measure_label_type = parse_enum(t, MeasureLabelType::None)
                            }
                            _ => {}
                        }
                    }
                }
                "DrawingStyle" => d.header.style.import_kva(c),
                _ => {}
            }
        }
        d.visible = d.visible.max(0);
        d.invisible = d.invisible.max(0);
        d
    }

    fn write_kva(&self, filter: Filter, _refs: RefValue) -> Vec<Element> {
        let mut out = Vec::new();
        if filter.core {
            out.push(point_leaf("Position", self.position));
            let mut values = Element::new("Values");
            values.push_leaf("Visible", &write_open_timestamp(self.visible));
            values.push_leaf("Invisible", &write_open_timestamp(self.invisible));
            if !self.beats.is_empty() {
                let mut beats = Element::new("Beats");
                for b in &self.beats {
                    beats.push_leaf("Beat", &b.to_string());
                }
                values.push(beats);
            }
            values.push_leaf("Locked", &write_boolean(self.locked));
            values.push_leaf("ZeroBased", &write_boolean(self.zero_based));
            values.push_leaf("HalfCadence", &write_boolean(self.half_cadence));
            values.push_leaf("DoubleCadence", &write_boolean(self.double_cadence));
            values.push_leaf("ExtraData", self.measure_label_type.name());
            out.push(values);
        }
        if filter.style {
            push_label_and_style(&mut out, self.show_label, &self.header.style);
        }
        out
    }

    fn content_hash(&self) -> i32 {
        let long = |v: i64| (v as i32) ^ ((v >> 32) as i32);
        let flags = i32::from(self.show_label)
            | i32::from(self.locked) << 1
            | i32::from(self.zero_based) << 2
            | i32::from(self.half_cadence) << 3
            | i32::from(self.double_cadence) << 4;
        long(self.visible)
            ^ long(self.invisible)
            ^ self.beats.iter().fold(0, |h, b| h ^ long(*b))
            ^ self.header.style.content_hash()
            ^ flags
    }
}
