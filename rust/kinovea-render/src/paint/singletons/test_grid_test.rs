//! Expected calls of `DrawingTestGrid.Draw` on a 200 by 100 image. Normalized points map through
//! `Map(p) = ((p.X * 0.5f + 0.5f) * W, (p.Y * 0.5f + 0.5f) * H)` and then truncate, as in `Transform`.

use super::test_grid::paint_test_grid;
use crate::backend::recording::Call;
use crate::paint::helpers::frame::scene::{style, Scene, COLOR};
use crate::renderer::{LineCap, LineJoin, Pen, Transform};
use crate::viewport::Point;
use kinovea_geometry::{PointF, Size};
use kinovea_metadata::singletons::TestGrid;
use kinovea_metadata::{Header, Metadata};

/// `GetPen(255)`: width 1, round caps and joins, never stretched.
fn grid_pen() -> Pen {
    Pen {
        start_cap: LineCap::Round,
        end_cap: LineCap::Round,
        line_join: LineJoin::Round,
        ..Pen::new(COLOR, 1.0)
    }
}

fn line(a: (f32, f32), b: (f32, f32)) -> Call {
    Call::StrokeLine(
        PointF::new(a.0, a.1),
        PointF::new(b.0, b.1),
        grid_pen(),
        Transform::identity(),
    )
}

/// A visible grid with the defaults of `DrawingTestGrid`: both axes and the framing, no thirds.
fn grid() -> TestGrid {
    TestGrid {
        visible: true,
        header: Header::new("", style(1)),
        ..TestGrid::default()
    }
}

fn scene(location: Point, scale: f64) -> Scene {
    let mut scene = Scene::new(location, scale);
    scene.metadata = Metadata::new(Size::new(200, 100));
    scene
}

fn paint(scene: &Scene, d: &TestGrid) -> Vec<Call> {
    scene.record(0, |ctx, r| paint_test_grid(d, &ctx, r))
}

#[test]
fn hidden_grid_paints_nothing() {
    let mut d = grid();
    d.visible = false;
    assert!(paint(&scene(Point::new(0, 0), 1.0), &d).is_empty());
}

#[test]
fn grid_on_an_image_without_size_paints_nothing() {
    let scene = Scene::new(Point::new(0, 0), 1.0);
    assert!(paint(&scene, &grid()).is_empty());
}

#[test]
fn default_grid_paints_axes_then_framing() {
    // Horizontal axis at normalized y = 0: (0 * 0.5 + 0.5) * 100 = 50. Vertical axis at x = 0: 100.
    // The framing margin 0.8f is stored just above 0.8, so the left edge (-0.8) maps to 19.999998 -> 19
    // and its y to 9.999999 -> 9. The right edge (0.8) maps to 180.0 and its bottom to 90.0.
    let expected = vec![
        line((0.0, 50.0), (200.0, 50.0)),
        line((100.0, 0.0), (100.0, 100.0)),
        line((19.0, 9.0), (19.0, 90.0)),
        line((19.0, 9.0), (180.0, 9.0)),
        line((180.0, 9.0), (180.0, 90.0)),
        line((19.0, 90.0), (180.0, 90.0)),
    ];
    assert_eq!(paint(&scene(Point::new(0, 0), 1.0), &grid()), expected);
}

#[test]
fn hidden_axes_are_skipped() {
    let mut d = grid();
    d.show_horizontal_axis = false;
    d.show_framing = false;
    assert_eq!(
        paint(&scene(Point::new(0, 0), 1.0), &d),
        vec![line((100.0, 0.0), (100.0, 100.0))]
    );
}

#[test]
fn thirds_follow_the_framing_in_the_original_order() {
    // Thirds at -1/3 and +1/3 in f32 truncate to x = 66 and 133 (W = 200) and y = 33 and 66 (H = 100).
    let mut d = grid();
    d.show_horizontal_axis = false;
    d.show_vertical_axis = false;
    d.show_framing = false;
    d.show_thirds = true;
    let expected = vec![
        line((66.0, 0.0), (66.0, 100.0)),
        line((0.0, 33.0), (200.0, 33.0)),
        line((133.0, 0.0), (133.0, 100.0)),
        line((0.0, 66.0), (200.0, 66.0)),
    ];
    assert_eq!(paint(&scene(Point::new(0, 0), 1.0), &d), expected);
}

#[test]
fn viewport_location_and_scale_move_the_axes_and_keep_the_pen_width() {
    // Image x * 2 + 10 and y * 2 + 20; the pen is not scaled by the viewport.
    let mut d = grid();
    d.show_framing = false;
    let expected = vec![
        line((10.0, 120.0), (410.0, 120.0)),
        line((210.0, 20.0), (210.0, 220.0)),
    ];
    assert_eq!(paint(&scene(Point::new(10, 20), 2.0), &d), expected);
}
