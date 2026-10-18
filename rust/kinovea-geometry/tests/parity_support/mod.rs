//! CSV helpers shared by the parity tests.

// Each integration test compiles this module separately and uses a subset of it.
#![allow(dead_code)]

mod mod_test;

use kinovea_fixtures::fixtures_root;
use kinovea_geometry::PointF;

pub fn rows(file: &str) -> Vec<Vec<String>> {
    let path = fixtures_root().join("geometry").join(file);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    text.lines()
        .skip(1)
        .filter(|l| !l.is_empty())
        .map(|l| l.split(',').map(str::to_owned).collect())
        .collect()
}

pub fn f(s: &str) -> f64 {
    s.parse().unwrap_or_else(|e| panic!("{s:?}: {e}"))
}

pub fn list(s: &str) -> Vec<f64> {
    s.split(' ').map(f).collect()
}

pub fn pt(r: &[String], at: usize) -> PointF {
    PointF::new(f(&r[at]) as f32, f(&r[at + 1]) as f32)
}

/// Largest magnitude among the expected values, so tolerances scale with single precision.
pub fn scale(expected: &[String]) -> f64 {
    expected.iter().map(|s| f(s).abs()).fold(1.0, f64::max)
}

/// Absolute comparison; a NaN expectation requires a NaN result.
pub fn assert_f(got: f64, want: f64, tol: f64, ctx: &str) {
    if want.is_nan() {
        assert!(got.is_nan(), "{ctx}: got {got}, want NaN");
        return;
    }
    assert!(
        kinovea_fixtures::approx_eq(got, want, tol),
        "{ctx}: got {got}, want {want}"
    );
}

pub fn assert_pt(got: PointF, want: PointF, tol: f64, ctx: &str) {
    assert_f(
        f64::from(got.x),
        f64::from(want.x),
        tol,
        &format!("{ctx} x"),
    );
    assert_f(
        f64::from(got.y),
        f64::from(want.y),
        tol,
        &format!("{ctx} y"),
    );
}
