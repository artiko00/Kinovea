//! Tests for `synthetic_reader.rs`: the summary the synthetic source produces without opening it.

use crate::reader::VideoReader;
use crate::synthetic::{decode_timestamp, SyntheticReader};
use kinovea_geometry::Size;

#[test]
fn summary_thumbs_are_bounded_and_the_reader_stays_closed() {
    let mut reader = SyntheticReader::new(Size::new(320, 240), 25.0, 250, 1);

    let summary = reader.extract_summary("clip.synthetic", 3, Size::new(100, 100));

    assert_eq!(summary.thumbs.len(), 3);
    assert!(summary.thumbs.iter().all(|thumb| thumb.width() <= 100));
    assert_eq!(decode_timestamp(&summary.thumbs[1]), 84);
    assert_eq!(summary.duration_milliseconds, 10_000);
    assert_eq!(summary.framerate, 25.0);
    assert!(!summary.is_image);
    assert!(!reader.loaded());
}
