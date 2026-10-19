use super::common::testing::Scene;
use super::label::*;
use super::Hit;
use kinovea_geometry::{PointF, RectangleF};
use kinovea_metadata::drawing::Label;

fn p(x: f32, y: f32) -> PointF {
    PointF::new(x, y)
}

// The text box is the measured text untransformed (DrawingText.cs:215-219): the same size in image
// pixels at any scale, because the font is stretched by the scale.
#[test]
fn text_box_keeps_its_size_in_image_pixels() {
    let at_one = Scene::at_scale(1.0);
    let at_two = Scene::at_scale(2.0);
    let header = kinovea_metadata::Header::new("", Default::default());
    let a = text_background(p(10.0, 10.0), "Hello", &header, &at_one.ctx(0));
    let b = text_background(p(10.0, 10.0), "Hello", &header, &at_two.ctx(0));
    assert!((a.size().width - b.size().width).abs() < 0.01);
    assert!((a.size().height - b.size().height).abs() < 0.01);
}

// DrawingText.cs:317: the hidden resizer sits on the bottom-right corner of the background.
#[test]
fn background_has_a_hidden_resizer_at_its_corner() {
    let rect = RectangleF::new(100.0, 100.0, 44.0, 20.0);
    assert_eq!(background_hit(rect, 7, p(143.0, 118.0)), Hit::Handle(1));
    assert_eq!(background_hit(rect, 7, p(120.0, 110.0)), Hit::Body);
    assert_eq!(background_hit(rect, 7, p(10.0, 10.0)), Hit::Miss);
}

// DrawingText.cs:293-306: the arrow end disc (handle 2) comes before the background.
#[test]
fn arrow_end_is_handle_two_when_the_arrow_is_shown() {
    let scene = Scene::at_scale(1.0);
    let mut label = Label::new("Hello", p(100.0, 100.0));
    label.arrow_visible = true;
    label.arrow_end = p(200.0, 200.0);
    assert_eq!(
        hit_label(&label, p(200.0, 200.0), &scene.ctx(0)),
        Hit::Handle(2)
    );
    assert_eq!(hit_label(&label, p(110.0, 110.0), &scene.ctx(0)), Hit::Body);
}
