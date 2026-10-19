//! Sub-pixel refinement of a correlation peak as `RefineLocationParabola` in
//! `TrackerTemplateMatching.cs` does it: per axis, the vertex of the parabola through the peak and
//! its two neighbours. That is the degree-2 MathNet fit on samples at -1, 0 and 1, in closed form.

use crate::ncc::Map;

/// Vertex offset of the parabola through `(-1, a)`, `(0, b)`, `(1, c)`:
/// `(a - c) / (2 (a - 2b + c))`. `None` when the samples are collinear (no vertex).
pub fn vertex_offset(a: f64, b: f64, c: f64) -> Option<f64> {
    let curvature = a - 2.0 * b + c;
    (curvature != 0.0).then(|| (a - c) / (2.0 * curvature))
}

/// Sub-pixel offset `(dx, dy)` of the peak at `(x, y)` with respect to the integer location.
/// `(0.0, 0.0)` when the peak is on the border or equals 1.0, where the original skips the
/// refinement, and when a neighbourhood is collinear, where no vertex exists.
pub fn peak_offset(map: &Map, x: usize, y: usize) -> (f32, f32) {
    let centre = map.at(x, y);
    let on_border = x == 0 || y == 0 || x + 1 == map.width || y + 1 == map.height;
    if on_border || centre == 1.0 {
        return (0.0, 0.0);
    }
    let axis = |before: f32, after: f32| {
        vertex_offset(f64::from(before), f64::from(centre), f64::from(after)).unwrap_or(0.0) as f32
    };
    (
        axis(map.at(x - 1, y), map.at(x + 1, y)),
        axis(map.at(x, y - 1), map.at(x, y + 1)),
    )
}
