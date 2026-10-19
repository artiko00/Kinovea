//! Gradient vote accumulator of `HoughCirclesAccumInvoker` and the center search of
//! `HoughCirclesFindCentersInvoker` (OpenCV `hough.cpp`). Positions are fixed point with
//! 1/1024 px units, as in OpenCV, so the walk rounds the same way.

#[cfg(test)]
#[path = "accum_test.rs"]
mod accum_test;

use super::gradient::{cell, Gradient};
use std::cmp::Reverse;

const ONE: f32 = 1024.0;

/// Votes per accumulator cell, `cols x rows` at `1 / dp` resolution.
pub struct Accumulator {
    cols: usize,
    rows: usize,
    votes: Vec<i32>,
}

impl Accumulator {
    fn at(&self, x: isize, y: isize) -> i32 {
        cell(x, y, self.cols, self.rows).map_or(0, |i| self.votes[i])
    }

    /// Votes `count` consecutive positions of a ray, stopping at the first cell outside.
    fn walk(&mut self, mut x1: i32, mut y1: i32, sx: i32, sy: i32, count: i32) {
        for _ in 0..count {
            match cell(
                (x1 >> 10) as isize,
                (y1 >> 10) as isize,
                self.cols,
                self.rows,
            ) {
                Some(i) => self.votes[i] += 1,
                None => break,
            }
            x1 += sx;
            y1 += sy;
        }
    }
}

/// Every edge pixel votes at each integer radius in `min_r..=max_r`, along both directions of its
/// gradient. `idp` is `1 / dp`.
pub fn vote(grad: &Gradient, edges: &[bool], idp: f32, min_r: i32, max_r: i32) -> Accumulator {
    let cols = (grad.width as f32 * idp).ceil() as usize;
    let rows = (grad.height as f32 * idp).ceil() as usize;
    let mut acc = Accumulator {
        cols,
        rows,
        votes: vec![0; cols * rows],
    };
    let edge_pixels = edges
        .iter()
        .enumerate()
        .filter_map(|(i, &e)| e.then_some(i));
    for i in edge_pixels {
        let (x, y) = (i % grad.width, i / grad.width);
        let (vx, vy) = (f32::from(grad.dx[i]), f32::from(grad.dy[i]));
        let mag = (vx * vx + vy * vy).sqrt();
        let mut sx = (((vx * idp) * ONE) / mag).round_ties_even() as i32;
        let mut sy = (((vy * idp) * ONE) / mag).round_ties_even() as i32;
        let x0 = ((x as f32 * idp) * ONE).round_ties_even() as i32;
        let y0 = ((y as f32 * idp) * ONE).round_ties_even() as i32;
        for _ in 0..2 {
            acc.walk(x0 + min_r * sx, y0 + min_r * sy, sx, sy, max_r - min_r + 1);
            sx = -sx;
            sy = -sy;
        }
    }
    acc
}

/// Cells above `threshold` that beat their left and up neighbours strictly and their right and
/// down neighbours with `>=`, as OpenCV. Most voted first; ties keep raster order.
pub fn centers(acc: &Accumulator, threshold: i32) -> Vec<(usize, usize)> {
    let mut found = Vec::new();
    for y in 1..acc.rows as isize {
        for x in 1..acc.cols as isize {
            let v = acc.at(x, y);
            let is_peak = v > threshold
                && v > acc.at(x - 1, y)
                && v >= acc.at(x + 1, y)
                && v > acc.at(x, y - 1)
                && v >= acc.at(x, y + 1);
            if is_peak {
                found.push((x as usize, y as usize));
            }
        }
    }
    found.sort_by_key(|&(x, y)| Reverse(acc.at(x as isize, y as isize)));
    found
}
