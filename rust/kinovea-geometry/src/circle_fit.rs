//! Linear least-squares circle fit (Coope 1993), replacing the `MathNet` solve of the
//! original with Cramer's rule on the 3×3 normal equations.

use crate::point::PointF;
use crate::shapes::Circle;

/// Determinant below which the normal equations are treated as singular.
const SINGULAR: f64 = 1e-12;

/// Fits a circle to `points`; fewer than three points or a singular system give
/// [`Circle::EMPTY`].
pub fn fit_circle(points: &[PointF]) -> Circle {
    if points.len() < 3 {
        return Circle::EMPTY;
    }
    // Normal equations B·z = c with rows [x, y, 1] and right-hand side x² + y².
    let mut b = [[0.0f64; 3]; 3];
    let mut c = [0.0f64; 3];
    for p in points {
        let row = [f64::from(p.x), f64::from(p.y), 1.0];
        let rhs = row[0] * row[0] + row[1] * row[1];
        for i in 0..3 {
            for j in 0..3 {
                b[i][j] += row[i] * row[j];
            }
            c[i] += row[i] * rhs;
        }
    }
    let Some(z) = solve(&b, &c) else {
        return Circle::EMPTY;
    };
    let center = PointF::new((z[0] * 0.5) as f32, (z[1] * 0.5) as f32);
    let radius = (z[2] + f64::from(center.x * center.x) + f64::from(center.y * center.y)).sqrt();
    Circle::new(center, radius as f32)
}

fn det(m: &[[f64; 3]; 3]) -> f64 {
    m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
        - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
        + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
}

/// Cramer's rule; `None` when the system is singular.
fn solve(b: &[[f64; 3]; 3], c: &[f64; 3]) -> Option<[f64; 3]> {
    let d = det(b);
    if d.abs() < SINGULAR {
        return None;
    }
    let mut z = [0.0; 3];
    for (k, zk) in z.iter_mut().enumerate() {
        let mut m = *b;
        for (row, ck) in m.iter_mut().zip(c) {
            row[k] = *ck;
        }
        *zk = det(&m) / d;
    }
    Some(z)
}
