use super::posture::paint_posture;
use crate::backend::recording::RecordingRenderer;
use crate::paint::helpers::support_test::Scene;
use kinovea_metadata::drawing::GenericPostureDrawing;
use uuid::Uuid;

#[test]
fn a_drawing_without_its_posture_paints_nothing() {
    let d = GenericPostureDrawing::new(Uuid::nil(), None);
    let scene = Scene::new((0, 0), 1.0);
    let mut r = RecordingRenderer::new(800.0, 600.0);
    paint_posture(&d, &scene.ctx(0), &mut r);
    assert!(r.calls.is_empty());
}
