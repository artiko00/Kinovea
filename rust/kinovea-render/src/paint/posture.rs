//! Owner: P1. Painters ported from `DrawingGenericPosture.Draw`.

use super::PaintContext;
use crate::renderer::Renderer;
use kinovea_metadata::drawing::GenericPostureDrawing;

/// Port of `DrawingGenericPosture.Draw`.
pub fn paint_posture(d: &GenericPostureDrawing, ctx: &PaintContext, r: &mut dyn Renderer) {
    let _ = (d, ctx, r);
}
