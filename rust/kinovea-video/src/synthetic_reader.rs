//! `VideoReader` over `SyntheticReader`. Only `Timestamp` player requests are served; the
//! other actions are refused. Reports on the request sink like a reader that decodes on demand.

use crate::capabilities::{VideoCapabilities, VideoDecodingMode};
use crate::frame::VideoFrame;
use crate::geometry::{VideoGeometry, VideoGeometryRequest};
use crate::info::VideoInfo;
use crate::reader::{RequestFulfilled, VideoReader};
use crate::results::OpenVideoResult;
use crate::state::{PlayerAction, PlayerState};
use crate::summary::{thumbnail_indices, thumbnail_size, VideoSummary};
use crate::synthetic::SyntheticReader;
use kinovea_core::video_section::VideoSection;
use kinovea_geometry::Size;
use std::sync::mpsc::Sender;

impl VideoReader for SyntheticReader {
    fn current(&self) -> &VideoFrame {
        &self.current
    }

    fn flags(&self) -> VideoCapabilities {
        VideoCapabilities::CAN_DECODE_ON_DEMAND | VideoCapabilities::CAN_CHANGE_WORKING_ZONE
    }

    fn info(&self) -> &VideoInfo {
        &self.info
    }

    fn loaded(&self) -> bool {
        self.loaded
    }

    fn working_zone(&self) -> VideoSection {
        VideoSection::new(self.info.first_timestamp, self.info.last_timestamp)
    }

    fn decoding_mode(&self) -> VideoDecodingMode {
        if self.loaded {
            VideoDecodingMode::OnDemand
        } else {
            VideoDecodingMode::NotInitialized
        }
    }

    fn geometry(&self) -> &VideoGeometry {
        &self.geometry
    }

    fn open(&mut self, file_path: &str) -> OpenVideoResult {
        self.info = self.build_info(file_path);
        self.current = self.frame(0);
        self.loaded = true;
        OpenVideoResult::Success
    }

    fn close(&mut self) {
        self.loaded = false;
    }

    /// Generates the thumbnails from the synthetic frames; the reader is never opened.
    fn extract_summary(
        &mut self,
        file_path: &str,
        thumbs_to_load: usize,
        max_image_size: Size,
    ) -> VideoSummary {
        let info = self.build_info(file_path);
        let mut summary = VideoSummary::new(file_path);
        summary.is_image = self.frame_count == 1;
        summary.image_size = self.size;
        summary.duration_milliseconds = (self.frame_count as f64 * info.frame_interval_ms) as i64;
        summary.framerate = self.frames_per_second;

        let thumb_size = thumbnail_size(self.size, max_image_size);
        let tpf = self.timestamps_per_frame;
        summary.thumbs = thumbnail_indices(self.frame_count, thumbs_to_load)
            .into_iter()
            .map(|index| Self::paint(index * tpf, thumb_size))
            .collect();
        summary
    }

    fn player_request(&mut self, state: PlayerState) -> bool {
        let fulfilled = match state.action {
            PlayerAction::Timestamp => self.move_to(state.reference_timestamp),
            _ => false,
        };
        if let Some(sink) = &self.sink {
            let _ = sink.send(RequestFulfilled {
                player_state: state,
                fulfilled,
                timestamp: self.current.timestamp,
            });
        }
        fulfilled
    }

    fn move_request(&mut self, next: bool, timestamp: i64) -> bool {
        if !self.loaded {
            return false;
        }
        let target = if next {
            self.current.timestamp + self.timestamps_per_frame
        } else {
            timestamp
        };
        if !self.working_zone().contains(target) {
            return false;
        }
        // Land on the frame at or before the target.
        let frame_timestamp = target / self.timestamps_per_frame * self.timestamps_per_frame;
        self.current = self.frame(frame_timestamp);
        true
    }

    /// Keeps the default geometry: a synthetic source has no cache to invalidate.
    fn update_video_geometry(&mut self, _request: VideoGeometryRequest) -> bool {
        false
    }

    fn set_request_sink(&mut self, sink: Sender<RequestFulfilled>) {
        self.sink = Some(sink);
    }
}
