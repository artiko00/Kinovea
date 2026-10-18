use crate::calibrator_type::CalibratorType;

#[test]
fn names_round_trip() {
    assert_eq!(CalibratorType::ALL.len(), 3);
    for t in CalibratorType::ALL {
        assert_eq!(t.name().parse::<CalibratorType>().unwrap(), *t);
    }
    assert_eq!("Plane".parse::<CalibratorType>(), Ok(CalibratorType::Plane));
}
