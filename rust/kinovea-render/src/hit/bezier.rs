//! Cubic Bezier helpers standing in for GDI+ cardinal splines, arcs and ellipses.
use kinovea_geometry::{helpers::distance, PointF, RectangleF};

/// Cardinal spline through `points` as cubic Beziers. Neighbours of the ends are the ends
/// themselves; control offsets are `tension / 3` of the neighbour difference (0.5 is Catmull-Rom).
pub(super) fn cardinal_to_bezier(points: &[PointF], tension: f32) -> Vec<[PointF; 4]> {
    let n = points.len();
    let at = |i: usize| points[i.min(n - 1)];
    let k = tension / 3.0;
    (0..n.saturating_sub(1))
        .map(|i| {
            let (before, p1, p2, after) = (at(i.saturating_sub(1)), at(i), at(i + 1), at(i + 2));
            let c1 = PointF::new(p1.x + k * (p2.x - before.x), p1.y + k * (p2.y - before.y));
            let c2 = PointF::new(p2.x - k * (after.x - p1.x), p2.y - k * (after.y - p1.y));
            [p1, c1, c2, p2]
        })
        .collect()
}

/// Point on the ellipse inscribed in `r` at `degrees`, measured clockwise (y points down).
pub(super) fn ellipse_point(r: RectangleF, degrees: f32) -> PointF {
    let (cx, cy, rx, ry) = centre_and_radii(r);
    let t = f64::from(degrees).to_radians();
    PointF::new((cx + rx * t.cos()) as f32, (cy + ry * t.sin()) as f32)
}

/// Arc of the ellipse inscribed in `r` as cubic Beziers, each covering at most 90 degrees.
pub(super) fn arc_to_bezier(r: RectangleF, start: f32, sweep: f32) -> Vec<[PointF; 4]> {
    let (cx, cy, rx, ry) = centre_and_radii(r);
    let (start, sweep) = (f64::from(start), f64::from(sweep));
    let parts = (sweep.abs() / 90.0).ceil() as usize;
    let step = sweep / parts.max(1) as f64;
    let k = 4.0 / 3.0 * (step.to_radians() / 4.0).tan();
    (0..parts)
        .map(|i| {
            let t1 = (start + step * i as f64).to_radians();
            let t2 = t1 + step.to_radians();
            let pt = |x: f64, y: f64| PointF::new((cx + rx * x) as f32, (cy + ry * y) as f32);
            [
                pt(t1.cos(), t1.sin()),
                pt(t1.cos() - k * t1.sin(), t1.sin() + k * t1.cos()),
                pt(t2.cos() + k * t2.sin(), t2.sin() - k * t2.cos()),
                pt(t2.cos(), t2.sin()),
            ]
        })
        .collect()
}

/// Polyline from `start` through the curves, with about two pixels per step.
pub(super) fn flatten(start: PointF, curves: &[[PointF; 4]]) -> Vec<PointF> {
    let mut points = vec![start];
    for curve in curves {
        let length = distance(curve[0], curve[1])
            + distance(curve[1], curve[2])
            + distance(curve[2], curve[3]);
        let steps = ((length / 2.0).ceil() as usize).max(1);
        points.extend((1..=steps).map(|i| cubic_at(curve, i as f32 / steps as f32)));
    }
    points
}

fn cubic_at(c: &[PointF; 4], t: f32) -> PointF {
    let u = 1.0 - t;
    let (a, b, cc, d) = (u * u * u, 3.0 * u * u * t, 3.0 * u * t * t, t * t * t);
    PointF::new(
        a * c[0].x + b * c[1].x + cc * c[2].x + d * c[3].x,
        a * c[0].y + b * c[1].y + cc * c[2].y + d * c[3].y,
    )
}

fn centre_and_radii(r: RectangleF) -> (f64, f64, f64, f64) {
    (
        f64::from(r.x) + f64::from(r.width) / 2.0,
        f64::from(r.y) + f64::from(r.height) / 2.0,
        f64::from(r.width) / 2.0,
        f64::from(r.height) / 2.0,
    )
}
