use super::common::testing::Scene;
use super::strokes::*;
use super::Hit;
use kinovea_geometry::PointF;
use kinovea_metadata::drawing::{Line, Pencil, Polyline};

fn p(x: f32, y: f32) -> PointF {
    PointF::new(x, y)
}

// DrawingLine.cs:278-295: ends are handles 1 and 2 (point boxes), then the segment is the body.
#[test]
fn line_ends_are_handles_and_the_segment_is_the_body() {
    let scene = Scene::at_scale(1.0);
    let line = Line::new(p(100.0, 100.0), p(300.0, 100.0));
    let ctx = scene.ctx(0);
    assert_eq!(hit_line(&line, p(100.0, 100.0), &ctx), Hit::Handle(1));
    assert_eq!(hit_line(&line, p(300.0, 100.0), &ctx), Hit::Handle(2));
    assert_eq!(hit_line(&line, p(200.0, 100.0), &ctx), Hit::Body);
    assert_eq!(hit_line(&line, p(200.0, 150.0), &ctx), Hit::Miss);
}

// DrawingPencil.cs:178-186: the curve through the points, no handles.
#[test]
fn pencil_is_hit_on_its_curve_only() {
    let scene = Scene::at_scale(1.0);
    let pencil = Pencil::new(vec![p(0.0, 0.0), p(50.0, 0.0), p(100.0, 0.0)]);
    assert_eq!(hit_pencil(&pencil, p(50.0, 0.0), &scene.ctx(0)), Hit::Body);
    assert_eq!(hit_pencil(&pencil, p(50.0, 50.0), &scene.ctx(0)), Hit::Miss);
}

// DrawingPolyline.cs:222-239: handle key + 1, then the widened polyline as the body.
#[test]
fn polyline_handles_are_numbered_from_one() {
    let scene = Scene::at_scale(1.0);
    let poly = Polyline::new(vec![
        ("0".into(), p(10.0, 10.0)),
        ("1".into(), p(100.0, 10.0)),
        ("2".into(), p(100.0, 100.0)),
    ]);
    let ctx = scene.ctx(0);
    assert_eq!(hit_polyline(&poly, p(10.0, 10.0), &ctx), Hit::Handle(1));
    assert_eq!(hit_polyline(&poly, p(100.0, 100.0), &ctx), Hit::Handle(3));
    assert_eq!(hit_polyline(&poly, p(55.0, 10.0), &ctx), Hit::Body);
    assert_eq!(hit_polyline(&poly, p(50.0, 50.0), &ctx), Hit::Miss);
}
