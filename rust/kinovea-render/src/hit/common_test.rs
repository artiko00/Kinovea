use super::*;
use kinovea_geometry::PointF;
use kinovea_metadata::Header;

// PointF.Box(int) in Extensions.cs:90-93: side 2 * radius, top-left at centre minus radius.
#[test]
fn box_is_centred_with_side_twice_the_radius() {
    let b = box_around(PointF::new(10.0, 20.0), 5);
    assert_eq!(
        (b.left(), b.top(), b.right(), b.bottom()),
        (5.0, 15.0, 15.0, 25.0)
    );
}

// PointF.NearlyCoincideWith (Extensions.cs:99-102): strictly less than 0.5 on both axes.
#[test]
fn nearly_coincide_uses_half_a_pixel() {
    assert!(nearly_coincide(
        PointF::new(1.0, 1.0),
        PointF::new(1.4, 1.2)
    ));
    assert!(!nearly_coincide(
        PointF::new(1.0, 1.0),
        PointF::new(1.5, 1.0)
    ));
}

#[test]
fn trackable_opacity_is_the_baseline_fading_without_tracking() {
    let scene = testing::Scene::at_scale(1.0);
    let header = Header::new("", Default::default());
    assert_eq!(trackable_opacity(&header, &scene.ctx(0)), 1.0);
    assert_eq!(opacity(&header, &scene.ctx(0)), 1.0);
}
