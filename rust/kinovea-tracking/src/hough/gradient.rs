//! 3x3 Sobel gradients with replicated borders, the `spatialGradient(..., BORDER_REPLICATE)`
//! call that `HoughCirclesGradient` makes before its Canny stage.

#[cfg(test)]
#[path = "gradient_test.rs"]
mod gradient_test;

/// Horizontal and vertical derivatives of a gray image, row-major, like the `CV_16S` pair of OpenCV.
pub struct Gradient {
    pub width: usize,
    pub height: usize,
    pub dx: Vec<i16>,
    pub dy: Vec<i16>,
}

/// Row-major index of `(x, y)` when it lies inside a `width x height` grid.
pub(super) fn cell(x: isize, y: isize, width: usize, height: usize) -> Option<usize> {
    let inside = x >= 0 && y >= 0 && (x as usize) < width && (y as usize) < height;
    inside.then(|| y as usize * width + x as usize)
}

/// Sobel 3x3 derivatives without scaling; samples outside the image repeat the border.
pub fn sobel(data: &[u8], width: usize, height: usize) -> Gradient {
    let sample = |x: isize, y: isize| -> i32 {
        let cx = x.clamp(0, width as isize - 1) as usize;
        let cy = y.clamp(0, height as isize - 1) as usize;
        i32::from(data[cy * width + cx])
    };
    let mut dx = Vec::with_capacity(width * height);
    let mut dy = Vec::with_capacity(width * height);
    for y in 0..height as isize {
        for x in 0..width as isize {
            let gx = sample(x + 1, y - 1) + 2 * sample(x + 1, y) + sample(x + 1, y + 1)
                - sample(x - 1, y - 1)
                - 2 * sample(x - 1, y)
                - sample(x - 1, y + 1);
            let gy = sample(x - 1, y + 1) + 2 * sample(x, y + 1) + sample(x + 1, y + 1)
                - sample(x - 1, y - 1)
                - 2 * sample(x, y - 1)
                - sample(x + 1, y - 1);
            dx.push(gx as i16);
            dy.push(gy as i16);
        }
    }
    Gradient {
        width,
        height,
        dx,
        dy,
    }
}
