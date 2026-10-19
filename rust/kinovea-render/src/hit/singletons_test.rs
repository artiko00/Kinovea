use super::common::testing::Scene;
use super::singletons::*;
use super::Hit;
use kinovea_geometry::{PointF, Size};
use kinovea_metadata::singletons::{CoordinateSystem, TestGrid};
use kinovea_metadata::Metadata;

fn p(x: f32, y: f32) -> PointF {
    PointF::new(x, y)
}

// DrawingCoordinateSystem.cs:257-263: the origin is handle 1 when the grid is visible.
#[test]
fn origin_is_handle_one_only_when_visible() {
    let mut scene = Scene::at_scale(1.0);
    scene.metadata = Metadata::default();
    let mut cs = CoordinateSystem {
        origin: p(50.0, 50.0),
        visible: true,
        ..CoordinateSystem::default()
    };
    assert_eq!(
        hit_coordinate_system(&cs, p(50.0, 50.0), &scene.ctx(0)),
        Hit::Handle(1)
    );
    cs.visible = false;
    assert_eq!(
        hit_coordinate_system(&cs, p(50.0, 50.0), &scene.ctx(0)),
        Hit::Miss
    );
}

// DrawingTestGrid.cs:197-242: every enabled line is stroked with a width of 2 px and the hit is the body.
#[test]
fn test_grid_lines_are_body_within_two_pixels() {
    let mut scene = Scene::at_scale(1.0);
    scene.metadata = Metadata::new(Size::new(200, 100));
    let grid = TestGrid {
        visible: true,
        ..TestGrid::default()
    };
    let ctx = scene.ctx(0);
    // The horizontal axis is y = 50 across the image; the framing is at x = 20 and 180, y = 10 and 90.
    assert_eq!(hit_test_grid(&grid, p(100.0, 50.0), &ctx), Hit::Body);
    assert_eq!(hit_test_grid(&grid, p(60.0, 40.0), &ctx), Hit::Miss);
    assert_eq!(hit_test_grid(&grid, p(30.0, 20.0), &ctx), Hit::Miss);
}

#[test]
fn hidden_test_grid_is_never_hit() {
    let mut scene = Scene::at_scale(1.0);
    scene.metadata = Metadata::new(Size::new(200, 100));
    let grid = TestGrid::default();
    assert_eq!(
        hit_test_grid(&grid, p(100.0, 50.0), &scene.ctx(0)),
        Hit::Miss
    );
}
