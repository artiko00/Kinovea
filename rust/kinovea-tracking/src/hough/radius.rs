//! Radius estimate of `HoughCircleEstimateRadiusInvoker` (OpenCV `hough.cpp`): a histogram of the
//! edge distances in 0.1 steps, read through one-pixel windows. The window with the best
//! support per radius wins.

#[cfg(test)]
#[path = "radius_test.rs"]
mod radius_test;

/// `nBinsPerDr` of OpenCV.
const BINS_PER_DR: f32 = 10.0;

/// Radius and support of the best window, or `None` when the support does not exceed `threshold`.
pub fn estimate_radius(
    points: &[(usize, usize)],
    center: (f32, f32),
    (min_r, max_r): (i32, i32),
    dr: f32,
    threshold: i32,
) -> Option<(f32, i32)> {
    let (min2, max2) = (
        (min_r as f32) * (min_r as f32),
        (max_r as f32) * (max_r as f32),
    );
    let n_bins = ((max_r - min_r) as f32 / dr * BINS_PER_DR).round_ties_even() as i32;
    if n_bins < 1 {
        return None;
    }
    let mut bins = vec![0i32; n_bins as usize];
    for &(x, y) in points {
        let (dx, dy) = (center.0 - x as f32, center.1 - y as f32);
        let r2 = dx * dx + dy * dy;
        if min2 <= r2 && r2 <= max2 {
            let d = r2.sqrt();
            let bin = (((d - min_r as f32) / dr * BINS_PER_DR).round_ties_even() as i32)
                .clamp(0, n_bins - 1);
            bins[bin as usize] += 1;
        }
    }
    let (mut best, mut max_count) = (0f32, 0i32);
    let mut j = n_bins - 1;
    while j > 0 {
        if bins[j as usize] != 0 {
            let upbin = j;
            let mut count = 0;
            while j > upbin - BINS_PER_DR as i32 && j >= 0 {
                count += bins[j as usize];
                j -= 1;
            }
            let r = ((upbin + j) as f32 / 2.0) / BINS_PER_DR * dr + min_r as f32;
            if count as f32 * best >= max_count as f32 * r
                || (best < f32::EPSILON && count >= max_count)
            {
                best = r;
                max_count = count;
            }
        }
        j -= 1;
    }
    (max_count > threshold).then_some((best, max_count))
}
