//! Hit testing as geometry (`HitTester`, design D5): point boxes, widened paths and lines.
//! Tolerances are in viewport pixels, converted to image pixels with the viewport scale.

mod bezier;
#[cfg(test)]
mod bezier_test;
mod path;
#[cfg(test)]
mod path_test;
mod region;
#[cfg(test)]
mod region_test;

use crate::viewport::ImageToViewport;
use kinovea_geometry::{helpers::distance, PointF, RectangleF};

pub use path::GraphicsPath;

/// `HitTester.HitPoint`: a box of `max(1, untransform(10))` around `target`, right and bottom exclusive.
pub fn hit_point(p: PointF, target: PointF, viewport: &ImageToViewport) -> bool {
    let radius = viewport.untransform_distance(10).max(1) as f32;
    RectangleF::new(
        target.x - radius,
        target.y - radius,
        radius * 2.0,
        radius * 2.0,
    )
    .contains(p)
}

/// `HitTester.HitPath`: a filled path hits inside; a polyline hits within half of
/// `line_size + untransform(10)`. A zero-extent path hits a circle around its first point.
pub fn hit_path(
    p: PointF,
    path: &GraphicsPath,
    line_size: i32,
    area: bool,
    viewport: &ImageToViewport,
) -> bool {
    if area {
        return path.contains_even_odd(p);
    }
    let Some(first) = path.first_point() else {
        return false;
    };
    let enlarger = viewport.untransform_distance(10).max(1);
    let half_width = (line_size + enlarger) as f32 / 2.0;
    if path.is_degenerate() {
        // HitTester.cs:45-51: radius is the truncated max of 5 and the half width.
        let radius = half_width.max(5.0).trunc();
        return distance(p, first) <= radius;
    }
    path.distance_to(p) <= half_width
}

/// `HitTester.HitLine` without distortion: a segment with a line size of 1.
pub fn hit_line(p: PointF, a: PointF, b: PointF, viewport: &ImageToViewport) -> bool {
    if a == b {
        return false;
    }
    let mut path = GraphicsPath::new();
    path.add_line(a, b);
    hit_path(p, &path, 1, false, viewport)
}
