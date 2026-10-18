//! Angular snapping and constructions relative to a pivot and a leg.

use super::{angle_at, distance, point_at_distance, radians, rotate};
use crate::point::PointF;
use crate::vector::Vector;
use std::f32::consts::PI;

/// Rotates `point` about `pivot` to the closest of `subdivisions` steps measured from `leg1`.
pub fn point_at_closest_rotation_step(
    pivot: PointF,
    leg1: PointF,
    point: PointF,
    subdivisions: i32,
) -> PointF {
    let mut angle = angle_at(pivot, leg1, point);
    if angle < 0.0 {
        angle += 2.0 * PI;
    }
    let step = (2.0 * std::f64::consts::PI / f64::from(subdivisions)) as f32;
    let mut section = (angle / step) as i32;
    if angle % step > step / 2.0 {
        section += 1;
    }
    let delta = (section as f32 * step) - angle;
    rotate(pivot, point, delta)
}

/// Same as [`point_at_closest_rotation_step`] with steps relative to the trigonometric circle.
pub fn point_at_closest_rotation_step_cardinal(
    pivot: PointF,
    point: PointF,
    subdivisions: i32,
) -> PointF {
    let zero = PointF::new(pivot.x + 100.0, pivot.y);
    point_at_closest_rotation_step(pivot, zero, point, subdivisions)
}

/// Point at `target_degrees` from the `[pivot, leg1]` segment, keeping the distance of `point`.
pub fn point_at_angle(pivot: PointF, leg1: PointF, point: PointF, target_degrees: f32) -> PointF {
    let d = distance(pivot, point);
    point_at_angle_and_distance(pivot, leg1, radians(target_degrees), d)
}

/// Point at `angle` radians from the `[origin, leg1]` segment and at `distance` from `origin`.
pub fn point_at_angle_and_distance(
    origin: PointF,
    leg1: PointF,
    angle: f32,
    distance: f32,
) -> PointF {
    let direction = rotate(origin, leg1, angle);
    point_at_distance(origin, direction, distance)
}

/// Point on the parallel to `[a, b]` passing through `c`, at the distance of `point` from `c`.
pub fn point_on_parallel(a: PointF, b: PointF, c: PointF, point: PointF) -> PointF {
    let ac = Vector::from_points(a, c);
    let d = b.translate(ac.x, ac.y);
    let angle = angle_at(c, a, d);
    let dist = distance(c, point);
    point_at_angle_and_distance(c, a, angle, dist)
}
