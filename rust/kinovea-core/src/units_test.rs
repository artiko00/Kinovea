use crate::units::*;

#[test]
fn abbreviations_of_every_value() {
    let lengths: Vec<_> = LengthUnit::ALL.iter().map(|u| u.abbreviation()).collect();
    assert_eq!(lengths, ["mm", "cm", "m", "in", "ft", "yd", "%", "px"]);
    let speeds: Vec<_> = SpeedUnit::ALL.iter().map(|u| u.abbreviation()).collect();
    assert_eq!(speeds, ["m/s", "km/h", "ft/s", "mph", "kn", "px/s"]);
    assert_eq!(
        AccelerationUnit::PixelsPerSecondSquared.abbreviation(),
        "px/s²"
    );
    assert_eq!(AngleUnit::Degree.abbreviation(), "°");
    assert_eq!(
        AngularVelocityUnit::RevolutionsPerMinute.abbreviation(),
        "rpm"
    );
    assert_eq!(
        AngularAccelerationUnit::DegreesPerSecondSquared.abbreviation(),
        "deg/s²"
    );
    assert_eq!(CadenceUnit::ALL.len(), 8);
    assert_eq!(CadenceUnit::StepsPerMinute.abbreviation(), "spm");
}

#[test]
fn names_round_trip_and_reject_unknown() {
    for u in SpeedUnit::ALL {
        assert_eq!(u.to_string().parse::<SpeedUnit>().unwrap(), *u);
    }
    assert_eq!("Meters".parse::<LengthUnit>().unwrap(), LengthUnit::Meters);
    let err = "Furlongs".parse::<LengthUnit>().unwrap_err();
    assert_eq!(err.to_string(), "unknown length unit: Furlongs");
}

#[test]
fn time_abbreviations() {
    use crate::timecode::TimecodeFormat::*;
    assert_eq!(time_abbreviation(Frames), "F");
    assert_eq!(time_abbreviation(Microseconds), "μs");
    assert_eq!(time_abbreviation(ClassicTime), "s");
    assert_eq!(time_abbreviation(Normalized), "s");
}
