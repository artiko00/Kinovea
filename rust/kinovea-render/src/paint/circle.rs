//! Owner: P1. Painters ported from `DrawingCircle.Draw`.

use super::PaintContext;
use crate::renderer::Renderer;
use kinovea_metadata::drawing::Circle;

/// Port of `DrawingCircle.Draw`.
pub fn paint_circle(d: &Circle, ctx: &PaintContext, r: &mut dyn Renderer) {
    let _ = (d, ctx, r);
}
