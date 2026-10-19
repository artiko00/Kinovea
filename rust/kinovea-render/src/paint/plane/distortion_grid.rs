//! Owner: P3. Port of `DrawingDistortionGrid.Draw`: the lattice of points over a distorted image.

use super::grid::CURVE_TENSION;
use crate::paint::helpers::frame::{opacity, point_box_f, to_viewport_all};
use crate::paint::PaintContext;
use crate::renderer::Renderer;
use crate::style::StyleData;
use kinovea_geometry::PointF;
use kinovea_metadata::drawing::DistortionGrid;

/// Radius of the ring drawn around each point, in pixels.
const RING_RADIUS: i32 = 4;

/// Port of `DrawingDistortionGrid.Draw`. The points come row by row; the side of the lattice is the
/// integer square root of their count, as the original derives its subdivisions from them.
pub fn paint_distortion_grid(d: &DistortionGrid, ctx: &PaintContext, r: &mut dyn Renderer) {
    let factor = opacity(&d.header.fading, ctx);
    if factor <= 0.0 {
        return;
    }
    let style = StyleData::from_elements(&d.header.style);
    let pen = style.pen((factor * 255.0) as i32, 1.0);
    let screen = to_viewport_all(ctx, &d.points);
    let side = (screen.len() as f64).sqrt().floor() as usize;

    for row in 0..side {
        r.stroke_curve(&screen[row * side..(row + 1) * side], CURVE_TENSION, &pen);
    }
    for col in 0..side {
        let column: Vec<PointF> = (0..side).map(|row| screen[row * side + col]).collect();
        r.stroke_curve(&column, CURVE_TENSION, &pen);
    }
    for p in &screen {
        r.stroke_ellipse(point_box_f(*p, RING_RADIUS), &pen);
    }
}
