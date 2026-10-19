//! Tests for `enumerate.rs`: `VideoReader.EnumerateFrames` over the working zone.

use crate::enumerate::FrameEnumerator;
use crate::reader::VideoReader;
use crate::results::OpenVideoResult;
use crate::synthetic::SyntheticReader;
use kinovea_geometry::Size;

fn opened(frames: i64) -> SyntheticReader {
    let mut reader = SyntheticReader::new(Size::new(4, 2), 10.0, frames, 1);
    assert_eq!(reader.open("clip.synthetic"), OpenVideoResult::Success);
    reader
}

fn timestamps(reader: &mut SyntheticReader, interval: f64) -> Vec<i64> {
    reader
        .enumerate_frames(interval)
        .map(|frame| frame.timestamp)
        .collect()
}

#[test]
fn full_enumeration_yields_every_frame_in_order() {
    let mut reader = opened(10);
    assert_eq!(timestamps(&mut reader, 0.0), (0..10).collect::<Vec<i64>>());
}

#[test]
fn interval_enumeration_yields_frames_on_the_grid() {
    let mut reader = opened(10);
    assert_eq!(timestamps(&mut reader, 3.0), vec![0, 3, 6, 9]);
}

#[test]
fn interval_targets_round_half_to_even() {
    let mut reader = opened(10);
    // 2.5, 4.5, 6.5 and 8.5 round to the even neighbour; 10.5 is past the zone.
    assert_eq!(timestamps(&mut reader, 2.5), vec![0, 2, 4, 6, 8]);
}

#[test]
fn single_frame_zone_yields_one_frame() {
    let mut reader = opened(1);
    assert_eq!(timestamps(&mut reader, 0.0), vec![0]);
}

#[test]
fn closed_reader_yields_nothing() {
    let mut reader = SyntheticReader::new(Size::new(4, 2), 10.0, 10, 1);
    assert!(timestamps(&mut reader, 0.0).is_empty());
}

#[test]
fn enumeration_runs_on_a_boxed_reader() {
    let mut reader: Box<dyn VideoReader> = Box::new(opened(4));
    let frames: Vec<i64> = FrameEnumerator::new(reader.as_mut(), 0.0)
        .map(|frame| frame.timestamp)
        .collect();
    assert_eq!(frames, vec![0, 1, 2, 3]);
}
