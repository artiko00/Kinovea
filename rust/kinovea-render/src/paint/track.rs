//! Owner: P2. Painters ported from `DrawingTrack.Draw`.

use super::PaintContext;
use crate::renderer::Renderer;
use kinovea_metadata::drawing::Track;

/// Port of `DrawingTrack.Draw`.
pub fn paint_track(d: &Track, ctx: &PaintContext, r: &mut dyn Renderer) {
    let _ = (d, ctx, r);
}
