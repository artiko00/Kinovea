//! Canny edge map over Sobel derivatives, with the L1 magnitude and the integer sector test of
//! OpenCV (`canny.cpp` and `canny_simd.hpp`, `canny_nms_row`). Only the strong edges and the weak
//! pixels connected to them are kept.

#[cfg(test)]
#[path = "canny_test.rs"]
mod canny_test;

use super::gradient::{cell, Gradient};

/// `tan(22.5 deg)` in 2^15 fixed point, the `TG22` constant of `canny_nms_row`.
const TG22: i32 = 13573;

const WEAK: u8 = 1;
const STRONG: u8 = 2;

/// Edge map with `width * height` entries, row-major.
pub fn canny(grad: &Gradient, low: i32, high: i32) -> Vec<bool> {
    let (w, h) = (grad.width, grad.height);
    let mag: Vec<i32> = grad
        .dx
        .iter()
        .zip(&grad.dy)
        .map(|(&x, &y)| i32::from(x).abs() + i32::from(y).abs())
        .collect();
    let map = Magnitude {
        values: &mag,
        width: w,
        height: h,
    };
    let mut state = vec![0u8; w * h];
    let mut stack = Vec::new();
    for y in 0..h {
        for x in 0..w {
            let i = y * w + x;
            let m = mag[i];
            if m <= low || !is_local_max(&map, grad, x as isize, y as isize) {
                continue;
            }
            if m > high {
                state[i] = STRONG;
                stack.push(i);
            } else {
                state[i] = WEAK;
            }
        }
    }
    while let Some(i) = stack.pop() {
        let (x, y) = ((i % w) as isize, (i / w) as isize);
        for ny in y - 1..=y + 1 {
            for nx in x - 1..=x + 1 {
                if let Some(j) = cell(nx, ny, w, h) {
                    if state[j] == WEAK {
                        state[j] = STRONG;
                        stack.push(j);
                    }
                }
            }
        }
    }
    state.iter().map(|&s| s == STRONG).collect()
}

/// Gradient magnitudes with zeros outside the image.
struct Magnitude<'a> {
    values: &'a [i32],
    width: usize,
    height: usize,
}

impl Magnitude<'_> {
    fn at(&self, x: isize, y: isize) -> i32 {
        cell(x, y, self.width, self.height).map_or(0, |i| self.values[i])
    }
}

/// Non-maximum suppression along the quantized gradient direction.
fn is_local_max(map: &Magnitude, grad: &Gradient, x: isize, y: isize) -> bool {
    let i = y as usize * grad.width + x as usize;
    let (xs, ys) = (i32::from(grad.dx[i]), i32::from(grad.dy[i]));
    let m = map.at(x, y);
    let (ax, ay) = (xs.abs(), ys.abs() << 15);
    let tg22x = ax * TG22;
    if ay < tg22x {
        m > map.at(x - 1, y) && m >= map.at(x + 1, y)
    } else if ay > tg22x + (ax << 16) {
        m > map.at(x, y - 1) && m >= map.at(x, y + 1)
    } else {
        let s = if (xs ^ ys) < 0 { -1 } else { 1 };
        m > map.at(x - s, y - 1) && m > map.at(x + s, y + 1)
    }
}
