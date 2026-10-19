//! Gradient Hough circles at a radius range, ported from OpenCV `HoughCirclesGradient`
//! (`cv::HoughCircles` with `HOUGH_GRADIENT`, OpenCV 4.x `modules/imgproc/src/hough.cpp`).
//! It is the detector that `TrackerCircle.FindBestCircleAtRadius` calls with `dp = 1`,
//! `minDist = max(cols / 16, 8)`, `param1 = 50` and a descending `param2`.
//!
//! Steps: Sobel 3x3 with replicated borders, Canny with `param1 / 2` and `param1`, votes along
//! both gradient directions for each radius, strict local maxima above `param2`, radius from the
//! edge distances, and greedy suppression of centers closer than `min_dist`.

#[cfg(test)]
#[path = "hough_test.rs"]
mod hough_test;

mod accum;
mod canny;
mod gradient;
mod radius;

/// Parameters with the names of `cv::HoughCircles`.
pub struct HoughParams {
    /// Inverse ratio of the accumulator resolution (`dp`), clamped to at least 1.
    pub dp: f32,
    /// Minimum distance between two returned centers (`minDist`).
    pub min_dist: f32,
    /// High Canny threshold (`param1`); the low one is half of it.
    pub canny_threshold: i32,
    /// Vote threshold (`param2`): a center and its radius support must be strictly above it.
    pub acc_threshold: i32,
    pub min_radius: i32,
    /// Not above `min_radius`: the range becomes `min_radius + 2`, as OpenCV does.
    pub max_radius: i32,
}

/// A detected circle. The center is `(cell + 0.5) * dp`, the convention of OpenCV.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HoughCircle {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}

/// Circles of a row-major gray image, most supported first. Empty when nothing passes.
pub fn hough_circles(
    data: &[u8],
    width: usize,
    height: usize,
    params: &HoughParams,
) -> Vec<HoughCircle> {
    let dp = params.dp.max(1.0);
    let min_r = params.min_radius.max(0);
    let max_r = if params.max_radius <= min_r {
        min_r + 2
    } else {
        params.max_radius
    };
    let grad = gradient::sobel(data, width, height);
    let low = (params.canny_threshold / 2).max(1);
    let edges = canny::canny(&grad, low, params.canny_threshold);
    let points = edge_points(&edges, width);
    if points.is_empty() {
        return Vec::new();
    }
    let acc = accum::vote(&grad, &edges, 1.0 / dp, min_r, max_r);
    let mut found: Vec<(HoughCircle, i32)> = accum::centers(&acc, params.acc_threshold)
        .into_iter()
        .filter_map(|(x, y)| {
            let (cx, cy) = ((x as f32 + 0.5) * dp, (y as f32 + 0.5) * dp);
            let fit = radius::estimate_radius(
                &points,
                (cx, cy),
                (min_r, max_r),
                dp,
                params.acc_threshold,
            );
            fit.map(|(r, support)| {
                (
                    HoughCircle {
                        x: cx,
                        y: cy,
                        radius: r,
                    },
                    support,
                )
            })
        })
        .collect();
    found.sort_by(|(a, sa), (b, sb)| {
        sb.cmp(sa)
            .then(b.radius.total_cmp(&a.radius))
            .then(a.x.total_cmp(&b.x))
            .then(a.y.total_cmp(&b.y))
    });
    remove_overlaps(found, params.min_dist)
}

fn edge_points(edges: &[bool], width: usize) -> Vec<(usize, usize)> {
    edges
        .iter()
        .enumerate()
        .filter_map(|(i, &e)| e.then_some((i % width, i / width)))
        .collect()
}

/// Keeps a circle only when its center is at least `min_dist` from every kept one (`RemoveOverlaps`).
fn remove_overlaps(found: Vec<(HoughCircle, i32)>, min_dist: f32) -> Vec<HoughCircle> {
    let min2 = min_dist * min_dist;
    let mut kept: Vec<HoughCircle> = Vec::new();
    for (c, _) in found {
        let far = kept.iter().all(|k| {
            let (dx, dy) = (k.x - c.x, k.y - c.y);
            dx * dx + dy * dy >= min2
        });
        if far {
            kept.push(c);
        }
    }
    kept
}
