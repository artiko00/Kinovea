//! Owner: P2. Painters for `DrawingSpotlight.Draw` and `DrawingNumberSequence.Draw`.

use super::PaintContext;
use crate::renderer::Renderer;
use kinovea_metadata::multi::{NumberSequence, Spotlights};

/// Port of `DrawingSpotlight.Draw` (dims the frame outside the spots, then the borders).
pub fn paint_spotlights(d: &Spotlights, ctx: &PaintContext, r: &mut dyn Renderer) {
    let _ = (d, ctx, r);
}

/// Port of `DrawingNumberSequence.Draw`.
pub fn paint_numbers(d: &NumberSequence, ctx: &PaintContext, r: &mut dyn Renderer) {
    let _ = (d, ctx, r);
}
