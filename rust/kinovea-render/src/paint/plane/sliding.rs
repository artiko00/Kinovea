//! Owner: P3. Sliding lines of the calibration plane (`DrawingPlane.DrawSlidingLines`) with their
//! graduation ticks (`UpdateTickMarks`, `AddTickMarks`), computed from the current state.

use super::grid::{draw_distorted_line, Grid};
use super::tick_mark::{draw_ticks, Alignment, TickMark};
use crate::paint::PaintContext;
use crate::renderer::{Pen, Renderer};
use kinovea_geometry::PointF;
use kinovea_measurement::CalibratorPlane;
use kinovea_metadata::drawing::Plane;

/// Sliding lines closer than this to an end hide the end tick on that side.
const NEAR_START: f32 = 0.1;
const NEAR_END: f32 = 0.9;

/// Port of `DrawingPlane.DrawSlidingLines`. Only the calibration drawing shows them.
pub fn draw_sliding_lines(
    r: &mut dyn Renderer,
    pen: &Pen,
    grid: &Grid,
    d: &Plane,
    ctx: &PaintContext,
    decimal_places: i32,
) {
    if ctx.metadata.calibration.drawing_id() != d.header.id {
        return;
    }
    draw_lines(r, pen, grid, d, ctx);
    if !d.show_x_line && !d.show_y_line {
        return;
    }
    let calibrator = ctx.metadata.calibration.calibrator();
    let ticks = tick_marks(d, grid, calibrator);
    draw_ticks(r, pen, &ticks, decimal_places, ctx);
}

fn draw_lines(r: &mut dyn Renderer, pen: &Pen, grid: &Grid, d: &Plane, ctx: &PaintContext) {
    let mut line = |a, b| draw_distorted_line(r, pen, grid.mapper, a, b, ctx);
    if d.show_x_line {
        let x = d.x_line_coord * grid.width;
        line(PointF::new(x, 0.0), PointF::new(x, grid.height));
    }
    if d.show_y_line {
        let y = d.y_line_coord * grid.height;
        line(PointF::new(0.0, y), PointF::new(grid.width, y));
    }
}

/// `UpdateTickMarks`: the start and end ticks of an axis are left out when the sliding line is close
/// to them, so that the labels do not overlap.
fn tick_marks(d: &Plane, grid: &Grid, calibrator: &CalibratorPlane) -> Vec<TickMark> {
    let mut ticks = Vec::new();
    if d.show_x_line {
        let x = d.x_line_coord;
        if x > NEAR_START {
            add_ticks(&mut ticks, grid, calibrator, 0.0, true);
        }
        if x < NEAR_END {
            add_ticks(&mut ticks, grid, calibrator, grid.width, true);
        }
        add_ticks(&mut ticks, grid, calibrator, x * grid.width, true);
    }
    if d.show_y_line {
        let y = d.y_line_coord;
        if y > NEAR_START {
            add_ticks(&mut ticks, grid, calibrator, 0.0, false);
        }
        if y < NEAR_END {
            add_ticks(&mut ticks, grid, calibrator, grid.height, false);
        }
        add_ticks(&mut ticks, grid, calibrator, y * grid.height, false);
    }
    ticks
}

/// `AddTickMarks`: the two ticks of a grid coordinate, valued in world units. An x coordinate gives a
/// top and a bottom tick; a y coordinate a left and a right one.
fn add_ticks(
    ticks: &mut Vec<TickMark>,
    grid: &Grid,
    calibrator: &CalibratorPlane,
    coord: f32,
    x_axis: bool,
) {
    if x_axis {
        let top = PointF::new(coord, 0.0);
        let bottom = PointF::new(coord, grid.height);
        let value = calibrator.grid_to_world_with_offset(top).x;
        ticks.push(TickMark {
            value,
            image_location: grid.mapper.forward(top),
            alignment: Alignment::Top,
        });
        ticks.push(TickMark {
            value,
            image_location: grid.mapper.forward(bottom),
            alignment: Alignment::Bottom,
        });
    } else {
        let left = PointF::new(0.0, coord);
        let right = PointF::new(grid.width, coord);
        let value = calibrator.grid_to_world_with_offset(left).y;
        ticks.push(TickMark {
            value,
            image_location: grid.mapper.forward(left),
            alignment: Alignment::Left,
        });
        ticks.push(TickMark {
            value,
            image_location: grid.mapper.forward(right),
            alignment: Alignment::Right,
        });
    }
}
