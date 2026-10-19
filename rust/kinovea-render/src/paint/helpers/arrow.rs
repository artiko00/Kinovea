//! Owner: P1. Port of `ArrowHelper` (`Helpers/ArrowHelper.cs`).

use crate::renderer::{Brush, Pen, Renderer};
use kinovea_geometry::helpers::distance;
use kinovea_geometry::PointF;

/// `Math.Max(penWidth, 4)`: arrows never shrink below 4 pixels.
fn ref_length(pen_width: f32) -> f32 {
    pen_width.max(4.0)
}

/// Port of `ArrowHelper.Draw(canvas, pen, a, b)`: arrow head at `tip` pointing away from `from`.
/// The base sits one reference length inside the segment and the tip three beyond it.
pub fn draw_arrow(r: &mut dyn Renderer, pen: &Pen, tip: PointF, from: PointF) {
    let (vx, vy) = (from.x - tip.x, from.y - tip.y);
    let norm = vx.hypot(vy);
    let length = ref_length(pen.width);
    let base = (tip.x + vx * length / norm, tip.y + vy * length / norm);
    let top = PointF::new(
        tip.x - vx * 3.0 * length / norm,
        tip.y - vy * 3.0 * length / norm,
    );
    let side = 1.5 * length / norm;
    let left = PointF::new(base.0 + vy * side, base.1 - vx * side);
    let right = PointF::new(base.0 - vy * side, base.1 + vx * side);
    r.fill_polygon(&[top, left, right], &Brush::solid(pen.color));
}

/// Port of `ArrowHelper.GetOffset`: how far an arrowed end moves inward so that the tip lands on the
/// original end. Zero-length segments give the empty offset.
pub fn arrow_offset(pen_width: f32, a: PointF, b: PointF) -> PointF {
    let (vx, vy) = (b.x - a.x, b.y - a.y);
    let norm = vx.hypot(vy);
    if norm == 0.0 {
        return PointF::EMPTY;
    }
    let t = 3.0 * ref_length(pen_width) / norm;
    PointF::new(vx * t, vy * t)
}

/// Port of `ArrowHelper.UpdateStartEnd`: moves the arrowed ends inward (truncated to whole pixels, as
/// `ToPoint`) and returns whether the segment is long enough to carry an arrow.
pub fn update_start_end(
    pen_width: f32,
    a: &mut PointF,
    b: &mut PointF,
    arrow_start: bool,
    arrow_end: bool,
) -> bool {
    let offset = arrow_offset(pen_width, *a, *b);
    let offset_length = offset.x.hypot(offset.y);
    let can_draw = distance(*a, *b) > offset_length;
    if can_draw {
        if arrow_start {
            *a = truncate(PointF::new(a.x + offset.x, a.y + offset.y));
        }
        if arrow_end {
            *b = truncate(PointF::new(b.x - offset.x, b.y - offset.y));
        }
    }
    can_draw
}

/// `ToPoint()`: truncation toward zero of both coordinates.
fn truncate(p: PointF) -> PointF {
    PointF::new(p.x.trunc(), p.y.trunc())
}
