//! Even-odd containment over the figures of a path.
use super::path::GraphicsPath;
use kinovea_geometry::{PointF, RectangleF};

fn p(x: f32, y: f32) -> PointF {
    PointF::new(x, y)
}

#[test]
fn contains_uses_even_odd_crossings() {
    let mut path = GraphicsPath::new();
    path.add_rectangle(RectangleF::new(0.0, 0.0, 10.0, 10.0));
    assert!(path.contains_even_odd(p(5.0, 5.0)));
    assert!(!path.contains_even_odd(p(15.0, 5.0)));
}
