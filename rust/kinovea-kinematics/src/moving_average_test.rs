use crate::moving_average::filter_samples;

#[test]
fn span_below_resolution_returns_the_input() {
    let s = vec![1.0, 5.0, 2.0, 8.0];
    assert_eq!(filter_samples(&s, 25.0, 40.0, 1), s);
    assert_eq!(filter_samples(&s, 50.0, 40.0, 1), s);
}

#[test]
fn window_is_span_over_twice_the_interval() {
    // 100 fps → 10 ms interval; 50 ms span → 2 samples each side.
    let mut s: Vec<f64> = (0..20).map(f64::from).collect();
    s[0] = f64::NAN;
    s[19] = f64::NAN;
    let out = filter_samples(&s, 100.0, 50.0, 1);
    assert_eq!(out.len(), 20);
    for (i, v) in out.iter().enumerate().take(18).skip(2) {
        assert!((v - i as f64).abs() < 1e-12, "{i}: {v}");
    }
    assert!(out[0].is_nan() || out[0].is_finite());
}

#[test]
fn padding_is_capped_by_the_sentinels_on_short_series() {
    let s = vec![f64::NAN, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, f64::NAN];
    let out = filter_samples(&s, 100.0, 40.0, 1);
    assert_eq!(out.len(), 8);
    assert!((out[3] - 3.0).abs() < 1e-12 && (out[4] - 4.0).abs() < 1e-12, "{out:?}");
}
