//! Dash style of a stroke from the shape elements (`LineShape`, `PenShape`, `TrackShape`).

use kinovea_metadata::style::{DashStyle, LineShape, PenShape, TrackShape};

/// Squiggles are drawn as curves by the drawing, so they are solid here.
pub fn dash_for_line_shape(shape: LineShape) -> DashStyle {
    match shape {
        LineShape::Dash => DashStyle::Dash,
        LineShape::Solid | LineShape::Squiggle => DashStyle::Solid,
    }
}

pub fn dash_for_pen_shape(shape: PenShape) -> DashStyle {
    match shape {
        PenShape::Dash => DashStyle::Dash,
        PenShape::Solid => DashStyle::Solid,
    }
}

pub fn dash_for_track_shape(shape: TrackShape) -> DashStyle {
    shape.dash
}
