//! KVA calibration fragments and camera pose against `tests/fixtures/calibration/`.

mod support;
use support::*;

use kinovea_core::preferences::dom::Element;
use kinovea_core::units::LengthUnit;
use kinovea_geometry::{PointF, ProjectiveMapper, Quadrilateral, Size, SizeF};
use kinovea_measurement::{
    compute_camera_position, CalibrationAxis, CalibrationHelper, CalibratorType,
    DistortionParameters, Units,
};
use uuid::Uuid;

const SIZE: Size = Size::new(1920, 1080);

fn p(x: f32, y: f32) -> PointF {
    PointF::new(x, y)
}

/// `tests/cases/calibration/kva.json` distortion parameters.
fn parameters() -> DistortionParameters {
    DistortionParameters::for_image(
        -0.12, 0.03, 0.0, 0.0005, -0.0003, 1100.0, 1090.0, 950.0, 530.0, SIZE,
    )
}

fn kva_helper(plane: bool, distortion: bool) -> CalibrationHelper {
    let id = Uuid::parse_str("11111111-2222-3333-4444-555555555555").unwrap();
    let mut h = CalibrationHelper::without_tracking(SIZE, Units::default(), 25.0);
    if distortion {
        h.distortion_mut().initialize(parameters(), SIZE);
    }
    if plane {
        h.set_calibrator_type(CalibratorType::Plane);
        let quad = Quadrilateral::new(
            p(300.0, 200.0),
            p(1500.0, 240.0),
            p(1700.0, 900.0),
            p(100.0, 880.0),
        );
        h.calibrate_by_plane(id, SizeF::new(4.0, 3.0), &quad);
        h.set_origin(p(900.0, 600.0));
        h.set_offset(p(0.5, -0.25));
    } else {
        h.calibrate_by_line(
            id,
            2.0,
            p(100.0, 500.0),
            p(300.0, 500.0),
            CalibrationAxis::LineHorizontal,
        );
    }
    h.set_length_unit(LengthUnit::Meters);
    h
}

#[test]
fn kva_fragments_are_byte_identical() {
    for (file, plane, distortion) in [
        ("line.xml", false, false),
        ("plane.xml", true, false),
        ("plane-distortion.xml", true, true),
    ] {
        let want = std::fs::read_to_string(fixture(&format!("kva/{file}"))).unwrap();
        let mut root = Element::new("Calibration");
        root.children = kva_helper(plane, distortion).write_xml();
        assert_eq!(root.to_document(), want, "{file}");
    }
}

#[test]
fn camera_pose_matches_within_1e_3() {
    let lens = DistortionParameters::defaults(SIZE);
    let (header, rows) = table("pose/cases.csv");
    assert_eq!(rows.len(), 20);
    for cells in &rows {
        let r = Row {
            header: &header,
            cells,
        };
        let plane = Quadrilateral::from_size(r.f("w") as f32, r.f("h") as f32);
        let mapping = ProjectiveMapper::new(&plane, &r.quad("i")).unwrap();
        let got = compute_camera_position(&plane, &mapping, &lens);
        for (name, g) in [("x", got.x), ("y", got.y), ("z", got.z)] {
            assert_f(
                f64::from(g),
                r.f(name),
                1e-3 * r.f(name).abs().max(1.0),
                &format!("pose {name} {cells:?}"),
            );
        }
    }
}
