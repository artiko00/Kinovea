//! Small value types: circles, ellipses, segments, named 3D points, line positions, clip results.

use crate::point::PointF;

/// Circle by centre and radius.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Circle {
    pub center: PointF,
    pub radius: f32,
}

impl Circle {
    pub const EMPTY: Circle = Circle {
        center: PointF::EMPTY,
        radius: 0.0,
    };

    pub const fn new(center: PointF, radius: f32) -> Self {
        Circle { center, radius }
    }
}

/// Ellipse by centre, semi-axes and rotation in radians.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Ellipse {
    pub center: PointF,
    pub semi_major_axis: f32,
    pub semi_minor_axis: f32,
    pub rotation: f32,
}

impl Ellipse {
    pub const fn new(
        center: PointF,
        semi_major_axis: f32,
        semi_minor_axis: f32,
        rotation: f32,
    ) -> Self {
        Ellipse {
            center,
            semi_major_axis,
            semi_minor_axis,
            rotation,
        }
    }

    /// Rotation strictly between π/4 and 3π/4.
    pub fn is_vertical(&self) -> bool {
        let r = f64::from(self.rotation);
        r > std::f64::consts::FRAC_PI_4 && r < std::f64::consts::PI * 0.75
    }
}

/// Oriented segment.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Segment {
    pub start: PointF,
    pub end: PointF,
}

impl Segment {
    pub const EMPTY: Segment = Segment {
        start: PointF::EMPTY,
        end: PointF::EMPTY,
    };

    pub const fn new(start: PointF, end: PointF) -> Self {
        Segment { start, end }
    }
}

/// Named 3D point used by calibration validation.
#[derive(Debug, Clone, PartialEq)]
pub struct NamedPoint {
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl NamedPoint {
    pub fn new(name: impl Into<String>, x: f32, y: f32, z: f32) -> Self {
        NamedPoint {
            name: name.into(),
            x,
            y,
            z,
        }
    }
}

/// Allowed position of a point on a line, relative to the segment defining the line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PointLinePosition {
    BeforeSegment,
    BeforeAndOnSegment,
    OnSegment,
    AfterAndOnSegment,
    AfterSegment,
    Anywhere,
}

/// Outcome of clipping a primitive against a rectangle.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ClipResult {
    pub visible: bool,
    pub a: PointF,
    pub b: PointF,
}

impl ClipResult {
    pub const INVISIBLE: ClipResult = ClipResult {
        visible: false,
        a: PointF::EMPTY,
        b: PointF::EMPTY,
    };

    pub const fn visible(a: PointF, b: PointF) -> Self {
        ClipResult {
            visible: true,
            a,
            b,
        }
    }
}
