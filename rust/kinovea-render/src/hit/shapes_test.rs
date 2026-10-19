use super::common::testing::Scene;
use super::shapes::*;
use super::Hit;
use kinovea_geometry::{PointF, Quadrilateral, RectangleF};
use kinovea_metadata::drawing::{Circle, CrossMark, Rectangle};

fn p(x: f32, y: f32) -> PointF {
    PointF::new(x, y)
}

// DrawingCircle.cs:319-338: centre is the body, the outline (ellipse box of the radius) the handle.
#[test]
fn circle_outline_is_handle_one_and_disc_is_body() {
    let scene = Scene::at_scale(1.0);
    let c = Circle::new(p(100.0, 100.0), 50);
    let ctx = scene.ctx(0);
    assert_eq!(hit_circle(&c, p(150.0, 100.0), &ctx), Hit::Handle(1));
    assert_eq!(hit_circle(&c, p(120.0, 100.0), &ctx), Hit::Body);
    assert_eq!(hit_circle(&c, p(100.0, 100.0), &ctx), Hit::Body);
    assert_eq!(hit_circle(&c, p(200.0, 200.0), &ctx), Hit::Miss);
}

// DrawingCircle.cs:321-322: opacity <= 0 gives a miss everywhere.
#[test]
fn faded_circle_is_never_hit() {
    let scene = Scene::at_scale(1.0);
    let c = Circle::new(p(100.0, 100.0), 50);
    assert_eq!(
        hit_circle(&c, p(100.0, 100.0), &scene.ctx(10_000)),
        Hit::Miss
    );
}

// DrawingCrossMark.cs:208-221: a point box of ten viewport pixels around the centre.
#[test]
fn cross_mark_is_hit_inside_its_point_box() {
    let scene = Scene::at_scale(1.0);
    let c = CrossMark::new(p(50.0, 50.0));
    assert_eq!(hit_crossmark(&c, p(55.0, 50.0), &scene.ctx(0)), Hit::Body);
    assert_eq!(hit_crossmark(&c, p(60.0, 50.0), &scene.ctx(0)), Hit::Miss);
}

// DrawingRectangle.cs:219-246: corners 1-4, edges 5-8, body inside.
#[test]
fn rectangle_handles_are_corners_then_edges() {
    let scene = Scene::at_scale(1.0);
    let r = Rectangle::new(Quadrilateral::from_rect(RectangleF::new(
        10.0, 10.0, 100.0, 100.0,
    )));
    let ctx = scene.ctx(0);
    assert_eq!(hit_rectangle(&r, p(10.0, 10.0), &ctx), Hit::Handle(1));
    assert_eq!(hit_rectangle(&r, p(110.0, 110.0), &ctx), Hit::Handle(3));
    assert_eq!(hit_rectangle(&r, p(60.0, 10.0), &ctx), Hit::Handle(5));
    assert_eq!(hit_rectangle(&r, p(110.0, 60.0), &ctx), Hit::Handle(6));
    assert_eq!(hit_rectangle(&r, p(60.0, 60.0), &ctx), Hit::Body);
    assert_eq!(hit_rectangle(&r, p(200.0, 200.0), &ctx), Hit::Miss);
}
