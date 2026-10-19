//! Port of the 8-bit filter the circle tracker can apply (`blurROI`, off by default in
//! `TrackerCircle.cs`): `medianBlur` replicates the edge pixel outside the image
//! (`copyMakeBorder(..., BORDER_REPLICATE)` in OpenCV). The Sobel of `HoughCircles` lives in `hough`.

use super::image::GrayImage;

/// Median of the `ksize` x `ksize` neighborhood. `ksize` must be odd.
pub fn median_blur(image: &GrayImage, ksize: u32) -> GrayImage {
    assert!(ksize % 2 == 1, "medianBlur needs an odd ksize");
    let r = i64::from(ksize / 2);
    let mut window = Vec::with_capacity((ksize * ksize) as usize);
    let mut data = Vec::with_capacity(image.data.len());
    for y in 0..image.height {
        for x in 0..image.width {
            window.clear();
            for dy in -r..=r {
                for dx in -r..=r {
                    let sx = replicate(i64::from(x) + dx, image.width);
                    let sy = replicate(i64::from(y) + dy, image.height);
                    window.push(image.at(sx, sy));
                }
            }
            let mid = window.len() / 2;
            data.push(*window.select_nth_unstable(mid).1);
        }
    }
    GrayImage {
        width: image.width,
        height: image.height,
        data,
    }
}

/// `BORDER_REPLICATE` index: the nearest in-image index.
fn replicate(i: i64, n: u32) -> u32 {
    i.clamp(0, i64::from(n) - 1) as u32
}
