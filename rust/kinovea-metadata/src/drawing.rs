//! Attached drawings as plain data: a common header plus one struct per KVA element kind.

mod angle;
#[cfg(test)]
mod angle_test;
mod chrono;
mod chrono_multi;
#[cfg(test)]
mod chrono_multi_test;
#[cfg(test)]
mod chrono_test;
mod circle;
#[cfg(test)]
mod circle_test;
mod counter;
#[cfg(test)]
mod counter_test;
mod crossmark;
#[cfg(test)]
mod crossmark_test;
mod generic_posture;
#[cfg(test)]
mod generic_posture_test;
mod label;
#[cfg(test)]
mod label_test;
mod line;
#[cfg(test)]
mod line_test;
mod media;
#[cfg(test)]
mod media_test;
mod paths;
#[cfg(test)]
mod paths_test;
mod plane;
#[cfg(test)]
mod plane_test;
mod rectangle;
#[cfg(test)]
mod rectangle_test;
mod time_segment;
#[cfg(test)]
mod time_segment_test;
mod track;
#[cfg(test)]
mod track_test;
pub mod tracking_parameters;
#[cfg(test)]
mod tracking_parameters_test;

pub use angle::Angle;
pub use chrono::Chrono;
pub use chrono_multi::{ChronoColumn, ChronoMulti, ChronoSection};
pub use circle::Circle;
pub use counter::Counter;
pub use crossmark::CrossMark;
pub use label::Label;
pub use line::Line;
pub use media::{Bitmap, Svg};
pub use paths::{Pencil, Polyline, TimePolyline};
pub use plane::{DistortionGrid, Plane};
pub use rectangle::Rectangle;
pub use time_segment::TimeSegment;
pub use track::{KeyframeLabel, Track};

mod any;
#[cfg(test)]
mod any_test;
pub use any::Drawing;

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
        Header {
            id: new_id(),
            name: name.to_owned(),
            reference_timestamp: -1,
            style,
            fading: Fading::default(),
        }
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
    /// `IsValid`: invalid drawings are dropped when a keyframe is read.
    fn is_valid(&self) -> bool {
        true
    }
}

/// Full drawing element: `<Name id="…" name="…">children</Name>`.
pub fn element<D: KvaDrawing>(d: &D, filter: Filter, refs: RefValue) -> Element {
    let mut e = Element::new(D::KVA_NAME);
    e.attributes
        .push(("id".into(), crate::ids::id_text(d.header().id)));
    e.attributes.push(("name".into(), d.header().name.clone()));
    e.children = d.write_kva(filter, refs);
    e
}
pub use generic_posture::GenericPostureDrawing;
