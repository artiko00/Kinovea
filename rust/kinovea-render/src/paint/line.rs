//! Owner: P1. Painters ported from `DrawingLine.Draw`.

use super::PaintContext;
use crate::renderer::Renderer;
use kinovea_metadata::drawing::Line;

/// Port of `DrawingLine.Draw`.
pub fn paint_line(d: &Line, ctx: &PaintContext, r: &mut dyn Renderer) {
    let _ = (d, ctx, r);
}
