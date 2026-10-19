//! Owner: P2. Painters for `DrawingSpotlight.Draw` (with `DrawingSpotlightItem`) and
//! `DrawingNumberSequence.Draw` (with `DrawingNumberSequenceItem`).

use super::helpers::rounded_rect::draw_text_box;
use super::PaintContext;
use crate::renderer::{Brush, Path, Pen, Renderer};
use crate::style::colors::with_alpha;
use crate::style::StyleData;
use kinovea_geometry::RectangleF;
use kinovea_metadata::multi::{AutoNumber, NumberSequence, Spotlight, Spotlights};
use kinovea_metadata::style::DashStyle;
use kinovea_metadata::{Color, Fading};

/// `defaultBackgroundAlpha`: alpha of the dim layer at full opacity.
const DIM_ALPHA: f64 = 160.0;

/// Port of `DrawingSpotlight.Draw`: a black layer over the clip with each visible spot cut out of
/// it (alternate fill), then a dashed white ring around each visible spot. The layer's alpha is
/// that of the most visible spot.
pub fn paint_spotlights(d: &Spotlights, ctx: &PaintContext, r: &mut dyn Renderer) {
    let spots: Vec<(&Spotlight, f64)> = d
        .items
        .iter()
        .map(|s| (s, opacity_from(&d.header.fading, s.timestamp, ctx)))
        .filter(|(_, o)| *o > 0.0)
        .collect();
    let Some(max) = spots.iter().map(|(_, o)| *o).reduce(f64::max) else {
        return;
    };
    let mut mask = Path::default();
    mask.rectangle(r.clip_bounds());
    for (s, _) in &spots {
        mask.ellipse(spot_rect(s, ctx));
    }
    let layer = with_alpha(Color::BLACK, (DIM_ALPHA * max) as i32);
    r.fill_path(&mask, &Brush::solid(layer));
    for (s, opacity) in &spots {
        let pen = Pen {
            dash: DashStyle::Dash,
            ..Pen::new(
                with_alpha(Color::argb(255, 255, 255, 255), (opacity * 255.0) as i32),
                2.0,
            )
        };
        r.stroke_ellipse(spot_rect(s, ctx), &pen);
    }
}

/// Port of `DrawingNumberSequence.Draw`: each number is a contoured box at its location.
pub fn paint_numbers(d: &NumberSequence, ctx: &PaintContext, r: &mut dyn Renderer) {
    let style = StyleData::from_elements(&d.header.style);
    for n in &d.items {
        paint_number(n, &d.header.fading, &style, ctx, r);
    }
}

/// `DrawingNumberSequenceItem.Draw`: the value in a box whose contour is a 2 px foreground pen.
fn paint_number(
    n: &AutoNumber,
    fading: &Fading,
    style: &StyleData,
    ctx: &PaintContext,
    r: &mut dyn Renderer,
) {
    let opacity = opacity_from(fading, n.timestamp, ctx);
    if opacity <= 0.0 {
        return;
    }
    let alpha = (opacity * 255.0) as i32;
    let back = style.background_brush(alpha);
    let fore = style.foreground_brush(alpha);
    let font = style.font(ctx.viewport.scale as f32);
    let contour = Pen {
        width: 2.0,
        ..style.foreground_pen(alpha)
    };
    let origin = ctx.viewport.transform_point_f(n.location);
    draw_text_box(
        r,
        &n.value.to_string(),
        &font,
        &back,
        &fore,
        origin,
        Some(&contour),
    );
}

/// Opacity of an item that fades from its own `reference` timestamp: each spot and number has its
/// own `InfosFading` in the original, with the frame duration and fading frames of the container.
fn opacity_from(fading: &Fading, reference: i64, ctx: &PaintContext) -> f64 {
    Fading {
        reference_timestamp: reference,
        ..fading.clone()
    }
    .opacity_factor(ctx.timestamp, ctx.default_fading)
}

/// Viewport box of a spot: `center.Box(transform(radius))`.
fn spot_rect(s: &Spotlight, ctx: &PaintContext) -> RectangleF {
    let centre = ctx.viewport.transform_point_f(s.center);
    let radius = ctx.viewport.transform_distance(s.radius) as f32;
    RectangleF::new(
        centre.x as f32 - radius,
        centre.y as f32 - radius,
        radius * 2.0,
        radius * 2.0,
    )
}

#[cfg(test)]
mod numbers_test;
