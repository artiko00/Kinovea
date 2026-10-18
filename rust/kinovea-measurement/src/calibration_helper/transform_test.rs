use crate::calibration_helper::CalibrationHelper;
use crate::calibration_helper_test::{full_hd, plane_quad};
use crate::calibrator_type::CalibratorType;
use crate::source::TrackedCalibrationSource;
use crate::units::Units;
use kinovea_geometry::shapes::Circle;
use kinovea_geometry::{PointF, Quadrilateral, Size, SizeF};
use uuid::Uuid;

fn p(x: f32, y: f32) -> PointF {
    PointF::new(x, y)
}

fn near(a: PointF, b: PointF, tol: f32) -> bool {
    (a.x - b.x).abs() < tol && (a.y - b.y).abs() < tol
}

/// Quad translated by 10·time px; origin tracked at (960 + time, 540).
struct Moving {
    tracked: bool,
}

impl TrackedCalibrationSource for Moving {
    fn calibration_origin(&self, time: i64) -> PointF {
        p(960.0 + time as f32, 540.0)
    }
    fn calibration_quad(&self, time: i64, _kind: CalibratorType, _id: Uuid) -> Quadrilateral {
        let mut q = plane_quad();
        q.translate(10.0 * time as f32, 0.0);
        q
    }
    fn has_tracking_data(&self, _id: Uuid) -> bool {
        self.tracked
    }
}

fn tracked(tracked: bool) -> CalibrationHelper {
    let mut h = CalibrationHelper::new(Size::new(1920, 1080), Units::default(), 100.0, Box::new(Moving { tracked }));
    h.set_calibrator_type(CalibratorType::Plane);
    h.calibrate_by_plane(Uuid::new_v4(), SizeF::new(4.0, 3.0), &plane_quad());
    h
}

#[test]
fn tracked_quad_equals_a_static_calibration_on_the_translated_quad() {
    let h = tracked(true);
    let mut q = plane_quad();
    q.translate(100.0, 0.0);
    let mut static_h = full_hd();
    static_h.set_calibrator_type(CalibratorType::Plane);
    static_h.calibrate_by_plane(Uuid::new_v4(), SizeF::new(4.0, 3.0), &q);
    let probe = p(900.0, 600.0);
    assert!(near(h.point_at_time(probe, 10), static_h.point(probe), 1e-3));
    assert!(near(h.point_at_time(probe, 0), h.point(probe), 1e-3));
}

#[test]
fn tracked_origin_only_shifts_the_origin() {
    let h = tracked(false);
    let probe = p(900.0, 600.0);
    let at = h.point_at_time(probe, 0);
    assert!(near(at, h.calibrator().transform_with_origin(probe, p(960.0, 540.0)), 1e-5));
    let later = h.point_at_time(probe, 50);
    assert!(!near(at, later, 1e-3));
    assert!(near(h.point_from_rectified(probe), h.point(probe), 1e-6));
}

#[test]
fn without_drawing_id_time_is_ignored() {
    let h = full_hd();
    assert_eq!(h.point_at_time(p(1060.0, 440.0), 99), h.point(p(1060.0, 440.0)));
}

#[test]
fn ellipses_of_circles() {
    let h = full_hd();
    let e = h.ellipse_from_circle(Circle::new(p(100.0, 100.0), 50.0));
    assert!(near(e.center, p(1060.0, 440.0), 1e-3));
    assert!((e.semi_major_axis - 50.0).abs() < 1e-3 && e.rotation == 0.0);
    let (e2, left, right) = h.ellipse_from_image_circle(p(500.0, 500.0), 20.0);
    assert_eq!((e2.center, left, right), (p(500.0, 500.0), p(480.0, 500.0), p(520.0, 500.0)));

    let plane = tracked(false);
    let (e3, left, right) = plane.ellipse_from_image_circle(p(900.0, 600.0), 40.0);
    assert!(near(e3.center, p(900.0, 600.0), 2.0), "{e3:?}");
    assert!(left.x < 900.0 && right.x > 900.0);
    let e4 = plane.ellipse_from_circle(Circle::new(p(2.0, 1.5), 0.5));
    assert!(e4.semi_major_axis > e4.semi_minor_axis && e4.semi_minor_axis > 0.0, "{e4:?}");
}
