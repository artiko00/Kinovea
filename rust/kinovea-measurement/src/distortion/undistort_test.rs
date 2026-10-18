use crate::distortion::undistort::{distort_normalized, undistort_normalized};
use crate::distortion::DistortionParameters;

fn radial() -> DistortionParameters {
    DistortionParameters::new(-0.2, 0.05, 0.0, 0.0, 0.0, 1000.0, 1000.0, 960.0, 540.0, 300.0)
}

#[test]
fn five_iterations_invert_a_pure_radial_distortion_closely() {
    let p = radial();
    for (x, y) in [(0.1, 0.05), (-0.3, 0.2), (0.45, -0.4)] {
        let (xd, yd) = distort_normalized(&p, x, y);
        let (xu, yu) = undistort_normalized(&p, xd, yd);
        assert!((xu - x).abs() < 1e-4 && (yu - y).abs() < 1e-4, "({x}, {y}) -> ({xu}, {yu})");
    }
}

#[test]
fn no_coefficients_is_the_identity() {
    let p = DistortionParameters::new(0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0);
    assert_eq!(undistort_normalized(&p, 0.3, -0.7), (0.3, -0.7));
    assert_eq!(distort_normalized(&p, 0.3, -0.7), (0.3, -0.7));
}

#[test]
fn negative_radial_factor_restarts_from_the_input() {
    let p = DistortionParameters::new(-5.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0);
    assert_eq!(undistort_normalized(&p, 1.0, 1.0), (1.0, 1.0));
}
