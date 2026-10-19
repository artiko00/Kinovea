//! Owner: P2. Painters ported from `DrawingTimeSegment.Draw`.

use super::PaintContext;
use crate::renderer::Renderer;
use kinovea_metadata::drawing::TimeSegment;

/// Port of `DrawingTimeSegment.Draw`.
pub fn paint_time_segment(d: &TimeSegment, ctx: &PaintContext, r: &mut dyn Renderer) {
    let _ = (d, ctx, r);
}
