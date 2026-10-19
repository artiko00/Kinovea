use crate::frame::VideoFrame;
use crate::snapshot::{contiguous_spans, is_contiguous, single_span, CacheSnapshot};
use kinovea_core::video_section::VideoSection;

fn frame(ts: i64, prev: i64) -> VideoFrame {
    VideoFrame::new(None, ts, prev)
}

fn text(spans: Vec<VideoSection>) -> String {
    CacheSnapshot::new(0, spans).to_string()
}

#[test]
fn empty_snapshot_has_version_minus_one_and_no_sections() {
    let snapshot = CacheSnapshot::empty();
    assert_eq!(snapshot.version, -1);
    assert_eq!(snapshot.to_string(), "");
}

#[test]
fn display_writes_each_section_with_trailing_space() {
    let spans = vec![VideoSection::new(0, 5), VideoSection::new(9, 9)];
    assert_eq!(text(spans), "[0;5] [9;9] ");
}

#[test]
fn single_span_runs_from_first_to_last_frame() {
    let frames = [frame(0, 0), frame(10, 0), frame(20, 10)];
    assert_eq!(text(single_span(&frames)), "[0;20] ");
    assert_eq!(text(single_span(&[])), "");
}

#[test]
fn contiguous_spans_break_where_previous_timestamp_does_not_match() {
    let frames = [frame(0, 0), frame(1, 0), frame(2, 1), frame(5, 4)];
    assert_eq!(text(contiguous_spans(&frames)), "[0;2] [5;5] ");
}

#[test]
fn contiguous_spans_of_empty_list_is_empty() {
    assert_eq!(text(contiguous_spans(&[])), "");
}

#[test]
fn is_contiguous_requires_every_frame_to_follow_its_predecessor() {
    assert!(is_contiguous(&[]));
    assert!(is_contiguous(&[frame(0, 0), frame(1, 0)]));
    assert!(!is_contiguous(&[frame(0, 0), frame(1, 0), frame(5, 4)]));
}
