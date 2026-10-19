use super::common::testing::Scene;
use super::time::*;
use super::Hit;
use kinovea_geometry::PointF;
use kinovea_metadata::drawing::{TimePolyline, TimeSegment};

fn p(x: f32, y: f32) -> PointF {
    PointF::new(x, y)
}

// DrawingTimeSegment.cs:159-177: the time point (mix at the fraction, default 0.5) is handle 3.
#[test]
fn time_point_is_handle_three_at_the_fraction() {
    let scene = Scene::at_scale(1.0);
    let ctx = scene.ctx(0);
    let seg = TimeSegment::new(p(100.0, 100.0), p(300.0, 100.0));
    assert_eq!(
        hit_time_segment(&seg, p(200.0, 100.0), &ctx),
        Hit::Handle(3)
    );
    assert_eq!(
        hit_time_segment(&seg, p(100.0, 100.0), &ctx),
        Hit::Handle(1)
    );
    assert_eq!(
        hit_time_segment(&seg, p(300.0, 100.0), &ctx),
        Hit::Handle(2)
    );
    assert_eq!(hit_time_segment(&seg, p(150.0, 100.0), &ctx), Hit::Body);
}

// DrawingTimePolyline.cs:175-193: handle key + 1 from the point boxes, body from the widened line.
#[test]
fn time_polyline_numbers_its_handles_from_one() {
    let scene = Scene::at_scale(1.0);
    let ctx = scene.ctx(0);
    let poly = TimePolyline::new(vec![
        ("0".into(), p(10.0, 10.0)),
        ("1".into(), p(100.0, 10.0)),
    ]);
    assert_eq!(
        hit_time_polyline(&poly, p(100.0, 10.0), &ctx),
        Hit::Handle(2)
    );
    assert_eq!(hit_time_polyline(&poly, p(55.0, 10.0), &ctx), Hit::Body);
    assert_eq!(hit_time_polyline(&poly, p(55.0, 80.0), &ctx), Hit::Miss);
}
