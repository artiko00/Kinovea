use crate::backend::recording::{Call, RecordingRenderer};
use crate::renderer::{Brush, Renderer};
use kinovea_geometry::RectangleF;
use kinovea_metadata::Color;

#[test]
fn painters_reach_the_recording_backend_through_the_renderer_trait() {
    let mut r = RecordingRenderer::new(100.0, 100.0);
    let rect = RectangleF::new(1.0, 2.0, 3.0, 4.0);
    {
        let surface: &mut dyn Renderer = &mut r;
        surface.fill_rectangle(rect, &Brush::solid(Color::BLACK));
    }
    assert!(matches!(r.calls.as_slice(), [Call::FillRectangle(x, _, _)] if *x == rect));
}
