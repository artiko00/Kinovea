//! Owner: P2. Painter ported from `DrawingCounter.Draw` and `BuildText`: the counts, cadences and
//! periods of the beats.

use super::chrono::{drawing_opacity, paint_box, LOCKED_MASTER};
use super::PaintContext;
use crate::renderer::Renderer;
use crate::style::StyleData;
use kinovea_core::units::CadenceUnit;
use kinovea_core::units_convert::convert_frequency;
use kinovea_core::units_precision::{format_f32_default, minimal_digits};
use kinovea_metadata::drawing::Counter;
use kinovea_metadata::measure_label::MeasureLabelType;
use kinovea_metadata::Metadata;

/// Unit of the cadence texts: the player preference, `Hertz` by default (PlayerPreferences.cs:333).
const CADENCE_UNIT: CadenceUnit = CadenceUnit::Hertz;

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
    let text = beat_text(d, ctx.timestamp, ctx.metadata);
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

/// `Metadata.GetCadence` (Metadata.cs:1549): `cycles` over the seconds of `timestamps`, in the
/// cadence unit with minimal digits, then the unit abbreviation.
pub(crate) fn cadence_text(cycles: f32, timestamps: i64, metadata: &Metadata) -> String {
    let seconds = timestamps as f64 / metadata.video.average_timestamps_per_second;
    let hertz = (f64::from(cycles) / seconds) as f32;
    let value = minimal_digits(convert_frequency(hertz, CADENCE_UNIT));
    format!(
        "{} {}",
        format_f32_default(value),
        CADENCE_UNIT.abbreviation()
    )
}

/// `BuildText` (DrawingCounter.cs:875-1000): the text of the box at `timestamp`. Counts scale with the
/// half and double cadence; cadences and periods come from the beats around the timestamp.
pub(crate) fn beat_text(d: &Counter, timestamp: i64, metadata: &Metadata) -> String {
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
        MeasureLabelType::CadenceInstant | MeasureLabelType::PeriodInstant => {
            instant_text(d, timestamp, index, metadata)
        }
        MeasureLabelType::CadenceAverage | MeasureLabelType::PeriodAverage => {
            average_text(d, metadata)
        }
        _ => NO_VALUE.into(),
    }
}

/// The instant cadence or period: the interval the timestamp is in, or the one before when it is
/// exactly on a beat (DrawingCounter.cs:960-985).
fn instant_text(d: &Counter, timestamp: i64, index: usize, metadata: &Metadata) -> String {
    let beats = &d.beats;
    let (start, end) = if timestamp == beats[index] {
        if index == 0 {
            return NO_VALUE.into();
        }
        (beats[index - 1], timestamp)
    } else {
        if index == beats.len() - 1 {
            return NO_VALUE.into();
        }
        (beats[index], beats[index + 1])
    };
    let duration = end - start;
    if d.measure_label_type == MeasureLabelType::CadenceInstant {
        let cycles = if d.half_cadence {
            0.5
        } else if d.double_cadence {
            2.0
        } else {
            1.0
        };
        return cadence_text(cycles, duration, metadata);
    }
    let duration = if d.half_cadence {
        duration * 2
    } else if d.double_cadence {
        duration / 2
    } else {
        duration
    };
    (metadata.timecode)(duration)
}

/// The average cadence or period over all the beats (DrawingCounter.cs:988-1012).
fn average_text(d: &Counter, metadata: &Metadata) -> String {
    if d.beats.len() == 1 {
        return NO_VALUE.into();
    }
    let duration = d.beats[d.beats.len() - 1] - d.beats[0];
    let mut cycles = (d.beats.len() - 1) as f32;
    if d.half_cadence {
        cycles /= 2.0;
    } else if d.double_cadence {
        cycles *= 2.0;
    }
    if d.measure_label_type == MeasureLabelType::CadenceAverage {
        return cadence_text(cycles, duration, metadata);
    }
    let average = (duration as f32 / cycles) as i64;
    (metadata.timecode)(average)
}
