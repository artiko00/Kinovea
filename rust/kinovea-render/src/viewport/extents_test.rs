//! `Transform` overloads for sizes, rectangles, lists and quadrilaterals.
use super::*;
use kinovea_geometry::{PointF, Quadrilateral, RectangleF, Size, SizeF};

// ImageToViewportTransformer.cs:85 Transform(Size) with Extensions.cs:119 (int)(w * scale).
#[test]
fn size_truncates_each_dimension() {
    let half = ImageToViewport::new(Point::default(), 0.5);
    assert_eq!(half.transform_size(Size::new(10, 7)), Size::new(5, 3));
}

// ImageToViewportTransformer.cs:89 Transform(SizeF) with Extensions.cs:130 ToSize.
#[test]
fn float_size_scales_then_truncates() {
    let double = ImageToViewport::new(Point::default(), 2.0);
    assert_eq!(
        double.transform_size_f(SizeF::new(10.5, 7.0)),
        Size::new(21, 14)
    );
    let half = ImageToViewport::new(Point::default(), 0.5);
    assert_eq!(half.transform_size_f(SizeF::new(7.0, 3.0)), Size::new(3, 1));
}

// ImageToViewportTransformer.cs:93 Transform(Rectangle): location via line 65, size via line 85.
#[test]
fn integer_rectangle_maps_location_and_size() {
    let vp = ImageToViewport::new(Point::new(5, 6), 0.5);
    let r = Rectangle::new(10, 20, 30, 40);
    assert_eq!(vp.transform_rect(r), Rectangle::new(10, 16, 15, 20));
}

// ImageToViewportTransformer.cs:97 Transform(RectangleF): location via line 69, size via line 89.
#[test]
fn float_rectangle_maps_location_and_size() {
    let vp = ImageToViewport::new(Point::default(), 2.0);
    let r = RectangleF::new(10.5, 20.5, 31.0, 41.0);
    assert_eq!(vp.transform_rect_f(r), Rectangle::new(21, 41, 62, 82));
}

// ImageToViewportTransformer.cs:101 Transform(List<Rectangle>).
#[test]
fn rectangle_list_maps_each_rectangle() {
    let vp = ImageToViewport::new(Point::new(1, 1), 2.0);
    let rects = [Rectangle::new(0, 0, 2, 2)];
    assert_eq!(vp.transform_rects(&rects), vec![Rectangle::new(1, 1, 4, 4)]);
}

// ImageToViewportTransformer.cs:105 Transform(QuadrilateralF): each vertex goes through the
// truncating PointF transform, so the result has whole-pixel coordinates.
#[test]
fn quadrilateral_vertices_are_whole_pixels() {
    let vp = ImageToViewport::new(Point::new(1, 1), 2.0);
    let q = Quadrilateral::new(
        PointF::new(0.5, 0.5),
        PointF::new(10.5, 0.5),
        PointF::new(10.5, 10.5),
        PointF::new(0.5, 10.5),
    );
    let expected = Quadrilateral::new(
        PointF::new(2.0, 2.0),
        PointF::new(22.0, 2.0),
        PointF::new(22.0, 22.0),
        PointF::new(2.0, 22.0),
    );
    assert_eq!(vp.transform_quad(q), expected);
    let half = ImageToViewport::new(Point::default(), 0.5);
    let q = Quadrilateral::new(
        PointF::new(3.0, 3.0),
        PointF::new(3.0, 3.0),
        PointF::new(3.0, 3.0),
        PointF::new(3.0, 3.0),
    );
    assert_eq!(half.transform_quad(q).a(), PointF::new(1.0, 1.0));
}
