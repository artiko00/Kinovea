//! Owner: P1. Painters for `DrawingCoordinateSystem.Draw` and `DrawingTestGrid.Draw`.

mod coordinate_system;
#[cfg(test)]
mod coordinate_system_test;
#[cfg(test)]
mod fixture_test;
mod grid;
mod grid_finder;
#[cfg(test)]
mod grid_finder_test;
mod grid_lines;
#[cfg(test)]
mod grid_lines_test;
#[cfg(test)]
mod grid_test;
mod range;
#[cfg(test)]
mod range_test;
mod test_grid;
#[cfg(test)]
mod test_grid_test;

pub use coordinate_system::paint_coordinate_system;
pub use test_grid::paint_test_grid;
