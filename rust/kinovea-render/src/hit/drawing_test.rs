use super::*;
use crate::hit::common::testing::Scene;
use kinovea_geometry::PointF;
use kinovea_metadata::drawing::CrossMark;

// The dispatcher routes each kind to its own port; a kind outside the port misses.
#[test]
fn routes_cross_marks_to_their_port() {
    let scene = Scene::at_scale(1.0);
    let d = Drawing::CrossMark(CrossMark::new(PointF::new(50.0, 50.0)));
    assert_eq!(
        hit_test(&d, PointF::new(51.0, 50.0), &scene.ctx(0)),
        Hit::Body
    );
    assert_eq!(
        hit_test(&d, PointF::new(200.0, 50.0), &scene.ctx(0)),
        Hit::Miss
    );
}

#[test]
fn kinds_without_a_port_are_reported() {
    assert!(is_ported(&Drawing::CrossMark(CrossMark::new(PointF::new(
        0.0, 0.0
    )))));
    assert!(!is_ported(&Drawing::Svg(kinovea_metadata::drawing::Svg::new("x.svg"))
    )));
}
