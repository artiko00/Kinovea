use crate::pixels::image::Roi;
use crate::template::capture::{aligned, capture, crop_bgra};
use kinovea_geometry::Size;
use kinovea_video::frame::FrameBuffer;
use std::sync::Arc;

/// Pixel `(x, y)` is `[x, y, 0, 255]`, so every copied block is recognizable.
fn gradient(width: u32, height: u32) -> FrameBuffer {
    let data: Vec<u8> = (0..height)
        .flat_map(|y| (0..width).flat_map(move |x| [x as u8, y as u8, 0, 255]))
        .collect();
    FrameBuffer::new(width, height, width as usize * 4, Arc::from(data))
}

#[test]
fn rounding_is_to_the_nearest_whole_pixel_with_ties_to_even() {
    assert_eq!(aligned(5.4), 5);
    assert_eq!(aligned(5.6), 6);
    assert_eq!(aligned(4.5), 4);
    assert_eq!(aligned(5.5), 6);
}

#[test]
fn the_block_is_centred_on_the_rounded_point() {
    let frame = gradient(12, 12);
    let patch = capture(&frame, (5.4, 5.6), Size::new(4, 4));
    assert_eq!((patch.width, patch.height), (4, 4));
    // Rounded point (5, 6), top-left at (5 - 2, 6 - 2) = (3, 4).
    assert_eq!(&patch.pixels[..4], &frame.pixel(3, 4).unwrap());
    assert_eq!(
        &patch.pixels[patch.pixels.len() - 4..],
        &frame.pixel(6, 7).unwrap()
    );
}

#[test]
fn a_block_clipped_by_the_right_edge_stays_empty() {
    let frame = gradient(12, 12);
    // Top-left at x = 9 leaves 3 of the 4 columns inside the image.
    let patch = capture(&frame, (11.0, 5.0), Size::new(4, 4));
    assert!(patch.pixels.iter().all(|&b| b == 0));
}

#[test]
fn a_block_clipped_by_the_left_edge_is_shifted_not_centred() {
    let frame = gradient(12, 12);
    // Top-left would be at x = -1; the original clamps it to 0 and copies from there.
    let patch = capture(&frame, (1.0, 5.0), Size::new(4, 4));
    assert_eq!(&patch.pixels[..4], &frame.pixel(0, 3).unwrap());
}

#[test]
fn crop_returns_the_rows_of_the_region_only() {
    let frame = gradient(6, 4);
    let bytes = crop_bgra(&frame, Roi::new(2, 1, 2, 2));
    let expected: Vec<u8> = [(2, 1), (3, 1), (2, 2), (3, 2)]
        .iter()
        .flat_map(|&(x, y)| frame.pixel(x, y).unwrap())
        .collect();
    assert_eq!(bytes, expected);
}
