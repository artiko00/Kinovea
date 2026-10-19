//! `CoordinateSystemGridFinder.Find` on a flat plane of 200 by 100 whose quad sits at (20, 10) in a
//! 240 by 120 image. The world origin is then at image (20, 110) and world y points up, so a world
//! point (x, y) projects to image (x + 20, 110 - y).
//!
//! Steps: the image bounds reproject to x in [-20, 220] (width 240) and y in [-10, 110] (height 120),
//! so the steps are 20 along x (240 / 15 = 16 -> 20) and 10 along y (120 / 15 = 8 -> 10). The axes
//! are visible and ticks come from the origin outwards: the vertical walk starts at 0 and steps -20
//! then +20, so the first ticks are 0, -20 and 20.

use super::fixture_test::flat_plane;
use super::grid::GridLine;
use super::grid_finder::find;
use crate::paint::helpers::tick_mark::{TextAlignment, TickMark};
use kinovea_geometry::PointF;

fn line(a: (f32, f32), b: (f32, f32)) -> GridLine {
    GridLine {
        start: PointF::new(a.0, a.1),
        end: PointF::new(b.0, b.1),
    }
}

/// Clipped ends carry the rounding of the line parameter (about 1e-14 px at y 0), as the original's
/// doubles do; the comparison is at 1e-3 px.
fn settled(l: GridLine) -> GridLine {
    let snap = |p: PointF| {
        PointF::new(
            (p.x * 1000.0).round() / 1000.0,
            (p.y * 1000.0).round() / 1000.0,
        )
    };
    GridLine {
        start: snap(l.start),
        end: snap(l.end),
    }
}

// CoordinateSystemGridFinder.cs:167 and :240 clip the line through the two projected ends with
// `LiangBarsky.ClipLine(clipWindow, a, b)` (LiangBarsky.cs:83-86, the overload that clips the infinite
// line). The ends are at image y 110 and 10 for the vertical walk, so the line runs over the whole
// image height: from y 120 (bottom edge) to y 0 (top edge). The horizontal line runs the full width.
#[test]
fn flat_plane_axes_run_across_the_image_through_the_origin() {
    let grid = find(&flat_plane());
    assert_eq!(
        grid.vertical_axis.map(settled),
        Some(line((20.0, 120.0), (20.0, 0.0)))
    );
    assert_eq!(
        grid.horizontal_axis.map(settled),
        Some(line((0.0, 110.0), (240.0, 110.0)))
    );
}

// The same infinite clip applies to every grid line (CoordinateSystemGridFinder.cs:167 and :240).
#[test]
fn flat_plane_grid_lines_step_outwards_from_the_origin() {
    let grid = find(&flat_plane());
    assert_eq!(settled(grid.grid_lines[0]), line((0.0, 120.0), (0.0, 0.0)));
    assert_eq!(
        settled(grid.grid_lines[1]),
        line((40.0, 120.0), (40.0, 0.0))
    );
}

#[test]
fn flat_plane_graduations_are_labelled_in_world_units() {
    let grid = find(&flat_plane());
    let tick = |value: f32, location: (f32, f32), alignment| TickMark {
        value,
        image_location: PointF::new(location.0, location.1),
        alignment,
    };
    assert_eq!(
        grid.tick_marks[0],
        tick(0.0, (20.0, 110.0), TextAlignment::BottomRight)
    );
    assert_eq!(
        grid.tick_marks[1],
        tick(-20.0, (0.0, 110.0), TextAlignment::Bottom)
    );
    assert_eq!(
        grid.tick_marks[2],
        tick(20.0, (40.0, 110.0), TextAlignment::Bottom)
    );
}
