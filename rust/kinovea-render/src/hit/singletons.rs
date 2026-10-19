//! `HitTest` of the single-instance singletons: the coordinate system and the test grid.

use super::context::{Hit, HitContext};
use super::{hit_line, hit_path, hit_point, GraphicsPath};
use crate::paint::singletons::{find as find_grid, image_lines, GridLine};
use kinovea_geometry::{PointF, SizeF};
use kinovea_metadata::singletons::{CoordinateSystem, TestGrid};

/// `DrawingCoordinateSystem.HitTest` (DrawingCoordinateSystem.cs:249-275): the origin is handle 1,
/// the horizontal axis 2 and the vertical axis 3. Nothing is hit while hidden. Lines are hit without
/// distortion, as `hit_line` does.
pub fn hit_coordinate_system(d: &CoordinateSystem, p: PointF, ctx: &HitContext) -> Hit {
    if !d.visible {
        return Hit::Miss;
    }
    if hit_point(p, d.origin, ctx.viewport) {
        return Hit::Handle(1);
    }
    let grid = find_grid(&ctx.metadata.calibration);
    let on =
        |axis: Option<GridLine>| axis.is_some_and(|a| hit_line(p, a.start, a.end, ctx.viewport));
    if on(grid.horizontal_axis) {
        return Hit::Handle(2);
    }
    if on(grid.vertical_axis) {
        return Hit::Handle(3);
    }
    Hit::Miss
}

/// `DrawingTestGrid.HitTest` (DrawingTestGrid.cs:197-242): the enabled lines, each a closed figure of
/// two points, widened by 2 px. The grid is never moved, so its only answer is the body.
pub fn hit_test_grid(d: &TestGrid, p: PointF, ctx: &HitContext) -> Hit {
    if !d.visible {
        return Hit::Miss;
    }
    let image = ctx.metadata.video.image_size;
    let size = SizeF::new(image.width as f32, image.height as f32);
    let mut path = GraphicsPath::new();
    for (a, b) in image_lines(d, size) {
        path.add_polygon(&[a, b]);
    }
    if hit_path(p, &path, 2, false, ctx.viewport) {
        Hit::Body
    } else {
        Hit::Miss
    }
}
