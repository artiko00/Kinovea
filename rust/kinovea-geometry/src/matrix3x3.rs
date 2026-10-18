//! 3×3 single precision matrix (AForge `Matrix3x3` as embedded in Kinovea).

mod rotations;
#[cfg(test)]
mod rotations_test;

use crate::svd::{svdcmp, SvdError};
use crate::vector3::Vector3;
use std::ops::{Add, Mul, Sub};

/// Row-major 3×3 matrix; `v[row][column]` matches the original `Vrc` fields.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Matrix3x3 {
    pub v: [[f32; 3]; 3],
}

impl Matrix3x3 {
    pub const IDENTITY: Matrix3x3 = Matrix3x3::from_diagonal(Vector3::splat(1.0));

    pub const fn from_array(v: [[f32; 3]; 3]) -> Self {
        Matrix3x3 { v }
    }

    pub const fn from_rows(r0: Vector3, r1: Vector3, r2: Vector3) -> Self {
        Matrix3x3::from_array([[r0.x, r0.y, r0.z], [r1.x, r1.y, r1.z], [r2.x, r2.y, r2.z]])
    }

    pub const fn from_columns(c0: Vector3, c1: Vector3, c2: Vector3) -> Self {
        Matrix3x3::from_array([[c0.x, c1.x, c2.x], [c0.y, c1.y, c2.y], [c0.z, c1.z, c2.z]])
    }

    pub const fn from_diagonal(d: Vector3) -> Self {
        Matrix3x3::from_array([[d.x, 0.0, 0.0], [0.0, d.y, 0.0], [0.0, 0.0, d.z]])
    }

    /// Row-major flattening `V00, V01, ..., V22`.
    pub fn to_array(&self) -> [f32; 9] {
        let v = &self.v;
        [
            v[0][0], v[0][1], v[0][2], v[1][0], v[1][1], v[1][2], v[2][0], v[2][1], v[2][2],
        ]
    }

    /// Row `index` (0..=2); panics outside that range like the original throws.
    pub fn row(&self, index: usize) -> Vector3 {
        let r = self.v[index];
        Vector3::new(r[0], r[1], r[2])
    }

    /// Column `index` (0..=2); panics outside that range like the original throws.
    pub fn column(&self, index: usize) -> Vector3 {
        Vector3::new(self.v[0][index], self.v[1][index], self.v[2][index])
    }

    /// Determinant in single precision, same term order as the original.
    pub fn determinant(&self) -> f32 {
        let v = &self.v;
        v[0][0] * v[1][1] * v[2][2] + v[0][1] * v[1][2] * v[2][0] + v[0][2] * v[1][0] * v[2][1]
            - v[0][0] * v[1][2] * v[2][1]
            - v[0][1] * v[1][0] * v[2][2]
            - v[0][2] * v[1][1] * v[2][0]
    }

    pub fn transpose(&self) -> Matrix3x3 {
        Matrix3x3::from_columns(self.row(0), self.row(1), self.row(2))
    }

    pub fn adjugate(&self) -> Matrix3x3 {
        let v = &self.v;
        let minor = |r0: usize, c0: usize, r1: usize, c1: usize| {
            v[r0][c0] * v[r1][c1] - v[r0][c1] * v[r1][c0]
        };
        Matrix3x3::from_array([
            [minor(1, 1, 2, 2), -minor(0, 1, 2, 2), minor(0, 1, 1, 2)],
            [-minor(1, 0, 2, 2), minor(0, 0, 2, 2), -minor(0, 0, 1, 2)],
            [minor(1, 0, 2, 1), -minor(0, 0, 2, 1), minor(0, 0, 1, 1)],
        ])
    }

    /// Inverse through the adjugate; `None` when the determinant is exactly zero.
    pub fn inverse(&self) -> Option<Matrix3x3> {
        let det = self.determinant();
        (det != 0.0).then(|| self.adjugate() * (1.0 / det))
    }

    /// `M · Mᵀ`.
    pub fn multiply_self_by_transpose(&self) -> Matrix3x3 {
        *self * self.transpose()
    }

    /// `Mᵀ · M`.
    pub fn multiply_transpose_by_self(&self) -> Matrix3x3 {
        self.transpose() * *self
    }

    /// Decomposition `M = U · diag(e) · Vᵀ` computed in double precision.
    pub fn svd(&self) -> Result<(Matrix3x3, Vector3, Matrix3x3), SvdError> {
        let rows: Vec<Vec<f64>> = self
            .v
            .iter()
            .map(|r| r.iter().map(|&x| f64::from(x)).collect())
            .collect();
        let svd = svdcmp(&rows)?;
        let to_m = |m: &[Vec<f64>]| {
            let mut out = Matrix3x3::default();
            for (r, row) in m.iter().enumerate() {
                for (c, x) in row.iter().enumerate() {
                    out.v[r][c] = *x as f32;
                }
            }
            out
        };
        let e = Vector3::new(svd.w[0] as f32, svd.w[1] as f32, svd.w[2] as f32);
        Ok((to_m(&svd.u), e, to_m(&svd.v)))
    }

    /// `V · diag(1/e) · Uᵀ`.
    pub fn pseudo_inverse(&self) -> Result<Matrix3x3, SvdError> {
        let (u, e, v) = self.svd()?;
        Ok(v * Matrix3x3::from_diagonal(e.inverse()) * u.transpose())
    }

    fn map(self, f: impl Fn(f32) -> f32) -> Matrix3x3 {
        Matrix3x3 {
            v: self.v.map(|r| r.map(&f)),
        }
    }

    fn zip(self, o: Matrix3x3, f: impl Fn(f32, f32) -> f32) -> Matrix3x3 {
        let mut out = self;
        for (r, row) in out.v.iter_mut().enumerate() {
            for (c, x) in row.iter_mut().enumerate() {
                *x = f(*x, o.v[r][c]);
            }
        }
        out
    }
}

impl Mul for Matrix3x3 {
    type Output = Matrix3x3;
    fn mul(self, o: Matrix3x3) -> Matrix3x3 {
        let mut out = Matrix3x3::default();
        for (r, row) in out.v.iter_mut().enumerate() {
            for (c, x) in row.iter_mut().enumerate() {
                *x = self.v[r][0] * o.v[0][c] + self.v[r][1] * o.v[1][c] + self.v[r][2] * o.v[2][c];
            }
        }
        out
    }
}

impl Mul<Vector3> for Matrix3x3 {
    type Output = Vector3;
    fn mul(self, p: Vector3) -> Vector3 {
        Vector3::new(self.row(0).dot(p), self.row(1).dot(p), self.row(2).dot(p))
    }
}

impl Mul<f32> for Matrix3x3 {
    type Output = Matrix3x3;
    fn mul(self, f: f32) -> Matrix3x3 {
        self.map(|x| x * f)
    }
}

impl Add<f32> for Matrix3x3 {
    type Output = Matrix3x3;
    fn add(self, f: f32) -> Matrix3x3 {
        self.map(|x| x + f)
    }
}

impl Add for Matrix3x3 {
    type Output = Matrix3x3;
    fn add(self, o: Matrix3x3) -> Matrix3x3 {
        self.zip(o, |a, b| a + b)
    }
}

impl Sub for Matrix3x3 {
    type Output = Matrix3x3;
    fn sub(self, o: Matrix3x3) -> Matrix3x3 {
        self.zip(o, |a, b| a - b)
    }
}
