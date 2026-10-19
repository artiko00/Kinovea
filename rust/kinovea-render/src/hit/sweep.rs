//! The swept pie of `AngleHelper` / `SweepAngle` (AngleHelper.cs:69-87, SweepAngle.cs:58-80, 139-146):
//! the sector between two legs from an origin, as `Update` computes it and `Hit` tests it.

use super::context::HitContext;
use super::{hit_path, GraphicsPath};
use kinovea_geometry::PointF;

/// Segments used to flatten the pie arc.
const ARC_STEPS: usize = 64;

/// The angle flags of `AngleHelper.Update`: signed, counter-clockwise, supplementary.
#[derive(Debug, Clone, Copy)]
pub struct SweepFlags {
    pub signed: bool,
    pub counter_clockwise: bool,
    pub supplementary: bool,
}

/// `SweepAngle.Hit` after `AngleHelper.Update(o, a, b, ..)` with a radius (`radius <= 0` falls back
/// to the shorter leg). `o == a` or `o == b` leave no pie. A zero-size box never hits.
pub fn sweep_contains(
    o: PointF,
    a: PointF,
    b: PointF,
    radius: i32,
    flags: SweepFlags,
    p: PointF,
    ctx: &HitContext,
) -> bool {
    if o == a || o == b {
        return false;
    }
    let (a, b) = if flags.supplementary {
        (b, PointF::new(2.0 * o.x - a.x, 2.0 * o.y - a.y))
    } else {
        (a, b)
    };
    let (oa, ob) = if flags.counter_clockwise {
        (absolute_angle(o, b), absolute_angle(o, a))
    } else {
        (absolute_angle(o, a), absolute_angle(o, b))
    };
    let mut start = oa;
    let mut sweep = if ob > oa { ob - oa } else { 360.0 - oa + ob };
    if flags.signed && sweep > 180.0 {
        start = ob;
        sweep = 360.0 - sweep;
    }
    let r = box_radius(radius, o, a, b);
    r != 0 && pie_contains(o, r, start, sweep, p, ctx)
}

/// `SweepAngle.GetAbsoluteAngle`: clockwise degrees in [0, 360) from +x, y pointing down.
fn absolute_angle(o: PointF, p: PointF) -> f32 {
    let dx = f64::from(p.x - o.x);
    let dy = f64::from(o.y - p.y);
    let angle = dy.atan2(dx);
    let draw = if angle > 0.0 {
        std::f64::consts::TAU - angle
    } else {
        -angle
    };
    draw.to_degrees() as f32
}

/// `UpdateBoundingBox` (SweepAngle.cs:113-150): a positive radius is used as is; otherwise the
/// shorter leg minus 10 px (the radius capped at -10), or 10 px for short legs, truncated.
fn box_radius(radius: i32, o: PointF, a: PointF, b: PointF) -> i32 {
    if radius > 0 {
        return radius;
    }
    let smallest = dist(o, a).min(dist(o, b));
    let r = (radius as f32).min(-10.0);
    let r = if smallest + r > 0.0 {
        smallest + r
    } else {
        smallest.min(10.0)
    };
    r.trunc() as i32
}

fn dist(a: PointF, b: PointF) -> f32 {
    kinovea_geometry::helpers::distance(a, b)
}

/// `Region(AddPie(box, start, sweep)).IsVisible(p)`, the pie flattened to a polygon.
fn pie_contains(
    o: PointF,
    radius: i32,
    start: f32,
    sweep: f32,
    p: PointF,
    ctx: &HitContext,
) -> bool {
    let r = radius as f64;
    let mut poly = vec![o];
    for k in 0..=ARC_STEPS {
        let deg = f64::from(start) + f64::from(sweep) * k as f64 / ARC_STEPS as f64;
        let rad = deg.to_radians();
        poly.push(PointF::new(
            o.x + (r * rad.cos()) as f32,
            o.y + (r * rad.sin()) as f32,
        ));
    }
    let mut path = GraphicsPath::new();
    path.add_polygon(&poly);
    hit_path(p, &path, 0, true, ctx.viewport)
}
