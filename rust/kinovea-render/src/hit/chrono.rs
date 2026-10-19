//! `HitTest` of the text boxes of the chrono family (`DrawingChrono`, `DrawingChronoMulti`,
//! `DrawingCounter`): the main text box with its hidden resizer, and the name tag above it. The texts
//! and the box geometry are the painter's (`paint::chrono`, `paint::counter`); the main box is measured
//! by the renderer, so it differs from the GDI+ boxes of the original by the text metrics.

use super::context::{Hit, HitContext};
use super::label::{background_hit, hidden_radius, text_background};
use crate::paint::chrono::multi_text::build_text;
use crate::paint::chrono::{chrono_text, name_tag_rect};
use crate::paint::counter::beat_text;
use crate::paint::helpers::rounded_rect::{line_height, text_box_rect};
use crate::style::StyleData;
use kinovea_geometry::{PointF, RectangleF};
use kinovea_metadata::drawing::{Chrono, ChronoMulti, Counter};
use kinovea_metadata::Header;

/// `allowedFramesOver`: frames the box stays hittable after its hiding point (DrawingChrono.cs:143,
/// DrawingChronoMulti.cs:125, DrawingCounter.cs:125).
const ALLOWED_FRAMES_OVER: f64 = 12.0;

/// `DrawingChrono.HitTest` (DrawingChrono.cs:280-300).
pub fn hit_chrono(d: &Chrono, p: PointF, ctx: &HitContext) -> Hit {
    if !in_window(d.visible, d.invisible, ctx) {
        return Hit::Miss;
    }
    let text = chrono_text(d, ctx.timestamp, ctx.metadata.timecode);
    let name = (d.show_label && !d.header.name.is_empty()).then(|| d.header.name.clone());
    text_widget(d.position, &text, name.as_deref(), &d.header, p, ctx)
}

/// `DrawingChronoMulti.HitTest` (DrawingChronoMulti.cs:340-362): the table text, the name tag with
/// the locked square.
pub fn hit_chrono_multi(d: &ChronoMulti, p: PointF, ctx: &HitContext) -> Hit {
    if !in_window(d.visible, d.invisible, ctx) {
        return Hit::Miss;
    }
    let text = build_text(
        &d.sections,
        ctx.timestamp,
        &d.visible_columns,
        ctx.metadata.timecode,
    );
    let name = tag_text(d.show_label, d.locked, &d.header.name);
    text_widget(d.position, &text, name.as_deref(), &d.header, p, ctx)
}

/// `DrawingCounter.HitTest` (DrawingCounter.cs:323-348): the beat count, the name tag with the
/// locked square. Cadence and period counts are not ported by the painter yet (`*`).
pub fn hit_counter(d: &Counter, p: PointF, ctx: &HitContext) -> Hit {
    if !in_window(d.visible, d.invisible, ctx) {
        return Hit::Miss;
    }
    let text = beat_text(d, ctx.timestamp, ctx.metadata);
    let name = tag_text(d.show_label, d.locked, &d.header.name);
    text_widget(d.position, &text, name.as_deref(), &d.header, p, ctx)
}

/// `(locked ? "■ " : "") + name`, shown only when the label is on and the name is not empty.
fn tag_text(show_label: bool, locked: bool, name: &str) -> Option<String> {
    (show_label && !name.is_empty()).then(|| format!("{}{name}", if locked { "■ " } else { "" }))
}

/// `(time < visible) || (time > invisible + allowedFramesOver * avg)`: the Hit window of the family.
fn in_window(visible: i64, invisible: i64, ctx: &HitContext) -> bool {
    if ctx.timestamp < visible {
        return false;
    }
    if invisible == i64::MAX {
        return true;
    }
    let frames = ALLOWED_FRAMES_OVER * ctx.metadata.video.average_timestamps_per_frame;
    ctx.timestamp <= (invisible as f64 + frames).round_ties_even() as i64
}

/// The main box, then the name tag (body) when there is one. The main box answers first.
fn text_widget(
    position: PointF,
    text: &str,
    name: Option<&str>,
    header: &Header,
    p: PointF,
    ctx: &HitContext,
) -> Hit {
    let main = text_background(position, text, header, ctx);
    let result = background_hit(main, hidden_radius(header, ctx), p);
    if result != Hit::Miss {
        return result;
    }
    match name.map(|n| name_box(position, text, n, header, ctx)) {
        Some(tag) if background_hit(tag, 0, p) == Hit::Body => Hit::Body,
        _ => Hit::Miss,
    }
}

/// The name tag in image coordinates: placed with the painter's geometry in viewport pixels, then
/// untransformed (`lblBackground = transformer.Untransform(lblRect)`).
fn name_box(
    position: PointF,
    text: &str,
    name: &str,
    header: &Header,
    ctx: &HitContext,
) -> RectangleF {
    let scale = ctx.viewport.scale as f32;
    let style = StyleData::from_elements(&header.style);
    let font = style.font(scale);
    let label_font = style.font(scale * 0.5);
    let origin = ctx.viewport.transform_point_f(position);
    let boxed = text_box_rect(ctx.text, text, &font, origin);
    let tag = name_tag_rect(
        ctx.text,
        name,
        &label_font,
        boxed,
        line_height(ctx.text, &font) / 4,
    );
    ctx.viewport.untransform_rect(tag)
}
