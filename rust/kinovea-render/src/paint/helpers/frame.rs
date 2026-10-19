//! Preamble shared by the `Draw` overrides: fading opacity, image-to-viewport mapping and the
//! stretch applied to pen widths and fonts.

use crate::paint::PaintContext;
use crate::viewport::{Point, Rectangle};
use kinovea_geometry::{PointF, RectangleF};
use kinovea_metadata::measure_label::MeasureLabel;
use kinovea_metadata::Fading;

/// `infosFading.GetOpacityTrackable(trackingTimestamps, timestamp)` for a drawing that is not
/// tracked: `PaintContext` carries no tracking timestamps.
pub fn opacity(fading: &Fading, ctx: &PaintContext) -> f64 {
    fading.opacity_factor(ctx.timestamp, ctx.default_fading)
}

/// `transformer.Transform(PointF)`: the viewport point truncated to whole pixels, widened back to `PointF`.
pub fn to_viewport(ctx: &PaintContext, p: PointF) -> PointF {
    ctx.viewport.transform_point_f(p).to_point_f()
}

/// `transformer.Transform(IEnumerable<PointF>)`: each point through `to_viewport`.
pub fn to_viewport_all(ctx: &PaintContext, points: &[PointF]) -> Vec<PointF> {
    points.iter().map(|p| to_viewport(ctx, *p)).collect()
}

/// `Point.Box(float radius)`: the integer square of side `2 * radius` centred on `c`, with the
/// truncating casts of the original.
pub fn point_box(c: Point, radius: f32) -> Rectangle {
    let side = (radius * 2.0) as i32;
    Rectangle::new(
        (c.x as f32 - radius) as i32,
        (c.y as f32 - radius) as i32,
        side,
        side,
    )
}

/// `PointF.Box(int radius)`: the square of side `2 * radius` centred on `c`.
pub fn point_box_f(c: PointF, radius: i32) -> RectangleF {
    let r = radius as f32;
    RectangleF::new(c.x - r, c.y - r, r * 2.0, r * 2.0)
}

/// Widening of an integer viewport rectangle to the `RectangleF` the renderer takes.
pub fn rect_f(r: Rectangle) -> RectangleF {
    RectangleF::new(r.x as f32, r.y as f32, r.width as f32, r.height as f32)
}

/// `transformer.Scale`: the stretch applied to pen widths and fonts.
pub fn stretch(ctx: &PaintContext) -> f32 {
    ctx.viewport.scale as f32
}

/// `MiniLabel.SetAttach(p, true)`: moves the label by the change of its attach point, so it keeps its
/// place relative to the point it hangs from.
pub fn attach_label(label: &MeasureLabel, attach: PointF) -> MeasureLabel {
    let dx = attach.x - label.attach.x;
    let dy = attach.y - label.attach.y;
    MeasureLabel {
        position: PointF::new(label.position.x + dx, label.position.y + dy),
        attach,
        ..*label
    }
}

/// Fixtures shared by the painter tests: a scene that owns the parts of a `PaintContext`, a style with
/// the test colour and the pen it produces.
#[cfg(test)]
pub mod scene {
    use crate::backend::recording::{Call, RecordingRenderer};
    use crate::paint::PaintContext;
    use crate::renderer::{LineJoin, Pen, Renderer};
    use crate::viewport::{ImageToViewport, Point};
    use kinovea_metadata::{Color, Fading, Metadata, StyleElements, StyleValue};

    pub const COLOR: Color = Color::rgb(200, 10, 20);

    pub struct Scene {
        pub viewport: ImageToViewport,
        pub metadata: Metadata,
        pub fading: Fading,
    }

    impl Scene {
        pub fn new(location: Point, scale: f64) -> Self {
            Scene {
                viewport: ImageToViewport::new(location, scale),
                metadata: Metadata::default(),
                fading: Fading::default(),
            }
        }

        pub fn ctx(&self, timestamp: i64) -> PaintContext<'_> {
            PaintContext {
                viewport: &self.viewport,
                timestamp,
                selected: false,
                metadata: &self.metadata,
                default_fading: &self.fading,
                distorter: None,
                camera: None,
            }
        }

        /// Paints with `paint` onto a 400 by 400 recording surface and returns the calls.
        pub fn record(
            &self,
            timestamp: i64,
            paint: impl FnOnce(PaintContext<'_>, &mut dyn Renderer),
        ) -> Vec<Call> {
            let mut r = RecordingRenderer::new(400.0, 400.0);
            paint(self.ctx(timestamp), &mut r);
            r.calls
        }
    }

    /// Style with the test colour and a line size; callers insert the other elements.
    pub fn style(size: i32) -> StyleElements {
        StyleElements::new(vec![
            ("color", StyleValue::Color(COLOR)),
            ("line size", StyleValue::LineSize(size)),
        ])
    }

    /// The pen of `GetPen`: round joins and flat caps.
    pub fn round_pen(width: f32) -> Pen {
        Pen {
            line_join: LineJoin::Round,
            ..Pen::new(COLOR, width)
        }
    }
}
