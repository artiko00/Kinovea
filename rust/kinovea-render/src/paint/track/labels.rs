//! Owner: P2. Mini labels of a track: the main label on the current point (`DrawMainLabel`) and one
//! label per keyframe (`DrawKeyframesLabels`), attached to the trajectory point at their time.

use super::super::helpers::mini_label::{anchor_to, draw_mini_label};
use super::super::PaintContext;
use crate::renderer::Renderer;
use crate::style::StyleData;
use kinovea_geometry::PointF;
use kinovea_metadata::drawing::Track;
use kinovea_metadata::measure_label::MeasureLabelType;

/// `opacityNormal` and `opacityFuture` of the track, as the labels use them.
const PAST: f64 = 0.9;
const FUTURE: f64 = 0.25;

/// Port of `DrawKeyframesLabels`: a label past the current point at the past opacity, one after it
/// at the future opacity, and none after it when the future is hidden. The text is the keyframe
/// name when labelling by name, the measure otherwise; the colour is the keyframe's when
/// `use_keyframe_colors` is set.
pub fn paint_keyframe_labels(
    d: &Track,
    ctx: &PaintContext,
    style: &StyleData,
    points: &[PointF],
    base: f64,
    r: &mut dyn Renderer,
) {
    let current = d.positions[d.closest_point(ctx.timestamp)].t;
    for kfl in &d.keyframe_labels {
        let future = kfl.label.timestamp > current;
        if future && !d.see_future {
            continue;
        }
        let opacity = base * if future { FUTURE } else { PAST };
        let attach = point_at_time(d, points, kfl.label.timestamp).unwrap_or(kfl.label.attach);
        let label = anchor_to(&kfl.label, attach);
        let keyframe = ctx
            .metadata
            .keyframes
            .iter()
            .find(|k| k.timestamp == kfl.label.timestamp);
        let text = match (d.measure_label_type, keyframe) {
            (MeasureLabelType::Name, Some(k)) => k.name.clone(),
            _ => measure_text(d, kfl.attach_index, ctx),
        };
        let colour = match keyframe {
            Some(k) if d.use_keyframe_colors => k.color,
            _ => style.color,
        };
        draw_mini_label(r, &label, &text, colour, style.font_size, opacity, ctx);
    }
}

/// Port of `DrawMainLabel` for the interactive view: the label attached to the current point.
pub fn paint_main_label(
    d: &Track,
    ctx: &PaintContext,
    style: &StyleData,
    points: &[PointF],
    r: &mut dyn Renderer,
) {
    let text = measure_text(d, d.closest_point(ctx.timestamp), ctx);
    let label = anchor_to(&d.main_label, main_attach(d, points, ctx.timestamp));
    draw_mini_label(r, &label, &text, style.color, style.font_size, 1.0, ctx);
}

/// Attach point of the main label (`GetPoints`): the first or last point outside the track, the
/// point at the current time, or the stored attach point between two positions.
fn main_attach(d: &Track, points: &[PointF], timestamp: i64) -> PointF {
    let first = d.positions[0].t;
    let last = d.positions[d.positions.len() - 1].t;
    if timestamp < first {
        points[0]
    } else if timestamp > last {
        points[points.len() - 1]
    } else {
        point_at_time(d, points, timestamp).unwrap_or(d.main_label.attach)
    }
}

fn point_at_time(d: &Track, points: &[PointF], timestamp: i64) -> Option<PointF> {
    d.positions
        .iter()
        .position(|p| p.t == timestamp)
        .map(|i| points[i])
}

/// `GetMeasureLabelText` for the name and time kinds. Distances and speeds need the kinematics,
/// which are not ported yet, so they show no text.
fn measure_text(d: &Track, index: usize, ctx: &PaintContext) -> String {
    let Some(t) = d.positions.get(index).map(|p| p.t) else {
        return String::new();
    };
    match d.measure_label_type {
        MeasureLabelType::Name => d.header.name.clone(),
        MeasureLabelType::Clock => (ctx.metadata.timecode)(t),
        MeasureLabelType::RelativeTime => (ctx.metadata.timecode)(t - d.positions[0].t),
        _ => String::new(),
    }
}
