//! Owner: P2. Painter ported from `DrawingTimeSegment.Draw`.

use super::helpers::mini_label::{anchor_to, draw_mini_label};
use super::PaintContext;
use crate::renderer::{Pen, Renderer};
use crate::style::StyleData;
use kinovea_geometry::helpers::mix_points;
use kinovea_geometry::PointF;
use kinovea_metadata::drawing::TimeSegment;

/// Port of `DrawingTimeSegment.Draw`: the segment and a tick at A, both 1 px wide at any zoom, then
/// the mini label anchored at the fraction of the segment with the time at that fraction.
pub fn paint_time_segment(d: &TimeSegment, ctx: &PaintContext, r: &mut dyn Renderer) {
    let opacity = d
        .header
        .fading
        .opacity_factor(ctx.timestamp, ctx.default_fading);
    if opacity <= 0.0 {
        return;
    }
    let style = StyleData::from_elements(&d.header.style);
    let pen = Pen {
        width: 1.0,
        ..style.pen((opacity * 255.0) as i32, ctx.viewport.scale as f32)
    };

    // `ab` rotated by a quarter turn and shortened to 0.2 of its length, both ways from A.
    let (dx, dy) = ((d.b.x - d.a.x) * 0.2, (d.b.y - d.a.y) * 0.2);
    let tick_a = PointF::new(d.a.x + dy, d.a.y - dx);
    let tick_b = PointF::new(d.a.x - dy, d.a.y + dx);
    let start = ctx.viewport.transform_point_f(d.a).to_point_f();
    let end = ctx.viewport.transform_point_f(d.b).to_point_f();
    r.stroke_line(start, end, &pen);
    r.stroke_line(
        ctx.viewport.transform_point_f(tick_a).to_point_f(),
        ctx.viewport.transform_point_f(tick_b).to_point_f(),
        &pen,
    );

    let label = anchor_to(&d.label, mix_points(d.a, d.b, d.fraction));
    let text = fraction_text(d, ctx);
    draw_mini_label(r, &label, &text, style.color, style.font_size, opacity, ctx);
}

/// `GetFractionTime`: the time at `fraction` of the frame holding the reference timestamp, measured
/// from the time origin, through the metadata timecode. The frame duration in timestamps stands in
/// for the baseline interval, so the text is not the original's formatting for every preference.
fn fraction_text(d: &TimeSegment, ctx: &PaintContext) -> String {
    let frame = ctx.metadata.video.average_timestamps_per_frame;
    let origin = ctx.metadata.video.time_origin;
    let reference = d.header.fading.reference_timestamp;
    let frames = if frame != 0.0 {
        ((reference - origin) as f64 / frame).round_ties_even()
    } else {
        0.0
    };
    (ctx.metadata.timecode)(((frames + f64::from(d.fraction)) * frame).round() as i64)
}
