//! `HitTest` of the bitmap drawing and of the bounding box it shares with other media.

use super::common::opacity;
use super::context::{Hit, HitContext};
use super::hit_point;
use kinovea_geometry::{PointF, RectangleF};
use kinovea_metadata::drawing::{Bitmap, KvaDrawing};

/// `DrawingBitmap.HitTest` (DrawingBitmap.cs:160-170): invalid or faded bitmaps miss, otherwise the
/// bounding box decides.
pub fn hit_bitmap(d: &Bitmap, p: PointF, ctx: &HitContext) -> Hit {
    if !d.is_valid() || opacity(&d.header, ctx) <= 0.0 {
        return Hit::Miss;
    }
    hit_bounding_box(d.bounding_box, p, ctx)
}

/// `BoundingBox.HitTest` (Helpers/BoundingBox.cs:90-110): corners 1-4 (top-left, top-right,
/// bottom-right, bottom-left) by point box, then the rectangle as body. The probe is truncated to
/// whole pixels first, as `PointF.ToPoint` does.
pub fn hit_bounding_box(r: RectangleF, p: PointF, ctx: &HitContext) -> Hit {
    let corners = [
        PointF::new(r.left(), r.top()),
        PointF::new(r.right(), r.top()),
        PointF::new(r.right(), r.bottom()),
        PointF::new(r.left(), r.bottom()),
    ];
    if let Some(i) = corners.iter().position(|c| hit_point(p, *c, ctx.viewport)) {
        return Hit::Handle(i as u32 + 1);
    }
    if r.contains(PointF::new(p.x.trunc(), p.y.trunc())) {
        return Hit::Body;
    }
    Hit::Miss
}
