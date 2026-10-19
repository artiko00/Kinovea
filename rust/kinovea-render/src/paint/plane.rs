//! Owner: P3. Painters ported from `DrawingPlane.Draw` and `DrawingDistortionGrid.Draw`.

pub mod distortion_grid;
#[cfg(test)]
mod distortion_grid_test;
pub mod grid;
#[cfg(test)]
mod grid_test;
pub mod sliding;
#[cfg(test)]
mod sliding_test;
pub mod tick_mark;
#[cfg(test)]
mod tick_mark_test;

pub use distortion_grid::paint_distortion_grid;

use super::helpers::frame::{opacity, point_box_f, to_viewport};
use super::PaintContext;
use crate::renderer::{Pen, Renderer};
use crate::style::colors::alpha_of;
use crate::style::StyleData;
use grid::{draw_grid, Grid};
use kinovea_geometry::{PointF, ProjectiveMapper, Quadrilateral, RectangleF, SizeF};
use kinovea_metadata::drawing::Plane;
use kinovea_metadata::{StyleElements, StyleValue};
use sliding::draw_sliding_lines;

/// Half length of a handle arm, in pixels.
const HANDLE_ARM: f32 = 4.0;
/// Radius of the ring around the origin of the calibration drawing.
const ORIGIN_RADIUS: i32 = 5;
/// Plane size in plane units when the drawing is not the calibration drawing.
const DEFAULT_SIZE: f32 = 100.0;
/// Decimal places of the tick labels: the default of `PlayerPreferences.decimalPlaces`
/// (`Kinovea.Services/Preferences/PlayerPreferences.cs:324`), used by the oracle. Replaced by the
/// preference once the paint context carries it.
pub const DEFAULT_DECIMAL_PLACES: i32 = 2;

/// Port of `DrawingPlane.Draw`: handles, origin ring, then either the grid with the sliding lines or,
/// when `drawEdgesOnly`, the four edges.
pub fn paint_plane(d: &Plane, ctx: &PaintContext, r: &mut dyn Renderer) {
    let factor = opacity(&d.header.fading, ctx);
    if factor <= 0.0 {
        return;
    }
    let style = StyleData::from_elements(&d.header.style);
    let pen = style.pen(alpha_of(factor as f32), 1.0);
    let vertices = d.quad.vertices.map(|v| to_viewport(ctx, v));

    draw_handles(r, &pen, &vertices);
    if ctx.metadata.calibration.drawing_id() == d.header.id {
        r.stroke_ellipse(point_box_f(vertices[3], ORIGIN_RADIUS), &pen);
    }
    if edges_only(d, perspective(&d.header.style)) {
        draw_edges(r, &pen, &vertices);
    } else {
        draw_plane_grid(r, &pen, d, ctx);
    }
}

/// `drawEdgesOnly`: a non-convex quadrilateral, or a non-rectangle when the plane is not in perspective.
fn edges_only(d: &Plane, perspective: bool) -> bool {
    !d.quad.is_convex() || (!perspective && !d.quad.is_axis_aligned_rectangle())
}

/// The grid and the sliding lines, mapped from the plane to the image. Under a known lens distortion
/// the image corners are undistorted first, as the original does.
fn draw_plane_grid(r: &mut dyn Renderer, pen: &Pen, d: &Plane, ctx: &PaintContext) {
    let size = plane_size(d, ctx);
    let distortion = ctx.metadata.calibration.distortion();
    let image = if distortion.is_initialized() {
        distortion.undistort_quad(&d.quad)
    } else {
        d.quad
    };
    let plane = Quadrilateral::from_rect(RectangleF::new(0.0, 0.0, size.width, size.height));
    let Ok(mapper) = ProjectiveMapper::new(&plane, &image) else {
        return;
    };
    let grid = Grid {
        mapper: &mapper,
        width: size.width,
        height: size.height,
        cols: int_element(&d.header.style, "cols", 10),
        rows: int_element(&d.header.style, "rows", 10),
    };
    draw_grid(r, pen, &grid, d.show_grid, ctx);
    draw_sliding_lines(r, pen, &grid, d, ctx, DEFAULT_DECIMAL_PLACES);
}

/// The calibration rectangle for the calibration drawing (`CalibrationByPlane_GetRectangleSize`),
/// otherwise the default 100 x 100.
fn plane_size(d: &Plane, ctx: &PaintContext) -> SizeF {
    let calibration = &ctx.metadata.calibration;
    if calibration.drawing_id() == d.header.id {
        calibration.plane_size()
    } else {
        SizeF::new(DEFAULT_SIZE, DEFAULT_SIZE)
    }
}

/// `DrawHandles` of the four vertices: a plus sign of arms `HANDLE_ARM` at each one.
fn draw_handles(r: &mut dyn Renderer, pen: &Pen, vertices: &[PointF; 4]) {
    for p in vertices {
        r.stroke_line(
            PointF::new(p.x - HANDLE_ARM, p.y),
            PointF::new(p.x + HANDLE_ARM, p.y),
            pen,
        );
        r.stroke_line(
            PointF::new(p.x, p.y - HANDLE_ARM),
            PointF::new(p.x, p.y + HANDLE_ARM),
            pen,
        );
    }
}

/// The edges A-B, B-C, C-D and D-A in viewport coordinates.
fn draw_edges(r: &mut dyn Renderer, pen: &Pen, vertices: &[PointF; 4]) {
    for i in 0..4 {
        r.stroke_line(vertices[i], vertices[(i + 1) % 4], pen);
    }
}

/// The `perspective` toggle of the style; on by default.
fn perspective(style: &StyleElements) -> bool {
    match style.get("perspective") {
        Some(StyleValue::Toggle { value, .. }) => *value,
        _ => true,
    }
}

/// An integer style element (`cols`, `rows`).
fn int_element(style: &StyleElements, key: &str, default: i32) -> i32 {
    match style.get(key) {
        Some(StyleValue::Int { value, .. }) => *value,
        _ => default,
    }
}
