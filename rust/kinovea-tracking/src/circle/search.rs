//! Port of `TrackerCircle.FindBestCircleAtRadius`: one Hough search at a radius range over the
//! gray search region. A candidate's votes are the vote threshold that found it, as in the
//! original, not the accumulator count.

#[cfg(test)]
#[path = "search_test.rs"]
mod search_test;

use crate::hough::{hough_circles, HoughCircle, HoughParams};
use crate::pixels::GrayImage;

/// First vote threshold tried, lowered by `GUESS_VOTE_STEP` until circles pass.
const MAX_VOTES: i32 = 50;
const GUESS_VOTE_STEP: i32 = 2;
/// `houghParam1`, the high Canny threshold.
const CANNY_THRESHOLD: i32 = 50;

/// A circle in image coordinates and the vote threshold that found it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct Candidate {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub votes: i32,
}

/// The gray search region, its origin in image coordinates and the point circles are compared with.
pub(super) struct Scan<'a> {
    pub gray: &'a GrayImage,
    pub origin: (i32, i32),
    pub last: (f32, f32),
}

impl Scan<'_> {
    /// From `MAX_VOTES` down, in steps, while above `min_votes`. The first level with circles
    /// answers: the one closest to `last` when there are several.
    pub(super) fn best_at(&self, min_r: f32, max_r: f32, min_votes: i32) -> Option<Candidate> {
        let levels = std::iter::successors(Some(MAX_VOTES), |v| Some(v - GUESS_VOTE_STEP))
            .take_while(|&v| v > min_votes);
        levels.into_iter().find_map(|votes| {
            let params = hough_params(self.gray.width, votes, min_r, max_r);
            let (w, h) = (self.gray.width as usize, self.gray.height as usize);
            let circles = hough_circles(&self.gray.data, w, h, &params);
            closest(&circles, self.origin, self.last).map(|c| Candidate {
                x: c.x,
                y: c.y,
                radius: c.radius,
                votes,
            })
        })
    }
}

/// The `HoughCircles` call of the original: `dp` 1, `minDist` as `max(cols / 16, 8)`, `param1` 50,
/// and the radius range truncated to integers.
fn hough_params(width: u32, votes: i32, min_r: f32, max_r: f32) -> HoughParams {
    HoughParams {
        dp: 1.0,
        min_dist: (width / 16).max(8) as f32,
        canny_threshold: CANNY_THRESHOLD,
        acc_threshold: votes,
        min_radius: min_r as i32,
        max_radius: max_r as i32,
    }
}

/// The circle nearest to `last`, in image coordinates. On ties the first one wins, as the strict
/// comparison of the original.
fn closest(circles: &[HoughCircle], origin: (i32, i32), last: (f32, f32)) -> Option<HoughCircle> {
    circles
        .iter()
        .map(|c| HoughCircle {
            x: c.x + origin.0 as f32,
            y: c.y + origin.1 as f32,
            radius: c.radius,
        })
        .min_by(|a, b| distance(a, last).total_cmp(&distance(b, last)))
}

fn distance(c: &HoughCircle, p: (f32, f32)) -> f32 {
    let (dx, dy) = (c.x - p.0, c.y - p.1);
    (dx * dx + dy * dy).sqrt()
}
