use super::frame::scene::Scene;
use super::frame::*;
use crate::viewport::Point;
use kinovea_geometry::PointF;
use kinovea_metadata::Fading;

#[test]
fn always_visible_fading_is_fully_opaque() {
    let scene = Scene::new(Point::new(0, 0), 1.0);
    let always = Fading {
        always_visible: true,
        use_default: false,
        ..Fading::default()
    };
    assert_eq!(opacity(&always, &scene.ctx(999)), 1.0);
}

#[test]
fn viewport_point_is_truncated_after_scale_and_translation() {
    // `Transform(PointF)`: (1.6, 2.4) * 2 + (10, 20) = (13.2, 24.8), truncated to (13, 24).
    let scene = Scene::new(Point::new(10, 20), 2.0);
    let p = to_viewport(&scene.ctx(0), PointF::new(1.6, 2.4));
    assert_eq!(p, PointF::new(13.0, 24.0));
}

#[test]
fn stretch_is_the_viewport_scale() {
    let scene = Scene::new(Point::new(0, 0), 0.5);
    assert_eq!(stretch(&scene.ctx(0)), 0.5);
}
