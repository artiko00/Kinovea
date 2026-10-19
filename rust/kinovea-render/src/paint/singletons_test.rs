use super::singletons::{paint_coordinate_system, paint_test_grid};
use crate::backend::recording::RecordingRenderer;
use crate::paint::helpers::support_test::Scene;
use kinovea_metadata::singletons::{CoordinateSystem, TestGrid};

#[test]
fn hidden_singletons_paint_nothing() {
    let scene = Scene::new((0, 0), 1.0);
    let mut r = RecordingRenderer::new(800.0, 600.0);
    paint_test_grid(&TestGrid::default(), &scene.ctx(0), &mut r);
    paint_coordinate_system(&CoordinateSystem::default(), &scene.ctx(0), &mut r);
    assert!(r.calls.is_empty());
}
