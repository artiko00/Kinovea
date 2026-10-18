use crate::stats::{durbin_watson, subtract};

#[test]
fn durbin_watson_of_zero_residuals_is_nan() {
    assert!(durbin_watson(&[0.0, 0.0, 0.0]).is_nan());
}

#[test]
fn durbin_watson_of_alternating_residuals_follows_the_formula() {
    // num = 3 · (±2)² = 12, den = 4 · 1 = 4.
    assert_eq!(durbin_watson(&[1.0, -1.0, 1.0, -1.0]), 3.0);
    // Uncorrelated-looking: num = 1 + 1 = 2... den = 1 + 0 + 1 = 2.
    assert_eq!(durbin_watson(&[1.0, 0.0, 1.0]), 1.0);
    assert_eq!(durbin_watson(&[2.0]), 0.0);
}

#[test]
fn subtract_is_element_wise() {
    assert_eq!(subtract(&[3.0, 2.0], &[1.0, 5.0]), vec![2.0, -3.0]);
}

#[test]
#[should_panic(expected = "same number of elements")]
fn subtract_rejects_different_lengths() {
    subtract(&[1.0], &[1.0, 2.0]);
}
