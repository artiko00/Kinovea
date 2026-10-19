//! Stroke outline of a polyline as GDI+ `Widen` builds it for a pen with flat caps and miter joins
//! (limit 10): one rectangle per segment and one join piece per vertex. Their union is the hit area.
//! Flat caps matter: a point just past an open end misses, unlike a distance to the end point.

use kinovea_geometry::PointF;

/// `Pen.MiterLimit` default of GDI+ (ratio of the miter length to the half width).
const MITER_LIMIT: f32 = 10.0;

/// Convex pieces of the stroke: quads, with a bevel repeating its vertex so the arrays stay fixed.
type Quad = [PointF; 4];

/// Whether `p` is inside the widened polyline of half width `half`. `closed` adds the closing edge
/// and a join at its first vertex.
pub(super) fn stroke_hit(points: &[PointF], closed: bool, half: f32, p: PointF) -> bool {
    let n = points.len();
    let segments = if closed { n } else { n.saturating_sub(1) };
    let in_segment = (0..segments).any(|i| {
        let (a, b) = (points[i], points[(i + 1) % n]);
        segment_quad(a, b, half).is_some_and(|q| convex_contains(&q, p) && !on_open_side(&q, p))
    });
    if in_segment {
        return true;
    }
    let joins = if closed { 0..n } else { 1..n.saturating_sub(1) };
    joins.into_iter().any(|i| {
        let (u, v, w) = (points[(i + n - 1) % n], points[i], points[(i + 1) % n]);
        join_quad(u, v, w, half).is_some_and(|q| convex_contains(&q, p))
    })
}

/// The rectangle of a segment: its two sides offset by the half width.
fn segment_quad(a: PointF, b: PointF, half: f32) -> Option<Quad> {
    let n = unit_normal(a, b)?;
    let off = |q: PointF, s: f32| PointF::new(q.x + n.x * half * s, q.y + n.y * half * s);
    Some([off(a, 1.0), off(b, 1.0), off(b, -1.0), off(a, -1.0)])
}

/// The outer wedge at `v` between the segments `u`-`v` and `v`-`w`: a miter when within the limit,
/// else a bevel. Nothing at straight joins.
fn join_quad(u: PointF, v: PointF, w: PointF, half: f32) -> Option<Quad> {
    let (n1, n2) = (unit_normal(u, v)?, unit_normal(v, w)?);
    let dot = n1.x * n2.x + n1.y * n2.y;
    let turn = (v.x - u.x) * (w.y - v.y) - (v.y - u.y) * (w.x - v.x);
    if turn.abs() < 1e-6 && dot > 0.0 {
        return None;
    }
    // The outer side is the one opposite the turn.
    let s = if turn > 0.0 { -1.0 } else { 1.0 };
    let o1 = PointF::new(v.x + n1.x * half * s, v.y + n1.y * half * s);
    let o2 = PointF::new(v.x + n2.x * half * s, v.y + n2.y * half * s);
    let k = half * s / (1.0 + dot);
    let tip = PointF::new(v.x + (n1.x + n2.x) * k, v.y + (n1.y + n2.y) * k);
    let within = (tip.x - v.x).hypot(tip.y - v.y) <= MITER_LIMIT * half;
    Some(if within {
        [v, o1, tip, o2]
    } else {
        [v, o1, o2, o2]
    })
}

/// Unit normal of `a`-`b`, or `None` for a zero-length segment.
fn unit_normal(a: PointF, b: PointF) -> Option<PointF> {
    let (dx, dy) = (b.x - a.x, b.y - a.y);
    let len = dx.hypot(dy);
    (len > 0.0).then(|| PointF::new(-dy / len, dx / len))
}

/// Whether `p` lies exactly on a long side of a segment piece that the fill leaves open: the bottom or
/// right side, as the top-left rule of `RectangleF.Contains` does. The flat ends stay closed.
/// Seen in `geometry` k0#3: a 2 px rectangle edge at y 120 with the 10 px enlarger has its band from 114
/// to 126, and the oracle hits y 114 but not y 126.
fn on_open_side(q: &Quad, p: PointF) -> bool {
    let sign = if signed_area(q) >= 0.0 { 1.0 } else { -1.0 };
    [0, 2].iter().any(|&i| {
        let (a, b) = (q[i], q[(i + 1) % 4]);
        let (dx, dy) = (b.x - a.x, b.y - a.y);
        let on_line = dx * (p.y - a.y) - dy * (p.x - a.x) == 0.0;
        let top_or_left = dy * sign < 0.0 || (dy == 0.0 && dx * sign > 0.0);
        on_line && !top_or_left
    })
}

fn signed_area(q: &Quad) -> f32 {
    (0..4)
        .map(|i| {
            let (a, b) = (q[i], q[(i + 1) % 4]);
            a.x * b.y - b.x * a.y
        })
        .sum()
}

/// Point inside a convex quad (either winding), boundary included. A flat quad has no inside.
fn convex_contains(q: &Quad, p: PointF) -> bool {
    let side = |i: usize| {
        let (a, b) = (q[i], q[(i + 1) % 4]);
        (b.x - a.x) * (p.y - a.y) - (b.y - a.y) * (p.x - a.x)
    };
    let s = [side(0), side(1), side(2), side(3)];
    let inside = s.iter().all(|x| *x >= 0.0) || s.iter().all(|x| *x <= 0.0);
    inside && !is_flat(q)
}

fn is_flat(q: &Quad) -> bool {
    signed_area(q).abs() < 1e-6
}
