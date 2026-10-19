//! Preamble of `DrawingGenericPosture.Draw` shared by its parts: the alphas derived from the opacity,
//! the style bound to the background, the viewport points and the option gating of each group.

use crate::paint::helpers::frame::{opacity, to_viewport, to_viewport_all};
use crate::paint::PaintContext;
use crate::renderer::{Font, LineCap, LineJoin, Pen};
use crate::style::colors::with_alpha;
use crate::style::StyleData;
use kinovea_geometry::PointF;
use kinovea_metadata::drawing::GenericPostureDrawing;
use kinovea_metadata::posture::{GenericPosture, TRANSPARENT};
use kinovea_metadata::style::DashStyle;
use kinovea_metadata::Color;

/// `defaultBackgroundAlpha`: the opacity of fills (angles, read-outs).
const BACKGROUND_ALPHA: f64 = 92.0;

pub struct PostureFrame<'a> {
    pub posture: &'a GenericPosture,
    pub ctx: &'a PaintContext<'a>,
    pub opacity: f64,
    pub style: StyleData,
    /// `transformer.Transform(PointList)`.
    pub points: Vec<PointF>,
    pub alpha: i32,
    pub alpha_background: i32,
    /// `infosFading.ReferenceTimestamp`, the time of the point read-outs.
    pub reference_timestamp: i64,
}

impl<'a> PostureFrame<'a> {
    /// `None` when the drawing is fully faded or has no tool description.
    pub fn new(d: &'a GenericPostureDrawing, ctx: &'a PaintContext<'a>) -> Option<Self> {
        let posture = d.posture.as_ref()?;
        let opacity = opacity(&d.header.fading, ctx);
        if opacity <= 0.0 {
            return None;
        }
        // `Bicolor` bound to `line color`: the colour is also the background colour.
        let mut style = StyleData::from_elements(&d.header.style);
        style.background_color = style.color;
        Some(PostureFrame {
            posture,
            ctx,
            opacity,
            style,
            points: to_viewport_all(ctx, &posture.point_list),
            alpha: scaled_alpha(opacity, 255.0),
            alpha_background: scaled_alpha(opacity, BACKGROUND_ALPHA),
            reference_timestamp: d.header.fading.reference_timestamp,
        })
    }

    /// `GetFont(Math.Max(scale, 1))`: the font of the labels.
    pub fn text_font(&self) -> Font {
        self.style.font((self.ctx.viewport.scale as f32).max(1.0))
    }

    /// Alpha of label text: `(int)(opacity * 255)`.
    pub fn text_alpha(&self) -> i32 {
        (self.opacity * 255.0) as i32
    }

    /// `IsActive`: the group is active when every option named in it is on; unknown keys are ignored.
    pub fn is_active(&self, group: &str) -> bool {
        group.is_empty()
            || group.split('|').all(|key| {
                self.posture
                    .options
                    .iter()
                    .find(|o| o.key == key)
                    .is_none_or(|o| o.value)
            })
    }

    /// Base colour of strokes and handles: the background at the frame alpha.
    pub fn edge_base(&self) -> Color {
        with_alpha(self.style.background_color, self.alpha)
    }

    /// Base colour of fills: the background at the background alpha.
    pub fn fill_base(&self) -> Color {
        with_alpha(self.style.background_color, self.alpha_background)
    }

    /// `points[index]` for a point (`index >= 0`) or the computed point `-index - 1`, in viewport
    /// coordinates. Missing points read as the origin, where the original would fail.
    pub fn point(&self, index: i32) -> PointF {
        if index >= 0 {
            self.points
                .get(index as usize)
                .copied()
                .unwrap_or(PointF::EMPTY)
        } else {
            to_viewport(self.ctx, self.computed(index))
        }
    }

    /// Image coordinates of a point or a computed point (`GetUntransformedComputedPoint`).
    pub fn image_point(&self, index: i32) -> PointF {
        if index >= 0 {
            self.posture
                .point_list
                .get(index as usize)
                .copied()
                .unwrap_or(PointF::EMPTY)
        } else {
            self.computed(index)
        }
    }

    /// Image coordinates of the computed point `-index - 1`, recomputed from the current points.
    fn computed(&self, index: i32) -> PointF {
        let slot = (-index - 1) as usize;
        self.posture
            .computed_points
            .get(slot)
            .map_or(PointF::EMPTY, |c| c.location(&self.posture.point_list))
    }
}

/// `(int)(opacity * factor)` clamped to 0..=255, as the original computes its alphas.
fn scaled_alpha(opacity: f64, factor: f64) -> i32 {
    ((opacity * factor) as i32).clamp(0, 255)
}

/// `Transparent` means "use the base colour"; any other colour keeps its RGB at `alpha`.
pub fn colored(color: Color, alpha: i32, base: Color) -> Color {
    if color == TRANSPARENT {
        base
    } else {
        with_alpha(color, alpha)
    }
}

/// Pen of the edges drawn after the segments: flat caps (`NoAnchor`) and round joins.
pub fn edge_pen(color: Color, width: f32, dash: DashStyle) -> Pen {
    Pen {
        dash,
        start_cap: LineCap::Flat,
        end_cap: LineCap::Flat,
        line_join: LineJoin::Round,
        ..Pen::new(color, width)
    }
}
