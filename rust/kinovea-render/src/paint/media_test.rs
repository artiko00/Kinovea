use crate::backend::recording::Call;
use crate::paint::helpers::support_test::{surface, Scene};
use crate::paint::media::{paint_bitmap, paint_svg};
use crate::renderer::{Brush, Font, FontFamily, Transform};
use kinovea_geometry::{PointF, RectangleF};
use kinovea_metadata::drawing::{Bitmap, Svg};
use kinovea_metadata::Color;

#[test]
fn media_painters_are_declared() {
    assert!(module_path!().contains("media"));
}

/// Placeholder font: 16 px Arial, regular.
fn font() -> Font {
    Font {
        family: FontFamily::Arial,
        em_px: 16.0,
        bold: false,
    }
}

fn bitmap(file: &str) -> Bitmap {
    let mut d = Bitmap::new(RectangleF::new(10.0, 20.0, 100.0, 50.0));
    d.file = file.into();
    d
}

#[test]
fn bitmap_is_a_grey_placeholder_named_after_its_file() {
    let scene = Scene::new((0, 0), 1.0);
    let mut r = surface();
    paint_bitmap(&bitmap("photo.png"), &scene.ctx(0), &mut r);
    let grey = Brush::solid(Color::argb(255, 128, 128, 128));
    assert_eq!(
        r.calls,
        vec![
            Call::FillRectangle(
                RectangleF::new(10.0, 20.0, 100.0, 50.0),
                grey,
                Transform::identity()
            ),
            Call::DrawText(
                "photo.png".into(),
                font(),
                Brush::solid(Color::argb(255, 0, 0, 0)),
                PointF::new(10.0, 20.0),
                Transform::identity()
            ),
        ]
    );
}

#[test]
fn sticker_placeholder_is_named_after_its_reference() {
    let mut d = bitmap("ignored.png");
    d.is_sticker = true;
    d.sticker_reference = "arrow".into();
    let scene = Scene::new((0, 0), 1.0);
    let mut r = surface();
    paint_bitmap(&d, &scene.ctx(0), &mut r);
    assert!(matches!(&r.calls[1], Call::DrawText(t, ..) if t == "arrow"));
}

#[test]
fn placeholder_follows_the_viewport_scale_and_origin() {
    // Origin (10, 10), scale 2: the box is (30, 50, 200, 100).
    let scene = Scene::new((10, 10), 2.0);
    let mut r = surface();
    paint_bitmap(&bitmap("photo.png"), &scene.ctx(0), &mut r);
    assert!(
        matches!(&r.calls[0], Call::FillRectangle(rect, _, _) if *rect == RectangleF::new(30.0, 50.0, 200.0, 100.0))
    );
}

#[test]
fn placeholder_is_left_out_at_zero_opacity() {
    let mut d = bitmap("photo.png");
    d.header.fading.average_timestamps_per_frame = 10.0;
    let scene = Scene::new((0, 0), 1.0);
    let mut r = surface();
    paint_bitmap(&d, &scene.ctx(200), &mut r);
    assert!(r.calls.is_empty());
}

#[test]
fn svg_is_not_placed_because_the_model_keeps_no_bounds() {
    let scene = Scene::new((0, 0), 1.0);
    let mut r = surface();
    paint_svg(&Svg::new("sticker.svg"), &scene.ctx(0), &mut r);
    assert!(r.calls.is_empty());
}
