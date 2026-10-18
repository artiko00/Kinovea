//! Stopwatch drawings: the single-section `Chrono` and the shared position/label plumbing of
//! the time-based detached drawings.

use super::{Header, KvaDrawing, RefValue};
use crate::kva_io::*;
use crate::style::{default_style, StyleElements, StyleValue};
use kinovea_core::preferences::dom::Element;
use kinovea_core::xml::{parse_boolean, write_boolean};
use kinovea_geometry::PointF;

/// `Label` block of the chrono family: only `Show` is persisted (`Text` renames the drawing).
pub(crate) fn read_label(c: &Element, name: &mut String, show_label: &mut bool) {
    for l in &c.children {
        match l.name.as_str() {
            "Text" if !l.text.is_empty() && l.text != *name => *name = l.text.clone(),
            "Show" => *show_label = parse_boolean(l.text.trim()),
            _ => {}
        }
    }
}

/// `<Label><Show/></Label>` followed by the style block.
pub(crate) fn push_label_and_style(
    out: &mut Vec<Element>,
    show_label: bool,
    style: &StyleElements,
) {
    let mut label = Element::new("Label");
    label.push_leaf("Show", &write_boolean(show_label));
    out.push(label);
    out.push(style_block(style));
}

/// Whether the `clock` toggle of a style is on.
pub(crate) fn is_clock(style: &StyleElements) -> bool {
    matches!(
        style.get("clock"),
        Some(StyleValue::Toggle { value: true, .. })
    )
}

#[derive(Debug, Clone, PartialEq)]
pub struct Chrono {
    pub header: Header,
    pub position: PointF,
    pub visible: i64,
    pub invisible: i64,
    pub start_counting: i64,
    pub stop_counting: i64,
    pub clock_origin: i64,
    pub show_label: bool,
}

impl Chrono {
    pub fn new(position: PointF) -> Self {
        Chrono {
            header: Header::new("", default_style("Chrono")),
            position,
            visible: 0,
            invisible: i64::MAX,
            start_counting: i64::MAX,
            stop_counting: i64::MAX,
            clock_origin: i64::MAX,
            show_label: false,
        }
    }

    fn sanity_check(&mut self) {
        self.visible = self.visible.max(0);
        self.invisible = self.invisible.max(0);
        self.start_counting = self.start_counting.max(0);
        self.stop_counting = self.stop_counting.max(0);
        if !is_clock(&self.header.style) && self.stop_counting < self.start_counting {
            self.stop_counting = i64::MAX;
        }
    }
}

impl KvaDrawing for Chrono {
    const KVA_NAME: &'static str = "Chrono";

    fn header(&self) -> &Header {
        &self.header
    }

    fn header_mut(&mut self) -> &mut Header {
        &mut self.header
    }

    fn read_kva(e: &Element, scale: Scale, map: TimestampMapper) -> Self {
        let mut d = Chrono::new(PointF::EMPTY);
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
                            "StartCounting" => d.start_counting = read_open_timestamp(t, map),
                            "StopCounting" => d.stop_counting = read_open_timestamp(t, map),
                            "ClockOrigin" => d.clock_origin = read_open_timestamp(t, map),
                            _ => {}
                        }
                    }
                }
                "DrawingStyle" => d.header.style.import_kva(c),
                "Label" => read_label(c, &mut d.header.name, &mut d.show_label),
                _ => {}
            }
        }
        d.sanity_check();
        d
    }

    fn write_kva(&self, filter: Filter, _refs: RefValue) -> Vec<Element> {
        let mut out = Vec::new();
        if filter.core {
            out.push(point_leaf("Position", self.position));
            let mut values = Element::new("Values");
            for (name, v) in [
                ("Visible", self.visible),
                ("Invisible", self.invisible),
                ("StartCounting", self.start_counting),
                ("StopCounting", self.stop_counting),
                ("ClockOrigin", self.clock_origin),
            ] {
                values.push_leaf(name, &write_open_timestamp(v));
            }
            out.push(values);
        }
        if filter.style {
            push_label_and_style(&mut out, self.show_label, &self.header.style);
        }
        out
    }

    fn content_hash(&self) -> i32 {
        let long = |v: i64| (v as i32) ^ ((v >> 32) as i32);
        long(self.visible)
            ^ long(self.invisible)
            ^ long(self.start_counting)
            ^ long(self.stop_counting)
            ^ long(self.clock_origin)
            ^ self.header.style.content_hash()
            ^ i32::from(self.show_label)
    }
}
