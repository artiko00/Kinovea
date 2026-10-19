//! Port of `SweepAngle.Update` (`Measurement/Geometry/SweepAngle.cs`) for the zero radius that
//! `AngleHelper` always passes. Start and sweep are GDI+ degrees (clockwise from +x, y down).

use crate::paint::helpers::frame::point_box_f;
use kinovea_geometry::helpers::{degrees_f64, distance, radians};
use kinovea_geometry::{PointF, RectangleF, Vector};

/// `Math.Min(radius, -10)`: the smallest leg is inset by this much for the arrow.
const INSET: f32 = 10.0;
/// `Math.Min(smallest, 10)`: the radius when the inset would leave no room.
const MAX_RADIUS: f32 = 10.0;
/// Distance of the arrow start from the arrow end, along the tangent.
const ARROW_LENGTH: f32 = 100.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sweep {
    pub start: f32,
    pub sweep: f32,
    pub bounding_box: RectangleF,
    pub arrow_start: PointF,
    pub arrow_end: PointF,
}

impl Sweep {
    /// The state of a `SweepAngle` that was never updated: a quarter turn with an empty box.
    pub fn empty() -> Self {
        Sweep {
            start: 270.0,
            sweep: 90.0,
            bounding_box: RectangleF::EMPTY,
            arrow_start: PointF::EMPTY,
            arrow_end: PointF::EMPTY,
        }
    }

    /// `SweepAngle.Update(o, a, b, 0, signed, ccw)`: from the first leg to the second one.
    pub fn update(o: PointF, a: PointF, b: PointF, signed: bool, ccw: bool) -> Self {
        let (oa, ob) = if ccw {
            (absolute_angle(o, b), absolute_angle(o, a))
        } else {
            (absolute_angle(o, a), absolute_angle(o, b))
        };
        let mut start = oa;
        let mut sweep = if ob > oa { ob - oa } else { (360.0 - oa) + ob };
        let mut neg = false;
        if signed && sweep > 180.0 {
            start = ob;
            sweep = 360.0 - sweep;
            neg = true;
        }
        let (bounding_box, arrow_start, arrow_end) = arrow_geometry(o, a, b, ccw, neg);
        Sweep {
            start,
            sweep,
            bounding_box,
            arrow_start,
            arrow_end,
        }
    }
}

/// `SweepAngle.GetAbsoluteAngle`: degrees clockwise from +x of the vector from `o` to `p`.
fn absolute_angle(o: PointF, p: PointF) -> f32 {
    let angle = f64::from(o.y - p.y).atan2(f64::from(p.x - o.x));
    let pie = if angle > 0.0 {
        std::f64::consts::TAU - angle
    } else {
        -angle
    };
    degrees_f64(pie) as f32
}

/// `UpdateBoundingBox` for the zero radius. The arrow end sits on the measure leg; the arrow start
/// is `ARROW_LENGTH` along the tangent, from a point one degree further round the circle.
fn arrow_geometry(
    o: PointF,
    a: PointF,
    b: PointF,
    ccw: bool,
    neg: bool,
) -> (RectangleF, PointF, PointF) {
    let smallest = distance(o, a).min(distance(o, b));
    let radius = if smallest - INSET > 0.0 {
        smallest - INSET
    } else {
        smallest.min(MAX_RADIUS)
    };
    let radius = radius.trunc();

    let to_b = Vector::from_points(o, b).normalized();
    let arrow_end = o + to_b * radius;

    let mut step = radians(1.0);
    if (ccw && neg) || (!ccw && !neg) {
        step = -step;
    }
    let cx = f64::from(step).cos() as f32;
    let cy = f64::from(step).sin() as f32;
    let dir_x = to_b * radius;
    let dir_y = Vector::new(-dir_x.y, dir_x.x);
    let arrow_start_on_circle = o + dir_x * cx + dir_y * cy;
    let tangent = Vector::from_points(arrow_end, arrow_start_on_circle).normalized();
    let arrow_start = arrow_end + tangent * ARROW_LENGTH;

    (point_box_f(o, radius as i32), arrow_start, arrow_end)
}
