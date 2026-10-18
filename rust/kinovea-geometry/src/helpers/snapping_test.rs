use crate::helpers::*;
use crate::point::PointF;

const O: PointF = PointF::new(0.0, 0.0);
const LEG: PointF = PointF::new(10.0, 0.0);

fn polar(deg: f32, r: f32) -> PointF {
    let (s, c) = f64::from(radians(deg)).sin_cos();
    PointF::new(r * c as f32, r * s as f32)
}

fn close(a: PointF, b: PointF, tol: f32) -> bool {
    (a.x - b.x).abs() < tol && (a.y - b.y).abs() < tol
}

#[test]
fn closest_rotation_step_snaps_50_degrees_to_45() {
    let p = point_at_closest_rotation_step(O, LEG, polar(50.0, 10.0), 8);
    assert!(close(p, polar(45.0, 10.0), 1e-4), "{p:?}");
    let q = point_at_closest_rotation_step(O, LEG, polar(-50.0, 10.0), 8);
    assert!(close(q, polar(-45.0, 10.0), 1e-4), "{q:?}");
    let r = point_at_closest_rotation_step(O, LEG, polar(70.0, 10.0), 8);
    assert!(close(r, polar(90.0, 10.0), 1e-4), "{r:?}");
}

#[test]
fn cardinal_snapping_uses_the_horizontal_axis() {
    let p = point_at_closest_rotation_step_cardinal(
        PointF::new(5.0, 5.0),
        PointF::new(5.0 + 10.0, 5.0 + 1.0),
        4,
    );
    assert!(
        close(p, PointF::new(5.0 + 101f32.sqrt(), 5.0), 1e-4),
        "{p:?}"
    );
}

#[test]
fn point_at_angle_keeps_distance() {
    let p = point_at_angle(O, LEG, PointF::new(0.0, 7.0), 90.0);
    assert!(close(p, PointF::new(0.0, 7.0), 1e-4), "{p:?}");
    let q = point_at_angle_and_distance(O, LEG, radians(180.0), 3.0);
    assert!(close(q, PointF::new(-3.0, 0.0), 1e-4), "{q:?}");
}

#[test]
fn point_on_parallel_projects_onto_the_parallel_through_c() {
    let a = PointF::new(0.0, 0.0);
    let b = PointF::new(10.0, 0.0);
    let c = PointF::new(0.0, 5.0);
    let p = point_on_parallel(a, b, c, PointF::new(3.0, 9.0));
    assert!(close(p, PointF::new(5.0, 5.0), 1e-4), "{p:?}");
}
