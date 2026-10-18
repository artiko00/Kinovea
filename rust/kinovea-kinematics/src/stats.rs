//! Residual statistics (`StatsHelper`) and the array subtraction extension.

/// Durbin-Watson statistic of residuals `e`: Σ(eᵢ₊₁ − eᵢ)² / Σeᵢ². NaN when all residuals are 0.
pub fn durbin_watson(e: &[f64]) -> f64 {
    let mut num = 0.0;
    let mut den = 0.0;
    for w in e.windows(2) {
        num += (w[1] - w[0]).powi(2);
        den += w[0].powi(2);
    }
    den += e.last().map_or(0.0, |last| last.powi(2));
    num / den
}

/// Element-wise `a − b`; panics when lengths differ, as the original throws.
pub fn subtract(a: &[f64], b: &[f64]) -> Vec<f64> {
    assert_eq!(a.len(), b.len(), "lists must have the same number of elements");
    a.iter().zip(b).map(|(x, y)| x - y).collect()
}
