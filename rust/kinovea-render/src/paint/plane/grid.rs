//! Owner: P3. Grid lines of a perspective plane (`DrawingPlane.DrawGrid`) and the segments that go
//! from plane units to the viewport (`DrawingPlane.DrawDistortedLine`).

use crate::paint::helpers::frame::{to_viewport, to_viewport_all};
use crate::paint::PaintContext;
use crate::renderer::{Pen, Renderer};
use kinovea_geometry::{PointF, ProjectiveMapper};

/// Tension of the cardinal curves that carry distorted lines (`DrawCurve` default).
pub const CURVE_TENSION: f32 = 0.5;
/// `tickMarkLengthFactor`: half length of the edge ticks relative to the plane size.
const TICK_FACTOR: f32 = 1.0 / 40.0;

/// The plane's grid: its size in plane units, its divisions and the plane-to-image mapping.
pub struct Grid<'a> {
    pub mapper: &'a ProjectiveMapper,
    pub width: f32,
    pub height: f32,
    pub cols: i32,
    pub rows: i32,
}

/// Port of `DrawingPlane.DrawGrid`: the full lattice, or without it the edges and their ticks.
pub fn draw_grid(
    r: &mut dyn Renderer,
    pen: &Pen,
    grid: &Grid,
    show_grid: bool,
    ctx: &PaintContext,
) {
    if show_grid {
        lattice(r, pen, grid, ctx);
    } else {
        edges_and_ticks(r, pen, grid, ctx);
    }
}

/// Port of `DrawDistortedLine`: a plane segment mapped to the image and then to the viewport. Under a
/// lens distortion the segment is a curve through the distorted points.
pub fn draw_distorted_line(
    r: &mut dyn Renderer,
    pen: &Pen,
    mapper: &ProjectiveMapper,
    a: PointF,
    b: PointF,
    ctx: &PaintContext,
) {
    let a = mapper.forward(a);
    let b = mapper.forward(b);
    let distortion = ctx.metadata.calibration.distortion();
    if distortion.is_initialized() {
        let curve = distortion.distort_line(distortion.distort(a), distortion.distort(b));
        r.stroke_curve(&to_viewport_all(ctx, &curve), CURVE_TENSION, pen);
    } else {
        r.stroke_line(to_viewport(ctx, a), to_viewport(ctx, b), pen);
    }
}

/// Horizontal lines at every row step, then vertical lines at every column step, both inclusive.
fn lattice(r: &mut dyn Renderer, pen: &Pen, grid: &Grid, ctx: &PaintContext) {
    let (w, h) = (grid.width, grid.height);
    let mut line = |a, b| draw_distorted_line(r, pen, grid.mapper, a, b, ctx);
    let step = h / grid.rows as f32;
    for i in 0..=grid.rows {
        let v = step * i as f32;
        line(PointF::new(0.0, v), PointF::new(w, v));
    }
    let step = w / grid.cols as f32;
    for i in 0..=grid.cols {
        let u = step * i as f32;
        line(PointF::new(u, 0.0), PointF::new(u, h));
    }
}

/// The four edges, then short ticks across the edges at each inner grid line.
fn edges_and_ticks(r: &mut dyn Renderer, pen: &Pen, grid: &Grid, ctx: &PaintContext) {
    let (w, h) = (grid.width, grid.height);
    let mut line = |a, b| draw_distorted_line(r, pen, grid.mapper, a, b, ctx);
    let origin = PointF::new(0.0, 0.0);
    line(origin, PointF::new(w, 0.0));
    line(PointF::new(0.0, h), PointF::new(w, h));
    line(origin, PointF::new(0.0, h));
    line(PointF::new(w, 0.0), PointF::new(w, h));

    let half_x = w * TICK_FACTOR;
    let step = h / grid.rows as f32;
    for i in 1..grid.rows {
        let v = step * i as f32;
        line(PointF::new(-half_x, v), PointF::new(half_x, v));
        line(PointF::new(w - half_x, v), PointF::new(w + half_x, v));
    }

    let half_y = h * TICK_FACTOR;
    let step = w / grid.cols as f32;
    for i in 1..grid.cols {
        let u = step * i as f32;
        line(PointF::new(u, -half_y), PointF::new(u, half_y));
        line(PointF::new(u, h - half_y), PointF::new(u, h + half_y));
    }
}
