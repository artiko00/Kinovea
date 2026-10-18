//! Quadrilateral with vertices A, B, C, D clockwise from top-left (`QuadrilateralF`).

mod ops;
#[cfg(test)]
mod ops_test;

use crate::point::PointF;
use crate::rect::RectangleF;
use std::ops::{Index, IndexMut};

/// Four vertices; index 0..=3 is A, B, C, D.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Quadrilateral {
    pub vertices: [PointF; 4],
}

impl Quadrilateral {
    pub const fn new(a: PointF, b: PointF, c: PointF, d: PointF) -> Self {
        Quadrilateral {
            vertices: [a, b, c, d],
        }
    }

    /// Rectangle at the origin with the given size.
    pub const fn from_size(width: f32, height: f32) -> Self {
        Quadrilateral::new(
            PointF::new(0.0, 0.0),
            PointF::new(width, 0.0),
            PointF::new(width, height),
            PointF::new(0.0, height),
        )
    }

    pub fn from_rect(rect: RectangleF) -> Self {
        Quadrilateral::new(
            rect.location(),
            PointF::new(rect.right(), rect.top()),
            PointF::new(rect.right(), rect.bottom()),
            PointF::new(rect.left(), rect.bottom()),
        )
    }

    pub const fn empty() -> Self {
        Quadrilateral::new(PointF::EMPTY, PointF::EMPTY, PointF::EMPTY, PointF::EMPTY)
    }

    pub const fn unit_square() -> Self {
        Quadrilateral::from_size(1.0, 1.0)
    }

    /// `(-1, 1), (1, 1), (1, -1), (-1, -1)`.
    pub const fn centered_unit_square() -> Self {
        Quadrilateral::new(
            PointF::new(-1.0, 1.0),
            PointF::new(1.0, 1.0),
            PointF::new(1.0, -1.0),
            PointF::new(-1.0, -1.0),
        )
    }

    pub fn a(&self) -> PointF {
        self.vertices[0]
    }

    pub fn b(&self) -> PointF {
        self.vertices[1]
    }

    pub fn c(&self) -> PointF {
        self.vertices[2]
    }

    pub fn d(&self) -> PointF {
        self.vertices[3]
    }

    /// `A == C`.
    pub fn is_empty(&self) -> bool {
        self.a() == self.c()
    }

    /// All four interior angles turn the same way.
    pub fn is_convex(&self) -> bool {
        let [a, b, c, d] = self.vertices;
        let angles = [
            angle(a, b, c),
            angle(b, c, d),
            angle(c, d, a),
            angle(d, a, b),
        ];
        angles.iter().all(|x| *x > 0.0) || angles.iter().all(|x| *x < 0.0)
    }

    pub fn is_axis_aligned_rectangle(&self) -> bool {
        let [a, b, c, d] = self.vertices;
        a.y == b.y && b.x == c.x && c.y == d.y && d.x == a.x
    }

    /// Point-in-convex-polygon test; boundary points count as inside. Non-convex: `false`.
    pub fn contains(&self, p: PointF) -> bool {
        if !self.is_convex() {
            return false;
        }
        let side = |i: usize| {
            let (u, v) = (self.vertices[i], self.vertices[(i + 1) % 4]);
            f64::from((v.x - u.x) * (p.y - u.y) - (v.y - u.y) * (p.x - u.x))
        };
        let sides = [side(0), side(1), side(2), side(3)];
        sides.iter().all(|s| *s >= 0.0) || sides.iter().all(|s| *s <= 0.0)
    }

    pub fn bounding_box(&self) -> RectangleF {
        if self.is_empty() {
            return RectangleF::EMPTY;
        }
        let (mut left, mut top, mut right, mut bottom) = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
        for v in &self.vertices {
            left = left.min(v.x);
            top = top.min(v.y);
            right = right.max(v.x);
            bottom = bottom.max(v.y);
        }
        RectangleF::new(left, top, right - left, bottom - top)
    }

    /// Stable hash with the shift pattern of the original `GetHashCode`.
    pub fn hash_code(&self) -> i32 {
        let [a, b, c, d] = self.vertices.map(PointF::hash_code);
        (a << 17) ^ (b >> 26) ^ (c << 6) ^ (d >> 25)
    }

    pub fn to_array(&self) -> [PointF; 4] {
        self.vertices
    }
}

impl Index<usize> for Quadrilateral {
    type Output = PointF;
    fn index(&self, i: usize) -> &PointF {
        &self.vertices[i]
    }
}

impl IndexMut<usize> for Quadrilateral {
    fn index_mut(&mut self, i: usize) -> &mut PointF {
        &mut self.vertices[i]
    }
}

/// Signed angle ABC in degrees, as in the original's private `GetAngle`; NaN when degenerate.
fn angle(a: PointF, b: PointF, c: PointF) -> f64 {
    let (bax, bay) = (f64::from(a.x - b.x), f64::from(a.y - b.y));
    let (bcx, bcy) = (f64::from(c.x - b.x), f64::from(c.y - b.y));
    let scal = bax * bcx + bay * bcy;
    let norm = (bax * bax + bay * bay).sqrt() * (bcx * bcx + bcy * bcy).sqrt();
    let angle = (scal / norm).acos();
    let signed = if bax * bcy - bay * bcx < 0.0 {
        -angle
    } else {
        angle
    };
    signed * (180.0 / std::f64::consts::PI)
}
