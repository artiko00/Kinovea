use super::helpers::frame::scene::Scene;
use super::plane::paint_plane;
use crate::backend::recording::Call;
use crate::viewport::Point;
use kinovea_geometry::{PointF, Quadrilateral, RectangleF, Size, SizeF};
use kinovea_measurement::DistortionParameters;
use kinovea_metadata::drawing::Plane;
use kinovea_metadata::StyleValue;

#[test]
fn plane_painters_are_declared() {
    assert!(module_path!().contains("plane"));
}

fn p(x: f32, y: f32) -> PointF {
    PointF::new(x, y)
}

fn rect_quad(x: f32, y: f32, w: f32, h: f32) -> Quadrilateral {
    Quadrilateral::from_rect(RectangleF::new(x, y, w, h))
}

/// A plane with `divisions` columns and rows, as the style elements `cols` and `rows`.
fn plane(quad: Quadrilateral, divisions: i32) -> Plane {
    let mut d = Plane::new(quad);
    d.header.style.insert("cols", StyleValue::int(divisions));
    d.header.style.insert("rows", StyleValue::int(divisions));
    d
}

fn lines(calls: &[Call]) -> Vec<(PointF, PointF)> {
    calls
        .iter()
        .filter_map(|c| match c {
            Call::StrokeLine(a, b, _, _) => Some((*a, *b)),
            _ => None,
        })
        .collect()
}

fn count_curves(calls: &[Call]) -> usize {
    calls
        .iter()
        .filter(|c| matches!(c, Call::StrokeCurve(..)))
        .count()
}

#[test]
fn non_convex_perspective_plane_draws_only_its_edges() {
    // C#: DrawingPlane.cs:226, 236, 264.
    // The vertex (50, 40) is reflex, so the quadrilateral is not convex and no grid is drawn.
    // The handles are 8 lines (two per vertex); the 4 edges A-B, B-C, C-D, D-A follow.
    let scene = Scene::new(Point::new(0, 0), 1.0);
    let quad = Quadrilateral::new(p(0.0, 0.0), p(100.0, 0.0), p(50.0, 40.0), p(0.0, 100.0));
    let d = plane(quad, 10);
    let calls = scene.record(0, |ctx, r| paint_plane(&d, &ctx, r));
    let drawn = lines(&calls);
    assert_eq!(drawn.len(), 12);
    assert_eq!(
        &drawn[8..],
        &[
            (p(0.0, 0.0), p(100.0, 0.0)),
            (p(100.0, 0.0), p(50.0, 40.0)),
            (p(50.0, 40.0), p(0.0, 100.0)),
            (p(0.0, 100.0), p(0.0, 0.0)),
        ]
    );
    assert_eq!(count_curves(&calls), 0);
}

#[test]
fn rectangle_plane_draws_a_two_by_two_grid_over_its_image() {
    // C#: DrawingPlane.cs:244-259, 282.
    // The 100 x 100 plane maps onto the 200 x 100 rectangle, so x doubles. Two divisions give the
    // horizontal lines y = 0, 50, 100 and the vertical lines at image x = 0, 100, 200.
    let scene = Scene::new(Point::new(0, 0), 1.0);
    let d = plane(rect_quad(0.0, 0.0, 200.0, 100.0), 2);
    let calls = scene.record(0, |ctx, r| paint_plane(&d, &ctx, r));
    let drawn = lines(&calls);
    assert_eq!(drawn.len(), 14);
    assert_eq!(
        &drawn[8..],
        &[
            (p(0.0, 0.0), p(200.0, 0.0)),
            (p(0.0, 50.0), p(200.0, 50.0)),
            (p(0.0, 100.0), p(200.0, 100.0)),
            (p(0.0, 0.0), p(0.0, 100.0)),
            (p(100.0, 0.0), p(100.0, 100.0)),
            (p(200.0, 0.0), p(200.0, 100.0)),
        ]
    );
}

#[test]
fn initialized_distortion_draws_every_grid_line_as_a_curve() {
    // C#: DrawingPlane.cs:375-391.
    // Six grid lines become curves; the eight handle lines stay straight.
    let mut scene = Scene::new(Point::new(0, 0), 1.0);
    let size = Size::new(200, 100);
    scene
        .metadata
        .calibration
        .distortion_mut()
        .initialize(DistortionParameters::defaults(size), size);
    let d = plane(rect_quad(0.0, 0.0, 200.0, 100.0), 2);
    let calls = scene.record(0, |ctx, r| paint_plane(&d, &ctx, r));
    assert_eq!(count_curves(&calls), 6);
    assert_eq!(lines(&calls).len(), 8);
}

#[test]
fn calibration_drawing_rings_its_origin_vertex() {
    // C#: DrawingPlane.cs:242.
    // The origin is o.Box(5) around vertex D (10, 110): the ring (5, 105, 10, 10). Only the
    // calibration drawing has one.
    let mut scene = Scene::new(Point::new(0, 0), 1.0);
    let d = plane(rect_quad(10.0, 10.0, 200.0, 100.0), 2);
    scene
        .metadata
        .calibration
        .calibrate_by_plane(d.header.id, SizeF::new(200.0, 100.0), &d.quad);
    let calls = scene.record(0, |ctx, r| paint_plane(&d, &ctx, r));
    let rings: Vec<RectangleF> = calls
        .iter()
        .filter_map(|c| match c {
            Call::StrokeEllipse(rect, _, _) => Some(*rect),
            _ => None,
        })
        .collect();
    assert_eq!(rings, vec![RectangleF::new(5.0, 105.0, 10.0, 10.0)]);
}

#[test]
fn viewport_scale_two_doubles_the_vertices_and_keeps_the_handle_arms() {
    // C#: DrawingPlane.cs:236.
    // Vertex B (100, 0) lands on (200, 0): its horizontal arm is (196, 0)-(204, 0) and its vertical
    // arm (200, -4)-(200, 4).
    let scene = Scene::new(Point::new(0, 0), 2.0);
    let d = plane(rect_quad(0.0, 0.0, 100.0, 50.0), 10);
    let calls = scene.record(0, |ctx, r| paint_plane(&d, &ctx, r));
    let drawn = lines(&calls);
    assert_eq!(drawn[2], (p(196.0, 0.0), p(204.0, 0.0)));
    assert_eq!(drawn[3], (p(200.0, -4.0), p(200.0, 4.0)));
}
