use crate::point::PointF;
use crate::shapes::*;

#[test]
fn circle_and_segment_empty_values() {
    assert_eq!(Circle::EMPTY, Circle::new(PointF::EMPTY, 0.0));
    assert_eq!(Circle::EMPTY, Circle::default());
    assert_eq!(Segment::EMPTY, Segment::default());
    assert_eq!(
        Segment::new(PointF::new(1.0, 2.0), PointF::new(3.0, 4.0)).end,
        PointF::new(3.0, 4.0)
    );
}

#[test]
fn ellipse_vertical_flag_depends_on_rotation() {
    let e = |rotation: f32| Ellipse::new(PointF::EMPTY, 2.0, 1.0, rotation);
    assert!(!e(0.0).is_vertical());
    assert!(!e(0.78).is_vertical());
    // f32 π/4 widens above the double π/4 of the comparison, as in the original.
    assert!(e(std::f32::consts::FRAC_PI_4).is_vertical());
    assert!(e(std::f32::consts::FRAC_PI_2).is_vertical());
    assert!(e(2.0).is_vertical());
    assert!(!e(2.5).is_vertical());
}

#[test]
fn named_point_and_clip_result() {
    let p = NamedPoint::new("origin", 1.0, 2.0, 3.0);
    assert_eq!(p.name, "origin");
    assert_eq!((p.x, p.y, p.z), (1.0, 2.0, 3.0));
    let invisible = ClipResult::INVISIBLE;
    assert_eq!((invisible.visible, invisible.a, invisible.b), (false, PointF::EMPTY, PointF::EMPTY));
    let c = ClipResult::visible(PointF::new(1.0, 1.0), PointF::new(2.0, 2.0));
    assert!(c.visible);
    assert_eq!(c.b, PointF::new(2.0, 2.0));
    assert_ne!(PointLinePosition::OnSegment, PointLinePosition::Anywhere);
}
