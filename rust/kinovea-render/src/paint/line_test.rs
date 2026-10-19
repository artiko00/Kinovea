use super::helpers::frame::scene::{round_pen, style, Scene};
use super::line::paint_line;
use crate::backend::recording::Call;
use crate::renderer::Transform;
use crate::viewport::Point;
use kinovea_geometry::PointF;
use kinovea_metadata::drawing::Line;
use kinovea_metadata::style::{DashStyle, LineShape};
use kinovea_metadata::StyleValue;

fn line(a: PointF, b: PointF, size: i32, shape: LineShape) -> Line {
    let mut d = Line::new(a, b);
    d.header.style = style(size);
    d.header
        .style
        .insert("line shape", StyleValue::LineShape(shape));
    d
}

#[test]
fn straight_line_maps_both_ends_and_scales_its_width() {
    // Size 4 at scale 0.5 -> width 2; (0, 0) and (100, 0) -> (0, 0) and (50, 0).
    let scene = Scene::new(Point::new(0, 0), 0.5);
    let d = line(
        PointF::new(0.0, 0.0),
        PointF::new(100.0, 0.0),
        4,
        LineShape::Solid,
    );
    assert_eq!(
        scene.record(0, |ctx, r| paint_line(&d, &ctx, r)),
        vec![Call::StrokeLine(
            PointF::new(0.0, 0.0),
            PointF::new(50.0, 0.0),
            round_pen(2.0),
            Transform::identity(),
        )]
    );
}

#[test]
fn viewport_offset_and_scale_two_double_coordinates_and_width() {
    // (10, 10) * 2 + (5, 7) = (25, 27); (30, 10) * 2 + (5, 7) = (65, 27); width 4 * 2 = 8.
    let scene = Scene::new(Point::new(5, 7), 2.0);
    let d = line(
        PointF::new(10.0, 10.0),
        PointF::new(30.0, 10.0),
        4,
        LineShape::Solid,
    );
    assert_eq!(
        scene.record(0, |ctx, r| paint_line(&d, &ctx, r)),
        vec![Call::StrokeLine(
            PointF::new(25.0, 27.0),
            PointF::new(65.0, 27.0),
            round_pen(8.0),
            Transform::identity(),
        )]
    );
}

#[test]
fn dashed_line_uses_the_dash_pattern() {
    let scene = Scene::new(Point::new(0, 0), 1.0);
    let d = line(
        PointF::new(0.0, 0.0),
        PointF::new(10.0, 0.0),
        1,
        LineShape::Dash,
    );
    let calls = scene.record(0, |ctx, r| paint_line(&d, &ctx, r));
    let Call::StrokeLine(_, _, pen, _) = &calls[0] else {
        panic!("expected a line");
    };
    assert_eq!(pen.dash, DashStyle::Dash);
}

#[test]
fn squiggly_line_is_a_squiggle_curve_with_tension_point_nine() {
    // Same zigzag as the squiggle helper for a segment of length 20 and width 1.
    let scene = Scene::new(Point::new(0, 0), 1.0);
    let d = line(
        PointF::new(0.0, 0.0),
        PointF::new(20.0, 0.0),
        1,
        LineShape::Squiggle,
    );
    let calls = scene.record(0, |ctx, r| paint_line(&d, &ctx, r));
    let Call::StrokeCurve(points, tension, _, _) = &calls[0] else {
        panic!("expected a curve");
    };
    assert_eq!(*tension, 0.9);
    assert_eq!(points[2], PointF::new(12.0, -3.0));
    assert_eq!(points.len(), 5);
}
