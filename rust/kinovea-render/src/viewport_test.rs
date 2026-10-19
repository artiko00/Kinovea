//! Image to viewport transform (`ImageToViewportTransformer`, design D3).
use crate::viewport::*;
use kinovea_geometry::PointF;

#[test]
fn viewport_module_is_declared() {
    assert!(module_path!().contains("viewport"));
}

// ImageToViewportTransformer.cs:65 Transform(Point) with Extensions.cs:41 Scale(double) (truncation).
#[test]
fn integer_point_scales_then_truncates_then_translates() {
    let vp = ImageToViewport::new(Point::new(10, 20), 2.0);
    assert_eq!(vp.transform_point(Point::new(3, 4)), Point::new(16, 28));
    let half = ImageToViewport::new(Point::new(10, 20), 0.5);
    assert_eq!(half.transform_point(Point::new(3, 5)), Point::new(11, 22));
    let identity = ImageToViewport::new(Point::default(), 1.0);
    assert_eq!(
        identity.transform_point(Point::new(7, -4)),
        Point::new(7, -4)
    );
}

// ImageToViewportTransformer.cs:69 Transform(PointF) with Extensions.cs:59 ToPoint (truncation).
#[test]
fn float_point_is_truncated_toward_zero_after_translation() {
    let vp = ImageToViewport::new(Point::new(10, 20), 2.0);
    assert_eq!(
        vp.transform_point_f(PointF::new(1.5, 2.25)),
        Point::new(13, 24)
    );
    let half = ImageToViewport::new(Point::new(10, 20), 0.5);
    assert_eq!(
        half.transform_point_f(PointF::new(3.0, 5.0)),
        Point::new(11, 22)
    );
    let identity = ImageToViewport::new(Point::default(), 1.0);
    assert_eq!(
        identity.transform_point_f(PointF::new(-3.2, 0.0)),
        Point::new(-3, 0)
    );
}

// ImageToViewportTransformer.cs:73 Transform(IEnumerable<PointF>).
#[test]
fn point_list_maps_each_point() {
    let vp = ImageToViewport::new(Point::new(10, 20), 2.0);
    let points = [PointF::new(1.5, 2.25), PointF::new(3.0, 5.0)];
    assert_eq!(
        vp.transform_points_f(&points),
        vec![Point::new(13, 24), Point::new(16, 30)]
    );
}

// ImageToViewportTransformer.cs:77 Transform(int) and :81 Transform(float):
// (int)Math.Round(distance * scale), Math.Round uses ties-to-even.
#[test]
fn distances_round_half_to_even() {
    let half = ImageToViewport::new(Point::default(), 0.5);
    let rounded: Vec<i32> = [1, 3, 5, 7]
        .iter()
        .map(|d| half.transform_distance(*d))
        .collect();
    assert_eq!(rounded, vec![0, 2, 2, 4]);
    assert_eq!(half.transform_distance_f(2.5), 1);
    let double = ImageToViewport::new(Point::new(10, 20), 2.0);
    assert_eq!(double.transform_distance(3), 6);
    assert_eq!(double.transform_distance_f(2.5), 5);
}

// Round trip Transform then Untransform (ImageToViewportTransformer.cs:116) is exact when the
// image coordinates are multiples of 1/scale, which holds for integers at scales 1 and 2.
#[test]
fn round_trip_is_exact_for_aligned_coordinates() {
    let location = Point::new(10, 20);
    for (scale, image) in [
        (1.0, PointF::new(3.0, 4.0)),
        (2.0, PointF::new(3.0, 4.0)),
        (0.5, PointF::new(4.0, 6.0)),
    ] {
        let vp = ImageToViewport::new(location, scale);
        let viewport = vp.transform_point_f(image);
        assert_eq!(vp.untransform_point(viewport), image);
    }
}
