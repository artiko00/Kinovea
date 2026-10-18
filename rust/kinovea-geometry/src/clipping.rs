//! Liang-Barsky clipping of segments, rays and infinite lines against a rectangle.

use crate::point::PointF;
use crate::rect::RectangleF;
use crate::shapes::ClipResult;

/// Clips the parametric line `a + t·(b − a)` for `t` in `[t0, t1]`.
/// Segments use `[0, 1]`; infinite lines `[f64::MIN, f64::MAX]`;
/// rays either `[0, f64::MAX]` or `[f64::MIN, 1]`.
pub fn clip_line(rect: RectangleF, a: PointF, b: PointF, mut t0: f64, mut t1: f64) -> ClipResult {
    let dx = f64::from(b.x) - f64::from(a.x);
    let dy = f64::from(b.y) - f64::from(a.y);
    let p = [-dx, dx, -dy, dy];
    let q = [
        f64::from(a.x) - f64::from(rect.left()),
        f64::from(rect.right()) - f64::from(a.x),
        f64::from(a.y) - f64::from(rect.top()),
        f64::from(rect.bottom()) - f64::from(a.y),
    ];
    for (p, q) in p.into_iter().zip(q) {
        if p == 0.0 {
            if q < 0.0 {
                return ClipResult::INVISIBLE;
            }
        } else if p < 0.0 {
            let t = q / p;
            if t > t1 {
                return ClipResult::INVISIBLE;
            }
            t0 = t.max(t0);
        } else {
            let t = q / p;
            if t < t0 {
                return ClipResult::INVISIBLE;
            }
            t1 = t.min(t1);
        }
    }
    let at = |t: f64| {
        PointF::new(
            (f64::from(a.x) + t * dx) as f32,
            (f64::from(a.y) + t * dy) as f32,
        )
    };
    ClipResult::visible(at(t0), at(t1))
}

/// Clips the segment `[a, b]`.
pub fn clip_segment(rect: RectangleF, a: PointF, b: PointF) -> ClipResult {
    clip_line(rect, a, b, 0.0, 1.0)
}

/// Clips the infinite line through `a` and `b`.
pub fn clip_infinite_line(rect: RectangleF, a: PointF, b: PointF) -> ClipResult {
    clip_line(rect, a, b, f64::MIN, f64::MAX)
}
