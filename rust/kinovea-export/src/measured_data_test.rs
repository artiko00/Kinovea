use crate::measured_data::{unit_symbols, ExportPrefs};
use kinovea_core::preferences::enums::ExportSpace;
use kinovea_core::timecode::TimecodeFormat;
use kinovea_core::units::LengthUnit;
use kinovea_metadata::Metadata;

fn prefs(space: ExportSpace) -> ExportPrefs {
    ExportPrefs {
        export_space: space,
        timecode_format: TimecodeFormat::ClassicTime,
        producer: "Kinovea.0.0".to_owned(),
    }
}

#[test]
fn uncalibrated_world_space_uses_pixels_seconds_and_degrees() {
    let m = Metadata::default();
    let u = unit_symbols(&m, &prefs(ExportSpace::WorldSpace));
    assert_eq!(
        (u.time_symbol, u.length_symbol, u.angle_symbol),
        ("s", "px", "°")
    );
}

#[test]
fn image_space_always_uses_pixels_even_when_calibrated() {
    let mut m = Metadata::default();
    m.calibration.set_length_unit(LengthUnit::Meters);
    assert_eq!(
        unit_symbols(&m, &prefs(ExportSpace::WorldSpace)).length_symbol,
        "m"
    );
    assert_eq!(
        unit_symbols(&m, &prefs(ExportSpace::ImageSpace)).length_symbol,
        "px"
    );
}
