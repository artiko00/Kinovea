//! QR diagonalization of the bidiagonal form (final phase of `svdcmp`).

#![allow(clippy::needless_range_loop)]

use super::{sign, State, SvdError};

pub(crate) fn pythag(a: f64, b: f64) -> f64 {
    let (at, bt) = (a.abs(), b.abs());
    if at > bt {
        let ct = bt / at;
        at * (1.0 + ct * ct).sqrt()
    } else if bt > 0.0 {
        let ct = at / bt;
        bt * (1.0 + ct * ct).sqrt()
    } else {
        0.0
    }
}

/// Finds the split point `l` for the sub-problem ending at `k`; `true` when a cancellation
/// step is needed.
fn split(s: &State, k: usize) -> (usize, bool) {
    let mut l = k;
    loop {
        if s.rv1[l].abs() + s.anorm == s.anorm || l == 0 {
            return (l, false);
        }
        if s.w[l - 1].abs() + s.anorm == s.anorm {
            return (l, true);
        }
        l -= 1;
    }
}

fn cancel(s: &mut State, l: usize, k: usize) {
    let nm = l - 1;
    let mut sn = 1.0;
    for i in l..=k {
        let f = sn * s.rv1[i];
        if f.abs() + s.anorm == s.anorm {
            continue;
        }
        let g = s.w[i];
        let h = pythag(f, g);
        s.w[i] = h;
        let h = 1.0 / h;
        let c = g * h;
        sn = -f * h;
        for j in 0..s.m {
            let (y, z) = (s.a[j][nm], s.a[j][i]);
            s.a[j][nm] = y * c + z * sn;
            s.a[j][i] = z * c - y * sn;
        }
    }
}

fn rotate(col: &mut [Vec<f64>], j: usize, i: usize, c: f64, sn: f64) {
    for row in col {
        let (x, z) = (row[j], row[i]);
        row[j] = x * c + z * sn;
        row[i] = z * c - x * sn;
    }
}

fn qr_step(s: &mut State, l: usize, k: usize) {
    let nm = k - 1;
    let (x, z) = (s.w[l], s.w[k]);
    let (y, g, h) = (s.w[nm], s.rv1[nm], s.rv1[k]);
    let mut f = ((y - z) * (y + z) + (g - h) * (g + h)) / (2.0 * h * y);
    let g = pythag(f, 1.0);
    f = ((x - z) * (x + z) + h * ((y / (f + sign(g, f))) - h)) / x;
    let (mut c, mut sn, mut x) = (1.0, 1.0, x);
    for j in l..=nm {
        let i = j + 1;
        let (mut g, mut y) = (s.rv1[i], s.w[i]);
        let mut h = sn * g;
        g *= c;
        let mut z = pythag(f, h);
        s.rv1[j] = z;
        c = f / z;
        sn = h / z;
        f = x * c + g * sn;
        g = g * c - x * sn;
        h = y * sn;
        y *= c;
        rotate(&mut s.v, j, i, c, sn);
        z = pythag(f, h);
        s.w[j] = z;
        if z != 0.0 {
            z = 1.0 / z;
            c = f * z;
            sn = h * z;
        }
        f = c * g + sn * y;
        x = c * y - sn * g;
        rotate(&mut s.a, j, i, c, sn);
    }
    s.rv1[l] = 0.0;
    s.rv1[k] = f;
    s.w[k] = x;
}

pub(crate) fn run(s: &mut State) -> Result<(), SvdError> {
    for k in (0..s.n).rev() {
        for its in 1..=30 {
            let (l, needs_cancel) = split(s, k);
            if needs_cancel {
                cancel(s, l, k);
            }
            if l == k {
                if s.w[k] < 0.0 {
                    s.w[k] = -s.w[k];
                    for row in &mut s.v {
                        row[k] = -row[k];
                    }
                }
                break;
            }
            if its == 30 {
                return Err(SvdError::NoConvergence);
            }
            qr_step(s, l, k);
        }
    }
    Ok(())
}
