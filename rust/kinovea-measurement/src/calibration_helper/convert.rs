//! Unit conversions and the `MeasurementContext` implementation.

use super::CalibrationHelper;
use kinovea_core::units::AngleUnit;
use kinovea_core::units_convert::*;
use kinovea_geometry::helpers::degrees;
use kinovea_geometry::PointF;
use kinovea_kinematics::MeasurementContext;

impl CalibrationHelper {
    /// Length units per second → speed unit.
    pub fn convert_speed(&self, v: f32) -> f32 {
        convert_velocity(v, self.units.length, self.units.speed)
    }

    /// Length units per second squared → acceleration unit.
    pub fn convert_acceleration(&self, a: f32) -> f32 {
        convert_acceleration(a, self.units.length, self.units.acceleration)
    }

    /// Acceleration in speed units per second → acceleration unit.
    pub fn convert_acceleration_from_velocity(&self, a: f32) -> f32 {
        let magnitude = convert_for_length_unit(a, self.units.speed, self.units.length);
        convert_acceleration(magnitude, self.units.length, self.units.acceleration)
    }

    pub fn convert_angle(&self, radians: f32) -> f32 {
        if self.units.angle == AngleUnit::Radian { radians } else { degrees(radians) }
    }

    pub fn convert_angular_velocity(&self, radians_per_second: f32) -> f32 {
        convert_angular_velocity(f64::from(radians_per_second), self.units.angular_velocity) as f32
    }

    pub fn convert_angular_acceleration(&self, radians_per_second_squared: f32) -> f32 {
        convert_angular_acceleration(radians_per_second_squared, self.units.angular_acceleration)
    }
}

/// Filtering and smoothing preferences are not part of the calibration; they are carried by
/// [`CalibrationHelper::with_preferences`].
pub struct CalibratedContext<'a> {
    pub helper: &'a CalibrationHelper,
    pub filtering: bool,
    pub smoothing: bool,
}

impl CalibrationHelper {
    /// Pairs the calibration with the two player preferences kinematics need.
    pub fn with_preferences(&self, filtering: bool, smoothing: bool) -> CalibratedContext<'_> {
        CalibratedContext {
            helper: self,
            filtering,
            smoothing,
        }
    }
}

/// Forwards trait methods whose name and signature match a `CalibrationHelper` method.
macro_rules! forward {
    ($($name:ident($($arg:ident: $ty:ty),*) -> $ret:ty;)*) => {
        $(fn $name(&self, $($arg: $ty),*) -> $ret { self.helper.$name($($arg),*) })*
    };
}

impl MeasurementContext for CalibratedContext<'_> {
    forward!(
        point_at_time(p: PointF, t: i64) -> PointF;
        capture_fps() -> f64;
        convert_speed(v: f32) -> f32;
        convert_acceleration(a: f32) -> f32;
        convert_acceleration_from_velocity(a: f32) -> f32;
        convert_angle(radians: f32) -> f32;
        convert_angular_velocity(radians_per_second: f32) -> f32;
        convert_angular_acceleration(radians_per_second_squared: f32) -> f32;
    );

    fn frames_to_seconds(&self, frames: i32) -> f32 {
        self.helper.time(frames)
    }

    fn filtering_enabled(&self) -> bool {
        self.filtering
    }

    fn high_speed_smoothing_enabled(&self) -> bool {
        self.smoothing
    }
}
