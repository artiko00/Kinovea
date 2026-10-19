//! Port of `Kinovea.Video/Caching/VideoFrame.cs`. The C# `Bitmap` becomes a BGRA8 buffer whose
//! bytes are shared through `Arc`, so cloning a frame never copies pixels.

use std::sync::Arc;

/// Top-down BGRA8 image (`Format32bppArgb` byte order, as FFmpeg `AV_PIX_FMT_BGRA`).
#[derive(Debug, Clone)]
pub struct FrameBuffer {
    width: u32,
    height: u32,
    /// Bytes per row, which may include padding after the pixels.
    stride: usize,
    data: Arc<[u8]>,
}

impl FrameBuffer {
    /// Panics when the stride cannot hold a row or the data cannot hold the image.
    pub fn new(width: u32, height: u32, stride: usize, data: Arc<[u8]>) -> Self {
        assert!(stride >= width as usize * 4, "stride shorter than a row");
        assert!(
            data.len() >= stride * height as usize,
            "data shorter than the image"
        );
        Self {
            width,
            height,
            stride,
            data,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn stride(&self) -> usize {
        self.stride
    }

    /// The shared pixel bytes.
    pub fn data(&self) -> &Arc<[u8]> {
        &self.data
    }

    /// BGRA bytes of the pixel at `(x, y)`, or `None` outside the image.
    pub fn pixel(&self, x: u32, y: u32) -> Option<[u8; 4]> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let offset = y as usize * self.stride + x as usize * 4;
        let p = &self.data[offset..offset + 4];
        Some([p[0], p[1], p[2], p[3]])
    }
}

/// `VideoFrame`: an image with its timestamp and the timestamp of the frame decoded before it.
#[derive(Debug, Clone)]
pub struct VideoFrame {
    pub image: Option<FrameBuffer>,
    pub timestamp: i64,
    pub previous_timestamp: i64,
}

impl VideoFrame {
    pub fn new(image: Option<FrameBuffer>, timestamp: i64, previous_timestamp: i64) -> Self {
        Self {
            image,
            timestamp,
            previous_timestamp,
        }
    }

    /// `VideoFrame.Empty()`: no image, both timestamps -1.
    pub fn empty() -> Self {
        Self::new(None, -1, -1)
    }
}
