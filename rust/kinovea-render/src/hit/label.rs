//! `HitTest` of the text label (`DrawingText`) and the rounded text background shared with the chrono
//! and counter boxes. Background sizes are the text measured by the renderer, so they differ from the
//! GDI+ boxes of the original by the renderer's text metrics.

use super::common::{box_around, line_size, opacity};
use super::context::{Hit, HitContext};
use super::{hit_path, GraphicsPath};
use crate::paint::helpers::mini_label::label_box;
use crate::style::StyleData;
use kinovea_geometry::{PointF, RectangleF, SizeF};
use kinovea_metadata::drawing::Label;
use kinovea_metadata::measure_label::MeasureLabel;

/// `DrawingText.HitTest` (DrawingText.cs:290-317): the arrow end is handle 2 (a disc of three line
/// sizes), the arrow is body, then the background with its hidden bottom-right resizer as handle 1.
pub fn hit_label(d: &Label, p: PointF, ctx: &HitContext) -> Hit {
    if opacity(&d.header, ctx) <= 0.0 {
        return Hit::Miss;
    }
    if d.arrow_visible || d.circle_visible {
        let mut disc = GraphicsPath::new();
        disc.add_ellipse(box_around(d.arrow_end, line_size(&d.header) * 3));
        if hit_path(p, &disc, 0, true, ctx.viewport) {
            return Hit::Handle(2);
        }
        let rect = text_background(d.position, &d.text, &d.header, ctx);
        if arrow_segment_hit(rect.center(), d.arrow_end, &d.header, p, ctx) {
            return Hit::Body;
        }
    }
    let rect = text_background(d.position, &d.text, &d.header, ctx);
    background_hit(rect, hidden_radius(&d.header, ctx), p)
}

/// `MiniLabel.HitTest`: the label box, as `Draw` left it (the text of `text` at the font size of
/// `header`, untransformed), contains `p`. The label is the one the owner anchored for drawing.
pub fn hit_mini_label(
    label: &MeasureLabel,
    header: &kinovea_metadata::Header,
    text: &str,
    p: PointF,
    ctx: &HitContext,
) -> bool {
    let font_size = StyleData::from_elements(&header.style).font_size;
    label_box(ctx.text, label, text, font_size, ctx.viewport).contains(p)
}

/// `lblBackground` / `background.Rectangle` after `Draw`: the text measured at the viewport stretch,
/// untransformed, placed at `location`.
pub fn text_background(
    location: PointF,
    text: &str,
    header: &kinovea_metadata::Header,
    ctx: &HitContext,
) -> RectangleF {
    let style = StyleData::from_elements(&header.style);
    let font = style.font(ctx.viewport.scale as f32);
    let size: SizeF = ctx
        .viewport
        .untransform_size_f(ctx.text.measure_text(text, &font));
    RectangleF::from_location_size(location, size)
}

/// `roundingRadius = fontText.Height / 4` then `(int)(roundingRadius * 1.8f)`, with the height at 1.0.
pub fn hidden_radius(header: &kinovea_metadata::Header, ctx: &HitContext) -> i32 {
    let style = StyleData::from_elements(&header.style);
    let height = ctx.text.measure_text("", &style.font(1.0)).height as i32;
    ((height / 4) as f32 * 1.8) as i32
}

/// `RoundedRectangle.HitTest` (Helpers/RoundedRectangle.cs:108-124): the hidden handle at the bottom
/// right corner (when `radius > 0`) is 1, the rectangle is 0.
pub fn background_hit(rect: RectangleF, radius: i32, p: PointF) -> Hit {
    if radius > 0 && box_around(PointF::new(rect.right(), rect.bottom()), radius).contains(p) {
        return Hit::Handle(1);
    }
    if rect.contains(p) {
        return Hit::Body;
    }
    Hit::Miss
}

/// `DrawingText.IsPointOnSegment` (DrawingText.cs:563-574): a stub to the right when both ends meet.
fn arrow_segment_hit(
    a: PointF,
    b: PointF,
    header: &kinovea_metadata::Header,
    p: PointF,
    ctx: &HitContext,
) -> bool {
    let mut path = GraphicsPath::new();
    if super::common::nearly_coincide(a, b) {
        path.add_line(a, PointF::new(a.x + 2.0, a.y));
    } else {
        path.add_line(a, b);
    }
    hit_path(p, &path, line_size(header), false, ctx.viewport)
}
