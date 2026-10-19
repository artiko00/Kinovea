//! Normalized cross-correlation `TM_CCOEFF_NORMED` as `cv::matchTemplate` computes it (OpenCV 4.x,
//! `modules/imgproc/src/templmatch.cpp`). Pixels are interleaved channels; the score accumulates
//! over channels. The unmasked variant lives in `ncc_unmasked`, the binary-mask one in `ncc_masked`.

pub use crate::ncc_map::{Map, MinMax};
use crate::ncc_masked::match_masked;
use crate::ncc_unmasked::match_unmasked;

/// Interleaved pixels in row-major order: `data[(y * width + x) * channels + c]`.
#[derive(Clone, Copy, Debug)]
pub struct Plane<'a, T> {
    pub data: &'a [T],
    pub width: usize,
    pub height: usize,
    pub channels: usize,
}

/// `cv::matchTemplate` with `TM_CCOEFF_NORMED`. `mask` has one byte per template pixel; a nonzero
/// byte includes the pixel (binary mask, as OpenCV reads `CV_8U` masks).
///
/// # Panics
/// If the template does not fit in the image, the channel counts differ or the mask length is not
/// the template size.
pub fn match_template<T: Copy + Into<f64>>(
    image: Plane<T>,
    template: Plane<T>,
    mask: Option<&[u8]>,
) -> Map {
    assert!(template.width <= image.width && template.height <= image.height);
    assert_eq!(template.channels, image.channels);
    match mask {
        Some(mask) => match_masked(image, template, mask),
        None => match_unmasked(image, template),
    }
}
