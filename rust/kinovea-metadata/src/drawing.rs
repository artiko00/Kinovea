//! Attached drawings as plain data: a common header plus one struct per KVA element kind.

mod angle;
#[cfg(test)]
mod angle_test;
mod circle;
#[cfg(test)]
mod circle_test;
mod crossmark;
#[cfg(test)]
mod crossmark_test;
mod label;
#[cfg(test)]
mod label_test;
mod line;
#[cfg(test)]
mod line_test;
mod paths;
#[cfg(test)]
mod paths_test;
mod rectangle;
#[cfg(test)]
mod rectangle_test;

pub use angle::Angle;
pub use circle::Circle;
pub use crossmark::CrossMark;
pub use label::Label;
pub use line::Line;
pub use paths::{Pencil, Polyline, TimePolyline};
pub use rectangle::Rectangle;

use crate::fading::Fading;
use crate::ids::new_id;
use crate::kva_io::{Filter, Scale, TimestampMapper};
use crate::style::StyleElements;
use kinovea_core::preferences::dom::Element;
use kinovea_geometry::PointF;
use uuid::Uuid;

/// Identity and the parts every attached drawing persists.
#[derive(Debug, Clone, PartialEq)]
pub struct Header {
    pub id: Uuid,
    pub name: String,
    pub reference_timestamp: i64,
    pub style: StyleElements,
    pub fading: Fading,
}

impl Header {
    pub fn new(name: &str, style: StyleElements) -> Self {
        Header { id: new_id(), name: name.to_owned(), reference_timestamp: -1, style, fading: Fading::default() }
    }

    /// Reads the `id` and `name` attributes of a drawing element.
    pub fn read_attributes(&mut self, e: &Element) {
        if let Some(id) = e.attribute("id").and_then(crate::ids::parse_id) {
            self.id = id;
        }
        if let Some(name) = e.attribute("name") {
            self.name = name.to_owned();
        }
    }
}

/// Reference positions of trackable points, as held by the trackability manager; the
/// original writes `PointF.Empty` for drawings it never registered.
pub type RefValue<'a> = &'a dyn Fn(Uuid, &str) -> PointF;

/// Behaviour shared by every attached drawing kind.
pub trait KvaDrawing: Sized {
    /// The KVA element name (`[XmlType]`).
    const KVA_NAME: &'static str;
    fn header(&self) -> &Header;
    fn header_mut(&mut self) -> &mut Header;
    /// Builds the drawing from its element, scaling coordinates and mapping timestamps.
    fn read_kva(e: &Element, scale: Scale, map: TimestampMapper) -> Self;
    /// Children of the drawing element.
    fn write_kva(&self, filter: Filter, refs: RefValue) -> Vec<Element>;
    /// Named points that tracking can move, in the original key order.
    fn trackable_points(&self) -> Vec<(String, PointF)> {
        Vec::new()
    }
    fn set_trackable_point(&mut self, _key: &str, _value: PointF) {}
    /// Hash of the persisted content, stable within Rust.
    fn content_hash(&self) -> i32;
}

/// Full drawing element: `<Name id="…" name="…">children</Name>`.
pub fn element<D: KvaDrawing>(d: &D, filter: Filter, refs: RefValue) -> Element {
    let mut e = Element::new(D::KVA_NAME);
    e.attributes.push(("id".into(), crate::ids::id_text(d.header().id)));
    e.attributes.push(("name".into(), d.header().name.clone()));
    e.children = d.write_kva(filter, refs);
    e
}

/// Every attached drawing kind.
#[derive(Debug, Clone, PartialEq)]
pub enum Drawing {
    Line(Line),
    Angle(Angle),
    CrossMark(CrossMark),
    Circle(Circle),
    Rectangle(Rectangle),
    Pencil(Pencil),
    Polyline(Polyline),
    TimePolyline(TimePolyline),
    Label(Label),
}

macro_rules! dispatch {
    ($self:expr, |$d:ident| $body:expr) => {
        match $self {
            Drawing::Line($d) => $body,
            Drawing::Angle($d) => $body,
            Drawing::CrossMark($d) => $body,
            Drawing::Circle($d) => $body,
            Drawing::Rectangle($d) => $body,
            Drawing::Pencil($d) => $body,
            Drawing::Polyline($d) => $body,
            Drawing::TimePolyline($d) => $body,
            Drawing::Label($d) => $body,
        }
    };
}

impl Drawing {
    /// Reads a drawing element by its name; `None` for unknown kinds.
    pub fn read_kva(e: &Element, scale: Scale, map: TimestampMapper) -> Option<Drawing> {
        Some(match e.name.as_str() {
            Line::KVA_NAME => Drawing::Line(Line::read_kva(e, scale, map)),
            Angle::KVA_NAME => Drawing::Angle(Angle::read_kva(e, scale, map)),
            CrossMark::KVA_NAME => Drawing::CrossMark(CrossMark::read_kva(e, scale, map)),
            Circle::KVA_NAME => Drawing::Circle(Circle::read_kva(e, scale, map)),
            Rectangle::KVA_NAME => Drawing::Rectangle(Rectangle::read_kva(e, scale, map)),
            Pencil::KVA_NAME => Drawing::Pencil(Pencil::read_kva(e, scale, map)),
            Polyline::KVA_NAME => Drawing::Polyline(Polyline::read_kva(e, scale, map)),
            TimePolyline::KVA_NAME => Drawing::TimePolyline(TimePolyline::read_kva(e, scale, map)),
            Label::KVA_NAME => Drawing::Label(Label::read_kva(e, scale, map)),
            _ => return None,
        })
    }

    pub fn kva_name(&self) -> &'static str {
        dispatch!(self, |d| kva_name_of(d))
    }

    pub fn header(&self) -> &Header {
        dispatch!(self, |d| d.header())
    }

    pub fn header_mut(&mut self) -> &mut Header {
        dispatch!(self, |d| d.header_mut())
    }

    pub fn id(&self) -> Uuid {
        self.header().id
    }

    pub fn to_element(&self, filter: Filter, refs: RefValue) -> Element {
        dispatch!(self, |d| element(d, filter, refs))
    }

    pub fn trackable_points(&self) -> Vec<(String, PointF)> {
        dispatch!(self, |d| d.trackable_points())
    }

    pub fn set_trackable_point(&mut self, key: &str, value: PointF) {
        dispatch!(self, |d| d.set_trackable_point(key, value))
    }

    pub fn content_hash(&self) -> i32 {
        dispatch!(self, |d| d.content_hash())
    }

    /// Planes are appended to a keyframe while every other kind is inserted first.
    pub fn is_plane(&self) -> bool {
        false
    }
}

fn kva_name_of<D: KvaDrawing>(_: &D) -> &'static str {
    D::KVA_NAME
}
