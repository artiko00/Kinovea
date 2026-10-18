use crate::units::Units;
use kinovea_core::units::{AngleUnit, LengthUnit, SpeedUnit};

#[test]
fn defaults_follow_the_player_preferences() {
    let u = Units::default();
    assert_eq!(u.length, LengthUnit::Pixels);
    assert_eq!(u.speed, SpeedUnit::MetersPerSecond);
    assert_eq!(u.angle, AngleUnit::Degree);
    assert_eq!(u.decimal_places, 2);
}
