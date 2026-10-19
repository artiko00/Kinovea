//! Byte-level parity: load each sample like the oracle does, write it back with producer
//! `Kinovea.0.0`, normalise ids and compare with `tests/fixtures/kva/<case>/roundtrip.kva`.

use kinovea_fixtures::fixtures_root;
use kinovea_metadata::ids::normalize_ids;
use kinovea_metadata::{header_metadata, read_kva, write_kva, ExportFlags, Metadata};
use std::path::PathBuf;

pub fn repo() -> PathBuf {
    fixtures_root().join("../..")
}

pub fn load(input: &str) -> (Metadata, String) {
    let xml = std::fs::read_to_string(repo().join(input)).unwrap();
    let xml = xml.trim_start_matches('\u{feff}').to_owned();
    let mut m = header_metadata(&xml);
    read_kva(&mut m, &xml).unwrap();
    (m, xml)
}

pub fn written(m: &Metadata, capture: bool, flags: ExportFlags) -> String {
    normalize_ids(&write_kva(m, "0.0", capture, flags).to_document())
}

/// Panics with the first differing lines.
pub fn assert_same_text(case: &str, got: &str, want: &str) {
    if got != want {
        let first = got
            .lines()
            .zip(want.lines())
            .position(|(g, w)| g != w)
            .unwrap_or(got.lines().count().min(want.lines().count()));
        let g: Vec<&str> = got.lines().skip(first.saturating_sub(2)).take(6).collect();
        let w: Vec<&str> = want.lines().skip(first.saturating_sub(2)).take(6).collect();
        panic!(
            "{case}: first difference near line {} ({} vs {} lines)\n got: {g:#?}\nwant: {w:#?}",
            first + 1,
            got.lines().count(),
            want.lines().count()
        );
    }
}

fn assert_same(case: &str) {
    let (m, _) = load(&format!("Tools/XML/Schema/samples/{case}.kva"));
    let want =
        std::fs::read_to_string(fixtures_root().join(format!("kva/{case}/roundtrip.kva"))).unwrap();
    assert_same_text(case, &written(&m, false, ExportFlags::FULL), &want);
}

#[test]
fn minimal_sample_round_trips() {
    assert_same("minimal");
}

#[test]
fn simple_sample_round_trips() {
    assert_same("simple");
}

#[test]
fn complete_sample_round_trips() {
    assert_same("complete");
}

#[test]
fn every_drawing_kind_round_trips() {
    let (m, _) = load("tests/cases/kva/inputs/all-drawings.kva");
    let want =
        std::fs::read_to_string(fixtures_root().join("kva/all-drawings/roundtrip.kva")).unwrap();
    assert_same_text(
        "all-drawings",
        &written(&m, false, ExportFlags::FULL),
        &want,
    );
    let drawings =
        std::fs::read_to_string(fixtures_root().join("kva/all-drawings/drawings.json")).unwrap();
    for d in m
        .attached_drawings()
        .chain(m.chronos.iter())
        .chain(m.tracks.iter())
    {
        let entry = format!("\"element\": \"{}\", \"id\": \"", d.kva_name());
        assert!(
            drawings.contains(&entry),
            "{} listed by the oracle",
            d.kva_name()
        );
        assert!(
            drawings.contains(&format!("\"name\": \"{}\"", d.header().name)),
            "name {}",
            d.header().name
        );
    }
    assert_eq!(
        m.attached_drawings().count(),
        14,
        "the SVG without a file is dropped"
    );
}

#[test]
fn export_flags_select_the_sections() {
    let (m, _) = load("tests/cases/kva/inputs/all-drawings.kva");
    let cases = [
        ("capture", true, ExportFlags::CAPTURE_RECORDING),
        ("drawings", false, ExportFlags::DRAWINGS),
        ("calibration", false, ExportFlags::CALIBRATION),
        ("video", false, ExportFlags::VIDEO_SPECIFIC),
    ];
    for (name, capture, flags) in cases {
        let want = std::fs::read_to_string(fixtures_root().join(format!("kva/capture/{name}.kva")))
            .unwrap();
        assert_same_text(name, &written(&m, capture, flags), &want);
    }
}

#[test]
fn files_written_by_rust_are_a_fixed_point_of_the_original() {
    let summary =
        std::fs::read_to_string(fixtures_root().join("kva/crossload/summary.json")).unwrap();
    assert!(!summary.contains("\"identical\": false"), "{summary}");
    for case in ["minimal", "simple", "complete"] {
        let ours =
            std::fs::read_to_string(repo().join(format!("tests/cases/kva/crossload/{case}.kva")))
                .unwrap();
        let theirs =
            std::fs::read_to_string(fixtures_root().join(format!("kva/crossload/{case}.kva")))
                .unwrap();
        assert_same_text(case, &ours, &theirs);
        let (m, _) = load(&format!("Tools/XML/Schema/samples/{case}.kva"));
        assert_same_text(case, &written(&m, false, ExportFlags::FULL), &ours);
    }
}
