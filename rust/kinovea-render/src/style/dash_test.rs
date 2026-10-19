use super::dash::*;
use kinovea_metadata::style::{DashStyle, LineShape, PenShape, TrackShape};

#[test]
fn only_dash_line_shape_is_dashed() {
    assert_eq!(dash_for_line_shape(LineShape::Dash), DashStyle::Dash);
    assert_eq!(dash_for_line_shape(LineShape::Solid), DashStyle::Solid);
    assert_eq!(dash_for_line_shape(LineShape::Squiggle), DashStyle::Solid);
}

#[test]
fn pen_shape_maps_dash_and_solid() {
    assert_eq!(dash_for_pen_shape(PenShape::Dash), DashStyle::Dash);
    assert_eq!(dash_for_pen_shape(PenShape::Solid), DashStyle::Solid);
}

#[test]
fn track_shape_uses_its_dash_style_whatever_the_steps() {
    let shape = TrackShape {
        dash: DashStyle::Dot,
        steps: true,
    };
    assert_eq!(dash_for_track_shape(shape), DashStyle::Dot);
}
