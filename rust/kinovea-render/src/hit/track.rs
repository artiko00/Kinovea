//! `HitTest` of the tracked trajectory (`DrawingTrack`) in its interactive status, the only status a
//! headless caller sees. Measure labels (handles 2 and 3 and up) are not hit here yet.

use super::common::line_size;
use super::context::{Hit, HitContext};
use super::{hit_path, hit_point, GraphicsPath};
use kinovea_geometry::{PointF, RectangleF};
use kinovea_metadata::drawing::Track;

/// `allowedFramesOver`: frames after the end of the visibility window that still hit.
const ALLOWED_FRAMES_OVER: f64 = 12.0;

/// `DrawingTrack.HitTest` (DrawingTrack.cs:645-690) with `HitTestInteractive` (720-737): the
/// trajectory point at the probe time is handle 1, the curve through the visible points is body.
pub fn hit_track(d: &Track, p: PointF, ctx: &HitContext) -> Hit {
    if !d.is_visible || ctx.timestamp < d.visible || ctx.timestamp > last_hit_timestamp(d, ctx) {
        return Hit::Miss;
    }
    let index = d.closest_point(ctx.timestamp);
    if let Some(pos) = d.positions.get(index) {
        if hit_point(p, PointF::new(pos.x, pos.y), ctx.viewport) {
            return Hit::Handle(1);
        }
    }
    if d.positions.len() > 1 && trajectory_hit(d, index, p, ctx) {
        return Hit::Body;
    }
    Hit::Miss
}

/// `maxHitTimeStamp`: the invisible time plus the allowed frames, rounded (`Math.Round`).
fn last_hit_timestamp(d: &Track, ctx: &HitContext) -> i64 {
    if d.invisible == i64::MAX {
        return i64::MAX;
    }
    let frames = ALLOWED_FRAMES_OVER * ctx.metadata.video.average_timestamps_per_frame;
    (d.invisible as f64 + frames).round_ties_even() as i64
}

/// `HitTestTrajectory` (DrawingTrack.cs:742-770): the curve through the points from the first one
/// up to the current one (or the last one when seeing the future), end excluded, truncated to
/// whole pixels. Fewer than two points, or a curve with no extent, miss.
fn trajectory_hit(d: &Track, index: usize, p: PointF, ctx: &HitContext) -> bool {
    let end = if d.see_future {
        d.positions.len() - 1
    } else {
        index
    };
    let points: Vec<PointF> = d.positions[..end]
        .iter()
        .map(|t| PointF::new(t.x.trunc(), t.y.trunc()))
        .collect();
    if points.len() < 2 || has_no_extent(&points) {
        return false;
    }
    let mut path = GraphicsPath::new();
    path.add_curve(&points, 0.5);
    hit_path(p, &path, line_size(&d.header), false, ctx.viewport)
}

/// `RectangleF.IsEmpty` of the curve bounds: a zero width or a zero height.
fn has_no_extent(points: &[PointF]) -> bool {
    let xs = points.iter().map(|p| p.x);
    let ys = points.iter().map(|p| p.y);
    let (min_x, max_x) = (
        xs.clone().fold(f32::MAX, f32::min),
        xs.fold(f32::MIN, f32::max),
    );
    let (min_y, max_y) = (
        ys.clone().fold(f32::MAX, f32::min),
        ys.fold(f32::MIN, f32::max),
    );
    RectangleF::new(min_x, min_y, max_x - min_x, max_y - min_y).is_empty()
}
