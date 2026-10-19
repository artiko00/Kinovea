//! Integer `Point` and `Rectangle` mirroring `System.Drawing`.
use super::*;
use kinovea_geometry::PointF;

#[test]
fn point_converts_to_float_point() {
    assert_eq!(Point::new(3, -4).to_point_f(), PointF::new(3.0, -4.0));
}

#[test]
fn rectangle_keeps_its_fields() {
    let r = Rectangle::new(1, 2, 3, 4);
    assert_eq!((r.x, r.y, r.width, r.height), (1, 2, 3, 4));
}
