use crate::calibration_axis::CalibrationAxis;
use crate::calibration_helper_test::{full_hd, plane_quad};
use crate::calibrator_type::CalibratorType;
use crate::distortion::DistortionParameters;
use kinovea_core::preferences::dom::Element;
use kinovea_core::units::LengthUnit;
use kinovea_geometry::{PointF, Size, SizeF};
use uuid::Uuid;

fn p(x: f32, y: f32) -> PointF {
    PointF::new(x, y)
}

fn document(children: Vec<Element>) -> String {
    let mut root = Element::new("Calibration");
    root.children = children;
    root.to_document()
}

const ID: &str = "11111111-2222-3333-4444-555555555555";

#[test]
fn line_fragment_is_byte_exact() {
    let mut h = full_hd();
    h.calibrate_by_line(Uuid::parse_str(ID).unwrap(), 2.0, p(100.0, 500.0), p(300.0, 500.0), CalibrationAxis::LineHorizontal);
    h.set_length_unit(LengthUnit::Meters);
    let want = "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<Calibration>\n  <CalibrationLine>\n    <Length>2</Length>\n    <Segment>\n      <A>100;500</A>\n      <B>300;500</B>\n    </Segment>\n    <Origin>0;2</Origin>\n    <Offset>0;0</Offset>\n    <Axis>LineHorizontal</Axis>\n  </CalibrationLine>\n  <CalibrationDrawingId>11111111-2222-3333-4444-555555555555</CalibrationDrawingId>\n  <Unit Abbreviation=\"m\">Meters</Unit>\n</Calibration>";
    assert_eq!(document(h.write_xml()), want);
}

#[test]
fn plane_with_distortion_fragment_and_round_trip_with_scale() {
    let mut h = full_hd();
    h.set_calibrator_type(CalibratorType::Plane);
    h.calibrate_by_plane(Uuid::parse_str(ID).unwrap(), SizeF::new(4.0, 3.0), &plane_quad());
    h.set_length_unit(LengthUnit::Centimeters);
    let lens = DistortionParameters::new(-0.25, 0.08, -0.01, 0.001, -0.0005, 1100.0, 1090.0, 950.0, 530.0, 1920.0 / 6.16);
    h.distortion_mut().initialize(lens, Size::new(1920, 1080));
    let text = document(h.write_xml());
    assert!(text.contains("  <CalibrationPlane>\n    <Size>4;3</Size>\n    <Quadrilateral>\n      <A>300;200</A>\n"));
    assert!(text.contains("  <Unit Abbreviation=\"cm\">Centimeters</Unit>\n  <CameraCalibration>\n    <Fx>1100</Fx>\n"));
    assert!(text.contains("    <PixelsPerMM>311.688311688312</PixelsPerMM>\n    <Path />\n  </CameraCalibration>\n</Calibration>"));

    let root = Element::parse(&text).unwrap();
    let mut read = full_hd();
    read.read_xml(&root, p(0.5, 0.5), Size::new(960, 540));
    assert_eq!(read.calibrator_type(), CalibratorType::Plane);
    assert_eq!(read.drawing_id().to_string(), ID);
    assert_eq!(read.units().length, LengthUnit::Centimeters);
    assert_eq!(read.projected_quad().a(), p(150.0, 100.0));
    // Without `ImageSize` in the fragment the intrinsics are kept as written.
    assert_eq!(read.distortion().parameters().unwrap().fx, 1100.0);
    assert_eq!(read.distortion().parameters().unwrap().k1, -0.25);
    let again = document(read.write_xml());
    assert!(again.contains("<A>150;100</A>") && again.contains("<Size>4;3</Size>") && again.contains("<Fx>1100</Fx>"));
}

#[test]
fn unknown_elements_and_bad_ids_are_ignored() {
    let mut root = Element::new("Calibration");
    root.push_leaf("Future", "x").push_leaf("CalibrationDrawingId", "not-a-guid").push_leaf("Unit", "Furlongs");
    let mut h = full_hd();
    h.read_xml(&root, p(1.0, 1.0), Size::new(1920, 1080));
    assert_eq!(h.drawing_id(), Uuid::nil());
    assert_eq!(h.units().length, LengthUnit::Pixels);
    h.set_calibrator_type(CalibratorType::None);
    assert_eq!(h.write_xml()[0].name, "CalibrationDrawingId");
}
