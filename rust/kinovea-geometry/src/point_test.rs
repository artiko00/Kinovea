use crate::point::{PointF, Size, SizeF};
use crate::vector::Vector;

#[test]
fn translate_and_scale() {
    let p = PointF::new(1.0, 2.0);
    assert_eq!(p.translate(3.0, -1.0), PointF::new(4.0, 1.0));
    assert_eq!(p.scale(2.0, 3.0), PointF::new(2.0, 6.0));
}

#[test]
fn adding_a_vector_moves_the_point() {
    assert_eq!(
        PointF::new(1.0, 1.0) + Vector::new(2.0, 3.0),
        PointF::new(3.0, 4.0)
    );
}

#[test]
fn empty_and_default_agree() {
    assert_eq!(PointF::EMPTY, PointF::default());
    assert_eq!(SizeF::default(), SizeF::new(0.0, 0.0));
}

#[test]
fn hash_code_is_stable_and_distinguishes_points() {
    let p = PointF::new(1.5, -2.0);
    assert_eq!(p.hash_code(), PointF::new(1.5, -2.0).hash_code());
    assert_ne!(p.hash_code(), PointF::new(2.0, 1.5).hash_code());
}

#[test]
fn integer_size_centre_uses_integer_division() {
    let s = Size::new(1921, 1081);
    assert_eq!(s.center(), PointF::new(960.0, 540.0));
    assert!(Size::default().is_empty());
    assert!(!s.is_empty());
}
