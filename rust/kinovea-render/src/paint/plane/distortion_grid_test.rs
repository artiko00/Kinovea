use super::distortion_grid::paint_distortion_grid;
use crate::backend::recording::Call;
use crate::paint::helpers::frame::scene::Scene;
use crate::viewport::Point;
use kinovea_geometry::{PointF, RectangleF};
use kinovea_metadata::drawing::DistortionGrid;

/// Three by three points, row-major, 10 pixels apart.
fn grid3() -> DistortionGrid {
    let points = (0..9)
        .map(|k| PointF::new(10.0 * (k % 3) as f32, 10.0 * (k / 3) as f32))
        .collect();
    DistortionGrid::new(points)
}

#[test]
fn rows_then_columns_are_curves_and_every_point_has_a_ring() {
    // C#: DrawingDistortionGrid.cs:147, 151, 165, 179, 184.
    // Three row curves, then three column curves; the column at index 3 is (0,0), (0,10), (0,20).
    // Rings are o.Box(4) around each point: the one at (0,0) is (-4, -4, 8, 8).
    let scene = Scene::new(Point::new(0, 0), 1.0);
    let d = grid3();
    let calls = scene.record(0, |ctx, r| paint_distortion_grid(&d, &ctx, r));
    assert_eq!(calls.len(), 15);
    let Call::StrokeCurve(row, tension, _, _) = &calls[0] else {
        panic!("the first row is a curve");
    };
    assert_eq!(*tension, 0.5);
    assert_eq!(
        row,
        &vec![
            PointF::new(0.0, 0.0),
            PointF::new(10.0, 0.0),
            PointF::new(20.0, 0.0)
        ]
    );
    let Call::StrokeCurve(column, _, _, _) = &calls[3] else {
        panic!("the first column is a curve");
    };
    assert_eq!(
        column,
        &vec![
            PointF::new(0.0, 0.0),
            PointF::new(0.0, 10.0),
            PointF::new(0.0, 20.0)
        ]
    );
    let Call::StrokeEllipse(first, _, _) = &calls[6] else {
        panic!("the first ring");
    };
    let Call::StrokeEllipse(last, _, _) = &calls[14] else {
        panic!("the last ring");
    };
    assert_eq!(*first, RectangleF::new(-4.0, -4.0, 8.0, 8.0));
    assert_eq!(*last, RectangleF::new(16.0, 16.0, 8.0, 8.0));
}

#[test]
fn fully_faded_grid_paints_nothing() {
    // C#: DrawingDistortionGrid.cs:143.
    let mut scene = Scene::new(Point::new(0, 0), 1.0);
    scene.fading.average_timestamps_per_frame = 1.0;
    let d = grid3();
    assert!(scene
        .record(1000, |ctx, r| paint_distortion_grid(&d, &ctx, r))
        .is_empty());
}
