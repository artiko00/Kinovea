//! Legacy 1.3 and 1.5 files: the Rust conversion must equal the stylesheet output the oracle
//! produced, and loading them must give the same saved file as the original.

use kinovea_core::preferences::dom::Element;
use kinovea_fixtures::fixtures_root;
use kinovea_metadata::ids::normalize_ids;
use kinovea_metadata::migration::migrate_legacy;
use kinovea_metadata::{header_metadata, read_kva, write_kva, ExportFlags, KvaError, Metadata};

fn check(case: &str, input: &str) {
    let repo = fixtures_root().join("../..");
    let xml = std::fs::read_to_string(repo.join(input)).unwrap();
    let converted = migrate_legacy(&Element::parse(&xml).unwrap()).to_document();
    let want_converted =
        std::fs::read_to_string(fixtures_root().join(format!("kva/{case}/converted.kva"))).unwrap();
    assert_eq!(converted, want_converted, "{case}: converted document");
    let mut m = header_metadata(&xml);
    read_kva(&mut m, &xml).unwrap();
    let got = normalize_ids(&write_kva(&m, "0.0", false, ExportFlags::FULL).to_document());
    let want =
        std::fs::read_to_string(fixtures_root().join(format!("kva/{case}/roundtrip.kva"))).unwrap();
    if got != want {
        let first = got
            .lines()
            .zip(want.lines())
            .position(|(g, w)| g != w)
            .unwrap_or(0);
        let g: Vec<&str> = got.lines().skip(first.saturating_sub(2)).take(6).collect();
        let w: Vec<&str> = want.lines().skip(first.saturating_sub(2)).take(6).collect();
        panic!(
            "{case}: first difference near line {}\n got: {g:#?}\nwant: {w:#?}",
            first + 1
        );
    }
    assert!(
        std::fs::read_to_string(fixtures_root().join(format!("kva/{case}/migration.json")))
            .unwrap()
            .contains("\"relative_trajectories\": true")
    );
}

#[test]
fn format_1_5_is_migrated_and_round_trips() {
    check("migration-15", "tests/cases/kva/inputs/legacy/v15.kva");
}

#[test]
fn format_1_3_is_migrated_and_round_trips() {
    check("migration-13", "tests/cases/kva/inputs/legacy/v13.kva");
}

#[test]
fn format_1_2_is_rejected() {
    let mut m = Metadata::default();
    let err = read_kva(
        &mut m,
        "<KinoveaVideoAnalysis><FormatVersion>1.2</FormatVersion></KinoveaVideoAnalysis>",
    )
    .unwrap_err();
    assert_eq!(err, KvaError::TooOld("1.2".into()));
}
