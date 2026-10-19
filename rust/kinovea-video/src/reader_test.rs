//! Tests for the `VideoReader` trait: open, navigation, relation, player requests and object
//! safety, driven through `SyntheticReader`.

use crate::capabilities::VideoDecodingMode;
use crate::reader::{RequestFulfilled, VideoReader};
use crate::relation::TimestampRelation;
use crate::results::OpenVideoResult;
use crate::state::{PlayerAction, PlayerState};
use crate::synthetic::{decode_timestamp, SyntheticReader};
use kinovea_geometry::Size;
use std::sync::mpsc;
use std::time::Duration;

fn source(frames: i64) -> SyntheticReader {
    SyntheticReader::new(Size::new(4, 2), 25.0, frames, 1)
}

fn opened(frames: i64) -> SyntheticReader {
    let mut reader = source(frames);
    assert_eq!(reader.open("clip.synthetic"), OpenVideoResult::Success);
    reader
}

fn painted_timestamp(reader: &dyn VideoReader) -> i64 {
    let image = reader.current().image.as_ref().expect("current image");
    decode_timestamp(image)
}

#[test]
fn open_reports_the_timing_model() {
    let reader = opened(250);
    let info = reader.info();
    assert_eq!(info.frame_interval_ms, 40.0);
    assert_eq!(info.average_timestamps_per_frame, 1.0);
    assert_eq!(info.first_timestamp, 0);
    assert_eq!(info.duration_timestamps, 250);
    assert_eq!(info.file_path, "clip.synthetic");
    assert_eq!(reader.decoding_mode(), VideoDecodingMode::OnDemand);
}

#[test]
fn close_leaves_the_reader_unloaded() {
    let mut reader = opened(5);
    assert!(reader.loaded());
    reader.close();
    assert!(!reader.loaded());
    assert_eq!(reader.decoding_mode(), VideoDecodingMode::NotInitialized);
    assert!(!reader.move_to(1));
}

#[test]
fn move_to_lands_on_the_requested_frame() {
    let mut reader = opened(10);
    assert!(reader.move_to(7));
    assert_eq!(reader.current().timestamp, 7);
    assert_eq!(painted_timestamp(&reader), 7);
}

#[test]
fn move_next_steps_and_stops_at_the_end_of_the_zone() {
    let mut reader = opened(3);
    assert!(reader.move_to(1));
    assert!(reader.move_next());
    assert_eq!(reader.current().timestamp, 2);
    assert!(!reader.move_next());
    assert_eq!(reader.current().timestamp, 2);
}

#[test]
fn move_outside_the_zone_is_refused() {
    let mut reader = opened(3);
    assert!(reader.move_to(1));
    assert!(!reader.move_to(3));
    assert!(!reader.move_to(-1));
    assert_eq!(reader.current().timestamp, 1);
}

#[test]
fn relate_timestamps_uses_the_average_of_the_reader() {
    let reader = opened(250);
    assert_eq!(reader.relate_timestamps(51, 0), TimestampRelation::FarAhead);
    assert_eq!(reader.relate_timestamps(0, 0), TimestampRelation::Match);
}

#[test]
fn playback_timestamp_uses_the_reader_timing() {
    let reader = opened(250);
    let state = PlayerState::new(1, PlayerAction::Playback, false, 100, 40.0, 0.0);
    assert_eq!(
        reader.playback_timestamp(&state, Duration::from_millis(80)),
        102
    );
}

#[test]
fn player_request_reports_on_the_sink() {
    let mut reader = opened(10);
    let (tx, rx) = mpsc::channel();
    reader.set_request_sink(tx);
    let state = PlayerState::new(3, PlayerAction::Timestamp, true, 4, 0.0, 0.0);

    assert!(reader.player_request(state.clone()));

    let event = rx.try_recv().expect("request event");
    assert_eq!(
        event,
        RequestFulfilled {
            player_state: state,
            fulfilled: true,
            timestamp: 4,
        }
    );
}

#[test]
fn refused_request_is_reported_as_not_fulfilled() {
    let mut reader = opened(10);
    let (tx, rx) = mpsc::channel();
    reader.set_request_sink(tx);
    let state = PlayerState::new(4, PlayerAction::Timestamp, true, 40, 0.0, 0.0);

    assert!(!reader.player_request(state));

    assert!(!rx.try_recv().expect("request event").fulfilled);
}

#[test]
fn trait_is_usable_as_a_trait_object() {
    let mut reader: Box<dyn VideoReader> = Box::new(source(5));
    assert_eq!(reader.open("clip.synthetic"), OpenVideoResult::Success);
    assert!(reader.move_to(2));
    assert_eq!(reader.current().timestamp, 2);
}
