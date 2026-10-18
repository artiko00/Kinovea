//! 2D single precision vector (`Kinovea.ScreenManager.Vector`).

use crate::point::PointF;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Vector {
    pub const fn new(x: f32, y: f32) -> Self {
        Vector { x, y }
    }

    /// Vector from `a` to `b`.
    pub fn from_points(a: PointF, b: PointF) -> Self {
        Vector::new(b.x - a.x, b.y - a.y)
    }

    pub fn dot(self, v: Vector) -> f32 {
        self.x * v.x + self.y * v.y
    }

    /// Squared norm, in single precision like the original.
    pub fn squared(self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn norm(self) -> f32 {
        f64::from(self.squared()).sqrt() as f32
    }

    /// `this * (1 / Norm())`: the zero vector yields NaN components, as in the original.
    pub fn normalized(self) -> Vector {
        self * (1.0 / self.norm())
    }

    pub fn to_point(self) -> PointF {
        PointF::new(self.x, self.y)
    }
}

impl Add for Vector {
    type Output = Vector;
    fn add(self, v: Vector) -> Vector {
        Vector::new(self.x + v.x, self.y + v.y)
    }
}

impl Sub for Vector {
    type Output = Vector;
    fn sub(self, v: Vector) -> Vector {
        Vector::new(self.x - v.x, self.y - v.y)
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;
    fn mul(self, f: f32) -> Vector {
        Vector::new(self.x * f, self.y * f)
    }
}

impl Div<f32> for Vector {
    type Output = Vector;
    fn div(self, f: f32) -> Vector {
        Vector::new(self.x / f, self.y / f)
    }
}

impl Neg for Vector {
    type Output = Vector;
    fn neg(self) -> Vector {
        Vector::new(-self.x, -self.y)
    }
}
