//! Port of `Kinovea.Video/VideoReaders/VideoReader.cs`. The abstract members become required
//! methods and the concrete helpers become default methods over `info()`. The C#
//! `RequestFulfilled` event becomes a `Sender` installed through `set_request_sink`.

use crate::capabilities::{VideoCapabilities, VideoDecodingMode};
use crate::enumerate::FrameEnumerator;
use crate::frame::VideoFrame;
use crate::geometry::{VideoGeometry, VideoGeometryRequest};
use crate::info::VideoInfo;
use crate::relation::{self, TimestampRelation};
use crate::results::OpenVideoResult;
use crate::state::PlayerState;
use crate::summary::VideoSummary;
use kinovea_core::video_section::VideoSection;
use kinovea_geometry::Size;
use std::sync::mpsc::Sender;
use std::time::Duration;

/// `RequestFulfilledEventArgs`: reports how a player request ended.
#[derive(Debug, Clone, PartialEq)]
pub struct RequestFulfilled {
    pub player_state: PlayerState,
    pub fulfilled: bool,
    pub timestamp: i64,
}

/// A video source. Implementations decode or generate frames; players, exporters and the
/// headless server only see this contract.
pub trait VideoReader {
    /// The frame at the current position.
    fn current(&self) -> &VideoFrame;
    fn flags(&self) -> VideoCapabilities;
    fn info(&self) -> &VideoInfo;
    fn loaded(&self) -> bool;
    fn working_zone(&self) -> VideoSection;
    fn decoding_mode(&self) -> VideoDecodingMode;
    fn geometry(&self) -> &VideoGeometry;

    /// On success `info()` and `geometry()` describe the file.
    fn open(&mut self, file_path: &str) -> OpenVideoResult;
    fn close(&mut self);
    /// Reads a summary of the file without leaving it open.
    fn extract_summary(
        &mut self,
        file_path: &str,
        thumbs_to_load: usize,
        max_image_size: Size,
    ) -> VideoSummary;

    /// Returns true when the frame was acquired before returning.
    fn player_request(&mut self, state: PlayerState) -> bool;
    /// Moves to `timestamp`, or to the frame after the current one when `next` is set.
    fn move_request(&mut self, next: bool, timestamp: i64) -> bool;
    /// Returns true when a cache was invalidated.
    fn update_video_geometry(&mut self, request: VideoGeometryRequest) -> bool;
    /// Installs the channel that receives `RequestFulfilled` events.
    fn set_request_sink(&mut self, sink: Sender<RequestFulfilled>);

    fn move_to(&mut self, timestamp: i64) -> bool {
        self.move_request(false, timestamp)
    }

    fn move_next(&mut self) -> bool {
        self.move_request(true, -1)
    }

    /// `GetPlaybackTimestamp`: the timestamp expected `elapsed` after playback started.
    fn playback_timestamp(&self, state: &PlayerState, elapsed: Duration) -> i64 {
        relation::playback_timestamp(self.info().average_timestamps_per_frame, state, elapsed)
    }

    /// `RelateTimestamps`: where `request` sits relative to `reference`.
    fn relate_timestamps(&self, request: i64, reference: i64) -> TimestampRelation {
        relation::relate_timestamps(self.info().average_timestamps_per_frame, request, reference)
    }

    /// Hook run before `enumerate_frames`, where a reader can stop its prebuffering.
    fn before_frame_enumeration(&mut self) {}

    /// Hook run after the enumeration, where a reader can restart its prebuffering.
    fn after_frame_enumeration(&mut self) {}

    /// `EnumerateFrames`: walks the working zone, see `FrameEnumerator`.
    fn enumerate_frames(&mut self, interval: f64) -> FrameEnumerator<'_, Self>
    where
        Self: Sized,
    {
        FrameEnumerator::new(self, interval)
    }
}
