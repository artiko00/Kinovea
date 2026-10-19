use crate::import::ImportError;
use crate::import_test::{assert_matches_oracle, document, repo};
use crate::trc::import_trc;
use kinovea_core::timed_point::TimedPoint;
use kinovea_metadata::drawing::Track;
use kinovea_metadata::style::StyleValue;
use kinovea_metadata::{Color, Drawing, Metadata};

fn clip_text() -> String {
    std::fs::read_to_string(repo().join("tests/data/import/clip.trc")).unwrap()
}

fn track(m: &Metadata, index: usize) -> &Track {
    match &m.tracks[index] {
        Drawing::Track(t) => t,
        other => panic!("expected a track, found {}", other.kva_name()),
    }
}

#[test]
fn clip_matches_the_oracle_with_one_posture_and_one_track_per_marker() {
    let mut m = document("trc");
    let report = import_trc(&mut m, &clip_text()).unwrap();
    assert_matches_oracle(&m, "trc");
    assert_eq!(m.keyframes.len(), 1);
    assert_eq!(report.keyframes, 1);
    assert_eq!(report.drawings, 1);
    assert_eq!(report.tracks, 22);
    assert_eq!(m.tracks.len(), 22);
}

#[test]
fn tracks_are_named_after_their_marker_and_keep_its_colour() {
    let mut m = document("trc");
    import_trc(&mut m, &clip_text()).unwrap();
    let hip = track(&m, 0);
    assert_eq!(hip.header.name, "Human.Hip");
    assert_eq!(
        hip.header.style.get("color"),
        Some(&StyleValue::Color(Color::rgb(255, 0, 0)))
    );
    assert_eq!(
        hip.header.style.get("line size"),
        Some(&StyleValue::LineSize(1))
    );
    assert_eq!(track(&m, 1).header.name, "Human.RHip");
}

#[test]
fn track_repeats_its_first_point_like_the_original_constructor() {
    let mut m = document("trc");
    import_trc(&mut m, &clip_text()).unwrap();
    let times: Vec<i64> = track(&m, 0)
        .positions
        .iter()
        .map(|p: &TimedPoint| p.t)
        .collect();
    assert_eq!(times, vec![0, 0, 400, 800, 1200, 1600]);
}

#[test]
fn num_frames_limits_the_rows_read_like_the_original() {
    // NumFrames 7 keeps rows 5 and 6 only: the original loops from line 6 while below NumFrames.
    let text = clip_text().replacen("\t10\t22\t", "\t7\t22\t", 1);
    let mut m = document("trc");
    import_trc(&mut m, &text).unwrap();
    assert_eq!(track(&m, 0).positions.len(), 3);
}

#[test]
fn marker_count_mismatch_is_invalid_data_and_leaves_the_document_unchanged() {
    let text = clip_text().replacen("\tHip\t", "\t", 1);
    let mut m = document("trc");
    let err = import_trc(&mut m, &text).unwrap_err();
    assert!(
        matches!(err, ImportError::InvalidData(ref msg) if msg.contains("line 4")),
        "{err}"
    );
    assert!(m.keyframes.is_empty());
    assert!(m.tracks.is_empty());
}
