//! Fixtures shared by the painter tests: a viewport at a given scale with the player's default
//! fading, and a recording surface. Not a test of its own beyond checking the fixture itself.

use crate::backend::recording::RecordingRenderer;
use crate::paint::PaintContext;
use crate::viewport::{ImageToViewport, Point};
use kinovea_metadata::{Fading, Metadata};

/// Owns what a `PaintContext` borrows. The player's default fading is `Fading::default()`; the
/// frame duration of a drawing's own fading is set by each test, as the loader does.
pub struct Scene {
    pub viewport: ImageToViewport,
    pub metadata: Metadata,
    pub default_fading: Fading,
}

impl Scene {
    /// Image origin at `origin` in viewport pixels, at `scale`.
    pub fn new(origin: (i32, i32), scale: f64) -> Self {
        Scene {
            viewport: ImageToViewport::new(Point::new(origin.0, origin.1), scale),
            metadata: Metadata::default(),
            default_fading: Fading::default(),
        }
    }

    pub fn ctx(&self, timestamp: i64) -> PaintContext<'_> {
        PaintContext {
            viewport: &self.viewport,
            timestamp,
            selected: false,
            metadata: &self.metadata,
            default_fading: &self.default_fading,
            distorter: None,
            camera: None,
        }
    }
}

pub fn surface() -> RecordingRenderer {
    RecordingRenderer::new(800.0, 600.0)
}

#[test]
fn scene_context_carries_origin_scale_and_timestamp() {
    let scene = Scene::new((5, 7), 2.0);
    let ctx = scene.ctx(42);
    assert_eq!(ctx.timestamp, 42);
    assert_eq!(ctx.viewport.scale, 2.0);
    assert_eq!(ctx.viewport.location, Point::new(5, 7));
}
