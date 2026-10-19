//! Owner: P2. Painters ported from `DrawingBitmap.Draw` (placeholder until image readers land), `DrawingSVG.Draw` (placeholder until image readers land).

use super::PaintContext;
use crate::renderer::Renderer;
use kinovea_metadata::drawing::{Bitmap, Svg};

/// Port of `DrawingBitmap.Draw` (placeholder until image readers land).
pub fn paint_bitmap(d: &Bitmap, ctx: &PaintContext, r: &mut dyn Renderer) {
    let _ = (d, ctx, r);
}

/// Port of `DrawingSVG.Draw` (placeholder until image readers land).
pub fn paint_svg(d: &Svg, ctx: &PaintContext, r: &mut dyn Renderer) {
    let _ = (d, ctx, r);
}
