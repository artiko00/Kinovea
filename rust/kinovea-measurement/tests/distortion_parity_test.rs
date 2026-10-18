//! Lens distortion, KVA fragments and camera pose against `tests/fixtures/calibration/`.

mod support;
use support::*;

use kinovea_geometry::Size;
use kinovea_measurement::distortion::{export_kinovea, import_agisoft, import_kinovea};
use kinovea_measurement::{DistortionHelper, DistortionParameters};

const SIZE: Size = Size::new(1920, 1080);

/// `tests/cases/calibration/distortion.json` parameters.
fn parameters() -> DistortionParameters {
    DistortionParameters::for_image(
        -0.12, 0.03, 0.0, 0.0005, -0.0003, 1100.0, 1090.0, 950.0, 530.0, SIZE,
    )
}

#[test]
fn points_and_lines_match_within_1e_3_px() {
    let mut helper = DistortionHelper::default();
    helper.initialize(parameters(), SIZE);
    let (header, rows) = table("distortion/points.csv");
    assert_eq!(rows.len(), 60);
    for cells in &rows {
        let r = Row {
            header: &header,
            cells,
        };
        let q = r.point("x", "y");
        assert_pt(
            helper.distort(q),
            r.point("dx", "dy"),
            1e-3,
            &format!("distort {q:?}"),
        );
        assert_pt(
            helper.undistort(q),
            r.point("ux", "uy"),
            1e-3,
            &format!("undistort {q:?}"),
        );
    }
    let (header, rows) = table("distortion/lines.csv");
    assert_eq!(rows.len(), 120);
    for cells in &rows {
        let r = Row {
            header: &header,
            cells,
        };
        let (s, e) = (r.point("sx", "sy"), r.point("ex", "ey"));
        let got = if r.text("kind") == "line" {
            helper.distort_line(s, e)
        } else {
            helper.distort_rectified_line(s, e)
        };
        for (i, g) in got.iter().enumerate() {
            assert_pt(
                *g,
                r.point(&format!("p{i}x"), &format!("p{i}y")),
                1e-3,
                &format!("{} {s:?}-{e:?} [{i}]", r.text("kind")),
            );
        }
    }
}

#[test]
fn kinovea_file_round_trips_and_agisoft_parameters_match() {
    let file = fixture("distortion/kinovea.xml");
    let want = std::fs::read_to_string(&file).unwrap();
    let mut imported = import_kinovea(&file, SIZE).unwrap();
    imported.path.clear();
    let out = std::env::temp_dir().join(format!("kinovea-parity-{}.xml", std::process::id()));
    export_kinovea(&out, &imported, SIZE).unwrap();
    assert_eq!(std::fs::read_to_string(&out).unwrap(), want);
    std::fs::remove_file(&out).unwrap();

    let (header, rows) = table("distortion/agisoft.csv");
    assert_eq!(rows.len(), 2);
    for cells in &rows {
        let r = Row {
            header: &header,
            cells,
        };
        let size = Size::new(r.f("width") as i32, r.f("height") as i32);
        let a = import_agisoft(&case("agisoft.xml"), size).unwrap();
        for (name, got) in [
            ("fx", a.fx),
            ("fy", a.fy),
            ("cx", a.cx),
            ("cy", a.cy),
            ("k1", a.k1),
            ("k2", a.k2),
            ("k3", a.k3),
            ("p1", a.p1),
            ("p2", a.p2),
            ("ppmm", a.pixels_per_millimeter),
        ] {
            assert_f(got, r.f(name), 1e-9, &format!("agisoft {size:?} {name}"));
        }
    }
}
