//! Impacts: what moving one point does to the others.

use super::super::model::*;
use super::{get, set, Space};
use kinovea_geometry::helpers::{angle_at, closest_point, distance, middle_point, point_at_angle, point_at_angle_and_distance, point_on_parallel, rotate};
use kinovea_geometry::{PointF, PointLinePosition, Vector};

/// Captures the angles and distances that `KeepAngle` impacts must preserve.
pub(super) fn prepare_impacts(p: &mut GenericPosture, handle: usize) {
    let points = p.point_list.clone();
    let at = |i: i32| points.get(i as usize).copied().unwrap_or(PointF::EMPTY);
    for impact in &mut p.handles[handle].impacts {
        if let Impact::KeepAngle { origin, leg1, leg2, old_angle, old_distance } = impact {
            let (o, a, b) = (at(*origin), at(*leg1), at(*leg2));
            if o == a || o == b {
                continue;
            }
            *old_angle = angle_at(o, a, b);
            *old_distance = distance(o, b);
        }
    }
}

pub(super) fn process_point_impacts(p: &mut GenericPosture, space: Space, handle: usize, old: PointF) {
    let impacts = p.handles[handle].impacts.clone();
    let reference = p.handles[handle].reference;
    for impact in &impacts {
        match impact {
            Impact::LineAlign { start, end, point_to_align } => {
                let (a, b) = (get(p, *start), get(p, *end));
                let target = if a == b { a } else { closest_point(a, b, get(p, *point_to_align), PointLinePosition::Anywhere, 10) };
                set(p, *point_to_align, target);
            }
            Impact::VerticalAlign { point } => {
                let (impacted, impacting) = (get(p, *point), get(p, reference));
                set(p, *point, PointF::new(impacted.x, impacting.y));
            }
            Impact::HorizontalAlign { point } => {
                let (impacted, impacting) = (get(p, *point), get(p, reference));
                set(p, *point, PointF::new(impacting.x, impacted.y));
            }
            Impact::Pivot { pivot, impacted } => {
                let a = get(p, *pivot);
                let radians = angle_at(a, old, get(p, reference));
                for i in impacted {
                    let rotated = rotate(a, get(p, *i), radians);
                    set(p, *i, rotated);
                }
            }
            Impact::KeepAngle { origin, leg1, leg2, old_angle, old_distance } => {
                let (o, a) = (get(p, *origin), get(p, *leg1));
                if o != a {
                    set(p, *leg2, point_at_angle_and_distance(o, a, *old_angle, *old_distance));
                }
            }
            Impact::SegmentCenter { point_to_move, point1, point2 } => {
                let middle = middle_point(space.world(get(p, *point1)), space.world(get(p, *point2)));
                set(p, *point_to_move, space.image(middle));
            }
            Impact::PerpendicularAlign { point_to_move, origin, leg1 } => {
                let (pivot, a) = (get(p, *origin), get(p, *leg1));
                if pivot != a {
                    let result = point_at_angle(space.world(pivot), space.world(a), space.world(get(p, *point_to_move)), 90.0);
                    set(p, *point_to_move, space.image(result));
                }
            }
            Impact::ParallelAlign { point_to_move, a, b, c } => {
                let result = point_on_parallel(space.world(get(p, *a)), space.world(get(p, *b)), space.world(get(p, *c)), space.world(get(p, *point_to_move)));
                set(p, *point_to_move, space.image(result));
            }
            Impact::HorizontalSymmetry { .. } => {}
        }
    }
}

/// Mirrors the `impacting` segment onto `impacted` across the vertical `axis` segment.
pub(super) fn segment_symmetry(p: &mut GenericPosture, impacting: i32, impacted: i32, axis: i32) {
    let (s_in, s_out, s_axis) = (p.segments[impacting as usize].clone(), p.segments[impacted as usize].clone(), p.segments[axis as usize].clone());
    let v_start = Vector::from_points(get(p, s_in.start), get(p, s_axis.start));
    set(p, s_out.start, PointF::new(get(p, s_axis.start).x + v_start.x, get(p, s_in.start).y));
    let v_end = Vector::from_points(get(p, s_in.end), get(p, s_axis.end));
    set(p, s_out.end, PointF::new(get(p, s_axis.end).x + v_end.x, get(p, s_in.end).y));
}
