//! Port of the elliptical template mask of `TrackerTemplateMatching.MatchTemplate`: a black
//! bitmap of the block size where `Graphics.FillEllipse(Brushes.White, Rectangle(0, 0, w, h))`
//! paints a white ellipse with the default `SmoothingMode` (no antialiasing), then `BGR2GRAY`.
//!
//! The ellipse is inscribed in the rectangle. A pixel is set when its top-left corner lies
//! inside or on the ellipse. This matches `tests/fixtures/tracking/template-mask/mask.csv`
//! everywhere except pixel (4, 2), where GDI+ leaves it empty; `mask_test.rs` pins that case.

use super::image::GrayImage;

/// Mask of `width` x `height` with 255 inside the ellipse and 0 outside.
pub fn ellipse_mask(width: u32, height: u32) -> GrayImage {
    let (rx, ry) = (f64::from(width) / 2.0, f64::from(height) / 2.0);
    let mut data = Vec::with_capacity((width * height) as usize);
    for y in 0..height {
        for x in 0..width {
            let dx = (f64::from(x) - rx) / rx;
            let dy = (f64::from(y) - ry) / ry;
            data.push(if dx * dx + dy * dy <= 1.0 { 255 } else { 0 });
        }
    }
    GrayImage {
        width,
        height,
        data,
    }
}
