use crate::backend::recording::RecordingRenderer;
use crate::paint::helpers::support_test::Scene;
use crate::paint::paint_drawing;
use kinovea_geometry::PointF;
use kinovea_metadata::drawing::Label;
use kinovea_metadata::Drawing;

#[test]
fn a_drawing_past_its_fading_paints_nothing() {
    let mut label = Label::new("Label", PointF::new(10.0, 20.0));
    label.header.fading.average_timestamps_per_frame = 10.0;
    let scene = Scene::new((0, 0), 1.0);
    let mut r = RecordingRenderer::new(800.0, 600.0);
    // 200 timestamps after the reference is the end of the fading: opacity 0.
    paint_drawing(&Drawing::Label(label), &scene.ctx(200), &mut r);
    assert!(r.calls.is_empty());
}
