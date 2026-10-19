//! Owner: P1. Painters for `DrawingCoordinateSystem.Draw` and `DrawingTestGrid.Draw`.

use super::PaintContext;
use crate::renderer::Renderer;
use kinovea_metadata::singletons::{CoordinateSystem, TestGrid};

/// Port of `DrawingCoordinateSystem.Draw`.
pub fn paint_coordinate_system(d: &CoordinateSystem, ctx: &PaintContext, r: &mut dyn Renderer) {
    let _ = (d, ctx, r);
}

/// Port of `DrawingTestGrid.Draw`.
pub fn paint_test_grid(d: &TestGrid, ctx: &PaintContext, r: &mut dyn Renderer) {
    let _ = (d, ctx, r);
}
