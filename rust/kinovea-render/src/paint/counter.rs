//! Owner: P2. Painters ported from `DrawingCounter.Draw`.

use super::PaintContext;
use crate::renderer::Renderer;
use kinovea_metadata::drawing::Counter;

/// Port of `DrawingCounter.Draw`.
pub fn paint_counter(d: &Counter, ctx: &PaintContext, r: &mut dyn Renderer) {
    let _ = (d, ctx, r);
}
