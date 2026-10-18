//! 3D single precision vector (AForge `Vector3` as embedded in Kinovea).

use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3 { x, y, z }
    }

    /// All three components set to `value`.
    pub const fn splat(value: f32) -> Self {
        Vector3::new(value, value, value)
    }

    pub fn max(self) -> f32 {
        self.x.max(self.y).max(self.z)
    }

    pub fn min(self) -> f32 {
        self.x.min(self.y).min(self.z)
    }

    /// Index of the largest component; ties resolve to the lowest index.
    pub fn max_index(self) -> usize {
        if self.x >= self.y {
            if self.x >= self.z {
                0
            } else {
                2
            }
        } else if self.y >= self.z {
            1
        } else {
            2
        }
    }

    /// Index of the smallest component; ties resolve to the lowest index.
    pub fn min_index(self) -> usize {
        if self.x <= self.y {
            if self.x <= self.z {
                0
            } else {
                2
            }
        } else if self.y <= self.z {
            1
        } else {
            2
        }
    }

    pub fn square(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn norm(self) -> f32 {
        f64::from(self.square()).sqrt() as f32
    }

    /// Normalizes in place and returns the previous norm.
    pub fn normalize(&mut self) -> f32 {
        let norm = self.norm();
        *self = *self * (1.0 / norm);
        norm
    }

    /// Component-wise reciprocal, zero components stay zero.
    pub fn inverse(self) -> Vector3 {
        let inv = |c: f32| if c == 0.0 { 0.0 } else { 1.0 / c };
        Vector3::new(inv(self.x), inv(self.y), inv(self.z))
    }

    pub fn negate(self) -> Vector3 {
        self * -1.0
    }

    pub fn abs(self) -> Vector3 {
        Vector3::new(self.x.abs(), self.y.abs(), self.z.abs())
    }

    pub fn cross(self, v: Vector3) -> Vector3 {
        Vector3::new(
            self.y * v.z - self.z * v.y,
            self.z * v.x - self.x * v.z,
            self.x * v.y - self.y * v.x,
        )
    }

    pub fn dot(self, v: Vector3) -> f32 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn to_array(self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }

    fn zip(self, v: Vector3, f: impl Fn(f32, f32) -> f32) -> Vector3 {
        Vector3::new(f(self.x, v.x), f(self.y, v.y), f(self.z, v.z))
    }
}

macro_rules! component_ops {
    ($($trait:ident $method:ident $op:tt),*) => {$(
        impl $trait for Vector3 {
            type Output = Vector3;
            fn $method(self, v: Vector3) -> Vector3 {
                self.zip(v, |a, b| a $op b)
            }
        }
        impl $trait<f32> for Vector3 {
            type Output = Vector3;
            fn $method(self, f: f32) -> Vector3 {
                self.zip(Vector3::splat(f), |a, b| a $op b)
            }
        }
    )*};
}

component_ops!(Add add +, Sub sub -, Mul mul *, Div div /);
