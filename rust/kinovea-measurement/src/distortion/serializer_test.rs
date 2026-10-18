use crate::distortion::serializer::{from_element, to_element};
use crate::distortion::{export_kinovea, import_kinovea, DistortionParameters, ImportError};
use kinovea_core::preferences::dom::Element;
use kinovea_geometry::Size;

fn params() -> DistortionParameters {
    DistortionParameters::new(-0.25, 0.08, -0.01, 0.001, -0.0005, 1100.0, 1090.0, 950.0, 530.0, 1920.0 / 6.16)
}

#[test]
fn writes_with_and_without_image_size() {
    let with = to_element(&params(), Some(Size::new(1920, 1080)));
    assert_eq!(with.children[0].name, "ImageSize");
    assert_eq!(with.children[0].text, "1920;1080");
    assert_eq!(with.child_text("PixelsPerMM"), Some("311.688311688312"));
    assert_eq!(with.child_text("K1"), Some("-0.25"));
    assert_eq!(with.child_text("Path"), Some(""));
    let without = to_element(&params(), None);
    assert_eq!(without.children[0].name, "Fx");
    assert_eq!(without.children.len(), 11);
}

#[test]
fn reads_with_rescale_and_default_pixels_per_mm() {
    let e = to_element(&params(), Some(Size::new(1920, 1080)));
    let mut same = from_element(&e, Size::new(1920, 1080));
    assert!((same.pixels_per_millimeter - params().pixels_per_millimeter).abs() < 1e-12);
    same.pixels_per_millimeter = params().pixels_per_millimeter;
    assert_eq!(same, params());
    let half = from_element(&e, Size::new(960, 540));
    assert_eq!((half.fx, half.fy, half.cx, half.cy), (550.0, 545.0, 475.0, 265.0));
    assert_eq!((half.k1, half.p2), (-0.25, -0.0005));
    let mut no_ppmm = Element::new("CameraCalibration");
    no_ppmm.push_leaf("Fx", "500").push_leaf("Unknown", "x");
    let p = from_element(&no_ppmm, Size::new(1232, 700));
    assert!((p.pixels_per_millimeter - 200.0).abs() < 1e-12);
    assert_eq!((p.fx, p.fy, p.cx), (500.0, 1.0, 0.0));
}

#[test]
fn file_round_trip_and_errors() {
    let dir = std::env::temp_dir().join(format!("kinovea-lens-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("lens.xml");
    export_kinovea(&file, &params(), Size::new(1920, 1080)).unwrap();
    let text = std::fs::read_to_string(&file).unwrap();
    assert!(text.starts_with("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<CameraCalibration>\n  <ImageSize>1920;1080</ImageSize>\n"));
    assert!(text.contains("  <Path />\n"));
    let imported = import_kinovea(&file, Size::new(1920, 1080)).unwrap();
    assert_eq!(imported.path, file.display().to_string());
    let mut cleared = imported.clone();
    cleared.path.clear();
    cleared.pixels_per_millimeter = params().pixels_per_millimeter;
    assert_eq!(cleared, params());

    let missing = dir.join("missing.xml");
    assert_eq!(import_kinovea(&missing, Size::new(1, 1)), Err(ImportError::NotFound(missing.display().to_string())));
    std::fs::write(&file, "<CameraCalibration><Fx>1").unwrap();
    assert!(matches!(import_kinovea(&file, Size::new(1, 1)), Err(ImportError::Malformed { .. })));
    assert!(ImportError::NotFound("x".into()).to_string().contains("not found: x"));
    std::fs::remove_dir_all(&dir).unwrap();
}
