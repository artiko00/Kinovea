use crate::calibration_axis::CalibrationAxis;
use crate::calibrator_plane::CalibratorPlane;
use kinovea_geometry::{PointF, Quadrilateral, SizeF, Vector3};

fn p(x: f32, y: f32) -> PointF {
    PointF::new(x, y)
}

fn near(a: PointF, b: PointF, tol: f32) -> bool {
    (a.x - b.x).abs() < tol && (a.y - b.y).abs() < tol
}

fn perspective_quad() -> Quadrilateral {
    Quadrilateral::new(p(300.0, 200.0), p(1500.0, 240.0), p(1700.0, 900.0), p(100.0, 880.0))
}

fn plane() -> CalibratorPlane {
    let mut c = CalibratorPlane::default();
    c.initialize_plane(SizeF::new(4.0, 3.0), perspective_quad());
    c
}

#[test]
fn uninitialized_is_the_identity() {
    let c = CalibratorPlane::default();
    assert_eq!(c.transform(p(3.0, 4.0)), p(3.0, 4.0));
    assert_eq!(c.untransform(p(3.0, 4.0)), p(3.0, 4.0));
    assert_eq!(c.project(p(3.0, 4.0)), Vector3::new(3.0, 4.0, 1.0));
    assert_eq!(c.project_vector3(Vector3::new(1.0, 1.0, 1.0)), Vector3::default());
    assert!(!c.is_valid());
}

#[test]
fn plane_corners_map_to_world_with_d_as_origin() {
    let c = plane();
    let q = perspective_quad();
    assert!(c.is_valid());
    assert!(near(c.transform(q.d()), p(0.0, 0.0), 1e-3));
    assert!(near(c.transform(q.c()), p(4.0, 0.0), 1e-3));
    assert!(near(c.transform(q.b()), p(4.0, 3.0), 1e-3));
    assert!(near(c.transform(q.a()), p(0.0, 3.0), 1e-3));
    assert!(near(c.untransform(p(4.0, 3.0)), q.b(), 1e-2));
    let h = c.project(p(4.0, 3.0));
    assert!(near(p(h.x / h.z, h.y / h.z), q.b(), 1e-2));
    assert_eq!(c.quad_world().c(), p(4.0, 3.0));
    assert_eq!(c.quad_image(), q);
    assert_eq!(c.mapper().forward(p(0.0, 0.0)), q.a());
}

#[test]
fn line_calibrations_horizontal_and_vertical() {
    let mut h = CalibratorPlane::default();
    h.initialize_line(2.0, p(100.0, 500.0), p(300.0, 500.0), CalibrationAxis::LineHorizontal);
    assert!(near(h.transform(p(200.0, 400.0)), p(1.0, 1.0), 1e-4));
    assert_eq!(h.axis(), CalibrationAxis::LineHorizontal);
    let mut v = CalibratorPlane::default();
    v.initialize_line(2.0, p(100.0, 500.0), p(100.0, 300.0), CalibrationAxis::LineVertical);
    assert!(near(v.transform(p(200.0, 400.0)), p(1.0, 1.0), 1e-4));
    v.update_line(p(100.0, 500.0), p(100.0, 400.0));
    assert!(near(v.transform(p(200.0, 400.0)), p(2.0, 2.0), 1e-4));
}

#[test]
fn non_convex_quad_is_invalid_but_still_transforms() {
    let mut c = plane();
    c.update_quad(Quadrilateral::new(p(0.0, 0.0), p(2.0, 2.0), p(2.0, 0.0), p(0.0, 2.0)));
    assert!(!c.is_valid());
    // Points still go through the last homography, whatever it yields.
    let _ = c.transform(p(1.0, 1.0));
    let mut empty = CalibratorPlane::default();
    empty.update_quad(perspective_quad());
    assert!(!empty.is_valid());
}

#[test]
fn origin_offset_and_clone() {
    let mut c = plane();
    let one_one = c.untransform(p(1.0, 1.0));
    c.set_origin(one_one);
    assert!(near(c.transform(one_one), p(0.0, 0.0), 1e-3));
    c.reset_origin();
    assert!(near(c.transform(perspective_quad().d()), p(0.0, 0.0), 1e-3));
    c.set_origin(one_one);
    c.offset = p(0.5, -0.25);
    assert!(near(c.transform(one_one), p(0.5, -0.25), 1e-3));
    assert!(near(c.grid_to_world_with_offset(c.origin()), p(0.5, -0.25), 1e-6));
    let clone = c.clone_calibrator();
    assert_eq!(clone.origin(), c.origin());
    assert_eq!(clone.offset, PointF::EMPTY);
    assert_eq!(clone.quad_image(), c.quad_image());
    assert!(near(c.transform_with_origin(one_one, one_one), p(0.5, -0.25), 1e-3));
    assert!(near(c.untransform_with_origin(p(0.0, 0.0), c.origin()), one_one, 1e-2));
}
