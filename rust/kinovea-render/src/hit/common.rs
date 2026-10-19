//! Pieces shared by the `HitTest` ports: the opacity gates, the square of a centre and the style
//! inputs that the original reads from its drawing.

use super::context::HitContext;
use crate::style::StyleData;
use kinovea_geometry::{PointF, RectangleF};
use kinovea_metadata::style::StyleValue;
use kinovea_metadata::Header;

/// `infosFading.GetOpacityFactor(timestamp)`: the drawing's fading against the player defaults.
pub fn opacity(header: &Header, ctx: &HitContext) -> f64 {
    header
        .fading
        .opacity_factor(ctx.timestamp, ctx.default_fading)
}

/// `infosFading.GetOpacityTrackable(trackingTimestamps, timestamp)` with `trackingTimestamps` at -1.
/// Only live tracking sets that value, a KVA load leaves it at -1, so this is the baseline fading.
pub fn trackable_opacity(header: &Header, ctx: &HitContext) -> f64 {
    header
        .fading
        .opacity_trackable(-1, ctx.timestamp, ctx.default_fading)
}

/// `PointF.Box(int radius)`: the square of side `2 * radius` centred on `c`.
pub fn box_around(c: PointF, radius: i32) -> RectangleF {
    let r = radius as f32;
    RectangleF::new(c.x - r, c.y - r, r * 2.0, r * 2.0)
}

/// `styleData.LineSize` of the drawing (`StyleData::from_elements`, keyed binding).
pub fn line_size(header: &Header) -> i32 {
    StyleData::from_elements(&header.style).line_size
}

/// A style toggle that is on (`StyleElements` key, e.g. `curved`, `perspective`).
pub fn toggle_on(header: &Header, key: &str) -> bool {
    matches!(
        header.style.get(key),
        Some(StyleValue::Toggle { value: true, .. })
    )
}

/// `PointF.NearlyCoincideWith`: both coordinates differ by less than half a pixel.
pub fn nearly_coincide(a: PointF, b: PointF) -> bool {
    (a.x - b.x).abs() < 0.5 && (a.y - b.y).abs() < 0.5
}

/// Test scaffolding: a context over an uncalibrated metadata at `scale`, with the oracle defaults.
#[cfg(test)]
pub mod testing {
    use super::super::context::HitContext;
    use crate::renderer::Renderer;
    use crate::viewport::{ImageToViewport, Point};
    use kinovea_metadata::{Fading, Metadata};

    /// Owns what a `HitContext` borrows.
    pub struct Scene {
        pub viewport: ImageToViewport,
        pub metadata: Metadata,
        pub fading: Fading,
        pub text: crate::backend::recording::RecordingRenderer,
    }

    impl Scene {
        pub fn at_scale(scale: f64) -> Self {
            Scene {
                viewport: ImageToViewport::new(Point::default(), scale),
                metadata: Metadata::default(),
                fading: Fading::default(),
                text: crate::backend::recording::RecordingRenderer::new(320.0, 240.0),
            }
        }

        pub fn ctx(&self, timestamp: i64) -> HitContext<'_> {
            let text: &dyn Renderer = &self.text;
            HitContext {
                viewport: &self.viewport,
                timestamp,
                metadata: &self.metadata,
                default_fading: &self.fading,
                distorter: None,
                camera: None,
                text,
            }
        }
    }
}

#[cfg(test)]
#[path = "common_test.rs"]
mod common_test;
