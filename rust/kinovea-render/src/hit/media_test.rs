use super::common::testing::Scene;
use super::media::*;
use super::Hit;
use kinovea_geometry::{PointF, RectangleF};
use kinovea_metadata::drawing::Bitmap;

fn p(x: f32, y: f32) -> PointF {
    PointF::new(x, y)
}

// BoundingBox.cs:90-110: corners are handles 1-4 in order, the inside is the body.
#[test]
fn bounding_box_corners_then_body() {
    let scene = Scene::at_scale(1.0);
    let ctx = scene.ctx(0);
    let r = RectangleF::new(100.0, 100.0, 100.0, 60.0);
    assert_eq!(hit_bounding_box(r, p(100.0, 100.0), &ctx), Hit::Handle(1));
    assert_eq!(hit_bounding_box(r, p(200.0, 100.0), &ctx), Hit::Handle(2));
    assert_eq!(hit_bounding_box(r, p(200.0, 160.0), &ctx), Hit::Handle(3));
    assert_eq!(hit_bounding_box(r, p(150.0, 130.0), &ctx), Hit::Body);
    assert_eq!(hit_bounding_box(r, p(10.0, 10.0), &ctx), Hit::Miss);
}

// DrawingBitmap.cs:160-170: a sticker without its reference is not valid and is never hit.
#[test]
fn invalid_bitmap_is_never_hit() {
    let scene = Scene::at_scale(1.0);
    let mut bitmap = Bitmap::new(RectangleF::new(100.0, 100.0, 100.0, 60.0));
    bitmap.is_sticker = true;
    bitmap.sticker_reference = String::new();
    assert_eq!(
        hit_bitmap(&bitmap, p(150.0, 130.0), &scene.ctx(0)),
        Hit::Miss
    );
}
