use crate::svd::svdcmp;

#[test]
fn identity_decomposes_to_orthogonal_factors_with_unit_values() {
    let eye: Vec<Vec<f64>> = (0..3)
        .map(|i| (0..3).map(|j| if i == j { 1.0 } else { 0.0 }).collect())
        .collect();
    let svd = svdcmp(&eye).unwrap();
    for w in &svd.w {
        assert!((w - 1.0).abs() < 1e-12);
    }
    for i in 0..3 {
        for j in 0..3 {
            let uu: f64 = (0..3).map(|k| svd.u[k][i] * svd.u[k][j]).sum();
            let vv: f64 = (0..3).map(|k| svd.v[k][i] * svd.v[k][j]).sum();
            let want = if i == j { 1.0 } else { 0.0 };
            assert!((uu - want).abs() < 1e-12 && (vv - want).abs() < 1e-12);
        }
    }
}
