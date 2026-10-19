use super::angle::*;
use super::common::testing::Scene;
use super::Hit;
use kinovea_geometry::PointF;
use kinovea_metadata::drawing::Angle;

fn p(x: f32, y: f32) -> PointF {
    PointF::new(x, y)
}

fn right_angle() -> Angle {
    Angle::new(p(100.0, 100.0), p(200.0, 100.0), p(100.0, 0.0))
}

// DrawingAngle.cs:254-272: origin is handle 1, first leg 2, second leg 3.
#[test]
fn legs_and_origin_are_handles() {
    let scene = Scene::at_scale(1.0);
    let ctx = scene.ctx(0);
    let angle = right_angle();
    assert_eq!(hit_angle(&angle, p(100.0, 100.0), &ctx), Hit::Handle(1));
    assert_eq!(hit_angle(&angle, p(200.0, 100.0), &ctx), Hit::Handle(2));
    assert_eq!(hit_angle(&angle, p(100.0, 0.0), &ctx), Hit::Handle(3));
}

// AngleHelper.cs:69-87 and SweepAngle.cs:58-80: counter-clockwise sweep from b (north) to a (east)
// is the upper-right quadrant, so the pie covers it and not the other quadrants.
#[test]
fn pie_is_the_swept_sector() {
    let scene = Scene::at_scale(1.0);
    let ctx = scene.ctx(0);
    let angle = right_angle();
    assert_eq!(hit_angle(&angle, p(130.0, 80.0), &ctx), Hit::Body);
    assert_eq!(hit_angle(&angle, p(70.0, 80.0), &ctx), Hit::Miss);
    assert_eq!(hit_angle(&angle, p(130.0, 130.0), &ctx), Hit::Miss);
}

// DrawingAngle.cs:255-256: nothing is hit while the drawing is fully faded.
#[test]
fn faded_angle_is_never_hit() {
    let scene = Scene::at_scale(1.0);
    assert_eq!(
        hit_angle(&right_angle(), p(130.0, 80.0), &scene.ctx(10_000)),
        Hit::Miss
    );
}
