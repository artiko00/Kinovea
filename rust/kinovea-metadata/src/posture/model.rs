//! Posture description data (`GenericPosture` and its element classes).

use crate::color::Color;
use crate::drawing::tracking_parameters::TrackingParameters;
use kinovea_geometry::{PointF, PointLinePosition};
use uuid::Uuid;

mod computed;
#[cfg(test)]
mod computed_test;
mod handles;
#[cfg(test)]
mod handles_test;
pub use computed::*;
pub use handles::*;

crate::text_enum!(SegmentLineStyle [Solid, Dash]);
crate::text_enum!(HandleType [Point, Segment, Ellipse, Circle]);

pub const TRANSPARENT: Color = Color::argb(0, 255, 255, 255);

#[derive(Debug, Clone, PartialEq)]
pub struct PosturePoint {
    pub name: String,
    pub value: PointF,
    pub color: Color,
}

/// Line style shared by segments, polylines and circles.
#[derive(Debug, Clone, PartialEq)]
pub struct Stroke {
    pub style: SegmentLineStyle,
    pub width: i32,
    pub arrow_begin: bool,
    pub arrow_end: bool,
    pub color: Color,
}

impl Default for Stroke {
    fn default() -> Self {
        Stroke { style: SegmentLineStyle::Solid, width: 2, arrow_begin: false, arrow_end: false, color: TRANSPARENT }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Segment {
    pub name: String,
    pub start: i32,
    pub end: i32,
    pub stroke: Stroke,
    pub option_group: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PostureCircle {
    pub name: String,
    pub center: i32,
    pub radius: i32,
    pub stroke: Stroke,
    pub option_group: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PosturePolyline {
    pub name: String,
    pub points: Vec<i32>,
    pub stroke: Stroke,
    pub option_group: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PostureAngle {
    pub name: String,
    pub origin: i32,
    pub leg1: i32,
    pub leg2: i32,
    pub text_distance: i32,
    pub radius: i32,
    pub tenth: bool,
    pub color: Color,
    pub symbol: String,
    pub signed: bool,
    pub ccw: bool,
    pub supplementary: bool,
    pub option_group: String,
}

/// `Distance` (two points) or `Position` (one point) read-outs.
#[derive(Debug, Clone, PartialEq)]
pub struct Measure {
    pub name: String,
    pub point1: i32,
    pub point2: i32,
    pub symbol: String,
    pub color: Color,
    pub option_group: String,
}

/// Polygon of point indices that grabs the whole drawing.
#[derive(Debug, Clone, PartialEq)]
pub struct HitZone {
    pub points: Vec<i32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PostureOption {
    pub key: String,
    pub label: String,
    pub default_value: bool,
    pub value: bool,
    pub hidden: bool,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct GenericPosture {
    pub id: Uuid,
    pub name: String,
    pub display_name: String,
    pub icon: String,
    pub point_list: Vec<PointF>,
    pub points: Vec<PosturePoint>,
    pub segments: Vec<Segment>,
    pub circles: Vec<PostureCircle>,
    pub angles: Vec<PostureAngle>,
    pub polylines: Vec<PosturePolyline>,
    pub handles: Vec<Handle>,
    pub hit_zones: Vec<HitZone>,
    pub distances: Vec<Measure>,
    pub positions: Vec<Measure>,
    pub computed_points: Vec<ComputedPoint>,
    pub flip_horizontal: bool,
    pub flip_vertical: bool,
    /// Insertion-ordered, unique by key.
    pub options: Vec<PostureOption>,
    pub has_non_hidden_options: bool,
    pub tracking_parameters: TrackingParameters,
    pub trackable_indices: Vec<i32>,
    pub from_kva: bool,
}

/// Allowed position of a point along a line, as spelled in the tool files.
pub fn parse_line_position(text: &str) -> PointLinePosition {
    match text {
        "BeforeSegment" => PointLinePosition::BeforeSegment,
        "BeforeAndOnSegment" => PointLinePosition::BeforeAndOnSegment,
        "OnSegment" => PointLinePosition::OnSegment,
        "AfterAndOnSegment" => PointLinePosition::AfterAndOnSegment,
        "AfterSegment" => PointLinePosition::AfterSegment,
        _ => PointLinePosition::Anywhere,
    }
}
