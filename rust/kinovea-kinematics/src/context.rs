//! The measurement context that kinematics read instead of `CalibrationHelper` and the
//! player preferences.

use kinovea_core::units::{AngularAccelerationUnit, AngularVelocityUnit};
use kinovea_core::units_convert::{convert_angular_acceleration, convert_angular_velocity};
use kinovea_geometry::helpers::degrees;
use kinovea_geometry::{PointF, SizeF};

/// What kinematics need from a calibration and from the preferences.
pub trait MeasurementContext {
    /// Image point at time `t` → world coordinates.
    fn point_at_time(&self, p: PointF, t: i64) -> PointF;
    /// `(float)(frames / fps)`.
    fn frames_to_seconds(&self, frames: i32) -> f32;
    fn capture_fps(&self) -> f64;
    /// Length units per second → speed unit.
    fn convert_speed(&self, v: f32) -> f32;
    /// Length units per second squared → acceleration unit.
    fn convert_acceleration(&self, a: f32) -> f32;
    /// Acceleration expressed in speed units per second → acceleration unit.
    fn convert_acceleration_from_velocity(&self, a: f32) -> f32;
    fn convert_angle(&self, radians: f32) -> f32;
    fn convert_angular_velocity(&self, radians_per_second: f32) -> f32;
    fn convert_angular_acceleration(&self, radians_per_second_squared: f32) -> f32;
    /// `PlayerPreferences.EnableFiltering`.
    fn filtering_enabled(&self) -> bool;
    /// `PlayerPreferences.EnableHighSpeedDerivativesSmoothing`.
    fn high_speed_smoothing_enabled(&self) -> bool;
}

/// Uncalibrated context: world = (x − cx, −(y − cy)) in pixels, angles in degrees, which is
/// what the default `CalibrationHelper` produces.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PixelContext {
    pub image_size: SizeF,
    pub fps: f64,
    pub filtering: bool,
    pub smoothing: bool,
}

impl PixelContext {
    pub fn new(image_size: SizeF, fps: f64) -> Self {
        PixelContext {
            image_size,
            fps,
            filtering: true,
            smoothing: true,
        }
    }
}

impl MeasurementContext for PixelContext {
    fn point_at_time(&self, p: PointF, _t: i64) -> PointF {
        let cx = self.image_size.width / 2.0;
        let cy = self.image_size.height / 2.0;
        PointF::new(p.x - cx, -(p.y - cy))
    }

    fn frames_to_seconds(&self, frames: i32) -> f32 {
        (f64::from(frames) / self.fps) as f32
    }

    fn capture_fps(&self) -> f64 {
        self.fps
    }

    fn convert_speed(&self, v: f32) -> f32 {
        v
    }

    fn convert_acceleration(&self, a: f32) -> f32 {
        a
    }

    fn convert_acceleration_from_velocity(&self, a: f32) -> f32 {
        a
    }

    fn convert_angle(&self, radians: f32) -> f32 {
        degrees(radians)
    }

    fn convert_angular_velocity(&self, radians_per_second: f32) -> f32 {
        convert_angular_velocity(f64::from(radians_per_second), AngularVelocityUnit::DegreesPerSecond) as f32
    }

    fn convert_angular_acceleration(&self, radians_per_second_squared: f32) -> f32 {
        convert_angular_acceleration(radians_per_second_squared, AngularAccelerationUnit::DegreesPerSecondSquared)
    }

    fn filtering_enabled(&self) -> bool {
        self.filtering
    }

    fn high_speed_smoothing_enabled(&self) -> bool {
        self.smoothing
    }
}
