//! Affine 2D transform with the GDI+ `Matrix` layout (`TranslateTransform`, `RotateTransform`, `ScaleTransform`).

use kinovea_geometry::PointF;

/// Matrix `[m11 m12 m21 m22 dx dy]` acting on points as `x' = m11*x + m21*y + dx`, `y' = m12*x + m22*y + dy`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    pub m11: f32,
    pub m12: f32,
    pub m21: f32,
    pub m22: f32,
    pub dx: f32,
    pub dy: f32,
}

impl Transform {
    pub const fn identity() -> Self {
        Transform {
            m11: 1.0,
            m12: 0.0,
            m21: 0.0,
            m22: 1.0,
            dx: 0.0,
            dy: 0.0,
        }
    }

    pub const fn translate(dx: f32, dy: f32) -> Self {
        Transform {
            dx,
            dy,
            ..Transform::identity()
        }
    }

    /// Clockwise in a y-down surface, as `RotateTransform`.
    pub fn rotate(degrees: f32) -> Self {
        let (sin, cos) = degrees.to_radians().sin_cos();
        Transform {
            m11: cos,
            m12: sin,
            m21: -sin,
            m22: cos,
            dx: 0.0,
            dy: 0.0,
        }
    }

    pub const fn scale(sx: f32, sy: f32) -> Self {
        Transform {
            m11: sx,
            m22: sy,
            ..Transform::identity()
        }
    }

    pub fn apply(&self, p: PointF) -> PointF {
        PointF::new(
            self.m11 * p.x + self.m21 * p.y + self.dx,
            self.m12 * p.x + self.m22 * p.y + self.dy,
        )
    }

    /// Transform that applies `inner` to a point first and then `self`.
    pub fn multiply(&self, inner: &Transform) -> Transform {
        Transform {
            m11: inner.m11 * self.m11 + inner.m12 * self.m21,
            m12: inner.m11 * self.m12 + inner.m12 * self.m22,
            m21: inner.m21 * self.m11 + inner.m22 * self.m21,
            m22: inner.m21 * self.m12 + inner.m22 * self.m22,
            dx: inner.dx * self.m11 + inner.dy * self.m21 + self.dx,
            dy: inner.dx * self.m12 + inner.dy * self.m22 + self.dy,
        }
    }
}
