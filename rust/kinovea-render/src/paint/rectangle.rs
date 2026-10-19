//! Owner: P1. Painters ported from `DrawingRectangle.Draw`.

use super::helpers::frame::{opacity, stretch, to_viewport};
use super::PaintContext;
use crate::renderer::{LineCap, Renderer};
use crate::style::colors::alpha_of;
use crate::style::StyleData;
use kinovea_metadata::drawing::Rectangle;
use kinovea_metadata::style::{DashStyle, PenShape};

/// Port of `DrawingRectangle.Draw`: four edges with square end caps, and the quadrilateral filled when
/// `filled`.
pub fn paint_rectangle(d: &Rectangle, ctx: &PaintContext, r: &mut dyn Renderer) {
    let factor = opacity(&d.header.fading, ctx);
    if factor <= 0.0 {
        return;
    }
    let style = StyleData::from_elements(&d.header.style);
    let alpha = alpha_of(factor as f32);
    let corners = d.quad.vertices.map(|p| to_viewport(ctx, p));
    let mut pen = style.pen(alpha, stretch(ctx));
    pen.end_cap = LineCap::Square;
    if style.pen_shape == PenShape::Dash {
        pen.dash = DashStyle::Dash;
    }
    for i in 0..4 {
        r.stroke_line(corners[i], corners[(i + 1) % 4], &pen);
    }
    if d.filled {
        r.fill_polygon(&corners, &style.brush(alpha));
    }
}
