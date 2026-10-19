//! Multi-item singletons: spotlights (`Spotlights`/`Spotlight`) and the number sequence
//! (`AutoNumbers`/`AutoNumber`).

use crate::color::Color;
use crate::drawing::Header;
use crate::ids::{id_text, new_id, parse_id};
use crate::kva_io::*;
use crate::style::{StyleElements, StyleValue};
use kinovea_core::preferences::dom::Element;
use kinovea_geometry::PointF;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct Spotlight {
    pub id: Uuid,
    pub name: String,
    pub timestamp: i64,
    pub center: PointF,
    pub radius: i32,
}

impl Spotlight {
    pub const KVA_NAME: &'static str = "Spotlight";

    pub fn new(timestamp: i64, center: PointF, radius: i32) -> Self {
        Spotlight { id: new_id(), name: String::new(), timestamp, center, radius }
    }

    pub fn read_kva(e: &Element, scale: Scale, map: TimestampMapper) -> Spotlight {
        let mut s = Spotlight::new(0, PointF::EMPTY, 0);
        if let Some(id) = e.attribute("id").and_then(parse_id) {
            s.id = id;
        }
        if let Some(name) = e.attribute("name") {
            s.name = name.to_owned();
        }
        for c in &e.children {
            let t = c.text.trim();
            match c.name.as_str() {
                "Time" => s.timestamp = map(parse_timestamp(t)),
                "Center" => s.center = read_point(c, scale),
                "Radius" => s.radius = (t.parse::<i32>().unwrap_or(0) as f32 * scale.min()) as i32,
                _ => {}
            }
        }
        s
    }

    pub fn to_element(&self, filter: Filter) -> Element {
        let mut e = item_element(Spotlight::KVA_NAME, self.id, Spotlight::KVA_NAME);
        if filter.core {
            e.push_leaf("Time", &self.timestamp.to_string());
            e.push(point_leaf("Center", self.center));
            e.push_leaf("Radius", &self.radius.to_string());
        }
        e
    }

    pub fn content_hash(&self) -> i32 {
        ((self.timestamp as i32) ^ ((self.timestamp >> 32) as i32)) ^ self.radius ^ self.center.hash_code()
    }
}

fn item_element(name: &str, id: Uuid, item_name: &str) -> Element {
    let mut e = Element::new(name);
    e.attributes.push(("id".into(), id_text(id)));
    e.attributes.push(("name".into(), item_name.to_owned()));
    e
}

/// All spotlights of the analysis; the container has no style of its own.
#[derive(Debug, Clone, PartialEq)]
pub struct Spotlights {
    pub header: Header,
    pub items: Vec<Spotlight>,
}

impl Default for Spotlights {
    fn default() -> Self {
        Spotlights { header: Header::new("", StyleElements::default()), items: Vec::new() }
    }
}

impl Spotlights {
    pub const KVA_NAME: &'static str = "Spotlights";

    /// Items with a known id are replaced in place, others appended.
    pub fn read_kva(&mut self, e: &Element, scale: Scale, map: TimestampMapper) {
        for c in e.children.iter().filter(|c| c.name == Spotlight::KVA_NAME) {
            let item = Spotlight::read_kva(c, scale, map);
            match self.items.iter().position(|s| s.id == item.id) {
                Some(i) => self.items[i] = item,
                None => self.items.push(item),
            }
        }
    }

    pub fn write_kva(&self, filter: Filter) -> Vec<Element> {
        self.items.iter().map(|s| s.to_element(filter)).collect()
    }

    pub fn content_hash(&self) -> i32 {
        self.items.iter().fold(0, |h, s| h ^ s.content_hash())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AutoNumber {
    pub id: Uuid,
    pub name: String,
    pub timestamp: i64,
    pub location: PointF,
    pub value: i32,
}

impl AutoNumber {
    pub const KVA_NAME: &'static str = "AutoNumber";

    pub fn new(timestamp: i64, location: PointF, value: i32) -> Self {
        AutoNumber { id: new_id(), name: String::new(), timestamp, location, value }
    }

    pub fn read_kva(e: &Element, scale: Scale, map: TimestampMapper) -> AutoNumber {
        let mut n = AutoNumber::new(0, PointF::EMPTY, 0);
        if let Some(id) = e.attribute("id").and_then(parse_id) {
            n.id = id;
        }
        if let Some(name) = e.attribute("name") {
            n.name = name.to_owned();
        }
        for c in &e.children {
            let t = c.text.trim();
            match c.name.as_str() {
                "Time" => n.timestamp = map(parse_timestamp(t)),
                "Location" => n.location = read_point(c, scale),
                "Value" => n.value = t.parse().unwrap_or(0),
                _ => {}
            }
        }
        n
    }

    pub fn to_element(&self, filter: Filter) -> Element {
        let mut e = item_element(AutoNumber::KVA_NAME, self.id, AutoNumber::KVA_NAME);
        if filter.core {
            e.push_leaf("Time", &self.timestamp.to_string());
            e.push(point_leaf("Location", self.location));
            e.push_leaf("Value", &self.value.to_string());
        }
        e
    }
}

/// Numbered markers (`DrawingNumberSequence`, KVA `AutoNumbers`/`NumberSequence`).
#[derive(Debug, Clone, PartialEq)]
pub struct NumberSequence {
    pub header: Header,
    pub items: Vec<AutoNumber>,
}

impl Default for NumberSequence {
    fn default() -> Self {
        let style = StyleElements::new(vec![("back color", StyleValue::Color(Color::BLACK)), ("font size", StyleValue::FontSize(18))]);
        NumberSequence { header: Header::new("", style), items: Vec::new() }
    }
}

impl NumberSequence {
    pub const KVA_NAME: &'static str = "NumberSequence";

    pub fn read_kva(&mut self, e: &Element, scale: Scale, map: TimestampMapper) {
        for c in &e.children {
            match c.name.as_str() {
                "DrawingStyle" => self.header.style.import_kva(c),
                AutoNumber::KVA_NAME => {
                    let item = AutoNumber::read_kva(c, scale, map);
                    match self.items.iter().position(|n| n.id == item.id) {
                        Some(i) => self.items[i] = item,
                        None => self.items.push(item),
                    }
                }
                _ => {}
            }
        }
    }

    pub fn write_kva(&self, filter: Filter) -> Vec<Element> {
        let mut out: Vec<Element> = if filter.core { self.items.iter().map(|n| n.to_element(filter)).collect() } else { Vec::new() };
        if filter.style {
            out.push(style_block(&self.header.style));
        }
        out
    }

    pub fn content_hash(&self) -> i32 {
        self.items.iter().fold(0, |h, n| h ^ n.value ^ n.location.hash_code())
    }
}
