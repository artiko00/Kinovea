use super::calibrations::{build, quad, NAMES};
use kinovea_core::units::LengthUnit;
use kinovea_measurement::CalibratorType;

#[test]
fn every_named_calibration_builds() {
    for name in NAMES {
        let h = build(name);
        let plane = name.starts_with("plane");
        assert_eq!(
            h.calibrator_type() == CalibratorType::Plane,
            plane,
            "{name}"
        );
        assert_eq!(
            h.units().length == LengthUnit::Pixels,
            name == "default",
            "{name}"
        );
    }
    assert!(quad().is_convex());
}
