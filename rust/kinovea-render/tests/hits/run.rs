//! Runs every hit table of `HIT_CASES` through the Rust hit tests and keeps the results per drawing.

use super::cases;
use super::grid;
use super::load;
use super::table;
use kinovea_fixtures::fixtures_root;
use kinovea_render::backend::recording::RecordingRenderer;
use kinovea_render::hit::context::HitContext;
use kinovea_render::hit::drawing::{hit_test, is_ported};
use kinovea_render::viewport::{ImageToViewport, Point};

/// Minimum agreement per drawing and scale (spec: "at least 99 percent").
pub const MIN_PERCENT: f64 = 99.0;
/// Radius, in image pixels, around a mismatch in which Rust must change its answer.
pub const BOUNDARY_RADIUS: i32 = 2;
/// Search radius, in image pixels, for the worst distance of a mismatch to a Rust boundary.
pub const WORST_RADIUS: i32 = 8;

/// One drawing at one viewport scale.
#[derive(Debug, Clone, PartialEq)]
pub struct ScaleResult {
    pub scale: f64,
    /// Agreement with the oracle over the grid, in percent.
    pub percent: f64,
    /// Mismatches not within `BOUNDARY_RADIUS` of a Rust boundary.
    pub off_boundary: usize,
}

/// One drawing of one case, judged at every probed scale.
#[derive(Debug)]
pub struct Judged {
    pub case: &'static str,
    /// `container#index`, as in the probe table.
    pub id: String,
    pub element: String,
    /// False when the kind has no port: it misses everywhere.
    pub ported: bool,
    pub scales: Vec<ScaleResult>,
}

impl Judged {
    /// The strict rule: ported, at least 99 percent at every scale, no mismatch off a boundary.
    pub fn meets_rule(&self) -> bool {
        self.ported
            && self
                .scales
                .iter()
                .all(|s| s.percent >= MIN_PERCENT && s.off_boundary == 0)
    }
}

/// Evaluates every drawing of every case with a probe table.
pub fn evaluate() -> Vec<Judged> {
    let mut out = Vec::new();
    for name in cases::HIT_CASES {
        let case = cases::load(name);
        let csv = std::fs::read_to_string(fixtures_root().join(format!("render/{name}/hits.csv")))
            .expect("hits.csv");
        let rows = table::parse(&csv);
        let m = load::metadata(&case.kva);
        let size = m.video.image_size;
        for (key, timestamp) in table::drawings(&rows) {
            let id = format!("{}#{}", key.container, key.index);
            let label = format!("{name} {id} {}", key.element);
            let drawing = load::drawing(&m, &key);
            let mut judged = Judged {
                case: name,
                id,
                element: key.element.clone(),
                ported: is_ported(drawing),
                scales: Vec::new(),
            };
            if !judged.ported {
                out.push(judged);
                continue;
            }
            let when = if key.container.starts_with('k') {
                timestamp
            } else {
                case.detached_timestamp
            };
            for &scale in &case.scales {
                let expected = table::expected(&rows, &key, scale);
                if expected.exception {
                    println!("{label} x{scale}: skipped, the original throws");
                    continue;
                }
                let viewport = ImageToViewport::new(Point::default(), scale);
                let text = RecordingRenderer::new(size.width as f32, size.height as f32);
                let ctx = HitContext {
                    viewport: &viewport,
                    timestamp: when,
                    metadata: &m,
                    default_fading: &Default::default(),
                    distorter: None,
                    camera: None,
                    text: &text,
                };
                let hit = |p| hit_test(drawing, p, &ctx).code();
                let score = grid::score(&expected, size.width, size.height, case.step, hit);
                let outside = grid::outside_boundary(&score, BOUNDARY_RADIUS, hit);
                let worst = score
                    .mismatches
                    .iter()
                    .map(|m| {
                        grid::boundary_distance((m.x, m.y), WORST_RADIUS, hit)
                            .unwrap_or(f32::INFINITY)
                    })
                    .fold(0.0_f32, f32::max);
                println!(
                    "{label} x{scale}: {:.2}% agree, {:.2}% on active probes, {} mismatches, {outside} off boundary, worst {worst:.1} px",
                    score.percent(),
                    score.active_percent(),
                    score.mismatches.len()
                );
                judged.scales.push(ScaleResult {
                    scale,
                    percent: score.percent(),
                    off_boundary: outside,
                });
            }
            out.push(judged);
        }
    }
    out
}

#[cfg(test)]
#[path = "run_test.rs"]
mod run_test;
