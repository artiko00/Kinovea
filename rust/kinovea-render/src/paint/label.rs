//! Owner: P2. Painters ported from `DrawingText.Draw`.

use super::PaintContext;
use crate::renderer::Renderer;
use kinovea_metadata::drawing::Label;

/// Port of `DrawingText.Draw`.
pub fn paint_label(d: &Label, ctx: &PaintContext, r: &mut dyn Renderer) {
    let _ = (d, ctx, r);
}
