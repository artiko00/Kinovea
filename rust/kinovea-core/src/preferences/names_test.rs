use super::names::NamedEnum;
use crate::timecode::TimecodeFormat;
use crate::units::SpeedUnit;

#[test]
fn core_enums_are_named() {
    assert_eq!(<TimecodeFormat as NamedEnum>::ALL.len(), 9);
    assert_eq!(NamedEnum::name(SpeedUnit::Knots), "Knots");
}
