//! Accumulation of the right (`V`) and left (`U`) transformations after bidiagonalization.

#![allow(clippy::needless_range_loop)]

use super::State;

/// Accumulates the right-hand transformations into `v`.
pub(crate) fn right(s: &mut State) {
    let n = s.n;
    for i in (0..n).rev() {
        if i < n - 1 {
            let l = s.l;
            if s.g != 0.0 {
                for j in l..n {
                    s.v[j][i] = (s.a[i][j] / s.a[i][l]) / s.g;
                }
                for j in l..n {
                    let dot: f64 = (l..n).map(|k| s.a[i][k] * s.v[k][j]).sum();
                    for k in l..n {
                        s.v[k][j] += dot * s.v[k][i];
                    }
                }
            }
            for j in l..n {
                s.v[i][j] = 0.0;
                s.v[j][i] = 0.0;
            }
        }
        s.v[i][i] = 1.0;
        s.g = s.rv1[i];
        s.l = i;
    }
}

/// Accumulates the left-hand transformations into `a`, which becomes `U`.
pub(crate) fn left(s: &mut State) {
    let (m, n) = (s.m, s.n);
    for i in (0..n).rev() {
        let l = i + 1;
        let mut g = s.w[i];
        if i < n - 1 {
            for j in l..n {
                s.a[i][j] = 0.0;
            }
        }
        if g != 0.0 {
            g = 1.0 / g;
            if i != n - 1 {
                for j in l..n {
                    let dot: f64 = (l..m).map(|k| s.a[k][i] * s.a[k][j]).sum();
                    let f = (dot / s.a[i][i]) * g;
                    for k in i..m {
                        s.a[k][j] += f * s.a[k][i];
                    }
                }
            }
            for j in i..m {
                s.a[j][i] *= g;
            }
        } else {
            for j in i..m {
                s.a[j][i] = 0.0;
            }
        }
        s.a[i][i] += 1.0;
    }
}
