//! Value texts: point, length and circumference with the unit abbreviation.

use super::CalibrationHelper;
use kinovea_core::units::{AccelerationUnit, SpeedUnit};
use kinovea_core::xml::format_f64_invariant;
use kinovea_geometry::helpers::distance;
use kinovea_geometry::PointF;

/// `.NET` `Math.Round(double, int)`: round half to even at `digits` decimals.
pub fn round_digits(value: f64, digits: i32) -> f64 {
    let p = 10f64.powi(digits);
    (value * p).round_ties_even() / p
}

impl CalibrationHelper {
    fn rounded(&self, value: f64) -> String {
        format_f64_invariant(round_digits(value, self.units.decimal_places))
    }

    fn with_abbreviation(&self, text: String, abbreviation: bool) -> String {
        if abbreviation { format!("{text} {}", self.length_abbreviation()) } else { text }
    }

    /// `x ; y` of the world point at `time`.
    pub fn point_text(&self, p: PointF, abbreviation: bool, time: i64) -> String {
        let a = self.point_at_time(p, time);
        let text = format!("{} ; {}", self.rounded(f64::from(a.x)), self.rounded(f64::from(a.y)));
        self.with_abbreviation(text, abbreviation)
    }

    /// World length between two image points.
    pub fn length_text(&self, p1: PointF, p2: PointF, abbreviation: bool) -> String {
        let length = distance(self.point(p1), self.point(p2));
        self.with_abbreviation(self.rounded(f64::from(length)), abbreviation)
    }

    /// Circumference of the world circle whose radius is the length between two image points.
    pub fn circumference_text(&self, p1: PointF, p2: PointF, abbreviation: bool) -> String {
        let length = distance(self.point(p1), self.point(p2));
        let circ = 2.0 * std::f64::consts::PI * f64::from(length);
        self.with_abbreviation(self.rounded(circ), abbreviation)
    }

    pub fn length_abbreviation(&self) -> &'static str {
        self.units.length.abbreviation()
    }

    /// Pixels per second unless calibrated.
    pub fn speed_abbreviation(&self) -> &'static str {
        if self.is_calibrated() { self.units.speed.abbreviation() } else { SpeedUnit::PixelsPerSecond.abbreviation() }
    }

    /// Pixels per second squared unless calibrated.
    pub fn acceleration_abbreviation(&self) -> &'static str {
        if self.is_calibrated() {
            self.units.acceleration.abbreviation()
        } else {
            AccelerationUnit::PixelsPerSecondSquared.abbreviation()
        }
    }

    pub fn angle_abbreviation(&self) -> &'static str {
        self.units.angle.abbreviation()
    }

    pub fn angular_velocity_abbreviation(&self) -> &'static str {
        self.units.angular_velocity.abbreviation()
    }

    pub fn angular_acceleration_abbreviation(&self) -> &'static str {
        self.units.angular_acceleration.abbreviation()
    }
}
