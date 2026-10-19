//! `HitTest` of the generic posture (`DrawingGenericPosture`): handles first, then the object.
//! Points with a negative index are computed points, located by their weighted children.

use super::common::{box_around, trackable_opacity};
use super::context::{Hit, HitContext};
use super::sweep::{sweep_contains, SweepFlags};
use super::{hit_path, hit_point, GraphicsPath};
use kinovea_geometry::PointF;
use kinovea_metadata::drawing::GenericPostureDrawing;
use kinovea_metadata::posture::model::{
    GenericPosture, Handle, HandleType, PostureCircle, Segment,
};

/// `DrawingGenericPosture.HitTest` (DrawingGenericPosture.cs:230-268). Handles are numbered from 1 in
/// order; the first one hit wins. The body is `IsPointInObject` (angles, hit zones, discs, segments,
/// polylines). A drawing without posture (tool not shipped) never hits.
pub fn hit_posture(d: &GenericPostureDrawing, p: PointF, ctx: &HitContext) -> Hit {
    let Some(posture) = &d.posture else {
        return Hit::Miss;
    };
    if trackable_opacity(&d.header, ctx) <= 0.0 {
        return Hit::Miss;
    }
    let handle = posture
        .handles
        .iter()
        .enumerate()
        .find_map(|(i, h)| handle_hit(posture, h, p, ctx).then_some(i as u32 + 1));
    match handle {
        Some(n) => Hit::Handle(n),
        None if in_object(posture, p, ctx) => Hit::Body,
        None => Hit::Miss,
    }
}

/// One handle of `HitTest`: skipped when its option group is off or its reference is negative.
fn handle_hit(posture: &GenericPosture, h: &Handle, p: PointF, ctx: &HitContext) -> bool {
    if !option_active(posture, &h.option_group) || h.reference < 0 {
        return false;
    }
    let r = h.reference as usize;
    match h.kind {
        HandleType::Point => posture
            .point_list
            .get(r)
            .is_some_and(|c| hit_point(p, *c, ctx.viewport)),
        HandleType::Segment => posture
            .segments
            .get(r)
            .is_some_and(|s| hit_segment(posture, s, p, ctx)),
        HandleType::Ellipse | HandleType::Circle => posture
            .circles
            .get(r)
            .is_some_and(|c| hit_arc(posture, c, p, ctx)),
    }
}

/// `IsActive` (DrawingGenericPosture.cs:1047-1067): every `|`-separated option that exists is on.
fn option_active(posture: &GenericPosture, group: &str) -> bool {
    group.is_empty()
        || group.split('|').all(|key| {
            posture
                .options
                .iter()
                .find(|o| o.key == key)
                .is_none_or(|o| o.value)
        })
}

/// `GetUntransformedComputedPoint` for negative indices (`-index - 1` into the computed points), and
/// the point list for the others. Out-of-range references give the empty point.
fn point_at(posture: &GenericPosture, index: i32) -> PointF {
    if index >= 0 {
        return posture
            .point_list
            .get(index as usize)
            .copied()
            .unwrap_or(PointF::EMPTY);
    }
    let k = (-index - 1) as usize;
    posture
        .computed_points
        .get(k)
        .map_or(PointF::EMPTY, |c| c.location(&posture.point_list))
}

/// `HitSegment` (DrawingGenericPosture.cs:1172-1185): a stroke of the segment's own width.
fn hit_segment(posture: &GenericPosture, s: &Segment, p: PointF, ctx: &HitContext) -> bool {
    let (start, end) = (point_at(posture, s.start), point_at(posture, s.end));
    if start == end {
        return false;
    }
    let mut path = GraphicsPath::new();
    path.add_line(start, end);
    hit_path(p, &path, s.stroke.width, false, ctx.viewport)
}

/// `HitArc` (DrawingGenericPosture.cs:1204-1212): the outline of the full circle.
fn hit_arc(posture: &GenericPosture, c: &PostureCircle, p: PointF, ctx: &HitContext) -> bool {
    let mut path = GraphicsPath::new();
    path.add_arc(
        box_around(point_at(posture, c.center), c.radius),
        0.0,
        360.0,
    );
    hit_path(p, &path, c.stroke.width, false, ctx.viewport)
}

/// `IsPointInObject` (DrawingGenericPosture.cs:1091-1131): the first angle, zone, disc, segment or
/// polyline that holds the point.
fn in_object(posture: &GenericPosture, p: PointF, ctx: &HitContext) -> bool {
    let angles = posture.angles.iter().any(|a| {
        let flags = SweepFlags {
            signed: a.signed,
            counter_clockwise: a.ccw,
            supplementary: a.supplementary,
        };
        let (o, l1, l2) = (
            point_at(posture, a.origin),
            point_at(posture, a.leg1),
            point_at(posture, a.leg2),
        );
        sweep_contains(o, l1, l2, a.radius, flags, p, ctx)
    });
    angles
        || posture.hit_zones.iter().any(|z| {
            let pts: Vec<PointF> = z.points.iter().map(|i| point_at(posture, *i)).collect();
            let mut path = GraphicsPath::new();
            path.add_polygon(&pts);
            hit_path(p, &path, 0, true, ctx.viewport)
        })
        || posture.circles.iter().any(|c| {
            let mut path = GraphicsPath::new();
            path.add_ellipse(box_around(point_at(posture, c.center), c.radius));
            hit_path(p, &path, 0, true, ctx.viewport)
        })
        || posture
            .segments
            .iter()
            .any(|s| hit_segment(posture, s, p, ctx))
        || posture.polylines.iter().any(|l| {
            let pts: Vec<PointF> = l.points.iter().map(|i| point_at(posture, *i)).collect();
            let mut path = GraphicsPath::new();
            path.add_curve(&pts, 0.5);
            hit_path(p, &path, l.stroke.width, false, ctx.viewport)
        })
}
