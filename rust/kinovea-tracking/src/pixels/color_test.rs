use crate::pixels::color::to_gray;
use crate::pixels::image::Roi;
use kinovea_video::frame::FrameBuffer;
use std::sync::Arc;

fn frame(width: u32, height: u32, bgra: &[[u8; 4]]) -> FrameBuffer {
    let data: Vec<u8> = bgra.iter().flatten().copied().collect();
    FrameBuffer::new(width, height, width as usize * 4, Arc::from(data))
}

fn full(frame: &FrameBuffer) -> Roi {
    Roi::new(0, 0, frame.width() as i32, frame.height() as i32)
}

#[test]
fn gray_uses_the_opencv_fixed_point_weights() {
    // Blue, green, red and white; the alpha byte is ignored.
    let f = frame(
        4,
        1,
        &[
            [255, 0, 0, 255],
            [0, 255, 0, 255],
            [0, 0, 255, 255],
            [255, 255, 255, 255],
        ],
    );
    let g = to_gray(&f, full(&f));
    assert_eq!((g.width, g.height), (4, 1));
    assert_eq!(g.data, vec![29, 150, 76, 255]);
}

#[test]
fn gray_reads_through_the_stride_and_the_roi() {
    // 2x2 frame with 4 bytes of padding per row (stride 12). Only pixel (1, 1) is white.
    let data: Vec<u8> = vec![
        0, 0, 0, 0, 0, 0, 0, 0, 9, 9, 9, 9, //
        0, 0, 0, 0, 255, 255, 255, 255, 9, 9, 9, 9,
    ];
    let f = FrameBuffer::new(2, 2, 12, Arc::from(data));
    assert_eq!(to_gray(&f, Roi::new(1, 1, 1, 1)).data, vec![255]);
}

#[test]
fn gray_roi_outside_the_image_is_clamped() {
    let f = frame(4, 1, &[[0, 0, 255, 255]; 4]);
    let g = to_gray(&f, Roi::new(2, 0, 10, 1));
    assert_eq!((g.width, g.height), (2, 1));
    assert_eq!(g.data, vec![76, 76]);
}
