//! Owner: P1. Port of `GraphicsExtensions.DrawSquigglyLine(s)` (`Helpers/GraphicsExtensions.cs`).

use crate::renderer::{Pen, Renderer};
use kinovea_geometry::PointF;

/// `GraphicsExtensions.tension`: cardinal tension of the squiggle curve.
const TENSION: f32 = 0.9;
/// `GraphicsExtensions.dodgeFactor`: half period as a multiple of the reference length.
const DODGE_FACTOR: f32 = 1.5;

/// Port of `GraphicsExtensions.DrawSquigglyLine`.
pub fn draw_squiggly_line(r: &mut dyn Renderer, pen: &Pen, a: PointF, b: PointF) {
    let mut points = vec![a];
    add_squiggle_points(&mut points, a, b, pen.width);
    r.stroke_curve(&points, TENSION, pen);
}

/// Port of `GraphicsExtensions.DrawSquigglyLines`: a squiggle along each non-degenerate segment.
pub fn draw_squiggly_lines(r: &mut dyn Renderer, pen: &Pen, points: &[PointF]) {
    if points.len() < 2 {
        return;
    }
    let mut curve = vec![points[0]];
    for pair in points.windows(2) {
        if pair[0] != pair[1] {
            add_squiggle_points(&mut curve, pair[0], pair[1], pen.width);
        }
    }
    curve.push(points[points.len() - 1]);
    r.stroke_curve(&curve, TENSION, pen);
}

/// Port of `GraphicsExtensions.AddSquigglePoints`: zigzag control points from `a` to `b`.
fn add_squiggle_points(points: &mut Vec<PointF>, a: PointF, b: PointF, ref_length: f32) {
    let length = ref_length.max(4.0);
    let half_period = length * DODGE_FACTOR;
    let (vx, vy) = (b.x - a.x, b.y - a.y);
    let norm = vx.hypot(vy);
    let ratio = half_period / norm;
    let (sx, sy) = (vx * ratio, vy * ratio);
    let step_up = (sy / 2.0, -sx / 2.0);
    let step_down = (-sy / 2.0, sx / 2.0);
    let mut current = PointF::new(a.x + sx, a.y + sy);
    points.push(current);
    let steps = norm / half_period;
    let mut i = 0;
    while (i as f32) < steps - 3.0 {
        let dir = if i % 2 == 0 { step_up } else { step_down };
        current = PointF::new(current.x + sx, current.y + sy);
        points.push(PointF::new(current.x + dir.0, current.y + dir.1));
        i += 1;
    }
    points.push(PointF::new(current.x + sx / 2.0, current.y + sy / 2.0));
    points.push(b);
}
