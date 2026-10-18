use crate::butterworth::{coefficients, correction_factor, filter_samples, two_pass, TooFewSamples};

#[test]
fn rejects_ten_samples_or_fewer() {
    assert_eq!(filter_samples(&[1.0; 10], 100.0, 100), Err(TooFewSamples));
    assert_eq!(TooFewSamples.to_string(), "number of samples must be superior to 10");
}

#[test]
fn coefficients_match_the_hand_computed_values() {
    // fs = 100, fc = 10, two passes: C = (2^(1/2) − 1)^(1/4), ω = tan(π/10)/C.
    let c = correction_factor(2);
    assert!((c - 0.802_243).abs() < 1e-6, "{c}");
    let o = (std::f64::consts::PI * 0.1).tan() / c;
    let k = coefficients(100.0, 10.0, c);
    let a0 = o * o / (1.0 + 2f64.sqrt() * o + o * o);
    assert!((k.a0 - a0).abs() < 1e-15);
    assert_eq!(k.a1, 2.0 * k.a0);
    assert_eq!(k.a2, k.a0);
    assert!((k.a0 + k.a1 + k.a2 + k.b1 + k.b2 - 1.0).abs() < 1e-12, "unit DC gain");
}

#[test]
fn a_constant_signal_passes_unchanged_and_best_index_is_reported() {
    let c = correction_factor(2);
    let flat = vec![5.0; 30];
    for v in two_pass(&flat, coefficients(100.0, 10.0, c)) {
        assert!((v - 5.0).abs() < 1e-12);
    }
    let noisy: Vec<f64> = (0..40).map(|i| (i as f64 * 0.7).sin() + if i % 2 == 0 { 0.1 } else { -0.1 }).collect();
    let f = filter_samples(&noisy, 100.0, 100).unwrap();
    // The accumulating `fc += step` loop of the original yields 100 or 101 cutoffs.
    assert!(f.results.len() == 100 || f.results.len() == 101, "{}", f.results.len());
    assert!((f.results[0].cutoff_frequency - 0.5).abs() < 1e-12);
    let best = f.best_cutoff_index.unwrap();
    assert!(f.results.iter().all(|r| r.durbin_watson >= f.results[best].durbin_watson));
    assert_eq!(f.results[best].data.len(), 40);
}

#[test]
fn eleven_samples_are_accepted_and_every_result_keeps_the_input_length() {
    let samples: Vec<f64> = (0..11).map(|i| f64::from(i % 3)).collect();
    let f = filter_samples(&samples, 50.0, 10).unwrap();
    assert!(!f.results.is_empty());
    assert!(f.results.iter().all(|r| r.data.len() == 11));
    assert!(f.results.windows(2).all(|w| w[1].cutoff_frequency > w[0].cutoff_frequency));
}
