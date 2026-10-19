//! `HitTest` of the grids: the plane (`DrawingPlane`) and the distortion grid (`DrawingDistortionGrid`).

use super::common::{opacity, toggle_on, trackable_opacity};
use super::context::{Hit, HitContext};
use super::{hit_line, hit_point};
use kinovea_geometry::{PointF, ProjectiveMapper, Quadrilateral, RectangleF};
use kinovea_metadata::drawing::{DistortionGrid, Plane};

/// `planeWidth` and `planeHeight`: the plane is a 100 by 100 square, mapped onto the image quad.
const PLANE_SIZE: f32 = 100.0;

/// `DrawingPlane.HitTest` (DrawingPlane.cs:395-443). Handles 5 and 6 are the sliding lines, 1-4 the
/// corners. The body only counts while the user is tracking the grid, which is live UI state, so
/// it is never hit headlessly.
pub fn hit_plane(d: &Plane, p: PointF, ctx: &HitContext) -> Hit {
    if trackable_opacity(&d.header, ctx) <= 0.0 {
        return Hit::Miss;
    }
    if let Some(m) = grid_mapping(d) {
        if d.show_x_line {
            let h = PLANE_SIZE * d.x_line_coord;
            let a = m.forward(PointF::new(h, 0.0));
            let b = m.forward(PointF::new(h, PLANE_SIZE));
            if hit_line(p, a, b, ctx.viewport) {
                return Hit::Handle(5);
            }
        }
        if d.show_y_line {
            let v = PLANE_SIZE * d.y_line_coord;
            let a = m.forward(PointF::new(0.0, v));
            let b = m.forward(PointF::new(PLANE_SIZE, v));
            if hit_line(p, a, b, ctx.viewport) {
                return Hit::Handle(6);
            }
        }
    }
    match d
        .quad
        .to_array()
        .iter()
        .position(|c| hit_point(p, *c, ctx.viewport))
    {
        Some(i) => Hit::Handle(i as u32 + 1),
        None => Hit::Miss,
    }
}

/// `DrawingPlane.Draw` keeps `projectiveMapping` only when it is not `drawEdgesOnly`. Without it the
/// original throws on a sliding line; this port skips the sliding lines instead.
fn grid_mapping(d: &Plane) -> Option<ProjectiveMapper> {
    let perspective = toggle_on(&d.header, "perspective");
    let convex = perspective || d.quad.is_convex();
    let edges_only = !convex || (!perspective && !d.quad.is_axis_aligned_rectangle());
    if edges_only {
        return None;
    }
    let plane = Quadrilateral::from_rect(RectangleF::new(0.0, 0.0, PLANE_SIZE, PLANE_SIZE));
    ProjectiveMapper::new(&plane, &d.quad).ok()
}

/// `DrawingDistortionGrid.HitTest` (DrawingDistortionGrid.cs:187-200): the first point whose box
/// holds `p` is handle `i + 1`. There is no body.
pub fn hit_distortion_grid(d: &DistortionGrid, p: PointF, ctx: &HitContext) -> Hit {
    if opacity(&d.header, ctx) <= 0.0 {
        return Hit::Miss;
    }
    match d.points.iter().position(|c| hit_point(p, *c, ctx.viewport)) {
        Some(i) => Hit::Handle(i as u32 + 1),
        None => Hit::Miss,
    }
}
