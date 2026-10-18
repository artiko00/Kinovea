use crate::calibration_axis::CalibrationAxis;

#[test]
fn names_round_trip() {
    for a in CalibrationAxis::ALL {
        assert_eq!(a.name().parse::<CalibrationAxis>().unwrap(), *a);
    }
    assert_eq!(CalibrationAxis::default(), CalibrationAxis::LineHorizontal);
    assert!("Diagonal".parse::<CalibrationAxis>().is_err());
}
