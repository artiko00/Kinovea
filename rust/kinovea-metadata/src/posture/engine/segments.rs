//! Segment and circle handles: whole-segment translations with symmetry and the impacts of
//! both end points, and the circle radius handle.

use super::super::model::*;
use super::impacts::{prepare_impacts, process_point_impacts, segment_symmetry};
use super::{get, is_active, set, Space};
use kinovea_geometry::helpers::middle_point;
use kinovea_geometry::{PointF, Vector};

pub(super) fn move_segment_handle(p: &mut GenericPosture, space: Space, handle: usize, point: PointF) {
    let segment = &p.segments[p.handles[handle].reference as usize];
    let (start, end) = (segment.start, segment.end);
    let (old_start, old_end) = (get(p, start), get(p, end));
    let constraint = p.handles[handle].constraint.clone();
    let moved = Vector::from_points(p.handles[handle].grab_point, point);
    let translation = match constraint.as_ref().filter(|_| is_active(constraint.as_ref(), p)).map(|c| &c.kind) {
        None => Some(moved),
        Some(ConstraintKind::HorizontalSlide) => Some(Vector::new(moved.x, 0.0)),
        Some(ConstraintKind::VerticalSlide) => Some(Vector::new(0.0, moved.y)),
        Some(_) => None,
    };
    if let Some(v) = translation {
        set(p, start, get(p, start) + v);
        set(p, end, get(p, end) + v);
        p.handles[handle].grab_point = p.handles[handle].grab_point + v;
    }
    let impacts = p.handles[handle].impacts.clone();
    for impact in &impacts {
        if let Impact::HorizontalSymmetry { impacting, impacted, axis } = impact {
            segment_symmetry(p, *impacting, *impacted, *axis);
        }
    }
    for i in 0..p.handles.len() {
        if p.handles[i].kind != HandleType::Point {
            continue;
        }
        let reference = p.handles[i].reference;
        let old = if reference == start {
            old_start
        } else if reference == end {
            old_end
        } else {
            continue;
        };
        // The original passes the point index where a handle index is expected. An index past
        // the handles throws there, which aborts the move without undoing what was already done.
        let Some(handle) = usize::try_from(reference).ok().filter(|&h| h < p.handles.len()) else {
            return;
        };
        prepare_impacts(p, handle);
        process_point_impacts(p, space, handle, old);
    }
}

pub(super) fn move_circle_handle(p: &mut GenericPosture, handle: usize, point: PointF) {
    let constraint = p.handles[handle].constraint.clone();
    if is_active(constraint.as_ref(), p) {
        return;
    }
    let index = p.handles[handle].reference as usize;
    let center = get(p, p.circles[index].center);
    p.circles[index].radius = Vector::from_points(center, point).norm() as i32;
}

/// Grab point of a segment handle when picked at its middle (what the oracle does).
pub fn grab_segment_middle(p: &mut GenericPosture, handle: usize) {
    let segment = &p.segments[p.handles[handle].reference as usize];
    let middle = middle_point(get(p, segment.start), get(p, segment.end));
    p.handles[handle].grab_point = middle;
}
