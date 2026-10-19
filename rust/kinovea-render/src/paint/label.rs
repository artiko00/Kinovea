//! Owner: P2. Painters ported from `DrawingText.Draw`.

use super::helpers::arrow::{draw_arrow, update_start_end};
use super::helpers::rounded_rect::{draw_text_box, text_box_rect, to_rect_f};
use super::PaintContext;
use crate::renderer::{LineCap, Pen, Renderer};
use crate::style::StyleData;
use kinovea_geometry::helpers::intersection_rectangle_center;
use kinovea_geometry::{PointF, RectangleF};
use kinovea_metadata::drawing::Label;

/// Port of `DrawingText.Draw`: the arrow or the circle first, then the background and the text,
/// so the box covers the start of the leader. Painted in the background colour, with the text
/// colour taken from the contrast with the background. The leader is shortened by the arrow size
/// (`ArrowHelper.UpdateStartEnd`) for both the arrow and the circle (`DrawArrow`, `DrawCircle`);
/// the circle stays centred on the original end.
pub fn paint_label(d: &Label, ctx: &PaintContext, r: &mut dyn Renderer) {
    let opacity = d
        .header
        .fading
        .opacity_factor(ctx.timestamp, ctx.default_fading);
    if opacity <= 0.0 {
        return;
    }
    let style = StyleData::from_elements(&d.header.style);
    let alpha = (opacity * 255.0) as i32;
    let back = style.background_brush(alpha);
    let fore = style.foreground_brush(alpha);
    let font = style.font(ctx.viewport.scale as f32);
    let origin = ctx.viewport.transform_point_f(d.position);

    if d.arrow_visible || d.circle_visible {
        let rect = to_rect_f(text_box_rect(r, &d.text, &font, origin));
        let end = ctx.viewport.transform_point_f(d.arrow_end).to_point_f();
        let start = intersection_rectangle_center(rect, end);
        let pen = Pen {
            color: back.color,
            start_cap: LineCap::Round,
            ..style.pen(255, ctx.viewport.scale as f32)
        };
        let (mut a, mut b) = (start, end);
        if update_start_end(pen.width, &mut a, &mut b, false, true) {
            r.stroke_line(a, b, &pen);
            if d.arrow_visible {
                draw_arrow(r, &pen, b, a);
            } else {
                stroke_ring(r, &pen, end);
            }
        }
    }

    if d.background_visible {
        draw_text_box(r, &d.text, &font, &back, &fore, origin, None);
    } else {
        r.draw_text(&d.text, &font, &back, origin.to_point_f());
    }
}

/// `center.Box(width * 3)`: the circle around the arrow end, three pen widths in radius (the
/// width is truncated to whole pixels, as the original casts it).
fn stroke_ring(r: &mut dyn Renderer, pen: &Pen, centre: PointF) {
    let radius = pen.width as i32 as f32 * 3.0;
    let rect = RectangleF::new(
        centre.x - radius,
        centre.y - radius,
        radius * 2.0,
        radius * 2.0,
    );
    r.stroke_ellipse(rect, pen);
}
