use crate::pixels::{ellipse_mask, median_blur, to_gray, Roi};
use kinovea_video::frame::FrameBuffer;
use std::sync::Arc;

#[test]
fn conversions_and_filters_compose_from_a_frame() {
    let data: Vec<u8> = [[0u8, 0, 0, 255], [255, 255, 255, 255], [255, 255, 255, 255]]
        .iter()
        .flatten()
        .copied()
        .collect();
    let frame = FrameBuffer::new(3, 1, 12, Arc::from(data));

    let gray = to_gray(&frame, Roi::new(0, 0, 3, 1));
    assert_eq!(gray.data, vec![0, 255, 255]);
    assert_eq!(median_blur(&gray, 1).data, gray.data);
    assert_eq!(ellipse_mask(3, 1).data.len(), 3);
}

/// Frame of 4x3 pixels with `b = (x*37 + y*11) % 256`, `g = (x*5 + y*90) % 256`,
/// `r = (x*200 + y*3 + 17) % 256` and alpha 255. The expected values come from OpenCV 4.10
/// (`cvtColor`, `medianBlur`).
#[test]
fn outputs_match_opencv_4_10_on_a_small_frame() {
    let mut bytes = Vec::new();
    for y in 0..3u32 {
        for x in 0..4u32 {
            let b = (x * 37 + y * 11) % 256;
            let g = (x * 5 + y * 90) % 256;
            let r = (x * 200 + y * 3 + 17) % 256;
            bytes.extend([b as u8, g as u8, r as u8, 255]);
        }
    }
    let frame = FrameBuffer::new(4, 3, 16, Arc::from(bytes));

    let gray = to_gray(&frame, Roi::new(0, 0, 4, 3));
    assert_eq!(gray.data, GRAY);
    assert_eq!(median_blur(&gray, 3).data, MEDIAN_3);
}

const GRAY: [u8; 12] = [5, 72, 62, 53, 60, 127, 117, 108, 115, 182, 172, 163];
const MEDIAN_3: [u8; 12] = [60, 62, 72, 62, 72, 115, 117, 108, 115, 127, 163, 163];
