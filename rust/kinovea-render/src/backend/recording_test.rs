use super::recording::RecordingRenderer;
use crate::renderer::Renderer;
use kinovea_geometry::RectangleF;

#[test]
fn the_clip_starts_as_the_whole_surface() {
    let r = RecordingRenderer::new(320.0, 240.0);
    assert_eq!(r.clip_bounds(), RectangleF::new(0.0, 0.0, 320.0, 240.0));
    assert!(r.calls.is_empty());
}
