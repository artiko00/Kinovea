//! `HitTest` of the angle drawing: three handles and the swept pie as body.

use super::common::{nearly_coincide, trackable_opacity};
use super::context::{Hit, HitContext};
use super::hit_point;
use super::sweep::{sweep_contains, SweepFlags};
use kinovea_geometry::PointF;
use kinovea_metadata::drawing::Angle;

/// `DrawingAngle.HitTest` (DrawingAngle.cs:254-272): handles o=1, a=2, b=3, then the swept pie as
/// the body (`SweepAngle.Hit`). The legs are fixed up first, as `FixIfNull` does on draw.
pub fn hit_angle(d: &Angle, p: PointF, ctx: &HitContext) -> Hit {
    if trackable_opacity(&d.header, ctx) <= 0.0 {
        return Hit::Miss;
    }
    let (o, a, b) = fixed_legs(d.o, d.a, d.b, ctx.viewport.untransform_distance(50));
    if hit_point(p, o, ctx.viewport) {
        return Hit::Handle(1);
    }
    if hit_point(p, a, ctx.viewport) {
        return Hit::Handle(2);
    }
    if hit_point(p, b, ctx.viewport) {
        return Hit::Handle(3);
    }
    let flags = SweepFlags {
        signed: d.signed,
        counter_clockwise: d.counter_clockwise,
        supplementary: d.supplementary,
    };
    if sweep_contains(o, a, b, 0, flags, p, ctx) {
        return Hit::Body;
    }
    Hit::Miss
}

/// `FixIfNull`: a leg that coincides with the origin is moved `length` away (up for a, right for b).
fn fixed_legs(o: PointF, a: PointF, b: PointF, length: i32) -> (PointF, PointF, PointF) {
    let len = length as f32;
    let a = if nearly_coincide(a, o) {
        o.translate(0.0, -len)
    } else {
        a
    };
    let b = if nearly_coincide(b, o) {
        o.translate(len, 0.0)
    } else {
        b
    };
    (o, a, b)
}
