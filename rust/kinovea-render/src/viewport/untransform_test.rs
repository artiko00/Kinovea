//! `Untransform` overloads: viewport pixels back to image coordinates.
use super::*;
use kinovea_geometry::{PointF, RectangleF, SizeF};

// ImageToViewportTransformer.cs:116 Untransform(Point): (p - location) * (1 / scale).
#[test]
fn point_subtracts_location_then_divides_by_scale() {
    let double = ImageToViewport::new(Point::new(10, 20), 2.0);
    assert_eq!(
        double.untransform_point(Point::new(16, 28)),
        PointF::new(3.0, 4.0)
    );
    let half = ImageToViewport::new(Point::new(10, 20), 0.5);
    assert_eq!(
        half.untransform_point(Point::new(11, 22)),
        PointF::new(2.0, 4.0)
    );
    let identity = ImageToViewport::new(Point::new(10, 20), 1.0);
    assert_eq!(
        identity.untransform_point(Point::new(13, 24)),
        PointF::new(3.0, 4.0)
    );
}

// ImageToViewportTransformer.cs:122 Untransform(SizeF).
#[test]
fn float_size_divides_by_scale() {
    let half = ImageToViewport::new(Point::default(), 0.5);
    assert_eq!(
        half.untransform_size_f(SizeF::new(10.0, 6.0)),
        SizeF::new(20.0, 12.0)
    );
    let double = ImageToViewport::new(Point::default(), 2.0);
    assert_eq!(
        double.untransform_size_f(SizeF::new(3.0, 5.0)),
        SizeF::new(1.5, 2.5)
    );
}

// ImageToViewportTransformer.cs:128 Untransform(Rectangle): location via line 116, size via line 122.
#[test]
fn rectangle_maps_location_and_size_back() {
    let half = ImageToViewport::new(Point::new(5, 6), 0.5);
    let r = half.untransform_rect(Rectangle::new(10, 16, 15, 20));
    assert_eq!(r, RectangleF::new(10.0, 20.0, 30.0, 40.0));
}

// ImageToViewportTransformer.cs:132 Untransform(int): (int)(distance / scale), truncated.
#[test]
fn integer_distance_divides_and_truncates() {
    let double = ImageToViewport::new(Point::default(), 2.0);
    assert_eq!(double.untransform_distance(10), 5);
    assert_eq!(double.untransform_distance(7), 3);
    let half = ImageToViewport::new(Point::default(), 0.5);
    assert_eq!(half.untransform_distance(10), 20);
    let identity = ImageToViewport::new(Point::default(), 1.0);
    assert_eq!(identity.untransform_distance(10), 10);
}
