//! Port of `DrawingCoordinateSystem.Draw` (`DrawGrid`, `DrawGridLine`): axes, grid and graduations of
//! the calibration. The line colour is bound to the background colour (`BindStyle`, `Bicolor`).
//! Curved grid lines under lens distortion are drawn straight: `DistortRectifiedLine` is not on the trait.

use super::grid::GridLine;
use super::grid_finder::find;
use crate::paint::helpers::decimal::DEFAULT_DECIMAL_PLACES;
use crate::paint::helpers::frame::to_viewport;
use crate::paint::helpers::tick_mark::{draw_tick_mark, TickLabel};
use crate::paint::PaintContext;
use crate::renderer::{Pen, Renderer};
use crate::style::colors::with_alpha;
use crate::style::StyleData;
use kinovea_geometry::PointF;
use kinovea_measurement::CalibratorType;
use kinovea_metadata::singletons::CoordinateSystem;
use kinovea_metadata::style::DashStyle;

const AXES_ALPHA: i32 = 255;
/// `(int)(255 * 0.5f)`.
const GRID_ALPHA: i32 = 127;
const LABELS_ALPHA: i32 = 127;
const TEXT_MARGIN: f32 = 8.0;

/// Port of `DrawingCoordinateSystem.Draw`.
pub fn paint_coordinate_system(d: &CoordinateSystem, ctx: &PaintContext, r: &mut dyn Renderer) {
    let calibration = &ctx.metadata.calibration;
    if !d.visible {
        return;
    }
    if calibration.calibrator_type() == CalibratorType::Plane && !calibration.plane_is_valid() {
        return;
    }
    let grid = find(calibration);

    let mut style = StyleData::from_elements(&d.header.style);
    style.background_color = style.color;

    // Axes: solid, width 1.
    let mut pen = style.background_pen(AXES_ALPHA);
    if let Some(axis) = grid.vertical_axis {
        stroke(ctx, r, &pen, axis);
    }
    if let Some(axis) = grid.horizontal_axis {
        stroke(ctx, r, &pen, axis);
    }

    if d.show_grid {
        pen.color = with_alpha(pen.color, GRID_ALPHA);
        pen.dash = DashStyle::Dash;
        for line in &grid.grid_lines {
            stroke(ctx, r, &pen, *line);
        }
    }

    if d.show_graduations {
        let back = style.background_brush(LABELS_ALPHA);
        let fore = style.foreground_brush(255);
        let font = style.font(1.0);
        let label = TickLabel {
            fill: &back,
            text: &fore,
            font: &font,
            decimal_places: DEFAULT_DECIMAL_PLACES,
        };
        let distort = |p: PointF| ctx.distorter.map_or(p, |distorter| distorter.distort(p));
        for tick in &grid.tick_marks {
            draw_tick_mark(r, tick, &label, ctx, &distort, TEXT_MARGIN);
        }
    }
}

/// `DrawGridLine`, straight branch.
fn stroke(ctx: &PaintContext, r: &mut dyn Renderer, pen: &Pen, line: GridLine) {
    r.stroke_line(
        to_viewport(ctx, line.start),
        to_viewport(ctx, line.end),
        pen,
    );
}
