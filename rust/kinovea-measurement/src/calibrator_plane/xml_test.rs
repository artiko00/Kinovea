use crate::calibration_axis::CalibrationAxis;
use crate::calibrator_plane::CalibratorPlane;
use kinovea_core::preferences::dom::Element;
use kinovea_geometry::{PointF, Quadrilateral, SizeF};

fn p(x: f32, y: f32) -> PointF {
    PointF::new(x, y)
}

fn wrap(name: &str, children: Vec<Element>) -> Element {
    let mut e = Element::new(name);
    e.children = children;
    e
}

#[test]
fn plane_fragment_matches_the_expected_text() {
    let mut c = CalibratorPlane::default();
    c.initialize_plane(SizeF::new(4.0, 3.0), Quadrilateral::new(p(300.0, 200.0), p(1500.0, 240.0), p(1700.0, 900.0), p(100.0, 880.0)));
    c.offset = p(0.5, -0.25);
    let text = wrap("CalibrationPlane", c.write_plane_xml()).to_document();
    let want = "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<CalibrationPlane>\n  <Size>4;3</Size>\n  <Quadrilateral>\n    <A>300;200</A>\n    <B>1500;240</B>\n    <C>1700;900</C>\n    <D>100;880</D>\n  </Quadrilateral>\n  <Origin>0;3</Origin>\n  <Offset>0.5;-0.25</Offset>\n</CalibrationPlane>";
    assert_eq!(text, want);
}

#[test]
fn line_fragment_matches_the_expected_text() {
    let mut c = CalibratorPlane::default();
    c.initialize_line(2.5, p(100.0, 500.0), p(300.0, 500.0), CalibrationAxis::LineVertical);
    let text = wrap("CalibrationLine", c.write_line_xml()).to_document();
    let want = "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<CalibrationLine>\n  <Length>2.5</Length>\n  <Segment>\n    <A>100;500</A>\n    <B>100;700</B>\n  </Segment>\n  <Origin>0;2.5</Origin>\n  <Offset>0;0</Offset>\n  <Axis>LineVertical</Axis>\n</CalibrationLine>";
    assert_eq!(text, want);
}

#[test]
fn reading_a_plane_with_scale_halves_the_quad_and_ignores_unknowns() {
    let mut c = CalibratorPlane::default();
    c.initialize_plane(SizeF::new(4.0, 3.0), Quadrilateral::new(p(300.0, 200.0), p(1500.0, 240.0), p(1700.0, 900.0), p(100.0, 880.0)));
    let mut e = wrap("CalibrationPlane", c.write_plane_xml());
    e.push_leaf("Future", "ignored");
    let mut read = CalibratorPlane::default();
    read.read_plane_xml(&e, p(0.5, 0.5));
    assert_eq!(read.size, SizeF::new(4.0, 3.0));
    assert_eq!(read.quad_image().a(), p(150.0, 100.0));
    assert_eq!(read.origin(), c.origin());
    assert!(read.is_valid());
    let again = wrap("CalibrationPlane", read.write_plane_xml());
    assert_eq!(again.child("Quadrilateral").unwrap().child_text("C"), Some("850;450"));
}

#[test]
fn reading_a_line_scales_origin_and_segment_and_handles_legacy_scale() {
    let mut c = CalibratorPlane::default();
    c.initialize_line(2.0, p(100.0, 500.0), p(300.0, 500.0), CalibrationAxis::LineHorizontal);
    let e = wrap("CalibrationLine", c.write_line_xml());
    let mut read = CalibratorPlane::default();
    read.read_line_xml(&e, p(0.5, 0.5));
    assert_eq!(read.size.width, 2.0);
    // The original scales the origin but leaves the segment untouched.
    assert_eq!(read.quad_image().d(), p(100.0, 500.0));
    assert_eq!(read.origin(), p(0.0, 1.0));
    assert_eq!(read.axis(), CalibrationAxis::LineHorizontal);
    let w = read.transform(p(200.0, 400.0));
    assert!((w.x - 1.0).abs() < 1e-4 && (w.y - 0.0).abs() < 1e-4, "{w:?}");

    let mut legacy = Element::new("CalibrationLine");
    legacy.push_leaf("Origin", "200;300").push_leaf("Scale", "0.02");
    let mut old = CalibratorPlane::default();
    old.read_line_xml(&legacy, p(1.0, 1.0));
    assert_eq!(old.size.width, 2.0);
    assert_eq!(old.quad_image().d(), p(200.0, 300.0));
    assert_eq!(old.quad_image().c(), p(300.0, 300.0));
    assert_eq!(old.origin(), p(0.0, 2.0));
}
