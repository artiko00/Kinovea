//! Port of `AngleHelper` (`Measurement/Geometry/AngleHelper.cs`): the sweep of an angle in image space,
//! its value in calibrated space and the position of its text. Shared by the angle and posture painters.

#[cfg(test)]
mod radius_test;
pub mod sweep;
#[cfg(test)]
mod sweep_test;

use kinovea_geometry::helpers::{angle_at, radians};
use kinovea_geometry::{PointF, SizeF};
use kinovea_measurement::CalibrationHelper;
use sweep::Sweep;

/// `defaultTextDistance`: distance of the text from the origin for a font of 12 points.
const DEFAULT_TEXT_DISTANCE: f32 = 40.0;
const DEFAULT_FONT_SIZE: f32 = 12.0;

/// `GenericPostureAngle` / `AngleHelper` flags: the radius of the sweep and its orientation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AngleFlags {
    pub radius: i32,
    pub signed: bool,
    pub ccw: bool,
    pub supplementary: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AngleHelper {
    pub sweep: Sweep,
    /// Radians in calibrated space: [-π, π] when signed, [0, 2π) otherwise.
    pub calibrated_angle: f32,
}

impl AngleHelper {
    /// `AngleHelper.Update`: a leg that coincides with the origin leaves the helper empty.
    pub fn new(
        o: PointF,
        a: PointF,
        b: PointF,
        flags: AngleFlags,
        calibration: &CalibrationHelper,
    ) -> Self {
        if o == a || o == b {
            return AngleHelper {
                sweep: Sweep::empty(),
                calibrated_angle: 0.0,
            };
        }
        // Point symmetry around o gives the actual second leg; drawing and value both use it.
        let (a, b) = if flags.supplementary {
            (b, PointF::new(2.0 * o.x - a.x, 2.0 * o.y - a.y))
        } else {
            (a, b)
        };
        AngleHelper {
            sweep: Sweep::update(o, a, b, flags.radius as f32, flags.signed, flags.ccw),
            calibrated_angle: calibrated_angle(o, a, b, flags, calibration),
        }
    }

    /// `GetTextPosition`: offset of the text box from the origin. The text centre sits at
    /// `text_distance` on the bisector of the sweep.
    pub fn text_position(&self, text_distance: i32, label: SizeF) -> PointF {
        let bisector = f64::from(radians(self.sweep.start + self.sweep.sweep / 2.0));
        let distance = f64::from(text_distance);
        let x = bisector.cos() * distance - f64::from(label.width / 2.0);
        let y = bisector.sin() * distance - f64::from(label.height / 2.0);
        PointF::new(x as f32, y as f32)
    }
}

/// `UpdateTextDistance`: the distance grows with the font size.
pub fn text_distance(font_size: i32) -> i32 {
    (DEFAULT_TEXT_DISTANCE * (font_size as f32 / DEFAULT_FONT_SIZE)) as i32
}

/// `ComputeCalibratedAngle`: the value of the legs in world space (`GetPoint`).
fn calibrated_angle(
    o: PointF,
    a: PointF,
    b: PointF,
    flags: AngleFlags,
    calibration: &CalibrationHelper,
) -> f32 {
    let (o, a, b) = (
        calibration.point(o),
        calibration.point(a),
        calibration.point(b),
    );
    let value = if flags.ccw {
        angle_at(o, a, b)
    } else {
        angle_at(o, b, a)
    };
    if !flags.signed && value < 0.0 {
        (std::f64::consts::TAU + f64::from(value)) as f32
    } else {
        value
    }
}
