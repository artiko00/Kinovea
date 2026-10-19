//! Handles with their movement constraint and the impacts they have on other points.

use super::HandleType;
use crate::color::Color;
use kinovea_geometry::{PointF, PointLinePosition};

#[derive(Debug, Clone, PartialEq)]
pub struct Handle {
    pub name: String,
    pub kind: HandleType,
    pub color: Color,
    pub reference: i32,
    pub trackable: bool,
    pub option_group: String,
    /// Where the segment handle was grabbed; segment moves are relative to it.
    pub grab_point: PointF,
    pub constraint: Option<Constraint>,
    pub impacts: Vec<Impact>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Constraint {
    pub kind: ConstraintKind,
    /// `a|b` option keys; the constraint is inactive when any of them is off.
    pub option_group: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConstraintKind {
    LineSlide { start: i32, end: i32, allowed: PointLinePosition, margin: i32 },
    VerticalSlide,
    HorizontalSlide,
    DistanceToPoint { point: i32 },
    RotationSteps { origin: i32, leg1: i32, step: i32, keep_distance: bool },
    PerpendicularSlide { origin: i32, leg1: i32 },
    ParallelSlide { a: i32, b: i32, c: i32 },
    LockedInPlace,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Impact {
    LineAlign { start: i32, end: i32, point_to_align: i32 },
    VerticalAlign { point: i32 },
    HorizontalAlign { point: i32 },
    Pivot { pivot: i32, impacted: Vec<i32> },
    /// `old_angle` and `old_distance` are captured before the move (`PrepareImpacts`).
    KeepAngle { origin: i32, leg1: i32, leg2: i32, old_angle: f32, old_distance: f32 },
    HorizontalSymmetry { impacting: i32, impacted: i32, axis: i32 },
    SegmentCenter { point_to_move: i32, point1: i32, point2: i32 },
    PerpendicularAlign { point_to_move: i32, origin: i32, leg1: i32 },
    ParallelAlign { point_to_move: i32, a: i32, b: i32, c: i32 },
}
