use crate::import::{import_file, ImportError};
use kinovea_fixtures::fixtures_root;
use kinovea_geometry::Size;
use kinovea_metadata::ids::normalize_ids;
use kinovea_metadata::{write_kva, ExportFlags, Metadata};
use std::path::{Path, PathBuf};

/// The repository root, located as the metadata parity tests do.
pub(crate) fn repo() -> PathBuf {
    fixtures_root().join("../..")
}

/// An empty document with the context the oracle applies to every import (`KvaArea.ApplyContext`).
pub(crate) fn document(case: &str) -> Metadata {
    let mut m = Metadata::new(Size::new(1920, 1080));
    m.video.average_timestamps_per_frame = 400.0;
    m.video.average_timestamps_per_second = 10000.0;
    m.video.baseline_frame_interval_ms = 40.0;
    m.video.video_path = format!("{case}.mp4");
    m
}

/// KVA text as the oracle writes it, with ids normalised.
pub(crate) fn kva_text(m: &Metadata) -> String {
    normalize_ids(&write_kva(m, "0.0", false, ExportFlags::FULL).to_document())
}

/// Compares the document with `tests/fixtures/import/<case>/imported.kva`, naming the first difference.
pub(crate) fn assert_matches_oracle(m: &Metadata, case: &str) {
    let want = std::fs::read_to_string(fixtures_root().join(format!("import/{case}/imported.kva")))
        .unwrap();
    let got = kva_text(m);
    if got != want {
        let first = got
            .lines()
            .zip(want.lines())
            .position(|(g, w)| g != w)
            .unwrap_or(got.lines().count().min(want.lines().count()));
        panic!(
            "{case}: first difference at line {}\n got: {:?}\nwant: {:?}",
            first + 1,
            got.lines().nth(first),
            want.lines().nth(first)
        );
    }
}

/// An empty scratch directory for one test.
pub(crate) fn scratch_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("kinovea-import-{}-{tag}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// Copies a repository file into `dir` under `name`.
pub(crate) fn copy_into(dir: &Path, source: &str, name: &str) -> PathBuf {
    let target = dir.join(name);
    std::fs::copy(repo().join(source), &target).unwrap();
    target
}

#[test]
fn trc_file_dispatches_by_extension() {
    let dir = scratch_dir("trc");
    let path = copy_into(&dir, "tests/data/import/clip.trc", "clip.trc");
    let mut m = document("clip");
    let report = import_file(&mut m, &path).unwrap();
    assert_eq!(report.tracks, 22);
    assert_eq!(m.keyframes.len(), 1);
}

#[test]
fn srt_file_dispatches_by_extension_in_any_case() {
    let dir = scratch_dir("srt");
    let path = copy_into(&dir, "tests/data/import/clip.srt", "CLIP.SRT");
    let mut m = document("clip");
    let report = import_file(&mut m, &path).unwrap();
    assert_eq!(report.keyframes, 3);
}

#[test]
fn keypoints_file_dispatches_to_the_folder_importer() {
    let dir = scratch_dir("openpose");
    for frame in 0..3 {
        let name = format!("clip_{frame:012}_keypoints.json");
        copy_into(&dir, &format!("tests/data/import/openpose/{name}"), &name);
    }
    let mut m = document("clip");
    let report = import_file(&mut m, &dir.join("clip_000000000000_keypoints.json")).unwrap();
    assert_eq!(report.keyframes, 3);
}

#[test]
fn kva_file_is_read_into_the_document() {
    let mut m = document("sample");
    let path = repo().join("Tools/XML/Schema/samples/complete.kva");
    import_file(&mut m, &path).unwrap();
    assert_eq!(m.keyframes.len(), 9);
}

#[test]
fn unknown_extension_is_an_error() {
    let mut m = document("notes");
    let err = import_file(&mut m, Path::new("notes.txt")).unwrap_err();
    assert!(
        matches!(err, ImportError::Unsupported(ref name) if name == "notes.txt"),
        "{err}"
    );
    assert!(m.keyframes.is_empty());
}
