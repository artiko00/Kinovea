use crate::frame::{FrameBuffer, VideoFrame};
use std::sync::Arc;

/// 2x2 BGRA image with 4 bytes of padding after each row (stride 12).
fn padded_image() -> FrameBuffer {
    let data: Arc<[u8]> = Arc::from(vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 9, 9, 9, //
        10, 11, 12, 13, 14, 15, 16, 17, 9, 9, 9, 9,
    ]);
    FrameBuffer::new(2, 2, 12, data)
}

#[test]
fn buffer_reports_its_geometry() {
    let image = padded_image();
    assert_eq!(image.width(), 2);
    assert_eq!(image.height(), 2);
    assert_eq!(image.stride(), 12);
}

#[test]
fn pixel_reads_bgra_through_the_stride() {
    let image = padded_image();
    assert_eq!(image.pixel(0, 0), Some([1, 2, 3, 4]));
    assert_eq!(image.pixel(1, 0), Some([5, 6, 7, 8]));
    assert_eq!(image.pixel(0, 1), Some([10, 11, 12, 13]));
    assert_eq!(image.pixel(1, 1), Some([14, 15, 16, 17]));
}

#[test]
fn pixel_outside_the_image_is_none() {
    let image = padded_image();
    assert_eq!(image.pixel(2, 0), None);
    assert_eq!(image.pixel(0, 2), None);
}

#[test]
fn clones_share_the_pixel_bytes() {
    let bytes: Arc<[u8]> = Arc::from(vec![0u8; 4]);
    let image = FrameBuffer::new(1, 1, 4, bytes.clone());
    let copy = image.clone();
    assert!(Arc::ptr_eq(image.data(), &bytes));
    assert!(Arc::ptr_eq(copy.data(), &bytes));
}

#[test]
#[should_panic]
fn stride_shorter_than_a_row_is_rejected() {
    let bytes: Arc<[u8]> = Arc::from(vec![0u8; 16]);
    FrameBuffer::new(2, 2, 4, bytes);
}

#[test]
fn empty_frame_has_no_image_and_no_timestamps() {
    let frame = VideoFrame::empty();
    assert!(frame.image.is_none());
    assert_eq!(frame.timestamp, -1);
    assert_eq!(frame.previous_timestamp, -1);
}

#[test]
fn frame_keeps_its_timestamps() {
    let frame = VideoFrame::new(Some(padded_image()), 40, 0);
    assert!(frame.image.is_some());
    assert_eq!(frame.timestamp, 40);
    assert_eq!(frame.previous_timestamp, 0);
}
