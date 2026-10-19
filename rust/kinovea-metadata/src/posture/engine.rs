//! Constraint engine (`GenericPostureConstraintEngine`): moves a handle to a target point,
//! honouring its constraint and then applying its impacts on the other points.

use super::model::*;
use kinovea_geometry::helpers::{closest_point, distance, point_at_angle, point_at_closest_rotation_step, point_at_closest_rotation_step_cardinal, point_at_distance, point_on_parallel};
use kinovea_geometry::PointF;
use kinovea_measurement::CalibrationHelper;

mod impacts;
#[cfg(test)]
mod impacts_test;
mod segments;
#[cfg(test)]
mod segments_test;
use impacts::{prepare_impacts, process_point_impacts};
pub use segments::grab_segment_middle;
use segments::{move_circle_handle, move_segment_handle};

/// Image ↔ world transforms used by the perpendicular, parallel and centre rules; identity
/// when no calibration is given.
#[derive(Clone, Copy)]
pub struct Space<'a>(pub Option<&'a CalibrationHelper>);

impl Space<'_> {
    pub fn world(&self, p: PointF) -> PointF {
        self.0.map_or(p, |c| c.point(p))
    }

    pub fn image(&self, p: PointF) -> PointF {
        self.0.map_or(p, |c| c.image_point(p))
    }
}

/// Moves handle `handle` towards `point`; `shift` snaps distance-constrained points to 45°.
pub fn move_handle(p: &mut GenericPosture, space: Space, handle: usize, point: PointF, shift: bool) {
    let Some(h) = p.handles.get(handle) else {
        return;
    };
    match h.kind {
        HandleType::Point => move_point_handle(p, space, handle, point, shift),
        HandleType::Segment => move_segment_handle(p, space, handle, point),
        HandleType::Ellipse | HandleType::Circle => move_circle_handle(p, handle, point),
    }
}

pub(super) fn is_active(constraint: Option<&Constraint>, p: &GenericPosture) -> bool {
    let Some(c) = constraint else {
        return false;
    };
    if c.option_group.is_empty() {
        return true;
    }
    c.option_group.split('|').all(|key| p.options.iter().find(|o| o.key == key).is_none_or(|o| o.value))
}

pub(super) fn set(p: &mut GenericPosture, index: i32, value: PointF) {
    if let Some(slot) = p.point_list.get_mut(index as usize) {
        *slot = value;
    }
}

pub(super) fn get(p: &GenericPosture, index: i32) -> PointF {
    p.point_list.get(index as usize).copied().unwrap_or(PointF::EMPTY)
}

fn move_point_handle(p: &mut GenericPosture, space: Space, handle: usize, point: PointF, shift: bool) {
    let reference = p.handles[handle].reference;
    let old = get(p, reference);
    prepare_impacts(p, handle);
    let constraint = p.handles[handle].constraint.clone();
    let target = match constraint.as_ref().filter(|_| is_active(constraint.as_ref(), p)).map(|c| &c.kind) {
        None | Some(ConstraintKind::LineSlide { .. }) if constraint.is_none() => Some(point),
        None => Some(point),
        Some(ConstraintKind::LineSlide { start, end, allowed, margin }) => Some(closest_point(get(p, *start), get(p, *end), point, *allowed, *margin)),
        Some(ConstraintKind::VerticalSlide) => Some(PointF::new(old.x, point.y)),
        Some(ConstraintKind::HorizontalSlide) => Some(PointF::new(point.x, old.y)),
        Some(ConstraintKind::DistanceToPoint { point: parent }) => {
            let parent = get(p, *parent);
            let temp = point_at_distance(parent, point, distance(parent, old));
            Some(if shift { point_at_closest_rotation_step_cardinal(parent, temp, 8) } else { temp })
        }
        Some(ConstraintKind::RotationSteps { origin, leg1, step, keep_distance }) => {
            let (parent, leg1) = (get(p, *origin), get(p, *leg1));
            if parent == leg1 || *step == 0 {
                None
            } else {
                let candidate = if *keep_distance { point_at_distance(parent, point, distance(parent, old)) } else { point };
                Some(point_at_closest_rotation_step(parent, leg1, candidate, 360 / *step))
            }
        }
        Some(ConstraintKind::PerpendicularSlide { origin, leg1 }) => {
            let (pivot, leg1) = (get(p, *origin), get(p, *leg1));
            (pivot != leg1).then(|| space.image(point_at_angle(space.world(pivot), space.world(leg1), space.world(point), 90.0)))
        }
        Some(ConstraintKind::ParallelSlide { a, b, c }) => {
            let (a, b, c) = (space.world(get(p, *a)), space.world(get(p, *b)), space.world(get(p, *c)));
            Some(space.image(point_on_parallel(a, b, c, space.world(point))))
        }
        Some(ConstraintKind::LockedInPlace) => None,
    };
    if let Some(target) = target {
        set(p, reference, target);
    }
    process_point_impacts(p, space, handle, old);
}
