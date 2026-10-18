//! `PointF` and `SizeF` mirroring `System.Drawing`.

use crate::vector::Vector;
use std::ops::Add;

/// Single precision 2D point.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct PointF {
    pub x: f32,
    pub y: f32,
}

impl PointF {
    /// `PointF.Empty`.
    pub const EMPTY: PointF = PointF { x: 0.0, y: 0.0 };

    pub const fn new(x: f32, y: f32) -> Self {
        PointF { x, y }
    }

    /// `PointF.Translate(x, y)` extension.
    pub fn translate(self, dx: f32, dy: f32) -> Self {
        PointF::new(self.x + dx, self.y + dy)
    }

    /// `PointF.Scale(scaleX, scaleY)` extension.
    pub fn scale(self, sx: f32, sy: f32) -> Self {
        PointF::new(self.x * sx, self.y * sy)
    }

    /// `PointF.GetHashCode()`-like stable hash over the bit patterns.
    pub fn hash_code(self) -> i32 {
        (self.x.to_bits() ^ self.y.to_bits()) as i32
    }
}

impl Add<Vector> for PointF {
    type Output = PointF;

    fn add(self, v: Vector) -> PointF {
        PointF::new(self.x + v.x, self.y + v.y)
    }
}

/// Single precision 2D size.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct SizeF {
    pub width: f32,
    pub height: f32,
}

impl SizeF {
    pub const fn new(width: f32, height: f32) -> Self {
        SizeF { width, height }
    }
}
