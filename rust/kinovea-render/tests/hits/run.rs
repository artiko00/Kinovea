//! Runs every drawing of every case with a probe table through the Rust hit tests, and keeps the verdicts.

use super::cases;
use super::check::rows_match;
use super::grid;
use super::load;
use super::rule::pending_codes;
use super::table;
use super::targets;
use kinovea_fixtures::fixtures_root;
use kinovea_geometry::PointF;
use kinovea_render::backend::recording::RecordingRenderer;
use kinovea_render::hit::context::HitContext;
use kinovea_render::viewport::{ImageToViewport, Point};

pub use grid::Verdict;

/// One drawing at one viewport scale: the strict verdict, and the verdict of its text zone (the same as
/// the strict one when the drawing is not listed in `PENDING_TEXT_METRICS`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScaleResult {
    pub scale: f64,
    pub plain: Verdict,
    pub text: Verdict,
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
    /// The scales that were compared.
    pub scales: Vec<ScaleResult>,
    /// The scales where the original throws (`exception` rows), not compared.
    pub exceptions: Vec<f64>,
}

impl Judged {
    /// The strict rule, without exemptions: ported, at least 99 percent at every scale, no mismatch off a
    /// boundary. A drawing with no compared scale cannot meet it.
    pub fn meets_rule(&self) -> bool {
        self.ported
            && !self.scales.is_empty()
            && self
                .scales
                .iter()
                .all(|s| s.plain.percent >= grid::MIN_PERCENT && s.plain.off_boundary == 0)
    }
}

/// The result of the whole run: the drawings, and the problems of the tables themselves.
#[derive(Debug, Default)]
pub struct Evaluation {
    pub judged: Vec<Judged>,
    pub problems: Vec<String>,
}

/// Evaluates every drawing of every case with a probe table.
pub fn evaluate() -> Evaluation {
    let mut out = Evaluation::default();
    for name in cases::HIT_CASES {
        let case = cases::load(name);
        let path = fixtures_root().join(format!("render/{name}/hits.csv"));
        let rows = table::parse(&std::fs::read_to_string(path).expect("hits.csv"));
        let m = load::metadata(&case.kva);
        let size = m.video.image_size;
        let entries = targets::entries(&m, case.detached_timestamp);
        out.problems.extend(rows_match(name, &rows, &entries));
        for e in &entries {
            let id = format!("{}#{}", e.key.container, e.key.index);
            let codes = pending_codes(name, &id).unwrap_or(&[]);
            let mut judged = Judged {
                case: name,
                id: id.clone(),
                element: e.key.element.clone(),
                ported: e.target.is_ported(),
                scales: Vec::new(),
                exceptions: Vec::new(),
            };
            for &scale in &case.scales {
                if !judged.ported {
                    break;
                }
                let expected = table::expected(&rows, &e.key, scale);
                if expected.exception {
                    println!(
                        "{name} {id} {} x{scale}: the original throws, not compared",
                        e.key.element
                    );
                    judged.exceptions.push(scale);
                    continue;
                }
                let viewport = ImageToViewport::new(Point::default(), scale);
                let text = RecordingRenderer::new(size.width as f32, size.height as f32);
                let ctx = HitContext {
                    viewport: &viewport,
                    timestamp: e.timestamp,
                    metadata: &m,
                    default_fading: &Default::default(),
                    distorter: None,
                    camera: None,
                    text: &text,
                };
                let hit = |p: PointF| e.target.hit(p, &ctx).code();
                let probes = grid::grid(&expected, size.width, size.height, case.step, hit);
                let (w, h) = (size.width, size.height);
                let result = ScaleResult {
                    scale,
                    plain: grid::verdict(&probes, w, h, &[]),
                    text: grid::verdict(&probes, w, h, codes),
                };
                println!(
                    "{name} {id} {} x{scale}: {:.2}% agree in the box, {} off boundary; text zone {:.2}%",
                    e.key.element, result.plain.percent, result.plain.off_boundary, result.text.percent
                );
                judged.scales.push(result);
            }
            out.judged.push(judged);
        }
    }
    out
}

#[cfg(test)]
#[path = "run_test.rs"]
mod run_test;
