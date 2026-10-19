//! Owner: P1. Painters ported from `DrawingPencil.Draw`, `DrawingPolyline.Draw`, `DrawingTimePolyline.Draw`.

use super::PaintContext;
use crate::renderer::Renderer;
use kinovea_metadata::drawing::{Pencil, Polyline, TimePolyline};

/// Port of `DrawingPencil.Draw`.
pub fn paint_pencil(d: &Pencil, ctx: &PaintContext, r: &mut dyn Renderer) {
    let _ = (d, ctx, r);
}

/// Port of `DrawingPolyline.Draw`.
pub fn paint_polyline(d: &Polyline, ctx: &PaintContext, r: &mut dyn Renderer) {
    let _ = (d, ctx, r);
}

/// Port of `DrawingTimePolyline.Draw`.
pub fn paint_time_polyline(d: &TimePolyline, ctx: &PaintContext, r: &mut dyn Renderer) {
    let _ = (d, ctx, r);
}
