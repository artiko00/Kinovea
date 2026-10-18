//! Byte-for-byte parity with `TimeHelper` using the oracle fixtures.

use kinovea_core::timecode::{ms_to_timecode, timestring, TimecodeFormat};
use kinovea_fixtures::fixtures_root;
use std::path::PathBuf;

fn rows(file: &str) -> Vec<Vec<String>> {
    let path: PathBuf = fixtures_root().join("timecode/formats").join(file);
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()));
    let mut lines = text.lines();
    let header = lines.next().expect("header");
    assert!(!header.is_empty());
    lines
        .filter(|l| !l.is_empty())
        .map(|l| {
            assert!(!l.contains('"'), "quoted CSV cell not expected: {l}");
            l.split(',').map(str::to_owned).collect()
        })
        .collect()
}

#[test]
fn ms_to_timecode_matches_every_fixture_row() {
    let rows = rows("ms_to_timecode.csv");
    assert!(rows.len() > 50);
    for r in rows {
        let ms: f64 = r[0].parse().unwrap();
        let precision: i32 = r[1].parse().unwrap();
        assert_eq!(
            ms_to_timecode(ms, precision),
            r[2],
            "ms={ms} precision={precision}"
        );
    }
}

#[test]
fn timestring_matches_every_fixture_row() {
    let rows = rows("timecodes.csv");
    assert!(rows.len() > 800);
    for r in rows {
        let fps: f64 = r[0].parse().unwrap();
        let ms: f64 = r[1].parse().unwrap();
        let frames: i64 = r[2].parse().unwrap();
        let format: TimecodeFormat = r[3].parse().unwrap();
        let symbol = r[4] == "1";
        // The generator uses duration 36000 s for the enclosing segment.
        let total_frames = (36_000.0 * fps).round();
        let got = timestring(
            fps,
            frames,
            ms,
            frames,
            total_frames,
            total_frames,
            format,
            symbol,
        );
        assert_eq!(
            got, r[5],
            "fps={fps} ms={ms} format={format} symbol={symbol}"
        );
    }
}
