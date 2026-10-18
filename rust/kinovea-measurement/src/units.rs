//! Measurement units selected by the user, injected instead of read from the preferences.

use kinovea_core::units::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Units {
    pub length: LengthUnit,
    pub speed: SpeedUnit,
    pub acceleration: AccelerationUnit,
    pub angle: AngleUnit,
    pub angular_velocity: AngularVelocityUnit,
    pub angular_acceleration: AngularAccelerationUnit,
    /// `PlayerPreferences.DecimalPlaces`, used by the value texts.
    pub decimal_places: i32,
}

impl Default for Units {
    /// The player preference defaults with the pixel length unit of a fresh calibration.
    fn default() -> Self {
        Units {
            length: LengthUnit::Pixels,
            speed: SpeedUnit::MetersPerSecond,
            acceleration: AccelerationUnit::MetersPerSecondSquared,
            angle: AngleUnit::Degree,
            angular_velocity: AngularVelocityUnit::DegreesPerSecond,
            angular_acceleration: AngularAccelerationUnit::DegreesPerSecondSquared,
            decimal_places: 2,
        }
    }
}
