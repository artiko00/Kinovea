//! Angle state of `AngleHelper.Update` and `SweepAngle.Update`: the sweep of a posture angle in
//! image coordinates and its calibrated value in radians. Arrow positions and the hit region are not
//! ported: the posture never draws the arrows of an angle.

use kinovea_geometry::helpers::distance;
use kinovea_geometry::{PointF, RectangleF};
use kinovea_measurement::CalibrationHelper;

const TAU: f64 = std::f64::consts::PI * 2.0;

/// Flags of a `GenericPostureAngle` (`Radius`, `Signed`, `CCW`, `Supplementary`).
#[derive(Debug, Clone, Copy)]
pub struct AngleFlags {
    pub radius: i32,
    pub signed: bool,
    pub ccw: bool,
    pub supplementary: bool,
}

/// `SweepAngle`: start and sweep in degrees (clockwise from +x), the origin and the bounding box.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sweep {
    pub origin: PointF,
    pub start: f32,
    pub sweep: f32,
    pub bounding: RectangleF,
}

/// `AngleHelper` after an update: the sweep and the calibrated value in radians.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Angle {
    pub sweep: Sweep,
    pub calibrated: f32,
}

/// `AngleHelper.Update`. `None` when a leg coincides with the origin: the original then keeps the
/// previous state, which a stateless painter does not have.
pub fn compute(
    o: PointF,
    a: PointF,
    b: PointF,
    flags: AngleFlags,
    calibration: &CalibrationHelper,
) -> Option<Angle> {
    if o == a || o == b {
        return None;
    }
    let (a, b) = if flags.supplementary {
        (b, PointF::new(2.0 * o.x - a.x, 2.0 * o.y - a.y))
    } else {
        (a, b)
    };
    Some(Angle {
        sweep: sweep_of(o, a, b, flags),
        calibrated: calibrated_angle(o, a, b, flags, calibration),
    })
}

/// `SweepAngle.Update`: the start is the first leg, the sweep runs to the second one.
fn sweep_of(o: PointF, a: PointF, b: PointF, flags: AngleFlags) -> Sweep {
    let (oa, ob) = if flags.ccw {
        (absolute_angle(o, b), absolute_angle(o, a))
    } else {
        (absolute_angle(o, a), absolute_angle(o, b))
    };
    let mut start = oa;
    let mut sweep = if ob > oa { ob - oa } else { (360.0 - oa) + ob };
    if flags.signed && sweep > 180.0 {
        start = ob;
        sweep = 360.0 - sweep;
    }
    Sweep {
        origin: o,
        start,
        sweep,
        bounding: bounding_box(o, a, b, flags.radius as f32),
    }
}

/// `GetAbsoluteAngle`: clockwise angle of `p` seen from `o`, in degrees.
fn absolute_angle(o: PointF, p: PointF) -> f32 {
    let dx = p.x - o.x;
    let dy = o.y - p.y;
    let angle = f64::from(dy).atan2(f64::from(dx));
    let pie = if angle > 0.0 { TAU - angle } else { -angle };
    (pie * 180.0 / std::f64::consts::PI) as f32
}

/// `UpdateBoundingBox`. A non-positive radius is counted from the end of the smallest leg.
fn bounding_box(o: PointF, a: PointF, b: PointF, radius: f32) -> RectangleF {
    let radius = if radius <= 0.0 {
        let r = radius.min(-10.0);
        let smallest = distance(o, a).min(distance(o, b));
        let r = if smallest + r > 0.0 {
            smallest + r
        } else {
            smallest.min(10.0)
        };
        r.trunc()
    } else {
        radius
    };
    let r = radius as i32;
    let r_f = r as f32;
    RectangleF::new(o.x - r_f, o.y - r_f, r_f * 2.0, r_f * 2.0)
}

/// `ComputeCalibratedAngle`: the angle in radians, unsigned values wrapped to [0, tau).
fn calibrated_angle(
    o: PointF,
    a: PointF,
    b: PointF,
    flags: AngleFlags,
    calibration: &CalibrationHelper,
) -> f32 {
    let o2 = calibration.point(o);
    let a2 = calibration.point(a);
    let b2 = calibration.point(b);
    let mut value = if flags.ccw {
        get_angle(o2, a2, b2)
    } else {
        get_angle(o2, b2, a2)
    };
    if !flags.signed && value < 0.0 {
        value = (TAU + f64::from(value)) as f32;
    }
    value
}

/// `GeometryHelper.GetAngle(a, b, a, c)`: signed angle from `ab` to `ac` in [-pi, pi].
fn get_angle(a: PointF, b: PointF, c: PointF) -> f32 {
    let (abx, aby) = (b.x - a.x, b.y - a.y);
    let (acx, acy) = (c.x - a.x, c.y - a.y);
    let perp = abx * acy - aby * acx;
    f64::from(perp).atan2(f64::from(abx * acx + aby * acy)) as f32
}
