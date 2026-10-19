//! Grid of the coordinate system in image coordinates (`CoordinateSystemGrid`, `GridLine`, `GridAxis`).

use crate::paint::helpers::tick_mark::TickMark;
use kinovea_geometry::PointF;

/// A segment of the grid, already clipped to the image. Axes use the same shape.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GridLine {
    pub start: PointF,
    pub end: PointF,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CoordinateSystemGrid {
    pub vertical_axis: Option<GridLine>,
    pub horizontal_axis: Option<GridLine>,
    pub grid_lines: Vec<GridLine>,
    pub tick_marks: Vec<TickMark>,
}
