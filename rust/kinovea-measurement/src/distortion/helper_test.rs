use crate::distortion::{DistortionHelper, DistortionParameters};
use kinovea_geometry::{PointF, Quadrilateral, Size};

fn helper() -> DistortionHelper {
    let mut h = DistortionHelper::default();
    let p = DistortionParameters::new(-0.12, 0.03, 0.0, 0.0005, -0.0003, 1100.0, 1090.0, 950.0, 530.0, 311.0);
    h.initialize(p, Size::new(1920, 1080));
    h
}

#[test]
fn uninitialized_is_the_identity() {
    let h = DistortionHelper::default();
    assert!(!h.is_initialized());
    assert_eq!(h.content_hash(), 0);
    assert_eq!(h.distort(PointF::new(3.0, 4.0)), PointF::new(3.0, 4.0));
    assert_eq!(h.undistort(PointF::new(3.0, 4.0)), PointF::new(3.0, 4.0));
    assert_eq!(h.distort_line(PointF::new(0.0, 0.0), PointF::new(6.0, 0.0))[3], PointF::new(3.0, 0.0));
}

#[test]
fn zero_principal_point_falls_back_to_defaults_and_hash_is_kept() {
    let mut h = DistortionHelper::default();
    let given = DistortionParameters::new(0.1, 0.0, 0.0, 0.0, 0.0, 500.0, 500.0, 0.0, 0.0, 100.0);
    h.initialize(given.clone(), Size::new(640, 480));
    assert_eq!(h.parameters().unwrap().cx, 320.0);
    assert_eq!(h.content_hash(), given.content_hash());
    assert_eq!(h.image_size(), Size::new(640, 480));
    h.uninitialize();
    assert!(!h.is_initialized());
}

#[test]
fn principal_point_is_fixed_and_round_trips_are_close() {
    let h = helper();
    assert_eq!(h.distort(PointF::new(950.0, 530.0)), PointF::new(950.0, 530.0));
    let mut seed = 7u32;
    for _ in 0..200 {
        seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        let x = 200.0 + (seed >> 16) as f32 % 1500.0;
        seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        let y = 100.0 + (seed >> 16) as f32 % 880.0;
        let p = PointF::new(x, y);
        let back = h.undistort(h.distort(p));
        assert!((back.x - x).abs() < 1e-3 && (back.y - y).abs() < 1e-3, "{p:?} -> {back:?}");
    }
}

#[test]
fn lines_have_seven_points_and_quads_are_rectified_per_vertex() {
    let h = helper();
    let (a, b) = (PointF::new(100.0, 100.0), PointF::new(1800.0, 900.0));
    let line = h.distort_line(a, b);
    assert_eq!(line.len(), 7);
    assert_eq!((line[0], line[6]), (a, b));
    let rect = h.distort_rectified_line(a, b);
    assert_eq!(rect.len(), 7);
    assert_eq!(rect[0], h.distort(a));
    let q = Quadrilateral::new(a, PointF::new(1800.0, 100.0), b, PointF::new(100.0, 900.0));
    let u = h.undistort_quad(&q);
    assert_eq!(u.a(), h.undistort(a));
    assert_eq!(u.c(), h.undistort(b));
}
