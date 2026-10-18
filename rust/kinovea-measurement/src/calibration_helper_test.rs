use crate::calibration_axis::CalibrationAxis;
use crate::calibration_helper::CalibrationHelper;
use crate::calibrator_type::CalibratorType;
use crate::units::Units;
use kinovea_core::units::LengthUnit;
use kinovea_geometry::{PointF, Quadrilateral, Size, SizeF};
use uuid::Uuid;

fn p(x: f32, y: f32) -> PointF {
    PointF::new(x, y)
}

fn near(a: PointF, b: PointF, tol: f32) -> bool {
    (a.x - b.x).abs() < tol && (a.y - b.y).abs() < tol
}

pub(crate) fn full_hd() -> CalibrationHelper {
    CalibrationHelper::without_tracking(Size::new(1920, 1080), Units::default(), 100.0)
}

pub(crate) fn plane_quad() -> Quadrilateral {
    Quadrilateral::new(p(300.0, 200.0), p(1500.0, 240.0), p(1700.0, 900.0), p(100.0, 880.0))
}

#[test]
fn default_mapping_is_pixels_from_the_centre_with_y_up() {
    let h = full_hd();
    assert!(near(h.point(p(1060.0, 440.0)), p(100.0, 100.0), 1e-4));
    assert!(near(h.image_point(p(100.0, 100.0)), p(1060.0, 440.0), 1e-3));
    assert_eq!(h.calibrator_type(), CalibratorType::Line);
    assert!(!h.is_calibrated());
    assert_eq!(h.drawing_id(), Uuid::nil());
    assert_eq!(h.image_size(), Size::new(1920, 1080));
    assert_eq!(h.capture_fps(), 100.0);
    assert!(near(h.origin(), p(960.0, 540.0), 1e-3));
    assert!((h.scalar(50.0) - 50.0).abs() < 1e-4 && (h.scalar(-50.0) + 50.0).abs() < 1e-4);
    assert!((h.image_scalar(-20.0) + 20.0).abs() < 1e-3);
    assert!((h.image_length(p(0.0, 0.0), p(3.0, 4.0)) - 5.0).abs() < 1e-3);
    assert_eq!(h.time(2), 0.02);
    assert_eq!(h.calibration_axis(), CalibrationAxis::LineHorizontal);
}

#[test]
fn calibrate_by_line_and_update() {
    let mut h = full_hd();
    let id = Uuid::new_v4();
    h.calibrate_by_line(id, 2.0, p(100.0, 500.0), p(300.0, 500.0), CalibrationAxis::LineHorizontal);
    h.set_length_unit(LengthUnit::Meters);
    assert!(h.is_calibrated());
    assert!(near(h.point(p(200.0, 400.0)), p(1.0, 1.0), 1e-4));
    h.update_line(Uuid::new_v4(), p(100.0, 500.0), p(200.0, 500.0));
    assert!(near(h.point(p(200.0, 400.0)), p(1.0, 1.0), 1e-4), "other id is ignored");
    h.update_line(id, p(100.0, 500.0), p(200.0, 500.0));
    assert!(near(h.point(p(200.0, 400.0)), p(2.0, 2.0), 1e-4));
    assert_eq!(h.drawing_id(), id);
}

#[test]
fn calibrate_by_plane_origin_offset_and_reset() {
    let mut h = full_hd();
    let id = Uuid::new_v4();
    h.set_calibrator_type(CalibratorType::Plane);
    h.calibrate_by_plane(id, SizeF::new(4.0, 3.0), &plane_quad());
    assert!(h.plane_is_valid());
    assert_eq!(h.plane_size(), SizeF::new(4.0, 3.0));
    assert_eq!(h.projected_quad(), plane_quad());
    assert!(near(h.point(plane_quad().b()), p(4.0, 3.0), 1e-3));
    let one_one = h.image_point(p(1.0, 1.0));
    h.set_origin(one_one);
    assert!(near(h.point(one_one), p(0.0, 0.0), 1e-3));
    h.set_offset(p(0.5, 0.0));
    assert_eq!(h.world_offset(), p(0.5, 0.0));
    assert!(near(h.point(one_one), p(0.5, 0.0), 1e-3));
    h.reset_origin();
    assert!(near(h.point(plane_quad().d()), p(0.5, 0.0), 1e-3));
    let mut moved = plane_quad();
    moved.translate(10.0, 0.0);
    h.update_plane(id, &moved);
    assert_eq!(h.projected_quad(), moved);
    let hash = h.content_hash();
    assert_eq!(hash, h.content_hash());
    h.reset();
    assert_eq!(h.calibrator_type(), CalibratorType::Line);
    assert!(near(h.point(p(1060.0, 440.0)), p(100.0, 100.0), 1e-4));
    assert_ne!(hash, h.content_hash());
}

#[test]
fn units_can_be_replaced() {
    let mut h = full_hd();
    h.set_units(Units {
        decimal_places: 3,
        length: LengthUnit::Centimeters,
        ..Units::default()
    });
    assert_eq!(h.units().decimal_places, 3);
    assert_eq!(h.length_abbreviation(), "cm");
    h.set_capture_fps(50.0);
    assert_eq!(h.time(1), 0.02);
}
