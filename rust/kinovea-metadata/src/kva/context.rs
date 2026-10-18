//! Import context: whether the file comes from the current video and how to remap its
//! coordinates and timestamps otherwise (`GetScaling`, `RemapTimestamp`, `IsSameContext`).

use crate::kva_io::Scale;
use crate::metadata::VideoContext;
use kinovea_geometry::Size;

/// Header values of the file being imported.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ImportContext {
    pub file_name: String,
    pub path: String,
    pub byte_length: i64,
    pub is_capture_recording: bool,
    pub image_size: Size,
    pub average_timestamps_per_frame: f64,
    pub first_timestamp: i64,
    pub time_origin: i64,
}

impl ImportContext {
    /// Ratio of the current image size to the file's; identity when either is unknown.
    pub fn scaling(&self, current: &VideoContext) -> Scale {
        if current.image_size.is_empty() || self.image_size.is_empty() {
            return Scale::IDENTITY;
        }
        Scale { x: current.image_size.width as f32 / self.image_size.width as f32, y: current.image_size.height as f32 / self.image_size.height as f32 }
    }

    /// `IsSameContext` without touching the file system: no current video path means the same
    /// context; otherwise the paths, then the byte lengths, then the file names are compared.
    pub fn is_same_context(&self, current: &VideoContext) -> bool {
        if current.video_path.is_empty() {
            return true;
        }
        if !self.path.is_empty() && self.path == current.video_path {
            return true;
        }
        if self.byte_length > 0 && current.video_byte_length > 0 {
            return self.byte_length == current.video_byte_length;
        }
        let stem = |p: &str| std::path::Path::new(p).file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();
        if !self.path.is_empty() {
            return stem(&self.path) == stem(&current.video_path);
        }
        !self.file_name.is_empty() && self.file_name == stem(&current.video_path)
    }

    /// Maps a file timestamp onto the current video's time base.
    pub fn remap(&self, input: i64, current: &VideoContext, same_context: bool) -> i64 {
        if self.average_timestamps_per_frame == 0.0 || same_context {
            return input;
        }
        if self.is_capture_recording {
            self.remap_from(input, self.first_timestamp, current.first_timestamp, current)
        } else {
            self.remap_from(input, self.time_origin, current.time_origin, current)
        }
    }

    pub fn remap_from(&self, input: i64, input_reference: i64, output_reference: i64, current: &VideoContext) -> i64 {
        let frame = (input - input_reference) as f64 / self.average_timestamps_per_frame;
        let output_atpf = current.average_timestamps_per_second / (1000.0 / current.baseline_frame_interval_ms);
        (output_reference as f64 + frame * output_atpf).round() as i64
    }
}
