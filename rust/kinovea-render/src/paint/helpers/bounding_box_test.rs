use crate::backend::recording::Call;
use crate::paint::helpers::bounding_box::draw_bounding_box;
use crate::paint::helpers::support_test::surface;
use crate::renderer::{Brush, Pen, Transform};
use kinovea_geometry::RectangleF;
use kinovea_metadata::Color;

/// `BoundingBox.Draw` for rect (10, 20, 100, 50) and `widen` 3: the outline, then four handles
/// centred on the corners (diameter `2 * widen`, so each ellipse starts `widen` before the corner).
#[test]
fn outline_then_four_corner_handles() {
    let mut r = surface();
    let pen = Pen::new(Color::rgb(1, 2, 3), 1.0);
    let brush = Brush::solid(Color::rgb(4, 5, 6));
    draw_bounding_box(
        &mut r,
        RectangleF::new(10.0, 20.0, 100.0, 50.0),
        &pen,
        &brush,
        3,
    );
    let handle = |x: f32, y: f32| {
        Call::FillEllipse(
            RectangleF::new(x, y, 6.0, 6.0),
            brush,
            Transform::identity(),
        )
    };
    assert_eq!(
        r.calls,
        vec![
            Call::StrokeRectangle(
                RectangleF::new(10.0, 20.0, 100.0, 50.0),
                pen.clone(),
                Transform::identity()
            ),
            handle(7.0, 17.0),
            handle(7.0, 67.0),
            handle(107.0, 17.0),
            handle(107.0, 67.0),
        ]
    );
}
