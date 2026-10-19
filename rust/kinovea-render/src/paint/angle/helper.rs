//! Port of `AngleHelper` (`Measurement/Geometry/AngleHelper.cs`): the sweep of the angle in image
//! space, its value in calibrated space and the position of its text.

use super::sweep::Sweep;
use kinovea_geometry::helpers::{angle_at, radians};
use kinovea_geometry::{PointF, SizeF};
use kinovea_measurement::CalibrationHelper;

/// `defaultTextDistance`: distance of the text from the origin for a font of 12 points.
const DEFAULT_TEXT_DISTANCE: f32 = 40.0;
const DEFAULT_FONT_SIZE: f32 = 12.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AngleHelper {
    pub sweep: Sweep,
    /// Radians in calibrated space: [-π, π] when signed, [0, 2π) otherwise.
    pub calibrated_angle: f32,
}

impl AngleHelper {
    /// `AngleHelper.Update`: a leg that coincides with the origin leaves the helper unchanged.
    pub fn new(
        o: PointF,
        a: PointF,
        b: PointF,
        signed: bool,
        ccw: bool,
        supplementary: bool,
        calibration: &CalibrationHelper,
    ) -> Self {
        if o == a || o == b {
            return AngleHelper {
                sweep: Sweep::empty(),
                calibrated_angle: 0.0,
            };
        }
        // Point symmetry around o gives the actual second leg; drawing and value both use it.
        let (a, b) = if supplementary {
            (b, PointF::new(2.0 * o.x - a.x, 2.0 * o.y - a.y))
        } else {
            (a, b)
        };
        AngleHelper {
            sweep: Sweep::update(o, a, b, signed, ccw),
            calibrated_angle: calibrated_angle(o, a, b, signed, ccw, calibration),
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
    signed: bool,
    ccw: bool,
    calibration: &CalibrationHelper,
) -> f32 {
    let (o, a, b) = (
        calibration.point(o),
        calibration.point(a),
        calibration.point(b),
    );
    let value = if ccw {
        angle_at(o, a, b)
    } else {
        angle_at(o, b, a)
    };
    if !signed && value < 0.0 {
        (std::f64::consts::TAU + f64::from(value)) as f32
    } else {
        value
    }
}
