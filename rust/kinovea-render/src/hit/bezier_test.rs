//! Cubic Bezier helpers standing in for GDI+ cardinal splines, arcs and flattening.
use super::bezier::{arc_to_bezier, cardinal_to_bezier, ellipse_point, flatten};
use kinovea_geometry::{PointF, RectangleF};

fn p(x: f32, y: f32) -> PointF {
    PointF::new(x, y)
}

// Cardinal segment 0 of (0,0),(10,0),(20,0) at tension 0.5: control points are
// P0 + (0.5/3)(P1 - P0) = (5/3, 0) and P1 - (0.5/3)(P2 - P0) = (20/3, 0).
#[test]
fn cardinal_control_points_use_tension_over_three_of_the_neighbours() {
    let curves = cardinal_to_bezier(&[p(0.0, 0.0), p(10.0, 0.0), p(20.0, 0.0)], 0.5);
    assert_eq!(curves.len(), 2);
    assert!((curves[0][1].x - 5.0 / 3.0).abs() < 1e-5);
    assert!((curves[0][2].x - 20.0 / 3.0).abs() < 1e-5);
}

// Tension 0 makes the control points coincide with the end points: the curve is the chord.
#[test]
fn zero_tension_gives_straight_segments() {
    let curves = cardinal_to_bezier(&[p(0.0, 0.0), p(10.0, 0.0)], 0.0);
    assert_eq!(
        curves[0],
        [p(0.0, 0.0), p(0.0, 0.0), p(10.0, 0.0), p(10.0, 0.0)]
    );
}

// A single point has no segments.
#[test]
fn one_point_has_no_curves() {
    assert!(cardinal_to_bezier(&[p(1.0, 1.0)], 0.5).is_empty());
}

// Arc endpoints sit on the ellipse at the requested angles (y grows downwards).
#[test]
fn arc_endpoints_follow_parametric_angles() {
    let rect = RectangleF::new(0.0, 0.0, 20.0, 10.0);
    assert_eq!(ellipse_point(rect, 0.0), p(20.0, 5.0));
    let quarter = arc_to_bezier(rect, 0.0, 90.0);
    assert_eq!(quarter.len(), 1);
    assert!((quarter[0][3].x - 10.0).abs() < 1e-4 && (quarter[0][3].y - 10.0).abs() < 1e-4);
}

// Sweeps larger than 90 degrees are split into quarter-or-smaller cubics.
#[test]
fn full_turn_is_four_cubics_returning_to_start() {
    let rect = RectangleF::new(0.0, 0.0, 20.0, 20.0);
    let curves = arc_to_bezier(rect, 0.0, 360.0);
    assert_eq!(curves.len(), 4);
    assert!((curves[3][3].x - 20.0).abs() < 1e-4 && (curves[3][3].y - 10.0).abs() < 1e-4);
}

// Flattening starts at the given point and ends exactly on the cubic end point.
#[test]
fn flatten_starts_and_ends_on_the_curve_ends() {
    let curve = [p(0.0, 0.0), p(0.0, 10.0), p(10.0, 10.0), p(10.0, 0.0)];
    let points = flatten(p(0.0, 0.0), &[curve]);
    assert_eq!(points[0], p(0.0, 0.0));
    assert_eq!(points[points.len() - 1], p(10.0, 0.0));
    assert!(points.len() > 2);
}
