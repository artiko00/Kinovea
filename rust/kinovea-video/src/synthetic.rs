//! Synthetic source for tests and the headless server. Frame `t` paints its timestamp in the
//! first eight bytes (pixels 0 and 1 of row 0), so any frame a test receives says where it came
//! from. `synthetic_reader.rs` implements `VideoReader` on top of these helpers.

use crate::frame::{FrameBuffer, VideoFrame};
use crate::geometry::VideoGeometry;
use crate::info::{frame_interval_ms, Fraction, VideoInfo};
use crate::reader::RequestFulfilled;
use kinovea_core::image_options::ImageRotation;
use kinovea_geometry::Size;
use std::sync::mpsc::Sender;
use std::sync::Arc;

pub struct SyntheticReader {
    pub(crate) size: Size,
    pub(crate) frames_per_second: f64,
    pub(crate) frame_count: i64,
    pub(crate) timestamps_per_frame: i64,
    pub(crate) info: VideoInfo,
    pub(crate) geometry: VideoGeometry,
    pub(crate) current: VideoFrame,
    pub(crate) sink: Option<Sender<RequestFulfilled>>,
    pub(crate) loaded: bool,
}

impl SyntheticReader {
    pub fn new(
        size: Size,
        frames_per_second: f64,
        frame_count: i64,
        timestamps_per_frame: i64,
    ) -> Self {
        assert!(
            size.width >= 2 && size.height >= 1,
            "room for the timestamp"
        );
        assert!(
            frame_count >= 1 && timestamps_per_frame >= 1,
            "non-empty source"
        );
        Self {
            size,
            frames_per_second,
            frame_count,
            timestamps_per_frame,
            info: VideoInfo::empty(),
            geometry: VideoGeometry::default(),
            current: VideoFrame::empty(),
            sink: None,
            loaded: false,
        }
    }

    /// Image of the frame at `timestamp`, painted at `size`.
    pub fn paint(timestamp: i64, size: Size) -> FrameBuffer {
        let stride = size.width as usize * 4;
        let mut data = vec![0u8; stride * size.height as usize];
        data[..8].copy_from_slice(&timestamp.to_le_bytes());
        FrameBuffer::new(
            size.width as u32,
            size.height as u32,
            stride,
            Arc::from(data),
        )
    }

    pub(crate) fn frame(&self, timestamp: i64) -> VideoFrame {
        let previous = if timestamp > 0 {
            timestamp - self.timestamps_per_frame
        } else {
            -1
        };
        VideoFrame::new(Some(Self::paint(timestamp, self.size)), timestamp, previous)
    }

    pub(crate) fn build_info(&self, file_path: &str) -> VideoInfo {
        let tpf = self.timestamps_per_frame;
        VideoInfo {
            file_path: file_path.to_string(),
            original_size: self.size,
            original_rotation: ImageRotation::Rotate0,
            pixel_aspect_ratio: 1.0,
            sample_aspect_ratio: Fraction::new(1, 1),
            average_timestamps_per_frame: tpf as f64,
            average_timestamps_per_second: self.frames_per_second * tpf as f64,
            frames_per_second: self.frames_per_second,
            frame_interval_ms: frame_interval_ms(self.frames_per_second),
            last_timestamp: (self.frame_count - 1) * tpf,
            duration_timestamps: self.frame_count * tpf,
            ..VideoInfo::empty()
        }
    }
}

/// Reads back the timestamp painted by `SyntheticReader::paint`.
pub fn decode_timestamp(image: &FrameBuffer) -> i64 {
    let [a, b, c, d] = image.pixel(0, 0).expect("first pixel");
    let [e, f, g, h] = image.pixel(1, 0).expect("second pixel");
    i64::from_le_bytes([a, b, c, d, e, f, g, h])
}
