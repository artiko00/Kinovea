//! Iterative inverse of the distortion model, as OpenCV `undistortPoints` runs it without
//! termination criteria: 5 fixed-point iterations on normalized coordinates.

use super::DistortionParameters;

pub const ITERATIONS: usize = 5;

/// Takes a distorted point in normalized camera coordinates and returns the rectified one.
pub fn undistort_normalized(p: &DistortionParameters, x0: f64, y0: f64) -> (f64, f64) {
    let (mut x, mut y) = (x0, y0);
    for _ in 0..ITERATIONS {
        let r2 = x * x + y * y;
        let icdist = 1.0 / (1.0 + ((p.k3 * r2 + p.k2) * r2 + p.k1) * r2);
        if icdist < 0.0 {
            // OpenCV guards against a negative radial factor by restarting from the input.
            return (x0, y0);
        }
        let delta_x = 2.0 * p.p1 * x * y + p.p2 * (r2 + 2.0 * x * x);
        let delta_y = p.p1 * (r2 + 2.0 * y * y) + 2.0 * p.p2 * x * y;
        x = (x0 - delta_x) * icdist;
        y = (y0 - delta_y) * icdist;
    }
    (x, y)
}

/// Forward model on normalized coordinates: radial `k1..k3` and tangential `p1, p2`.
pub fn distort_normalized(p: &DistortionParameters, x: f64, y: f64) -> (f64, f64) {
    let r2 = x * x + y * y;
    let radial = 1.0 + p.k1 * r2 + p.k2 * r2 * r2 + p.k3 * r2 * r2 * r2;
    let xd = x * radial + (2.0 * p.p1 * x * y + p.p2 * (r2 + 2.0 * x * x));
    let yd = y * radial + (p.p1 * (r2 + 2.0 * y * y) + 2.0 * p.p2 * x * y);
    (xd, yd)
}
