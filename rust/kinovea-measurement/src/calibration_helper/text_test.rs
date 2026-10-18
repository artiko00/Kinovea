use crate::calibration_axis::CalibrationAxis;
use crate::calibration_helper::text::round_digits;
use crate::calibration_helper_test::full_hd;
use crate::units::Units;
use kinovea_core::units::{AccelerationUnit, LengthUnit, SpeedUnit};
use kinovea_geometry::PointF;
use uuid::Uuid;

fn p(x: f32, y: f32) -> PointF {
    PointF::new(x, y)
}

#[test]
fn rounding_is_half_even() {
    assert_eq!(round_digits(2.345, 2), 2.35);
    assert_eq!(round_digits(0.125, 2), 0.12);
    assert_eq!(round_digits(2.5, 0), 2.0);
    assert_eq!(round_digits(-1.2349, 3), -1.235);
}

#[test]
fn point_length_and_circumference_texts() {
    let h = full_hd();
    assert_eq!(h.point_text(p(1060.0, 440.0), false, 0), "100 ; 100");
    assert_eq!(h.point_text(p(1060.5, 440.25), true, 0), "100.5 ; 99.75 px");
    assert_eq!(h.length_text(p(0.0, 0.0), p(3.0, 4.0), false), "5");
    assert_eq!(h.length_text(p(0.0, 0.0), p(3.0, 4.0), true), "5 px");
    assert_eq!(h.circumference_text(p(0.0, 0.0), p(10.0, 0.0), true), "62.83 px");
    let mut m = full_hd();
    // With the nil drawing id the time-aware path falls back to the static calibration.
    m.calibrate_by_line(Uuid::nil(), 2.0, p(100.0, 500.0), p(300.0, 500.0), CalibrationAxis::LineHorizontal);
    m.set_units(Units {
        length: LengthUnit::Meters,
        decimal_places: 3,
        ..Units::default()
    });
    assert_eq!(m.point_text(p(200.0, 400.0), true, 0), "1 ; 1 m");
    assert_eq!(m.length_text(p(100.0, 500.0), p(133.0, 500.0), false), "0.33");
}

#[test]
fn abbreviations_depend_on_calibration_state() {
    let mut h = full_hd();
    assert_eq!(h.speed_abbreviation(), SpeedUnit::PixelsPerSecond.abbreviation());
    assert_eq!(h.acceleration_abbreviation(), AccelerationUnit::PixelsPerSecondSquared.abbreviation());
    assert_eq!(h.angle_abbreviation(), "°");
    assert_eq!(h.angular_velocity_abbreviation(), "deg/s");
    assert_eq!(h.angular_acceleration_abbreviation(), "deg/s²");
    h.set_length_unit(LengthUnit::Meters);
    assert_eq!(h.speed_abbreviation(), "m/s");
    assert_eq!(h.acceleration_abbreviation(), "m/s²");
    assert_eq!(h.length_abbreviation(), "m");
}
