use crate::point::PointF;
use crate::vector::Vector;

#[test]
fn from_two_points_norm_and_normalized() {
    let v = Vector::from_points(PointF::new(1.0, 2.0), PointF::new(4.0, 6.0));
    assert_eq!(v, Vector::new(3.0, 4.0));
    assert_eq!(v.norm(), 5.0);
    assert_eq!(v.squared(), 25.0);
    let n = v.normalized();
    assert!((n.x - 0.6).abs() < 1e-6 && (n.y - 0.8).abs() < 1e-6);
}

#[test]
fn arithmetic_operators() {
    let a = Vector::new(1.0, 2.0);
    let b = Vector::new(3.0, -1.0);
    assert_eq!(a + b, Vector::new(4.0, 1.0));
    assert_eq!(a - b, Vector::new(-2.0, 3.0));
    assert_eq!(a * 2.0, Vector::new(2.0, 4.0));
    assert_eq!(a / 2.0, Vector::new(0.5, 1.0));
    assert_eq!(-a, Vector::new(-1.0, -2.0));
    assert_eq!(a.dot(b), 1.0);
    assert_eq!(a.to_point(), PointF::new(1.0, 2.0));
}

#[test]
fn normalizing_the_zero_vector_gives_nan() {
    let n = Vector::default().normalized();
    assert!(n.x.is_nan() && n.y.is_nan());
}
