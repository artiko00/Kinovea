//! Owner: P1. Painters ported from `DrawingRectangle.Draw`.

use super::PaintContext;
use crate::renderer::Renderer;
use kinovea_metadata::drawing::Rectangle;

/// Port of `DrawingRectangle.Draw`.
pub fn paint_rectangle(d: &Rectangle, ctx: &PaintContext, r: &mut dyn Renderer) {
    let _ = (d, ctx, r);
}
