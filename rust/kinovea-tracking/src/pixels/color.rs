//! Port of the `cvtColor(BGR2GRAY)` calls of the original: `TrackerCircle.MatchCircle` on the
//! search ROI and the mask of `TrackerTemplateMatching`. The frame is BGRA; alpha is ignored,
//! as `BGRA2GRAY` does.
//!
//! `RGB2Gray<uchar>` with `yuv_shift` 15 and the weights 0.114/0.587/0.299 rounded to 15 bits
//! (checked against OpenCV 4.10 and 5.0): `(B*3735 + G*19235 + R*9798 + 2^14) >> 15`. The
//! weights sum to 32768, so the result stays in 0..255.

use super::image::{GrayImage, Roi};
use kinovea_video::frame::FrameBuffer;

const R2Y: u32 = 9798;
const G2Y: u32 = 19235;
const B2Y: u32 = 3735;
const YUV_SHIFT: u32 = 15;

pub fn to_gray(frame: &FrameBuffer, roi: Roi) -> GrayImage {
    let roi = roi.clamp_to(frame.width(), frame.height());
    let data = bgra(frame, roi)
        .map(|[b, g, r, _]| {
            let sum = u32::from(b) * B2Y + u32::from(g) * G2Y + u32::from(r) * R2Y;
            ((sum + (1 << (YUV_SHIFT - 1))) >> YUV_SHIFT) as u8
        })
        .collect();
    GrayImage {
        width: roi.size.width as u32,
        height: roi.size.height as u32,
        data,
    }
}

/// BGRA pixels of the ROI, row by row, read through the frame stride. The ROI must be clamped.
fn bgra(frame: &FrameBuffer, roi: Roi) -> impl Iterator<Item = [u8; 4]> + '_ {
    let x0 = roi.x as usize * 4;
    let row_len = roi.size.width as usize * 4;
    let rows = roi.y as usize..(roi.y + roi.size.height) as usize;
    rows.flat_map(move |y| {
        let start = y * frame.stride() + x0;
        frame.data()[start..start + row_len]
            .chunks_exact(4)
            .map(|p| [p[0], p[1], p[2], p[3]])
    })
}
