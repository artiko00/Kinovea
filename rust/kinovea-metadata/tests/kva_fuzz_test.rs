//! Seeded random files produced by the oracle's generator and their round trip through the
//! original: the Rust writer must produce the same bytes for each.

use kinovea_fixtures::fixtures_root;
use kinovea_metadata::ids::normalize_ids;
use kinovea_metadata::{header_metadata, read_kva, write_kva, ExportFlags};

#[test]
fn fuzzed_files_round_trip_like_the_original() {
    let dir = fixtures_root().join("kva/fuzz/fuzz");
    let mut checked = 0;
    for n in 1..=20 {
        let input = std::fs::read_to_string(dir.join(format!("{n:02}.kva"))).unwrap();
        let want = std::fs::read_to_string(dir.join(format!("{n:02}.roundtrip.kva"))).unwrap();
        let mut m = header_metadata(&input);
        read_kva(&mut m, &input).unwrap();
        let got = normalize_ids(&write_kva(&m, "0.0", false, ExportFlags::FULL).to_document());
        if got != want {
            let first = got
                .lines()
                .zip(want.lines())
                .position(|(g, w)| g != w)
                .unwrap_or(0);
            let g: Vec<&str> = got.lines().skip(first.saturating_sub(2)).take(6).collect();
            let w: Vec<&str> = want.lines().skip(first.saturating_sub(2)).take(6).collect();
            panic!(
                "fuzz {n:02}: first difference near line {}\n got: {g:#?}\nwant: {w:#?}",
                first + 1
            );
        }
        checked += 1;
    }
    assert_eq!(checked, 20);
}
