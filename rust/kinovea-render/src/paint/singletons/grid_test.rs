use super::grid::CoordinateSystemGrid;

#[test]
fn new_grid_has_no_axes_lines_or_ticks() {
    let grid = CoordinateSystemGrid::default();
    assert!(grid.vertical_axis.is_none());
    assert!(grid.horizontal_axis.is_none());
    assert!(grid.grid_lines.is_empty());
    assert!(grid.tick_marks.is_empty());
}
