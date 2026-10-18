use crate::{approx_eq, FixtureError, Manifest};
use std::fs;

#[test]
fn parses_manifest_fields() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("manifest.json");
    fs::write(
        &path,
        r#"{"commit":"abc123","ffmpeg_version":"8.1.2","case_sha256":"deadbeef"}"#,
    )
    .unwrap();
    let m = Manifest::load(&path).unwrap();
    assert_eq!(m.commit, "abc123");
    assert_eq!(m.ffmpeg_version, "8.1.2");
    assert_eq!(m.case_sha256, "deadbeef");
}

#[test]
fn missing_manifest_names_the_path() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("nope").join("manifest.json");
    let err = Manifest::load(&path).unwrap_err();
    assert!(matches!(err, FixtureError::Missing(_)));
    assert!(err.to_string().contains("nope"), "message was: {err}");
}

#[test]
fn approx_eq_respects_tolerance() {
    assert!(approx_eq(1.0, 1.0 + 1e-9, 1e-6));
    assert!(!approx_eq(1.0, 1.1, 1e-6));
    assert!(!approx_eq(f64::NAN, f64::NAN, 1e-6));
}

#[test]
fn fixtures_root_points_inside_repo() {
    let root = crate::fixtures_root();
    assert!(root.ends_with("tests/fixtures"), "{}", root.display());
    assert!(root
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("Kinovea.VS2019.sln")
        .exists());
}

#[test]
fn sha256_of_empty_input() {
    assert_eq!(
        crate::sha256_hex(b""),
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
}

#[test]
fn assert_close_accepts_tolerance_and_nan_pairs() {
    crate::assert_close(1.0, 1.0005, 1e-3, "close");
    crate::assert_close(f64::NAN, f64::NAN, 0.0, "nan");
}

#[test]
#[should_panic(expected = "want NaN")]
fn assert_close_rejects_a_number_for_nan() {
    crate::assert_close(1.0, f64::NAN, 0.0, "nan");
}
