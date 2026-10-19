use crate::collect_drawings::collect_keyframe;
use crate::measure::Scope;
use crate::measured_data::ExportPrefs;
use kinovea_core::preferences::enums::ExportSpace;
use kinovea_core::timecode::TimecodeFormat;
use kinovea_geometry::{PointF, Size};
use kinovea_metadata::drawing::Angle;
use kinovea_metadata::{read_kva, Drawing, Metadata};

/// A calibration with a lens distortion and an off-centre principal point, read through the KVA
/// reader as a saved document carries it.
const KVA: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<KinoveaVideoAnalysis>
  <FormatVersion>2.1</FormatVersion>
  <ImageSize>640;480</ImageSize>
  <Calibration>
    <CameraCalibration>
      <Fx>600</Fx>
      <Fy>610</Fy>
      <Cx>330</Cx>
      <Cy>250</Cy>
      <K1>-0.3</K1>
      <K2>0.1</K2>
      <K3>0</K3>
      <P1>0.001</P1>
      <P2>0</P2>
      <PixelsPerMM>10</PixelsPerMM>
    </CameraCalibration>
  </Calibration>
</KinoveaVideoAnalysis>"#;

fn distorted() -> Metadata {
    let mut m = Metadata::new(Size::new(640, 480));
    read_kva(&mut m, KVA).unwrap();
    assert!(m.calibration.distortion().is_initialized());
    m
}

fn prefs(space: ExportSpace) -> ExportPrefs {
    ExportPrefs {
        export_space: space,
        timecode_format: TimecodeFormat::ClassicTime,
        producer: String::new(),
    }
}

fn angle_in(m: &Metadata, space: ExportSpace) -> f32 {
    let p = prefs(space);
    let scope = Scope {
        metadata: m,
        prefs: &p,
    };
    let mut a = Angle::new(
        PointF::new(500.0, 400.0),
        PointF::new(560.0, 420.0),
        PointF::new(520.0, 300.0),
    );
    a.header.name = "Angle 1".to_owned();
    collect_keyframe(&scope, &[Drawing::Angle(a)], 0.0).angles[0].value
}

#[test]
fn world_points_are_rectified_before_the_calibration() {
    let m = distorted();
    let p = prefs(ExportSpace::WorldSpace);
    let scope = Scope {
        metadata: &m,
        prefs: &p,
    };
    let point = PointF::new(500.0, 400.0);
    let calibrator = m.calibration.calibrator();
    let rectified = m.calibration.distortion().undistort(point);
    let expected = calibrator.transform(rectified);
    let got = scope.point(point);
    assert!((got.x - expected.x).abs() < 1e-3 && (got.y - expected.y).abs() < 1e-3);
    let raw = calibrator.transform(point);
    assert!(
        (got.x - raw.x).abs() > 0.5 || (got.y - raw.y).abs() > 0.5,
        "the distortion moves the point"
    );
}

#[test]
fn image_space_keeps_the_raw_points() {
    let m = distorted();
    let p = prefs(ExportSpace::ImageSpace);
    let scope = Scope {
        metadata: &m,
        prefs: &p,
    };
    assert_eq!(
        scope.point(PointF::new(500.0, 400.0)),
        PointF::new(500.0, 400.0)
    );
}

#[test]
fn angles_are_rectified_in_both_spaces() {
    let m = distorted();
    let world = angle_in(&m, ExportSpace::WorldSpace);
    let image = angle_in(&m, ExportSpace::ImageSpace);
    assert!((world - image).abs() < 1e-4, "{world} != {image}");
    let plain = angle_in(&Metadata::new(Size::new(640, 480)), ExportSpace::WorldSpace);
    assert!(
        (world - plain).abs() > 1e-3,
        "the distortion changes the angle"
    );
}
