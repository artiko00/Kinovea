//! Owner: P2. Painter ported from `DrawingCounter.Draw`. Only the beat counts are ported; the
//! cadence and period texts need the cadence formatting of the metadata and show no value for now.

use super::chrono::{drawing_opacity, paint_box, LOCKED_MASTER};
use super::PaintContext;
use crate::renderer::Renderer;
use crate::style::StyleData;
use kinovea_metadata::drawing::Counter;
use kinovea_metadata::measure_label::MeasureLabelType;

/// `strNoValue`: shown before the first beat and for the kinds not ported.
const NO_VALUE: &str = "*";

/// Port of `DrawingCounter.Draw`: the count in a box, with the name tag when shown; a locked
/// counter is dimmed to 0.75 and its name prefixed by a square.
pub fn paint_counter(d: &Counter, ctx: &PaintContext, r: &mut dyn Renderer) {
    if ctx.timestamp < d.visible {
        return;
    }
    let master = if d.locked { LOCKED_MASTER } else { 1.0 };
    let opacity = if ctx.timestamp > d.invisible {
        drawing_opacity(&d.header.fading, master, ctx.timestamp, ctx.default_fading)
    } else {
        f64::from(master)
    };
    if opacity <= 0.0 {
        return;
    }
    let text = beat_text(d, ctx.timestamp);
    let prefix = if d.locked { "■ " } else { "" };
    let label =
        (d.show_label && !d.header.name.is_empty()).then(|| format!("{prefix}{}", d.header.name));
    let style = StyleData::from_elements(&d.header.style);
    paint_box(r, ctx, &style, &text, d.position, opacity, label);
}

/// `GetBeatIndexInterval`: the index of the last beat at or before `timestamp`, `None` before the first.
fn beat_index(beats: &[i64], timestamp: i64) -> Option<usize> {
    beats
        .iter()
        .take_while(|&&b| timestamp >= b)
        .count()
        .checked_sub(1)
}

/// `BuildText` for the counts. Half and double cadence scale the count, as the original.
fn beat_text(d: &Counter, timestamp: i64) -> String {
    let Some(index) = beat_index(&d.beats, timestamp) else {
        return NO_VALUE.into();
    };
    let scale = |n: f32| {
        if d.half_cadence {
            n / 2.0
        } else if d.double_cadence {
            n * 2.0
        } else {
            n
        }
    };
    match d.measure_label_type {
        MeasureLabelType::Count => {
            let count = if d.zero_based { index } else { index + 1 };
            scale(count as f32).to_string()
        }
        MeasureLabelType::CountReverse => scale((d.beats.len() - 1 - index) as f32).to_string(),
        _ => NO_VALUE.into(),
    }
}
