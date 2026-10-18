use crate::lib_test::Rng;
use crate::point::PointF;
use crate::quadrilateral::Quadrilateral;
use crate::rect::RectangleF;

fn p(x: f32, y: f32) -> PointF {
    PointF::new(x, y)
}

#[test]
fn convexity_of_bow_tie_and_square() {
    let bow_tie = Quadrilateral::new(p(0.0, 0.0), p(2.0, 2.0), p(2.0, 0.0), p(0.0, 2.0));
    assert!(!bow_tie.is_convex());
    assert!(Quadrilateral::unit_square().is_convex());
    assert!(Quadrilateral::centered_unit_square().is_convex());
    assert!(!Quadrilateral::empty().is_convex());
}

#[test]
fn axis_aligned_flag_and_constructors() {
    assert!(Quadrilateral::unit_square().is_axis_aligned_rectangle());
    assert!(
        Quadrilateral::from_rect(RectangleF::new(1.0, 2.0, 3.0, 4.0)).is_axis_aligned_rectangle()
    );
    assert_eq!(
        Quadrilateral::from_rect(RectangleF::new(1.0, 2.0, 3.0, 4.0)).c(),
        p(4.0, 6.0)
    );
    let mut skew = Quadrilateral::unit_square();
    skew[1] = p(1.0, 0.1);
    assert!(!skew.is_axis_aligned_rectangle());
    assert_eq!(skew[1], p(1.0, 0.1));
    assert_eq!(skew.to_array()[1], p(1.0, 0.1));
}

#[test]
fn contains_and_bounding_box() {
    let q = Quadrilateral::new(p(0.0, 0.0), p(4.0, 1.0), p(5.0, 3.0), p(-1.0, 2.0));
    assert!(q.contains(p(2.0, 1.5)));
    assert!(q.contains(p(0.0, 0.0)));
    assert!(!q.contains(p(10.0, 10.0)));
    assert!(!q.contains(p(4.0, 0.0)));
    let bow_tie = Quadrilateral::new(p(0.0, 0.0), p(2.0, 2.0), p(2.0, 0.0), p(0.0, 2.0));
    assert!(!bow_tie.contains(p(1.0, 0.5)));
    assert_eq!(q.bounding_box(), RectangleF::new(-1.0, 0.0, 6.0, 3.0));
    assert_eq!(Quadrilateral::empty().bounding_box(), RectangleF::EMPTY);
    assert!(Quadrilateral::empty().is_empty());
    assert!(!q.is_empty());
}

#[test]
fn hash_is_stable_and_not_trivially_zero() {
    let q = Quadrilateral::from_size(4.0, 2.0);
    assert_eq!(
        q.hash_code(),
        Quadrilateral::from_size(4.0, 2.0).hash_code()
    );
    assert_ne!(q.hash_code(), 0);
    let mut moved = q;
    moved.translate(1000.0, 0.0);
    assert_ne!(q.hash_code(), moved.hash_code());
}

#[test]
fn convexity_is_invariant_under_translation_and_scale() {
    let mut rng = Rng::new(21);
    for _ in 0..300 {
        let q = Quadrilateral::new(
            p(rng.range(-10.0, 10.0), rng.range(-10.0, 10.0)),
            p(rng.range(-10.0, 10.0), rng.range(-10.0, 10.0)),
            p(rng.range(-10.0, 10.0), rng.range(-10.0, 10.0)),
            p(rng.range(-10.0, 10.0), rng.range(-10.0, 10.0)),
        );
        let mut moved = q;
        moved.translate(rng.range(-100.0, 100.0), rng.range(-100.0, 100.0));
        moved.scale(rng.range(0.5, 3.0), rng.range(0.5, 3.0));
        assert_eq!(q.is_convex(), moved.is_convex(), "{q:?}");
    }
}
