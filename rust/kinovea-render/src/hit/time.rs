//! `HitTest` of the time drawings: the time segment (with its sliding time point) and the time
//! polyline.

use super::common::opacity;
use super::context::{Hit, HitContext};
use super::hit_point;
use super::label::hit_mini_label;
use super::strokes::{polyline_area, segment_area};
use crate::paint::helpers::frame::attach_label;
use crate::paint::time_segment::fraction_text;
use kinovea_geometry::{helpers::mix_points, PointF};
use kinovea_metadata::drawing::{TimePolyline, TimeSegment};

/// `DrawingTimeSegment.HitTest` (DrawingTimeSegment.cs:159-177): the mini label is handle 4 and wins
/// over everything, the time point is handle 3, the ends 1 and 2, then the segment as body.
pub fn hit_time_segment(d: &TimeSegment, p: PointF, ctx: &HitContext) -> Hit {
    if opacity(&d.header, ctx) <= 0.0 {
        return Hit::Miss;
    }
    let time_point = mix_points(d.a, d.b, d.fraction);
    let label = attach_label(&d.label, time_point);
    if hit_mini_label(&label, &d.header, &fraction_text(d, ctx.metadata), p, ctx) {
        return Hit::Handle(4);
    }
    if hit_point(p, time_point, ctx.viewport) {
        return Hit::Handle(3);
    }
    if hit_point(p, d.a, ctx.viewport) {
        return Hit::Handle(1);
    }
    if hit_point(p, d.b, ctx.viewport) {
        return Hit::Handle(2);
    }
    if segment_area(d.a, d.b, &d.header, p, ctx) {
        return Hit::Body;
    }
    Hit::Miss
}

/// `DrawingTimePolyline.HitTest` (DrawingTimePolyline.cs:175-193). It ignores the fading and tests
/// each box the other way round (`HitPoint(handle, probe)`), as the original does.
pub fn hit_time_polyline(d: &TimePolyline, p: PointF, ctx: &HitContext) -> Hit {
    let mut handle = None;
    for (key, point) in &d.points.0 {
        if hit_point(*point, p, ctx.viewport) {
            handle = key.parse::<u32>().ok().map(|k| k + 1);
        }
    }
    if let Some(n) = handle {
        return Hit::Handle(n);
    }
    if polyline_area(&d.points.0, &d.header, p, ctx) {
        return Hit::Body;
    }
    Hit::Miss
}
