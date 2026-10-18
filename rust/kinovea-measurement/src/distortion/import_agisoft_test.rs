use crate::distortion::{import_agisoft, ImportError};
use kinovea_geometry::Size;

const SAMPLE: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<calibration>\n  <projection>frame</projection>\n  <width>1920</width>\n  <height>1080</height>\n  <fx>1100.5</fx>\n  <fy>1098.25</fy>\n  <cx>955.5</cx>\n  <cy>542.75</cy>\n  <k1>-0.21</k1>\n  <k2>0.07</k2>\n  <k3>-0.012</k3>\n  <p1>0.0008</p1>\n  <p2>-0.0003</p2>\n  <date>2024-01-01</date>\n</calibration>\n";

fn write(name: &str, text: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("kinovea-agisoft-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join(name);
    std::fs::write(&file, text).unwrap();
    file
}

#[test]
fn valid_file_imports_and_rescales() {
    let file = write("valid.xml", SAMPLE);
    let p = import_agisoft(&file, Size::new(1920, 1080)).unwrap();
    assert_eq!((p.fx, p.fy, p.cx, p.cy), (1100.5, 1098.25, 955.5, 542.75));
    assert_eq!((p.k1, p.k2, p.k3, p.p1, p.p2), (-0.21, 0.07, -0.012, 0.0008, -0.0003));
    assert!((p.pixels_per_millimeter - 1920.0 / 6.16).abs() < 1e-12);
    let half = import_agisoft(&file, Size::new(960, 540)).unwrap();
    assert_eq!((half.fx, half.cx, half.cy), (550.25, 477.75, 271.375));
    assert_eq!(half.k1, -0.21);
}

#[test]
fn missing_and_malformed_files_are_errors() {
    let missing = std::env::temp_dir().join("kinovea-agisoft-does-not-exist.xml");
    assert_eq!(import_agisoft(&missing, Size::new(1, 1)), Err(ImportError::NotFound(missing.display().to_string())));
    let bad = write("bad.xml", "<calibration><width>1</width>");
    assert!(matches!(import_agisoft(&bad, Size::new(1, 1)), Err(ImportError::Malformed { .. })));
    let incomplete = write("incomplete.xml", "<calibration><width>1</width><height>1</height></calibration>");
    let err = import_agisoft(&incomplete, Size::new(1, 1)).unwrap_err();
    assert!(err.to_string().contains("missing fx"), "{err}");
    let wrong_root = write("root.xml", "<other/>");
    assert!(import_agisoft(&wrong_root, Size::new(1, 1)).unwrap_err().to_string().contains("root element"));
}
