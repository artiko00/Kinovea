//! The oracle's probe grid (`WriteHits`) and the comparison of a Rust hit function against it.

use super::table::Expected;
use kinovea_geometry::PointF;

/// `for (y = 0; y < h; y += step) for (x = 0; x < w; x += step)`, the probes of `WriteHits`.
pub fn probes(width: i32, height: i32, step: i32) -> Vec<(i32, i32)> {
    (0..height)
        .step_by(step as usize)
        .flat_map(|y| (0..width).step_by(step as usize).map(move |x| (x, y)))
        .collect()
}

/// A probe where Rust and the oracle disagree.
#[derive(Debug, Clone, PartialEq)]
pub struct Mismatch {
    pub x: i32,
    pub y: i32,
    pub expected: i32,
    pub got: i32,
}

/// Agreement over a whole grid for one drawing at one scale.
#[derive(Debug, Default)]
pub struct Score {
    pub total: usize,
    pub matches: usize,
    /// Probes where the oracle or Rust hits (code other than -1), and where both agree on a hit.
    pub active: usize,
    pub active_matches: usize,
    pub mismatches: Vec<Mismatch>,
}

impl Score {
    /// Percentage of probes where Rust and the oracle agree.
    pub fn percent(&self) -> f64 {
        if self.total == 0 {
            return 100.0;
        }
        100.0 * self.matches as f64 / self.total as f64
    }

    /// Diagnostic only: agreement over the probes where either side reports a hit.
    pub fn active_percent(&self) -> f64 {
        if self.active == 0 {
            return 100.0;
        }
        100.0 * self.active_matches as f64 / self.active as f64
    }
}

/// Compares `hit` (Rust, image coordinates) with `expected` over every probe of the grid. Probes
/// that the oracle does not list are expected to be misses (-1).
pub fn score(
    expected: &Expected,
    width: i32,
    height: i32,
    step: i32,
    hit: impl Fn(PointF) -> i32,
) -> Score {
    let mut out = Score::default();
    for (x, y) in probes(width, height, step) {
        let want = expected.hits.get(&(x, y)).copied().unwrap_or(-1);
        let got = hit(PointF::new(x as f32, y as f32));
        out.total += 1;
        if want != -1 || got != -1 {
            out.active += 1;
        }
        if want == got {
            out.matches += 1;
            if want != -1 {
                out.active_matches += 1;
            }
        } else {
            out.mismatches.push(Mismatch {
                x,
                y,
                expected: want,
                got,
            });
        }
    }
    out
}

/// Mismatches whose probe is not within `radius` pixels of a change in the Rust answer. Zero means
/// every disagreement lies on a Rust boundary, the second half of the spec's rule.
pub fn outside_boundary(score: &Score, radius: i32, hit: impl Fn(PointF) -> i32) -> usize {
    score
        .mismatches
        .iter()
        .filter(|m| !near_rust_boundary((m.x, m.y), radius, &hit))
        .count()
}

/// Distance from `p` to the nearest probe, within `max_radius`, where Rust answers differently than at
/// `p`, sampled on a 1 px grid. `None` when Rust does not change within the radius.
pub fn boundary_distance(
    p: (i32, i32),
    max_radius: i32,
    hit: impl Fn(PointF) -> i32,
) -> Option<f32> {
    let here = hit(PointF::new(p.0 as f32, p.1 as f32));
    let mut best: Option<f32> = None;
    for dy in -max_radius..=max_radius {
        for dx in -max_radius..=max_radius {
            let d2 = (dx * dx + dy * dy) as f32;
            if d2 == 0.0 || d2 > (max_radius * max_radius) as f32 {
                continue;
            }
            if hit(PointF::new((p.0 + dx) as f32, (p.1 + dy) as f32)) != here {
                best = Some(best.map_or(d2.sqrt(), |b| b.min(d2.sqrt())));
            }
        }
    }
    best
}

/// Whether Rust changes its answer within `radius` pixels of `p` (`p` is at a boundary of the Rust
/// result), sampled on a 1 px grid. Used to accept mismatches that lie on that boundary.
pub fn near_rust_boundary(p: (i32, i32), radius: i32, hit: impl Fn(PointF) -> i32) -> bool {
    let here = hit(PointF::new(p.0 as f32, p.1 as f32));
    (-radius..=radius).any(|dy| {
        (-radius..=radius).any(|dx| {
            let q = (p.0 + dx, p.1 + dy);
            dx * dx + dy * dy <= radius * radius && hit(PointF::new(q.0 as f32, q.1 as f32)) != here
        })
    })
}

#[cfg(test)]
#[path = "grid_test.rs"]
mod grid_test;
