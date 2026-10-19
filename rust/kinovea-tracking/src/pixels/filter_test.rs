use crate::pixels::filter::median_blur;
use crate::pixels::image::GrayImage;

fn gray(width: u32, height: u32, data: &[u8]) -> GrayImage {
    GrayImage {
        width,
        height,
        data: data.to_vec(),
    }
}

#[test]
fn median_blur_replicates_the_border() {
    let img = gray(3, 3, &[1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let out = median_blur(&img, 3);
    assert_eq!(out.data[4], 5);
    assert_eq!(out.data[0], 2);
    assert_eq!(out.data[8], 8);
}

#[test]
fn median_blur_removes_an_impulse() {
    let mut data = vec![10u8; 9];
    data[4] = 200;
    let out = median_blur(&gray(3, 3, &data), 3);
    assert_eq!(out.data, vec![10u8; 9]);
}
