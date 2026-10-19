//! Owner: P2. Painters ported from `DrawingChrono.Draw`, `DrawingChronoMulti.Draw`.

use super::PaintContext;
use crate::renderer::Renderer;
use kinovea_metadata::drawing::{Chrono, ChronoMulti};

/// Port of `DrawingChrono.Draw`.
pub fn paint_chrono(d: &Chrono, ctx: &PaintContext, r: &mut dyn Renderer) {
    let _ = (d, ctx, r);
}

/// Port of `DrawingChronoMulti.Draw`.
pub fn paint_chrono_multi(d: &ChronoMulti, ctx: &PaintContext, r: &mut dyn Renderer) {
    let _ = (d, ctx, r);
}
