//! Singular value decomposition: the Numerical Recipes `svdcmp` routine embedded in the
//! original (`SVD.cs`), ported on `f64` with 0-based indexing.
//!
//! Deviation: the original's cancellation loop runs `j = 1..=m`, which indexes past the
//! matrix on rank-deficient inputs; the port uses `0..m` (the intent of the 1-based source).

#![allow(clippy::needless_range_loop)]

mod accumulate;
#[cfg(test)]
mod accumulate_test;
mod diagonalize;
#[cfg(test)]
mod diagonalize_test;

use std::fmt;

/// Failure modes of [`svdcmp`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SvdError {
    /// `m < n`: the routine needs at least as many rows as columns.
    RowsLessThanColumns,
    /// The QR sweep did not converge in 30 iterations.
    NoConvergence,
}

impl fmt::Display for SvdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SvdError::RowsLessThanColumns => {
                write!(
                    f,
                    "number of rows must be greater or equal to number of columns"
                )
            }
            SvdError::NoConvergence => write!(f, "no convergence in 30 svdcmp iterations"),
        }
    }
}

impl std::error::Error for SvdError {}

/// Result of a decomposition: `A = U · diag(W) · Vᵀ`.
#[derive(Debug, Clone, PartialEq)]
pub struct Svd {
    /// `m × n` left singular vectors.
    pub u: Vec<Vec<f64>>,
    /// `n` singular values, unsorted, non-negative.
    pub w: Vec<f64>,
    /// `n × n` right singular vectors.
    pub v: Vec<Vec<f64>>,
}

pub(crate) struct State {
    pub a: Vec<Vec<f64>>,
    pub w: Vec<f64>,
    pub v: Vec<Vec<f64>>,
    pub rv1: Vec<f64>,
    pub m: usize,
    pub n: usize,
    pub anorm: f64,
    pub g: f64,
    pub l: usize,
}

/// Decomposes an `m × n` row-major matrix (`m ≥ n`).
pub fn svdcmp(a: &[Vec<f64>]) -> Result<Svd, SvdError> {
    let m = a.len();
    let n = a.first().map_or(0, Vec::len);
    if m < n {
        return Err(SvdError::RowsLessThanColumns);
    }
    let mut s = State {
        a: a.to_vec(),
        w: vec![0.0; n],
        v: vec![vec![0.0; n]; n],
        rv1: vec![0.0; n],
        m,
        n,
        anorm: 0.0,
        g: 0.0,
        l: 0,
    };
    householder(&mut s);
    accumulate::right(&mut s);
    accumulate::left(&mut s);
    diagonalize::run(&mut s)?;
    Ok(Svd {
        u: s.a,
        w: s.w,
        v: s.v,
    })
}

pub(crate) fn sign(a: f64, b: f64) -> f64 {
    if b >= 0.0 {
        a.abs()
    } else {
        -a.abs()
    }
}

/// Householder reduction to bidiagonal form.
fn householder(s: &mut State) {
    let (m, n) = (s.m, s.n);
    let mut scale = 0.0;
    for i in 0..n {
        s.l = i + 1;
        s.rv1[i] = scale * s.g;
        s.g = 0.0;
        scale = 0.0;
        if i < m {
            scale = (i..m).map(|k| s.a[k][i].abs()).sum();
            if scale != 0.0 {
                let mut sum = 0.0;
                for k in i..m {
                    s.a[k][i] /= scale;
                    sum += s.a[k][i] * s.a[k][i];
                }
                let f = s.a[i][i];
                s.g = -sign(sum.sqrt(), f);
                let h = f * s.g - sum;
                s.a[i][i] = f - s.g;
                if i != n - 1 {
                    for j in s.l..n {
                        let dot: f64 = (i..m).map(|k| s.a[k][i] * s.a[k][j]).sum();
                        let f = dot / h;
                        for k in i..m {
                            s.a[k][j] += f * s.a[k][i];
                        }
                    }
                }
                for k in i..m {
                    s.a[k][i] *= scale;
                }
            }
        }
        s.w[i] = scale * s.g;
        s.g = 0.0;
        scale = 0.0;
        if i < m && i != n - 1 {
            let l = s.l;
            scale = (l..n).map(|k| s.a[i][k].abs()).sum();
            if scale != 0.0 {
                let mut sum = 0.0;
                for k in l..n {
                    s.a[i][k] /= scale;
                    sum += s.a[i][k] * s.a[i][k];
                }
                let f = s.a[i][l];
                s.g = -sign(sum.sqrt(), f);
                let h = f * s.g - sum;
                s.a[i][l] = f - s.g;
                for k in l..n {
                    s.rv1[k] = s.a[i][k] / h;
                }
                if i != m - 1 {
                    for j in l..m {
                        let dot: f64 = (l..n).map(|k| s.a[j][k] * s.a[i][k]).sum();
                        for k in l..n {
                            s.a[j][k] += dot * s.rv1[k];
                        }
                    }
                }
                for k in l..n {
                    s.a[i][k] *= scale;
                }
            }
        }
        s.anorm = s.anorm.max(s.w[i].abs() + s.rv1[i].abs());
    }
}
