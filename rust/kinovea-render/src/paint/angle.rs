//! Owner: P1. Painters ported from `DrawingAngle.Draw`.

use super::PaintContext;
use crate::renderer::Renderer;
use kinovea_metadata::drawing::Angle;

/// Port of `DrawingAngle.Draw`.
pub fn paint_angle(d: &Angle, ctx: &PaintContext, r: &mut dyn Renderer) {
    let _ = (d, ctx, r);
}
