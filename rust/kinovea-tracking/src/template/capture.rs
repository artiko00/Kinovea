//! Template capture as `CreateTrackPoint` and `CreateReferenceTrackPoint` do it: the point is
//! rounded to a whole pixel (ties to even, `Math.Round`), the block is centred on it with the
//! integer half size, and clamped at the top and left edges. When the block still sticks out on the
//! right or bottom, the original copy fails on the size mismatch and the template stays blank.

use kinovea_geometry::Size;
use kinovea_video::frame::FrameBuffer;

use crate::pixels::image::Roi;

/// A template captured at `time`, with the similarity of the step that produced it (1 for a
/// reference point, 0 after a failed step).
#[derive(Debug, Clone, PartialEq)]
pub struct TrackingTemplate {
    pub score: f32,
    pub patch: Patch,
}

/// BGRA template pixels, row-major, 4 bytes per pixel.
#[derive(Debug, Clone, PartialEq)]
pub struct Patch {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u8>,
}

/// `Math.Round` of a coordinate to a whole pixel.
pub fn aligned(value: f32) -> i64 {
    f64::from(value).round_ties_even() as i64
}

/// The template of `block` centred on `center`. Blank when the block does not fit in the image.
pub fn capture(image: &FrameBuffer, center: (f32, f32), block: Size) -> Patch {
    let x = (aligned(center.0) - i64::from(block.width / 2)).max(0);
    let y = (aligned(center.1) - i64::from(block.height / 2)).max(0);
    let width = (i64::from(image.width()) - x).min(i64::from(block.width));
    let height = (i64::from(image.height()) - y).min(i64::from(block.height));
    let fits = width == i64::from(block.width) && height == i64::from(block.height);
    let pixels = if fits {
        let roi = Roi::new(x as i32, y as i32, block.width, block.height);
        crop_bgra(image, roi)
    } else {
        vec![0; block.width as usize * block.height as usize * 4]
    };
    Patch {
        width: block.width as usize,
        height: block.height as usize,
        pixels,
    }
}

/// BGRA bytes of `roi`, row by row. The region must lie inside the image.
pub fn crop_bgra(image: &FrameBuffer, roi: Roi) -> Vec<u8> {
    let row = roi.size.width as usize * 4;
    let x0 = roi.x as usize * 4;
    (roi.y as usize..(roi.y + roi.size.height) as usize)
        .flat_map(|y| {
            let start = y * image.stride() + x0;
            image.data()[start..start + row].iter().copied()
        })
        .collect()
}
