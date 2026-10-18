use crate::lib_test::Rng;
use crate::svd::{svdcmp, SvdError};

fn seeded(m: usize, n: usize, seed: u64) -> Vec<Vec<f64>> {
    let mut rng = Rng::new(seed);
    (0..m)
        .map(|_| (0..n).map(|_| f64::from(rng.range(-5.0, 5.0))).collect())
        .collect()
}

fn reconstruct(u: &[Vec<f64>], w: &[f64], v: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let (m, n) = (u.len(), w.len());
    (0..m)
        .map(|i| {
            (0..n)
                .map(|j| (0..n).map(|k| u[i][k] * w[k] * v[j][k]).sum())
                .collect()
        })
        .collect()
}

fn assert_orthonormal_columns(x: &[Vec<f64>]) {
    let n = x[0].len();
    for i in 0..n {
        for j in 0..n {
            let dot: f64 = x.iter().map(|r| r[i] * r[j]).sum();
            let want = if i == j { 1.0 } else { 0.0 };
            assert!((dot - want).abs() < 1e-9, "column {i}·{j} = {dot}");
        }
    }
}

#[test]
fn reconstruction_and_orthonormality_on_a_seeded_5x3_matrix() {
    let a = seeded(5, 3, 42);
    let svd = svdcmp(&a).unwrap();
    assert_eq!(svd.w.len(), 3);
    assert!(svd.w.iter().all(|w| *w >= 0.0));
    let back = reconstruct(&svd.u, &svd.w, &svd.v);
    for (ra, rb) in a.iter().zip(&back) {
        for (x, y) in ra.iter().zip(rb) {
            assert!((x - y).abs() < 1e-9, "{x} vs {y}");
        }
    }
    assert_orthonormal_columns(&svd.u);
    assert_orthonormal_columns(&svd.v);
}

#[test]
fn square_and_rank_deficient_matrices_decompose() {
    let a = seeded(4, 4, 3);
    let svd = svdcmp(&a).unwrap();
    assert_orthonormal_columns(&svd.v);
    let deficient = vec![vec![1.0, 2.0], vec![2.0, 4.0], vec![3.0, 6.0]];
    let svd = svdcmp(&deficient).unwrap();
    let mut w = svd.w.clone();
    w.sort_by(f64::total_cmp);
    assert!(w[0].abs() < 1e-12);
    assert!((w[1] - 70f64.sqrt()).abs() < 1e-9);
}

#[test]
fn more_columns_than_rows_is_rejected() {
    let wide = vec![vec![1.0, 2.0, 3.0]];
    assert_eq!(svdcmp(&wide), Err(SvdError::RowsLessThanColumns));
    assert_eq!(
        SvdError::NoConvergence.to_string(),
        "no convergence in 30 svdcmp iterations"
    );
}
