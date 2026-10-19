//! Port of `TrackerCircle.MatchCircle`: the Hough search around the reference radius, in the
//! search window clamped to the frame. The original's median blur (`blurROI`) is not ported: its
//! flag is a private field fixed to `false` (`TrackerCircle.cs:70`), so the branch is unreachable.

#[cfg(test)]
#[path = "matching_test.rs"]
mod matching_test;

use super::search::{Candidate, Scan};
use crate::pixels::{to_gray, Roi};
use kinovea_geometry::Size;
use kinovea_video::frame::FrameBuffer;

/// `minVotes` of the first search: the lowest vote threshold.
const MIN_VOTES: i32 = 4;

/// The circle that best matches `ref_radius` near `last`, or `None` for a tracking failure.
pub(super) fn match_circle(
    frame: &FrameBuffer,
    ref_radius: f32,
    last: (f32, f32),
    search: Size,
) -> Option<Candidate> {
    // `Math.Round` rounds half to even.
    let aligned = (
        last.0.round_ties_even() as i32,
        last.1.round_ties_even() as i32,
    );
    let roi = Roi::new(
        aligned.0 - search.width / 2,
        aligned.1 - search.height / 2,
        search.width,
        search.height,
    )
    .clamp_to(frame.width(), frame.height());
    let gray = to_gray(frame, roi);
    let scan = Scan {
        gray: &gray,
        origin: (roi.x, roi.y),
        last,
    };

    let mut best = scan.best_at(ref_radius, ref_radius + 1.0, MIN_VOTES);
    let min_votes = MIN_VOTES.max(votes(best));
    let mut best_diff = 0.0;

    // Radius space down, while the votes grow.
    let min_radius = ref_radius / 1.5;
    let mut guess = ref_radius - 1.0;
    loop {
        let candidate = scan.best_at(guess, guess + 1.0, min_votes);
        if votes(candidate) <= votes(best) {
            break;
        }
        best = candidate;
        best_diff = (guess - ref_radius).abs();
        guess -= 1.0;
        if guess < min_radius {
            break;
        }
    }

    // Radius space up. With equal votes a candidate wins when closer to the reference radius.
    let max_radius = ref_radius * 1.5;
    guess = ref_radius + 1.0;
    loop {
        let candidate = scan.best_at(guess, guess + 1.0, min_votes);
        let diff = (guess - ref_radius).abs();
        let wins =
            votes(candidate) > votes(best) || (votes(candidate) == votes(best) && diff < best_diff);
        if !wins {
            break;
        }
        best = candidate;
        guess += 1.0;
        if guess > max_radius {
            break;
        }
    }
    best
}

/// Votes of a candidate, zero for a failure.
fn votes(candidate: Option<Candidate>) -> i32 {
    candidate.map_or(0, |c| c.votes)
}
