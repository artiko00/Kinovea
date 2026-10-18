//! Ellipse of the plane unit circle seen through the homography.

use super::ProjectiveMapper;
use crate::point::PointF;
use crate::shapes::Ellipse;
use std::f64::consts::PI;

impl ProjectiveMapper {
    /// Maps the plane unit circle centred at the origin to its image ellipse.
    pub fn ellipse(&self) -> Ellipse {
        let u = self.unmap_matrix();
        let [[j, k, l], [m, n, o], [p, q, r]] = *u;
        // Ellipse: ax² + 2bxy + cy² + 2dx + 2fy + g = 0
        let a = j * j + m * m - p * p;
        let b = j * k + m * n - p * q;
        let c = k * k + n * n - q * q;
        let d = j * l + m * o - p * r;
        let f = k * l + n * o - q * r;
        let g = l * l + o * o - r * r;
        let factor = b * b - a * c;
        let x0 = (c * d - b * f) / factor;
        let y0 = (a * f - b * d) / factor;
        let num = 2.0 * (a * f * f + c * d * d + g * b * b - 2.0 * b * d * f - a * c * g);
        let factor2 = ((a - c) * (a - c) + 4.0 * b * b).sqrt();
        let semi_major = (num / (factor * (factor2 - (a + c)))).sqrt();
        let semi_minor = (num / (factor * (-factor2 - (a + c)))).sqrt();
        let mut rotation = if b == 0.0 {
            if a < c {
                0.0
            } else {
                PI / 2.0
            }
        } else if a < c {
            0.5 * arccotan((a - c) / (2.0 * b))
        } else {
            PI / 2.0 + 0.5 * arccotan((a - c) / (2.0 * b))
        };
        if b != 0.0 {
            // Correction of the formulas above, kept from the original.
            if rotation > PI / 4.0 && rotation < PI / 2.0 {
                rotation += PI / 2.0;
            } else if rotation > PI * 0.75 && rotation < PI {
                rotation -= PI / 2.0;
            }
        }
        Ellipse::new(
            PointF::new(x0 as f32, y0 as f32),
            semi_major as f32,
            semi_minor as f32,
            rotation as f32,
        )
    }
}

fn arccotan(x: f64) -> f64 {
    2.0 * 1f64.atan() - x.atan()
}
