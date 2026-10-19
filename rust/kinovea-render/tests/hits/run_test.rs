use super::*;
use kinovea_geometry::PointF;
use kinovea_render::hit::context::HitContext;
use kinovea_render::viewport::{ImageToViewport, Point};
use std::collections::BTreeSet;

/// The plane of `all-drawings`, which threw in the original before the video setup. It now answers over
/// the grid without a panic: its corners (1 to 4) and its X line (5). The Y line is not hit here.
#[test]
fn the_plane_answers_over_the_grid_without_throwing() {
    let case = cases::load("all-drawings");
    let m = load::metadata(&case.kva);
    let entries = targets::entries(&m, case.detached_timestamp);
    let plane = entries
        .iter()
        .find(|e| e.key.element == "Plane")
        .expect("the all-drawings case has a plane");
    let text = RecordingRenderer::new(
        m.video.image_size.width as f32,
        m.video.image_size.height as f32,
    );
    let mut answers = BTreeSet::new();
    for &scale in &case.scales {
        let viewport = ImageToViewport::new(Point::default(), scale);
        let ctx = HitContext {
            viewport: &viewport,
            timestamp: plane.timestamp,
            metadata: &m,
            default_fading: &Default::default(),
            distorter: None,
            camera: None,
            text: &text,
        };
        for (x, y) in grid::probes(
            m.video.image_size.width,
            m.video.image_size.height,
            case.step,
        ) {
            let code = plane
                .target
                .hit(PointF::new(x as f32, y as f32), &ctx)
                .code();
            answers.insert(code);
        }
    }
    println!("plane answers over the grid: {answers:?}");
    assert_eq!(answers, BTreeSet::from([-1, 1, 2, 3, 4, 5]));
}
