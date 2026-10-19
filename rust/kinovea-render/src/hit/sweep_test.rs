use super::common::testing::Scene;
use super::sweep::{sweep_contains, SweepFlags};
use kinovea_geometry::PointF;

fn p(x: f32, y: f32) -> PointF {
    PointF::new(x, y)
}

const FLAGS: SweepFlags = SweepFlags {
    signed: false,
    counter_clockwise: true,
    supplementary: false,
};

// SweepAngle.cs:113-150: a positive radius is the pie radius (here 30, not the leg-based 90).
#[test]
fn positive_radius_is_used_as_the_pie_radius() {
    let scene = Scene::at_scale(1.0);
    let ctx = scene.ctx(0);
    let (o, a, b) = (p(100.0, 100.0), p(200.0, 100.0), p(100.0, 0.0));
    assert!(sweep_contains(o, a, b, 30, FLAGS, p(120.0, 80.0), &ctx));
    assert!(!sweep_contains(o, a, b, 30, FLAGS, p(160.0, 60.0), &ctx));
}

// SweepAngle.cs:121-124: a zero radius takes the shorter leg minus 10 px (here 90).
#[test]
fn zero_radius_uses_the_shorter_leg() {
    let scene = Scene::at_scale(1.0);
    let ctx = scene.ctx(0);
    let (o, a, b) = (p(100.0, 100.0), p(200.0, 100.0), p(100.0, 0.0));
    assert!(sweep_contains(o, a, b, 0, FLAGS, p(120.0, 80.0), &ctx));
    assert!(!sweep_contains(o, a, b, 0, FLAGS, p(200.0, 20.0), &ctx));
}

// AngleHelper.cs:70-72: a coincident leg gives no pie.
#[test]
fn coincident_leg_gives_no_pie() {
    let scene = Scene::at_scale(1.0);
    assert!(!sweep_contains(
        p(0.0, 0.0),
        p(0.0, 0.0),
        p(0.0, 50.0),
        30,
        FLAGS,
        p(1.0, 1.0),
        &scene.ctx(0)
    ));
}
