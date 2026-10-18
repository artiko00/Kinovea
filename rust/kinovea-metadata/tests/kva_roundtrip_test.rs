//! Byte-level parity: load each sample like the oracle does, write it back with producer
//! `Kinovea.0.0`, normalise ids and compare with `tests/fixtures/kva/<case>/roundtrip.kva`.

use kinovea_core::preferences::dom::Element;
use kinovea_core::xml::parse_size;
use kinovea_fixtures::fixtures_root;
use kinovea_geometry::Size;
use kinovea_metadata::ids::normalize_ids;
use kinovea_metadata::{read_kva, write_kva, ExportFlags, Metadata};

/// Mirrors `KvaArea.ApplyVideoContext`: image size and timestamps per frame from the header.
fn metadata_for(xml: &str) -> Metadata {
    let root = Element::parse(xml).unwrap();
    let (w, h) = root
        .child_text("ImageSize")
        .map(parse_size)
        .unwrap_or((0, 0));
    let mut m = Metadata::new(Size::new(w, h));
    m.video.average_timestamps_per_frame = root
        .child_text("AverageTimeStampsPerFrame")
        .and_then(|t| t.trim().parse().ok())
        .unwrap_or(1.0);
    m
}

fn round_trip(case: &str) -> (String, String) {
    let repo = fixtures_root().join("../..");
    let input =
        std::fs::read_to_string(repo.join(format!("Tools/XML/Schema/samples/{case}.kva"))).unwrap();
    let want =
        std::fs::read_to_string(fixtures_root().join(format!("kva/{case}/roundtrip.kva"))).unwrap();
    let mut m = metadata_for(&input);
    read_kva(&mut m, input.trim_start_matches('\u{feff}')).unwrap();
    let got = normalize_ids(&write_kva(&m, "0.0", false, ExportFlags::FULL).to_document());
    (got, want)
}

fn assert_same(case: &str) {
    let (got, want) = round_trip(case);
    if got != want {
        let first = got
            .lines()
            .zip(want.lines())
            .position(|(g, w)| g != w)
            .unwrap_or(got.lines().count().min(want.lines().count()));
        eprintln!(
            "{case}: got {} lines, want {} lines",
            got.lines().count(),
            want.lines().count()
        );
        std::fs::write(std::env::temp_dir().join(format!("{case}-got.kva")), &got).unwrap();
        let g: Vec<&str> = got.lines().skip(first.saturating_sub(2)).take(6).collect();
        let w: Vec<&str> = want.lines().skip(first.saturating_sub(2)).take(6).collect();
        panic!(
            "{case}: first difference near line {}\n got: {g:#?}\nwant: {w:#?}",
            first + 1
        );
    }
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
