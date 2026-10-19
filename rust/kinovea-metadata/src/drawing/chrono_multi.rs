//! Multi-section stopwatch (`DrawingChronoMulti`).

use super::chrono::{push_label_and_style, read_label};
use super::{Header, KvaDrawing, RefValue};
use crate::kva_io::*;
use crate::style::default_style;
use kinovea_core::preferences::dom::Element;
use kinovea_core::video_section::VideoSection;
use kinovea_core::xml::{parse_boolean, write_boolean, write_video_section};
use kinovea_geometry::PointF;

crate::text_enum!(
    /// Columns of the measurement table (`ChronoColumns`).
    ChronoColumn [None, Name, Duration, Cumul, Tag]
);

#[derive(Debug, Clone, PartialEq)]
pub struct ChronoSection {
    pub section: VideoSection,
    pub name: String,
    pub tag: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChronoMulti {
    pub header: Header,
    pub position: PointF,
    pub visible: i64,
    pub invisible: i64,
    pub sections: Vec<ChronoSection>,
    pub locked: bool,
    pub visible_columns: Vec<ChronoColumn>,
    pub show_label: bool,
}

impl ChronoMulti {
    pub fn new(position: PointF) -> Self {
        ChronoMulti {
            header: Header::new("", default_style("ChronoMulti")),
            position,
            visible: 0,
            invisible: i64::MAX,
            sections: Vec::new(),
            locked: false,
            visible_columns: vec![
                ChronoColumn::Name,
                ChronoColumn::Duration,
                ChronoColumn::Cumul,
                ChronoColumn::Tag,
            ],
            show_label: false,
        }
    }

    /// Inserts keeping sections ordered by start (`InsertSection`).
    pub fn insert_section(&mut self, section: VideoSection, name: &str, tag: &str) {
        let entry = ChronoSection {
            section,
            name: name.to_owned(),
            tag: tag.to_owned(),
        };
        let at = self
            .sections
            .iter()
            .position(|s| s.section.start >= section.start)
            .unwrap_or(self.sections.len());
        self.sections.insert(at, entry);
    }

    fn read_values(&mut self, values: &Element, map: TimestampMapper) {
        for v in &values.children {
            let t = v.text.trim();
            match v.name.as_str() {
                "Visible" => self.visible = map(parse_timestamp(t)),
                "Invisible" => self.invisible = read_open_timestamp(t, map),
                "Sections" => {
                    self.sections.clear();
                    for s in v.children.iter().filter(|s| s.name == "Section") {
                        let section = parse_video_section(s.text.trim());
                        let mapped = VideoSection::new(map(section.start), map(section.end));
                        self.insert_section(
                            mapped,
                            s.attribute("name").unwrap_or(""),
                            s.attribute("tag").unwrap_or(""),
                        );
                    }
                }
                "Locked" => self.locked = parse_boolean(t),
                "VisibleColumns" => {
                    self.visible_columns = t.split(';').filter_map(|c| c.parse().ok()).collect()
                }
                _ => {}
            }
        }
    }
}

/// `XmlHelper.ParseVideoSection`: `start;end` with −1 standing for `long.MaxValue`.
pub fn parse_video_section(text: &str) -> VideoSection {
    let open = |v: i64| if v == -1 { i64::MAX } else { v };
    let mut parts = text.split(';').map(|p| p.trim().parse::<i64>());
    match (parts.next(), parts.next()) {
        (Some(Ok(start)), Some(Ok(end))) => VideoSection::new(open(start), open(end)),
        _ => VideoSection::new(i64::MAX, i64::MAX),
    }
}

impl KvaDrawing for ChronoMulti {
    const KVA_NAME: &'static str = "ChronoMulti";

    fn header(&self) -> &Header {
        &self.header
    }

    fn header_mut(&mut self) -> &mut Header {
        &mut self.header
    }

    fn read_kva(e: &Element, scale: Scale, map: TimestampMapper) -> Self {
        let mut d = ChronoMulti::new(PointF::EMPTY);
        d.header.read_attributes(e);
        for c in &e.children {
            match c.name.as_str() {
                "Position" => d.position = read_point(c, scale),
                "Values" => d.read_values(c, map),
                "DrawingStyle" => d.header.style.import_kva(c),
                "Label" => read_label(c, &mut d.header.name, &mut d.show_label),
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
            if !self.sections.is_empty() {
                let mut sections = Element::new("Sections");
                for s in &self.sections {
                    let mut e = Element::leaf("Section", &write_video_section(&s.section));
                    if !s.name.is_empty() {
                        e.attributes.push(("name".into(), s.name.clone()));
                    }
                    if !s.tag.is_empty() {
                        e.attributes.push(("tag".into(), s.tag.clone()));
                    }
                    sections.push(e);
                }
                values.push(sections);
            }
            values.push_leaf("Locked", &write_boolean(self.locked));
            let columns: Vec<&str> = self.visible_columns.iter().map(|c| c.name()).collect();
            values.push_leaf("VisibleColumns", &columns.join(";"));
            out.push(values);
        }
        if filter.style {
            push_label_and_style(&mut out, self.show_label, &self.header.style);
        }
        out
    }

    fn content_hash(&self) -> i32 {
        let long = |v: i64| (v as i32) ^ ((v >> 32) as i32);
        let sections = self
            .sections
            .iter()
            .fold(0, |h, s| h ^ long(s.section.start) ^ long(s.section.end));
        long(self.visible)
            ^ long(self.invisible)
            ^ sections
            ^ self.header.style.content_hash()
            ^ i32::from(self.show_label)
            ^ i32::from(self.locked) << 1
    }
}
