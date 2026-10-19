//! Hit testing as geometry (`HitTester`, design D5): point boxes, widened paths and lines.
use crate::hit::*;
use crate::viewport::{ImageToViewport, Point};
use kinovea_geometry::{PointF, RectangleF};

fn vp(scale: f64) -> ImageToViewport {
    ImageToViewport::new(Point::default(), scale)
}

fn p(x: f32, y: f32) -> PointF {
    PointF::new(x, y)
}

// HitTester.cs:20-24 HitPoint: box of max(1, Untransform(10)) around the target, Contains().
#[test]
fn point_box_is_ten_viewport_pixels_with_right_edge_exclusive() {
    let target = p(100.0, 100.0);
    assert!(hit_point(p(109.9, 100.0), target, &vp(1.0)));
    assert!(!hit_point(p(110.0, 100.0), target, &vp(1.0)));
    assert!(hit_point(p(90.0, 90.0), target, &vp(1.0)));
    assert!(!hit_point(p(89.9, 100.0), target, &vp(1.0)));
}

// Spec: a point 8 image pixels away is a miss at scale 2 and a hit at scale 0.5.
#[test]
fn point_tolerance_shrinks_when_zoomed_in() {
    let target = p(100.0, 100.0);
    assert!(hit_point(p(104.0, 100.0), target, &vp(2.0)));
    assert!(!hit_point(p(108.0, 100.0), target, &vp(2.0)));
    assert!(hit_point(p(108.0, 100.0), target, &vp(0.5)));
}

// HitTester.cs:22 with ImageToViewportTransformer.cs:132: the box never drops below one pixel.
#[test]
fn point_box_has_a_minimum_of_one_image_pixel() {
    let target = p(100.0, 100.0);
    assert!(hit_point(p(100.9, 100.0), target, &vp(100.0)));
    assert!(!hit_point(p(101.0, 100.0), target, &vp(100.0)));
}

// HitTester.cs:81 HitLine: coincident ends never hit.
#[test]
fn line_with_coincident_ends_never_hits() {
    assert!(!hit_line(
        p(50.0, 50.0),
        p(50.0, 50.0),
        p(50.0, 50.0),
        &vp(1.0)
    ));
}

// HitTester.cs:93 HitLine uses lineSize 1; HitPath widens by (1 + 10) so the half width is 5.5.
#[test]
fn line_hit_uses_half_of_line_size_plus_tolerance() {
    let (a, b) = (p(0.0, 0.0), p(100.0, 0.0));
    assert!(hit_line(p(50.0, 5.4), a, b, &vp(1.0)));
    assert!(!hit_line(p(50.0, 5.6), a, b, &vp(1.0)));
}

// HitTester.cs:40-55 HitPath: pen width is lineSize + enlarger, so the half width is (lineSize + enlarger) / 2.
#[test]
fn widened_polyline_half_width_is_line_size_plus_tolerance() {
    let mut path = GraphicsPath::new();
    path.add_line(p(0.0, 0.0), p(100.0, 0.0));
    assert!(hit_path(p(50.0, 5.9), &path, 2, false, &vp(1.0)));
    assert!(!hit_path(p(50.0, 6.5), &path, 2, false, &vp(1.0)));
}

// ImageToViewportTransformer.cs:132 at scales 2 and 0.5: enlarger 5 and 20.
#[test]
fn polyline_tolerance_follows_the_viewport_scale() {
    let mut path = GraphicsPath::new();
    path.add_line(p(0.0, 0.0), p(100.0, 0.0));
    assert!(hit_path(p(50.0, 3.4), &path, 2, false, &vp(2.0)));
    assert!(!hit_path(p(50.0, 3.6), &path, 2, false, &vp(2.0)));
    assert!(hit_path(p(50.0, 10.5), &path, 2, false, &vp(0.5)));
}

// HitTester.cs:45-52: a single point is a circle of radius max(5, (int)(width / 2)).
#[test]
fn single_point_radius_is_truncated_half_width_with_minimum_five() {
    let mut path = GraphicsPath::new();
    path.add_line(p(50.0, 50.0), p(50.0, 50.0));
    // width 11, half 5.5, truncated to 5.
    assert!(hit_path(p(54.9, 50.0), &path, 1, false, &vp(1.0)));
    assert!(!hit_path(p(55.4, 50.0), &path, 1, false, &vp(1.0)));
}

// HitTester.cs:45-52: a half width larger than five is used as the radius.
#[test]
fn single_point_radius_grows_with_line_size() {
    let mut path = GraphicsPath::new();
    path.add_lines(&[p(50.0, 50.0)]);
    // width 30, half 15.
    assert!(hit_path(p(64.9, 50.0), &path, 20, false, &vp(1.0)));
    assert!(!hit_path(p(65.1, 50.0), &path, 20, false, &vp(1.0)));
}

// HitTester.cs:66 Region(path).IsVisible for filled areas: anywhere inside, at any scale.
#[test]
fn filled_area_hits_anywhere_inside_at_any_scale() {
    let mut path = GraphicsPath::new();
    path.add_rectangle(RectangleF::new(0.0, 0.0, 10.0, 10.0));
    for scale in [0.5, 2.0] {
        assert!(hit_path(p(5.0, 5.0), &path, 1, true, &vp(scale)));
        assert!(!hit_path(p(11.0, 5.0), &path, 1, true, &vp(scale)));
    }
}

// The fill uses the alternate (even-odd) rule, so a nested rectangle is a hole.
#[test]
fn nested_figure_is_a_hole_in_the_fill() {
    let mut path = GraphicsPath::new();
    path.add_rectangle(RectangleF::new(0.0, 0.0, 10.0, 10.0));
    path.add_rectangle(RectangleF::new(2.0, 2.0, 6.0, 6.0));
    assert!(hit_path(p(1.0, 1.0), &path, 1, true, &vp(1.0)));
    assert!(!hit_path(p(5.0, 5.0), &path, 1, true, &vp(1.0)));
}

#[test]
fn empty_path_never_hits() {
    assert!(!hit_path(
        p(0.0, 0.0),
        &GraphicsPath::new(),
        1,
        false,
        &vp(1.0)
    ));
}
