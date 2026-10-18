use crate::point::{PointF, SizeF};
use crate::rect::RectangleF;

#[test]
fn right_and_bottom_derive_from_location_and_size() {
    let r = RectangleF::new(10.0, 20.0, 30.0, 40.0);
    assert_eq!(r.left(), 10.0);
    assert_eq!(r.top(), 20.0);
    assert_eq!(r.right(), 40.0);
    assert_eq!(r.bottom(), 60.0);
    assert_eq!(r.center(), PointF::new(25.0, 40.0));
    assert_eq!(r.location(), PointF::new(10.0, 20.0));
    assert_eq!(r.size(), SizeF::new(30.0, 40.0));
}

#[test]
fn contains_is_inclusive_on_left_top_and_exclusive_on_right_bottom() {
    let r = RectangleF::new(0.0, 0.0, 10.0, 10.0);
    assert!(r.contains(PointF::new(0.0, 0.0)));
    assert!(r.contains(PointF::new(9.99, 9.99)));
    assert!(!r.contains(PointF::new(10.0, 5.0)));
    assert!(!r.contains(PointF::new(5.0, 10.0)));
    assert!(!r.contains(PointF::new(-0.1, 5.0)));
}

#[test]
fn empty_when_width_or_height_is_not_positive() {
    assert!(RectangleF::EMPTY.is_empty());
    assert!(RectangleF::new(1.0, 1.0, 0.0, 5.0).is_empty());
    assert!(RectangleF::new(1.0, 1.0, 5.0, -1.0).is_empty());
    assert!(!RectangleF::new(1.0, 1.0, 5.0, 5.0).is_empty());
    assert_eq!(
        RectangleF::from_location_size(PointF::new(1.0, 2.0), SizeF::new(3.0, 4.0)),
        RectangleF::new(1.0, 2.0, 3.0, 4.0)
    );
}
