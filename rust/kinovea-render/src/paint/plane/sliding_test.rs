use super::grid::Grid;
use super::sliding::draw_sliding_lines;
use crate::backend::recording::Call;
use crate::paint::helpers::frame::scene::{Scene, COLOR};
use crate::paint::plane::paint_plane;
use crate::renderer::{Pen, Transform};
use crate::viewport::Point;
use kinovea_geometry::{PointF, ProjectiveMapper, Quadrilateral, RectangleF, SizeF};
use kinovea_metadata::drawing::Plane;

/// A scene whose calibration drawing is `d`, calibrated on the 200 x 100 plane over `image`.
fn calibrated(d: &Plane, image: &Quadrilateral) -> Scene {
    let mut scene = Scene::new(Point::new(0, 0), 1.0);
    scene
        .metadata
        .calibration
        .calibrate_by_plane(d.header.id, SizeF::new(200.0, 100.0), image);
    scene
}

/// Records the sliding lines of `d` over the 200 x 100 plane mapped to `image`, with 2 decimals.
fn record_sliding(scene: &Scene, d: &Plane, image: &Quadrilateral) -> Vec<Call> {
    let plane = Quadrilateral::from_rect(RectangleF::new(0.0, 0.0, 200.0, 100.0));
    let mapper = ProjectiveMapper::new(&plane, image).expect("plane maps to the image");
    let grid = Grid {
        mapper: &mapper,
        width: 200.0,
        height: 100.0,
        cols: 10,
        rows: 10,
    };
    let pen = Pen::new(COLOR, 1.0);
    scene.record(0, |ctx, r| draw_sliding_lines(r, &pen, &grid, d, &ctx, 2))
}

/// The labels of the drawn texts, in drawing order.
fn labels(calls: &[Call]) -> Vec<String> {
    calls
        .iter()
        .filter_map(|c| match c {
            Call::DrawText(text, _, _, _, _) => Some(text.clone()),
            _ => None,
        })
        .collect()
}

#[test]
fn x_sliding_line_runs_across_the_plane_at_its_coordinate() {
    // C#: DrawingPlane.cs:337-346 (DrawSlidingLines), 385-391.
    // The line at 0.25 of the width is plane x = 50, which is image x = 10 + 50 = 60 for the whole
    // height. The calibration drawing is this plane, so it is drawn.
    let image = Quadrilateral::from_rect(RectangleF::new(10.0, 10.0, 200.0, 100.0));
    let mut d = Plane::new(image);
    d.show_x_line = true;
    d.x_line_coord = 0.25;
    let calls = record_sliding(&calibrated(&d, &image), &d, &image);
    let lines: Vec<&Call> = calls
        .iter()
        .filter(|c| matches!(c, Call::StrokeLine(..)))
        .collect();
    assert_eq!(
        lines,
        vec![&Call::StrokeLine(
            PointF::new(60.0, 10.0),
            PointF::new(60.0, 110.0),
            Pen::new(COLOR, 1.0),
            Transform::identity(),
        )]
    );
}

#[test]
fn sliding_lines_are_only_drawn_for_the_calibration_drawing() {
    // C#: DrawingPlane.cs:340 (guard).
    // No calibration drawing is set, so the sliding line of this plane is not drawn.
    let image = Quadrilateral::from_rect(RectangleF::new(10.0, 10.0, 200.0, 100.0));
    let mut d = Plane::new(image);
    d.show_x_line = true;
    let calls = record_sliding(&Scene::new(Point::new(0, 0), 1.0), &d, &image);
    assert!(calls.is_empty());
}

#[test]
fn ticks_label_the_ends_and_the_sliding_value_rounded_to_two_decimals() {
    // C#: DrawingPlane.cs:838-846 (start and end ticks for x > 0.1 and x < 0.9), 866-883 (AddTickMarks);
    // TickMark.cs:26-27 (rounding). x = 0.3333 * 200 = 66.66; the calibration origin is the image
    // corner D mapped back, so world x equals grid x.
    let image = Quadrilateral::from_rect(RectangleF::new(10.0, 10.0, 200.0, 100.0));
    let mut d = Plane::new(image);
    d.show_x_line = true;
    d.x_line_coord = 0.3333;
    let calls = record_sliding(&calibrated(&d, &image), &d, &image);
    assert_eq!(
        labels(&calls),
        vec!["0", "0", "200", "200", "66.66", "66.66"]
    );
}

#[test]
fn painter_labels_its_ticks_with_the_default_two_decimals() {
    // C#: PlayerPreferences.cs:324 (decimalPlaces = 2), reached through DrawingPlane.Draw.
    let image = Quadrilateral::from_rect(RectangleF::new(0.0, 0.0, 200.0, 100.0));
    let mut d = Plane::new(image);
    d.show_x_line = true;
    d.x_line_coord = 0.3333;
    let calls = calibrated(&d, &image).record(0, |ctx, r| paint_plane(&d, &ctx, r));
    assert!(labels(&calls).contains(&"66.66".to_string()));
}
