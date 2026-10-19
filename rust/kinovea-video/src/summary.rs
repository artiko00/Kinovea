//! Port of `Kinovea.Video/VideoSummary.cs` (`HasCompanionKva`) and of the thumbnail selection in
//! `VideoReaderGif.ExtractSummary`: `step = ceil(count / thumbs)`. Thumbnails are fitted with
//! `fit_helper::fit` without upscaling.

use crate::fit_helper::fit;
use crate::frame::FrameBuffer;
use kinovea_geometry::Size;
use std::path::Path;

/// `VideoSummary`: what a fast extraction learns about a file without opening it for playback.
#[derive(Debug, Clone)]
pub struct VideoSummary {
    pub filename: String,
    pub is_image: bool,
    pub image_size: Size,
    pub duration_milliseconds: i64,
    pub framerate: f64,
    pub thumbs: Vec<FrameBuffer>,
}

impl VideoSummary {
    pub fn new(filename: &str) -> Self {
        Self {
            filename: filename.to_string(),
            is_image: false,
            image_size: Size::default(),
            duration_milliseconds: 0,
            framerate: 0.0,
            thumbs: Vec::new(),
        }
    }
}

/// `HasCompanionKva`: a `.kva` file with the same stem sits next to the video.
pub fn has_companion_kva(filename: &Path) -> bool {
    filename.with_extension("kva").is_file()
}

/// Indices of the frames to use as thumbnails: one every `ceil(frame_count / thumbs)` frames,
/// starting at the first, so there are never more than `thumbs` of them.
pub fn thumbnail_indices(frame_count: i64, thumbs: usize) -> Vec<i64> {
    if thumbs == 0 || frame_count <= 0 {
        return Vec::new();
    }
    let thumbs = thumbs as i64;
    let step = (frame_count + thumbs - 1) / thumbs;
    (0..frame_count).step_by(step as usize).collect()
}

/// Size of a thumbnail: the image fitted into `max_size`, never upscaled.
pub fn thumbnail_size(image: Size, max_size: Size) -> Size {
    fit(image, max_size, false)
}
