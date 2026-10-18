//! Planar helpers (`GeometryHelper`) and scalar helpers (`MathHelper`).
//!
//! Angles are in radians unless a function name says degrees; results are single precision
//! with the same intermediate widening as the original (`Math.*` on doubles, narrowed to float).

mod lines;
#[cfg(test)]
mod lines_test;
mod snapping;
#[cfg(test)]
mod snapping_test;

pub use lines::{closest_point, intersection_rectangle_center};
pub use snapping::{
    point_at_angle, point_at_angle_and_distance, point_at_closest_rotation_step,
    point_at_closest_rotation_step_cardinal, point_on_parallel,
};

use crate::point::PointF;
use crate::vector::Vector;

/// Distance between `a` and `b`.
pub fn distance(a: PointF, b: PointF) -> f32 {
    Vector::from_points(a, b).norm()
}

/// Point on line (AB) at `distance` pixels from `a`.
pub fn point_at_distance(a: PointF, b: PointF, distance: f32) -> PointF {
    let ab = Vector::from_points(a, b);
    a + ab * (distance / ab.norm())
}

pub fn middle_point(a: PointF, b: PointF) -> PointF {
    a + Vector::from_points(a, b) * 0.5
}

/// Oriented angle between vectors `ab` and `cd` in `[-π, π]`, positive counter-clockwise.
pub fn angle(a: PointF, b: PointF, c: PointF, d: PointF) -> f32 {
    let ab = Vector::from_points(a, b);
    let cd = Vector::from_points(c, d);
    let perp_dot = ab.x * cd.y - ab.y * cd.x;
    f64::from(perp_dot).atan2(f64::from(ab.dot(cd))) as f32
}

/// Oriented angle between vectors `ab` and `ac`.
pub fn angle_at(a: PointF, b: PointF, c: PointF) -> f32 {
    angle(a, b, a, c)
}

/// Rotates `b` around `a` by `radians`.
pub fn rotate(a: PointF, b: PointF, radians: f32) -> PointF {
    let v = Vector::from_points(a, b);
    let (sin, cos) = f64::from(radians).sin_cos();
    let (x, y) = (f64::from(v.x), f64::from(v.y));
    let dx = (x * cos - y * sin) as f32;
    let dy = (x * sin + y * cos) as f32;
    PointF::new(a.x + dx, a.y + dy)
}

/// `a * (1 - alpha) + b * alpha`.
pub fn mix(a: f32, b: f32, alpha: f32) -> f32 {
    a * (1.0 - alpha) + b * alpha
}

/// Component-wise [`mix`].
pub fn mix_points(a: PointF, b: PointF, alpha: f32) -> PointF {
    PointF::new(mix(a.x, b.x, alpha), mix(a.y, b.y, alpha))
}

const RADIANS_TO_DEGREES: f64 = 180.0 / std::f64::consts::PI;
const DEGREES_TO_RADIANS: f64 = std::f64::consts::PI / 180.0;

pub fn degrees(radians: f32) -> f32 {
    (f64::from(radians) * RADIANS_TO_DEGREES) as f32
}

pub fn degrees_f64(radians: f64) -> f64 {
    radians * RADIANS_TO_DEGREES
}

pub fn radians(degrees: f32) -> f32 {
    (f64::from(degrees) * DEGREES_TO_RADIANS) as f32
}

/// `MathHelper.Round`: banker's rounding to `decimal_places` digits; negative values round to
/// powers of ten.
pub fn round(value: f32, decimal_places: i32) -> f32 {
    if decimal_places >= 0 {
        let p = 10f64.powi(decimal_places);
        ((f64::from(value) * p).round_ties_even() / p) as f32
    } else {
        let magnitude = 10f64.powi(-decimal_places) as i32;
        let q = value / magnitude as f32;
        (f64::from(q).round_ties_even() * f64::from(magnitude)) as f32
    }
}
