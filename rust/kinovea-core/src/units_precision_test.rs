use crate::units::LengthUnit;
use crate::units_precision::*;

#[test]
fn minimal_digits_rule() {
    assert_eq!(minimal_digits(12.34), 12.0);
    assert_eq!(minimal_digits(3.456), 3.5);
    assert_eq!(minimal_digits(0.0234), 0.02);
}

#[test]
fn dotnet_float_default_format() {
    assert_eq!(format_f32_default(0.0), "0");
    assert_eq!(format_f32_default(12.0), "12");
    assert_eq!(format_f32_default(3.5), "3.5");
    assert_eq!(format_f32_default(0.02), "0.02");
    assert_eq!(format_f32_default(0.00001), "1E-05");
    assert_eq!(format_f32_default(1234567.0), "1234567");
    assert_eq!(format_f32_default(12345678.0), "1.234568E+07");
    assert_eq!(format_f32_default(-0.5), "-0.5");
}

#[test]
fn pixel_size_switches_metric_unit() {
    assert_eq!(
        pixel_size_text(1.0, 1.0, 100.0, 100.0, LengthUnit::Meters),
        "1 × 1 cm"
    );
    assert_eq!(
        pixel_size_text(1.0, 1.0, 100.0, 100.0, LengthUnit::Pixels),
        ""
    );
}
