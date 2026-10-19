//! Owner: P1. Painters ported from `DrawingPlane.Draw`, `DrawingDistortionGrid.Draw`.

use super::PaintContext;
use crate::renderer::Renderer;
use kinovea_metadata::drawing::{DistortionGrid, Plane};

/// Port of `DrawingPlane.Draw`.
pub fn paint_plane(d: &Plane, ctx: &PaintContext, r: &mut dyn Renderer) {
    let _ = (d, ctx, r);
}

/// Port of `DrawingDistortionGrid.Draw`.
pub fn paint_distortion_grid(d: &DistortionGrid, ctx: &PaintContext, r: &mut dyn Renderer) {
    let _ = (d, ctx, r);
}
