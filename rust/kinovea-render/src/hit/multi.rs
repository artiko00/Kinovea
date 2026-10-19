//! `HitTest` of the multi-item singletons: the spotlights and the number sequence. Each item has its
//! own fading, from its own timestamp.

use super::common::box_around;
use super::context::{Hit, HitContext};
use super::label::text_background;
use super::{hit_path, GraphicsPath};
use crate::paint::multi::item_fading;
use kinovea_geometry::PointF;
use kinovea_metadata::multi::{AutoNumber, NumberSequence, Spotlight, Spotlights};

/// `DrawingSpotlight.HitTest` (DrawingSpotlight.cs:158-174): the first spot whose item answers.
pub fn hit_spotlights(d: &Spotlights, p: PointF, ctx: &HitContext) -> Hit {
    d.items
        .iter()
        .map(|s| hit_spot(s, p, ctx))
        .find(|h| *h != Hit::Miss)
        .unwrap_or(Hit::Miss)
}

/// `DrawingSpotlightItem.HitTest` (DrawingSpotlightItem.cs:128-141): trackable opacity, the outline
/// (an arc of the radius, width 2) as handle 1, then the disc as body.
fn hit_spot(s: &Spotlight, p: PointF, ctx: &HitContext) -> Hit {
    let fading = item_fading(s.timestamp, ctx.metadata, ctx.default_fading);
    if fading.opacity_trackable(-1, ctx.timestamp, ctx.default_fading) <= 0.0 {
        return Hit::Miss;
    }
    let bounds = box_around(s.center, s.radius);
    let mut outline = GraphicsPath::new();
    outline.add_arc(bounds, 0.0, 360.0);
    if hit_path(p, &outline, 2, false, ctx.viewport) {
        return Hit::Handle(1);
    }
    let mut disc = GraphicsPath::new();
    disc.add_ellipse(bounds);
    if hit_path(p, &disc, 0, true, ctx.viewport) {
        return Hit::Body;
    }
    Hit::Miss
}

/// `DrawingNumberSequence.HitTest` (DrawingNumberSequence.cs:132-148): the first number whose box
/// holds `p` (`DrawingNumberSequenceItem.HitTest`, factor opacity, body only).
pub fn hit_numbers(d: &NumberSequence, p: PointF, ctx: &HitContext) -> Hit {
    d.items
        .iter()
        .find(|n| hit_number(&d.header, n, p, ctx))
        .map_or(Hit::Miss, |_| Hit::Body)
}

fn hit_number(
    header: &kinovea_metadata::Header,
    n: &AutoNumber,
    p: PointF,
    ctx: &HitContext,
) -> bool {
    let fading = item_fading(n.timestamp, ctx.metadata, ctx.default_fading);
    if fading.opacity_factor(ctx.timestamp, ctx.default_fading) <= 0.0 {
        return false;
    }
    let text = n.value.to_string();
    text_background(n.location, &text, header, ctx).contains(p)
}
