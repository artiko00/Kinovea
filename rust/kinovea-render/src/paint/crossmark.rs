//! Owner: P1. Painters ported from `DrawingCrossMark.Draw`.

use super::PaintContext;
use crate::renderer::Renderer;
use kinovea_metadata::drawing::CrossMark;

/// Port of `DrawingCrossMark.Draw`.
pub fn paint_crossmark(d: &CrossMark, ctx: &PaintContext, r: &mut dyn Renderer) {
    let _ = (d, ctx, r);
}
