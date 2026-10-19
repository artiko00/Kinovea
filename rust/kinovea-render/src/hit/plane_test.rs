use super::common::testing::Scene;
use super::plane::*;
use super::Hit;
use kinovea_geometry::{PointF, Quadrilateral, RectangleF};
use kinovea_metadata::drawing::{DistortionGrid, Plane};

fn p(x: f32, y: f32) -> PointF {
    PointF::new(x, y)
}

fn square_plane() -> Plane {
    Plane::new(Quadrilateral::from_rect(RectangleF::new(
        10.0, 10.0, 100.0, 100.0,
    )))
}

// DrawingPlane.cs:443-455: the four corners are handles 1-4 in quad order.
#[test]
fn plane_corners_are_handles_one_to_four() {
    let scene = Scene::at_scale(1.0);
    let plane = square_plane();
    assert_eq!(
        hit_plane(&plane, p(10.0, 10.0), &scene.ctx(0)),
        Hit::Handle(1)
    );
    assert_eq!(
        hit_plane(&plane, p(110.0, 110.0), &scene.ctx(0)),
        Hit::Handle(3)
    );
}

// DrawingPlane.cs:407-417 and 436-444: with the X line shown and the plane mapped (axis-aligned),
// the line at plane x = 50 is the image segment x = 60 from y = 10 to y = 110: handle 5.
#[test]
fn sliding_x_line_is_handle_five_when_mapped() {
    let scene = Scene::at_scale(1.0);
    let mut plane = square_plane();
    plane.show_x_line = true;
    plane.x_line_coord = 0.5;
    assert_eq!(
        hit_plane(&plane, p(60.0, 60.0), &scene.ctx(0)),
        Hit::Handle(5)
    );
    assert_eq!(hit_plane(&plane, p(20.0, 60.0), &scene.ctx(0)), Hit::Miss);
}

// DrawingDistortionGrid.cs:187-200: handle i + 1 for the first point whose box holds the probe.
#[test]
fn distortion_grid_points_are_handles() {
    let scene = Scene::at_scale(1.0);
    let grid = DistortionGrid::new(vec![p(100.0, 100.0), p(200.0, 100.0)]);
    assert_eq!(
        hit_distortion_grid(&grid, p(200.0, 100.0), &scene.ctx(0)),
        Hit::Handle(2)
    );
    assert_eq!(
        hit_distortion_grid(&grid, p(150.0, 150.0), &scene.ctx(0)),
        Hit::Miss
    );
}
