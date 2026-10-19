//! The oracle's probe grid (`WriteHits`) and the comparison of a Rust hit function against it.
//! Agreement is measured over the box of each drawing, not over the whole image.

use super::table::Expected;
use kinovea_geometry::PointF;

/// Agreement required per drawing and scale (spec: "at least 99 percent").
pub const MIN_PERCENT: f64 = 99.0;
/// Margin, in image pixels, that the box of a drawing adds around the probes where it answers.
pub const BOX_MARGIN: i32 = 16;
/// Radius, in image pixels, around a mismatch in which Rust must change its answer.
pub const BOUNDARY_RADIUS: i32 = 2;

/// `for (y = 0; y < h; y += step) for (x = 0; x < w; x += step)`, the probes of `WriteHits`.
pub fn probes(width: i32, height: i32, step: i32) -> Vec<(i32, i32)> {
    (0..height)
        .step_by(step as usize)
        .flat_map(|y| (0..width).step_by(step as usize).map(move |x| (x, y)))
        .collect()
}

/// One probe of the grid: the oracle's answer, Rust's, and for a mismatch whether it lies off a
/// Rust boundary (the second half of the rule).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Probe {
    pub x: i32,
    pub y: i32,
    pub expected: i32,
    pub got: i32,
    pub off_boundary: bool,
}

/// Every probe of the grid, compared with `expected`. Probes not listed are misses (-1).
pub fn grid(
    expected: &Expected,
    width: i32,
    height: i32,
    step: i32,
    hit: impl Fn(PointF) -> i32,
) -> Vec<Probe> {
    probes(width, height, step)
        .into_iter()
        .map(|(x, y)| {
            let want = expected.hits.get(&(x, y)).copied().unwrap_or(-1);
            let got = hit(PointF::new(x as f32, y as f32));
            let off_boundary = want != got && !near_rust_boundary((x, y), BOUNDARY_RADIUS, &hit);
            Probe {
                x,
                y,
                expected: want,
                got,
                off_boundary,
            }
        })
        .collect()
}

/// Agreement of one drawing at one scale, over the box of its probes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Verdict {
    /// Percentage of the probes in the box where Rust and the oracle agree.
    pub percent: f64,
    /// Mismatches in the box that are not within `BOUNDARY_RADIUS` of a Rust boundary.
    pub off_boundary: usize,
}

/// The box of a drawing, inclusive corners `(x0, y0, x1, y1)`: the probes where either side answers
/// (not a miss), grown by `BOX_MARGIN` and clipped to the image. `None` when neither side answers.
pub fn probe_box(probes: &[Probe], width: i32, height: i32) -> Option<(i32, i32, i32, i32)> {
    let active = probes.iter().filter(|p| p.expected != -1 || p.got != -1);
    let (mut x0, mut y0, mut x1, mut y1) = (i32::MAX, i32::MAX, i32::MIN, i32::MIN);
    for p in active {
        x0 = x0.min(p.x);
        y0 = y0.min(p.y);
        x1 = x1.max(p.x);
        y1 = y1.max(p.y);
    }
    let (xs, ys) = ((x0, x1), (y0, y1));
    (x0 <= x1).then(|| {
        let (lo_x, hi_x) = clip(xs, width);
        let (lo_y, hi_y) = clip(ys, height);
        (lo_x, lo_y, hi_x, hi_y)
    })
}

/// The span grown by `BOX_MARGIN` on both sides and clipped to `[0, size - 1]`.
fn clip(span: (i32, i32), size: i32) -> (i32, i32) {
    let (lo, hi) = span;
    (0.max(lo - BOX_MARGIN), (size - 1).min(hi + BOX_MARGIN))
}

/// The verdict of a drawing over its box. A probe where either side answers one of `exempt` (the
/// text zone of the drawing) leaves both the numerator and the denominator. An empty box agrees
/// everywhere: both sides miss.
pub fn verdict(probes: &[Probe], width: i32, height: i32, exempt: &[i32]) -> Verdict {
    let Some((x0, y0, x1, y1)) = probe_box(probes, width, height) else {
        return Verdict {
            percent: 100.0,
            off_boundary: 0,
        };
    };
    let (mut total, mut matches, mut off) = (0usize, 0usize, 0usize);
    let inside = probes
        .iter()
        .filter(|p| (x0..=x1).contains(&p.x) && (y0..=y1).contains(&p.y));
    for p in inside {
        if exempt.contains(&p.expected) || exempt.contains(&p.got) {
            continue;
        }
        total += 1;
        if p.expected == p.got {
            matches += 1;
        } else if p.off_boundary {
            off += 1;
        }
    }
    let percent = if total == 0 {
        100.0
    } else {
        100.0 * matches as f64 / total as f64
    };
    Verdict {
        percent,
        off_boundary: off,
    }
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
