//! Owner: P2. Painters ported from `DrawingChrono.Draw`, `DrawingChronoMulti.Draw` and the name tag
//! they share with `DrawingCounter.Draw`.

#[cfg(test)]
mod multi_box_test;
pub mod multi_text;
#[cfg(test)]
mod multi_text_test;

use super::helpers::rounded_rect::{draw_rounded_rectangle, draw_text_box, line_height, to_rect_f};
use super::PaintContext;
use crate::renderer::{Brush, Font, Renderer};
use crate::style::StyleData;
use crate::viewport::Rectangle;
use kinovea_geometry::PointF;
use kinovea_metadata::drawing::{Chrono, ChronoMulti};
use kinovea_metadata::style::StyleValue;
use kinovea_metadata::Fading;

/// `backgroundOpacity` of the stopwatch boxes (the text is at full opacity).
pub(super) const BOX_BACKGROUND_OPACITY: f64 = 225.0;
/// `MasterFactor` of a locked drawing.
pub(super) const LOCKED_MASTER: f32 = 0.75;

/// Port of `DrawingChrono.Draw`.
pub fn paint_chrono(d: &Chrono, ctx: &PaintContext, r: &mut dyn Renderer) {
    if ctx.timestamp < d.visible {
        return;
    }
    let opacity = if ctx.timestamp > d.invisible {
        drawing_opacity(&d.header.fading, 1.0, ctx.timestamp, ctx.default_fading)
    } else {
        1.0
    };
    if opacity <= 0.0 {
        return;
    }
    let text = chrono_text(d, ctx.timestamp, ctx.metadata.timecode);
    let label = (d.show_label && !d.header.name.is_empty()).then(|| d.header.name.clone());
    let style = StyleData::from_elements(&d.header.style);
    paint_box(r, ctx, &style, &text, d.position, opacity, label);
}

/// Port of `DrawingChronoMulti.Draw`: the table of the sections, with a locked drawing dimmed to
/// 0.75 and its name prefixed by a square, as the original.
pub fn paint_chrono_multi(d: &ChronoMulti, ctx: &PaintContext, r: &mut dyn Renderer) {
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
    let text = multi_text::build_text(
        &d.sections,
        ctx.timestamp,
        &d.visible_columns,
        ctx.metadata.timecode,
    );
    let prefix = if d.locked { "■ " } else { "" };
    let label =
        (d.show_label && !d.header.name.is_empty()).then(|| format!("{prefix}{}", d.header.name));
    let style = StyleData::from_elements(&d.header.style);
    paint_box(r, ctx, &style, &text, d.position, opacity, label);
}

/// Background box with `text` at `position`, then the name tag above it when there is a label.
pub(super) fn paint_box(
    r: &mut dyn Renderer,
    ctx: &PaintContext,
    style: &StyleData,
    text: &str,
    position: PointF,
    opacity: f64,
    label: Option<String>,
) {
    let back = style.background_brush((opacity * BOX_BACKGROUND_OPACITY) as i32);
    let fore = style.foreground_brush((opacity * 255.0) as i32);
    let font = style.font(ctx.viewport.scale as f32);
    let origin = ctx.viewport.transform_point_f(position);
    let boxed = draw_text_box(r, text, &font, &back, &fore, origin, None);
    if let Some(name) = label {
        let label_font = style.font(ctx.viewport.scale as f32 * 0.5);
        let radius = line_height(r, &font) / 4;
        draw_name_label(r, &name, &label_font, &back, &fore, boxed, radius);
    }
}

/// Name tag above a box: a drop shape of radius `height / 3`, its bottom `box_radius` and its own
/// height above the box (the alignment assumes the fixed margins of `RoundedRectangle`).
pub(super) fn draw_name_label(
    r: &mut dyn Renderer,
    name: &str,
    font: &Font,
    back: &Brush,
    fore: &Brush,
    boxed: Rectangle,
    box_radius: i32,
) {
    let radius = line_height(r, font) / 3;
    let label = name_tag_rect(r, name, font, boxed, box_radius);
    draw_rounded_rectangle(r, to_rect_f(label), back, radius, true, false, None);
    r.draw_text(
        name,
        font,
        fore,
        PointF::new(label.x as f32, label.y as f32),
    );
}

/// Box of the name tag above `boxed` (`DrawingChrono.Draw` lines 262-276, shared with the counter
/// and the multi chrono). Also the hit box of the tag, in viewport pixels.
pub(crate) fn name_tag_rect(
    r: &dyn Renderer,
    name: &str,
    font: &Font,
    boxed: Rectangle,
    box_radius: i32,
) -> Rectangle {
    let size = r.measure_text(name, font);
    let radius = line_height(r, font) / 3;
    let top = boxed.y - size.height as i32 - box_radius - radius;
    Rectangle::new(boxed.x, top, size.width as i32, size.height as i32)
}

/// `InfosFading.GetOpacityFactor` with `MasterFactor` set to `master`. The drawing keeps its own
/// reference and frame duration; the fading frames come from the player when `use_default` is set.
pub(super) fn drawing_opacity(
    fading: &Fading,
    master: f32,
    timestamp: i64,
    default: &Fading,
) -> f64 {
    let src = if fading.use_default { default } else { fading };
    Fading {
        use_default: false,
        always_visible: src.always_visible,
        opaque_frames: src.opaque_frames,
        fading_frames: src.fading_frames,
        master_factor: master,
        ..fading.clone()
    }
    .opacity_factor(timestamp, default)
}

/// `DrawingChrono.GetTimecode`: the time since the clock origin in clock mode, otherwise the
/// stopwatch value, clamped to the start and stop of the counting.
pub(crate) fn chrono_text(d: &Chrono, timestamp: i64, timecode: fn(i64) -> String) -> String {
    let clock = matches!(
        d.header.style.get("clock"),
        Some(StyleValue::Toggle { value: true, .. })
    );
    if clock {
        if d.clock_origin == i64::MAX {
            return timecode(timestamp);
        }
        return timecode(timestamp - d.clock_origin);
    }
    let duration = if timestamp <= d.start_counting {
        0
    } else if timestamp > d.stop_counting {
        d.stop_counting - d.start_counting
    } else {
        timestamp - d.start_counting
    };
    timecode(duration)
}
