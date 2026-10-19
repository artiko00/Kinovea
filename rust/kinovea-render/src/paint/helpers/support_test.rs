//! Fixtures shared by the painter tests: a viewport at a given scale with the player's default
//! fading, and a recording surface. It holds no test of its own.

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
            decimal_places: crate::paint::helpers::decimal::DEFAULT_DECIMAL_PLACES,
            distorter: None,
            camera: None,
        }
    }
}

pub fn surface() -> RecordingRenderer {
    RecordingRenderer::new(800.0, 600.0)
}
