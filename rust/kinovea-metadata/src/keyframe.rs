//! A key image with its attached drawings (`Keyframe`).

use crate::color::Color;
use crate::drawing::{Drawing, RefValue};
use crate::ids::{id_text, new_id, parse_id};
use crate::kva_io::*;
use kinovea_core::preferences::dom::Element;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct Keyframe {
    pub id: Uuid,
    pub timestamp: i64,
    /// Empty means "use the timecode" when displayed.
    pub name: String,
    pub comments: String,
    pub color: Color,
    /// Internal order: new drawings go first, planes last; the file order is the reverse.
    pub drawings: Vec<Drawing>,
}

impl Keyframe {
    pub const DEFAULT_COLOR: Color = Color::rgb(51, 152, 255);

    pub fn new(timestamp: i64, name: &str) -> Self {
        Keyframe { id: new_id(), timestamp, name: name.to_owned(), comments: String::new(), color: Keyframe::DEFAULT_COLOR, drawings: Vec::new() }
    }

    /// `AddDrawing`: planes are appended, everything else is inserted first.
    pub fn add_drawing(&mut self, drawing: Drawing) {
        if drawing.is_plane() {
            self.drawings.push(drawing);
        } else {
            self.drawings.insert(0, drawing);
        }
    }

    pub fn remove_drawing(&mut self, id: Uuid) -> Option<Drawing> {
        let i = self.drawings.iter().position(|d| d.id() == id)?;
        Some(self.drawings.remove(i))
    }

    /// `Keyframe.Name`: the timecode of the position when no name was given.
    pub fn display_name(&self, timecode: fn(i64) -> String) -> String {
        if self.name.is_empty() { timecode(self.timestamp) } else { self.name.clone() }
    }

    pub fn drawing(&self, id: Uuid) -> Option<&Drawing> {
        self.drawings.iter().find(|d| d.id() == id)
    }

    /// Reads a `Keyframe` element; drawings are scaled and reference the keyframe time.
    pub fn read_kva(e: &Element, scale: Scale, map: TimestampMapper) -> Keyframe {
        let mut kf = Keyframe::new(0, "");
        if let Some(id) = e.attribute("id").and_then(parse_id) {
            kf.id = id;
        }
        for c in &e.children {
            match c.name.as_str() {
                "Position" | "Timestamp" => kf.timestamp = map(c.text.trim().parse::<i32>().map_or(0, i64::from)),
                "Title" | "Name" => kf.name = c.text.clone(),
                "Color" => kf.color = Color::parse(c.text.trim(), Color::rgb(70, 130, 180)),
                "Comment" => kf.comments = c.text.clone(),
                "Drawings" => {
                    for d in &c.children {
                        if let Some(mut drawing) = Drawing::read_kva(d, scale, &identity) {
                            if !drawing.is_valid() || kf.drawings.iter().any(|x| x.id() == drawing.id()) {
                                continue;
                            }
                            drawing.header_mut().reference_timestamp = kf.timestamp;
                            drawing.header_mut().fading.reference_timestamp = kf.timestamp;
                            kf.add_drawing(drawing);
                        }
                    }
                }
                _ => {}
            }
        }
        kf
    }

    /// `<Keyframe id>` with `Timestamp`, `Name` (the timecode when unnamed), `Color`, optional
    /// `Comment` and the drawings in file order.
    pub fn to_element(&self, filter: Filter, refs: RefValue, timecode: fn(i64) -> String) -> Element {
        let mut e = Element::new("Keyframe");
        e.attributes.push(("id".into(), id_text(self.id)));
        if filter.core {
            e.push_leaf("Timestamp", &self.timestamp.to_string());
            e.push_leaf("Name", &self.display_name(timecode));
            e.push_leaf("Color", &self.color.write(false));
            if !self.comments.is_empty() {
                e.push_leaf("Comment", &self.comments);
            }
        }
        if filter.core && filter.style && filter.fading && !self.drawings.is_empty() {
            let mut list = Element::new("Drawings");
            list.children = self.drawings.iter().rev().map(|d| d.to_element(Filter::KVA, refs)).collect();
            e.push(list);
        }
        e
    }

    pub fn content_hash(&self) -> i32 {
        let long = (self.timestamp as i32) ^ ((self.timestamp >> 32) as i32);
        self.drawings.iter().fold(0, |h, d| h ^ d.content_hash()) ^ (self.comments.len() as i32) ^ (self.name.len() as i32) ^ self.color.hash_code() ^ long
    }
}
