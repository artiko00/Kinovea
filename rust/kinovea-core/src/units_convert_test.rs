use crate::units::{AngularVelocityUnit, CadenceUnit, LengthUnit, SpeedUnit};
use crate::units_convert::*;

fn close(a: f32, b: f32) -> bool {
    (a - b).abs() <= 1e-4 * b.abs().max(1.0)
}

#[test]
fn meters_per_second_to_kilometers_per_hour() {
    let v = convert_velocity(10.0, LengthUnit::Meters, SpeedUnit::KilometersPerHour);
    assert!(close(v, 36.0), "{v}");
}

#[test]
fn pixels_pass_through() {
    assert_eq!(
        convert_velocity(7.5, LengthUnit::Pixels, SpeedUnit::KilometersPerHour),
        27000.0
    );
    assert_eq!(
        convert_for_length_unit(3.0, SpeedUnit::PixelsPerSecond, LengthUnit::Meters),
        3.0
    );
}

#[test]
fn kilometers_per_hour_magnitude_into_centimeters() {
    let v = convert_for_length_unit(1.0, SpeedUnit::KilometersPerHour, LengthUnit::Centimeters);
    assert!((v - 100_000.0).abs() <= 1e-3, "{v}");
}

#[test]
fn angular_and_frequency() {
    let rpm = convert_angular_velocity(
        std::f64::consts::PI,
        AngularVelocityUnit::RevolutionsPerMinute,
    );
    assert!((rpm - 30.0).abs() < 1e-9);
    assert_eq!(convert_frequency(2.0, CadenceUnit::StepsPerMinute), 120.0);
}
