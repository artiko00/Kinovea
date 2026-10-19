use super::*;
use crate::renderer::{Brush, Transform};
use kinovea_geometry::RectangleF;
use kinovea_metadata::Color;

#[test]
fn calls_compare_by_arguments_including_the_transform() {
    let rect = RectangleF::new(0.0, 0.0, 1.0, 1.0);
    let brush = Brush::solid(Color::BLACK);
    let shifted = Transform::translate(1.0, 0.0);
    assert_eq!(
        Call::FillRectangle(rect, brush, shifted),
        Call::FillRectangle(rect, brush, shifted)
    );
    assert_ne!(
        Call::FillRectangle(rect, brush, shifted),
        Call::FillRectangle(rect, brush, Transform::identity())
    );
    assert_ne!(Call::PopTransform, Call::ResetTransform);
}
