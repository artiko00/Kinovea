use super::{assert_f, list, scale};

#[test]
fn scale_is_the_largest_magnitude_with_a_floor_of_one() {
    let cols = ["-0.5".to_string(), "250.25".to_string()];
    assert_eq!(scale(&cols), 250.25);
    assert_eq!(scale(&["0.1".to_string()]), 1.0);
    assert_eq!(list("1 -2.5 3E2"), vec![1.0, -2.5, 300.0]);
}

#[test]
fn nan_expectations_require_nan_results() {
    assert_f(f64::NAN, f64::NAN, 0.0, "nan");
    assert_f(1.0, 1.0005, 1e-3, "close");
}

#[test]
#[should_panic(expected = "want NaN")]
fn a_number_does_not_satisfy_a_nan_expectation() {
    assert_f(1.0, f64::NAN, 0.0, "nan");
}
