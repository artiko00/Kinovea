//! The calibrations of `tests/cases/calibration/transforms.json`, rebuilt in Rust.

use kinovea_core::units::LengthUnit;
use kinovea_geometry::{PointF, Quadrilateral, Size, SizeF};
use kinovea_measurement::{CalibrationAxis, CalibrationHelper, CalibratorType, Units};
use uuid::Uuid;

pub const NAMES: [&str; 5] = ["default", "line-h", "line-v", "plane", "plane-offset"];

pub fn p(x: f32, y: f32) -> PointF {
    PointF::new(x, y)
}

pub fn quad() -> Quadrilateral {
    Quadrilateral::new(
        p(300.0, 200.0),
        p(1500.0, 240.0),
        p(1700.0, 900.0),
        p(100.0, 880.0),
    )
}

/// Mirrors `tests/cases/calibration/transforms.json`.
pub fn build(name: &str) -> CalibrationHelper {
    let mut h = CalibrationHelper::without_tracking(Size::new(1920, 1080), Units::default(), 50.0);
    match name {
        "default" => {}
        "line-h" => h.calibrate_by_line(
            Uuid::nil(),
            2.0,
            p(100.0, 500.0),
            p(300.0, 500.0),
            CalibrationAxis::LineHorizontal,
        ),
        "line-v" => h.calibrate_by_line(
            Uuid::nil(),
            2.0,
            p(100.0, 500.0),
            p(100.0, 300.0),
            CalibrationAxis::LineVertical,
        ),
        "plane" => {
            h.set_calibrator_type(CalibratorType::Plane);
            h.calibrate_by_plane(Uuid::nil(), SizeF::new(4.0, 3.0), &quad());
        }
        "plane-offset" => {
            h.set_calibrator_type(CalibratorType::Plane);
            h.calibrate_by_plane(Uuid::nil(), SizeF::new(400.0, 300.0), &quad());
            h.set_origin(p(900.0, 600.0));
            h.set_offset(p(12.5, -7.25));
            h.set_length_unit(LengthUnit::Centimeters);
        }
        other => panic!("unknown calibration {other}"),
    }
    if name.starts_with("line") || name == "plane" {
        h.set_length_unit(LengthUnit::Meters);
    }
    h
}
