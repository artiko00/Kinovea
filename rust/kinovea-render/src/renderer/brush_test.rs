use super::*;
use kinovea_metadata::Color;

#[test]
fn solid_brush_keeps_its_colour() {
    assert_eq!(
        Brush::solid(Color::argb(128, 1, 2, 3)).color,
        Color::argb(128, 1, 2, 3)
    );
}
