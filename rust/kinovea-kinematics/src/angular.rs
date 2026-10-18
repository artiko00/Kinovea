//! Angular kinematics of an angle defined by three trajectories (`AngularKinematics`).

use crate::component::Kinematics::{self, *};
use crate::context::MeasurementContext;
use crate::time_series::{pad, TimeSeriesCollection};
use crate::trajectory::FilteredTrajectory;
use kinovea_geometry::helpers::{angle_at, distance};
use kinovea_geometry::PointF;
use std::fmt;

/// How the angle is measured, as in the angle tools.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AngleOptions {
    /// Keep the sign; otherwise wrap negatives to `[0, 2π)`.
    pub signed: bool,
    /// Measure from leg a to leg b counter-clockwise; otherwise from b to a.
    pub ccw: bool,
    /// Measure between b and the reflection of a through the origin.
    pub supplementary: bool,
}

/// Exactly three trajectories (origin, leg a, leg b) are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AngularError(pub usize);

impl fmt::Display for AngularError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "angular kinematics need 3 trajectories (origin, a, b), got {}", self.0)
    }
}

impl std::error::Error for AngularError {}

const COMPONENTS: [Kinematics; 9] = [
    AngularPosition,
    AngularDisplacement,
    TotalAngularDisplacement,
    AngularVelocity,
    TangentialVelocity,
    AngularAcceleration,
    TangentialAcceleration,
    CentripetalAcceleration,
    ResultantLinearAcceleration,
];

/// `trajs` is `[origin, a, b]` of equal length; the origin trajectory decides filtering.
pub fn build_angular_kinematics(
    trajs: &[&FilteredTrajectory],
    options: AngleOptions,
    ctx: &dyn MeasurementContext,
) -> Result<TimeSeriesCollection, AngularError> {
    let [o, a, b] = trajs else {
        return Err(AngularError(trajs.len()));
    };
    let n = o.length();
    let mut tsc = TimeSeriesCollection::new(n);
    tsc.set_times(o.times.clone());
    tsc.initialize_components(&COMPONENTS);
    let mut positions = vec![0f32; n];
    let mut radii = vec![0f32; n];
    for i in 0..n {
        let coords = |t: &FilteredTrajectory| if o.can_filter { t.coordinates(i) } else { t.raw_coordinates(i) };
        let (po, mut pa, mut pb) = (coords(o), coords(a), coords(b));
        if options.supplementary {
            let c = PointF::new(2.0 * po.x - pa.x, 2.0 * po.y - pa.y);
            pa = pb;
            pb = c;
        }
        let mut angle = if options.ccw { angle_at(po, pa, pb) } else { angle_at(po, pb, pa) };
        if !options.signed && angle < 0.0 {
            angle = (std::f64::consts::TAU + f64::from(angle)) as f32;
        }
        positions[i] = angle;
        radii[i] = distance(po, pb);
        tsc[AngularPosition][i] = f64::from(ctx.convert_angle(angle));
        if i > 0 {
            tsc[AngularDisplacement][i] = f64::from(ctx.convert_angle(angle - positions[i - 1]));
            tsc[TotalAngularDisplacement][i] = f64::from(ctx.convert_angle(angle - positions[0]));
        }
    }
    let velocities = compute_velocities(&mut tsc, ctx, &positions, &radii);
    compute_accelerations(&mut tsc, ctx, &velocities, &radii);
    Ok(tsc)
}

fn compute_velocities(tsc: &mut TimeSeriesCollection, ctx: &dyn MeasurementContext, positions: &[f32], radii: &[f32]) -> Vec<f32> {
    let n = positions.len();
    let mut velocities = vec![0f32; n];
    if n > 2 {
        let t = ctx.frames_to_seconds(2);
        for i in 1..n - 1 {
            let omega = (positions[i + 1] - positions[i - 1]) / t;
            velocities[i] = omega;
            tsc[AngularVelocity][i] = f64::from(ctx.convert_angular_velocity(omega));
            tsc[TangentialVelocity][i] = f64::from(ctx.convert_speed(radii[i] * omega));
        }
    }
    pad(&mut tsc[AngularVelocity], 1);
    pad(&mut tsc[TangentialVelocity], 1);
    velocities
}

fn compute_accelerations(tsc: &mut TimeSeriesCollection, ctx: &dyn MeasurementContext, velocities: &[f32], radii: &[f32]) {
    let n = velocities.len();
    if n > 2 {
        let t = ctx.frames_to_seconds(2);
        for i in 1..n - 1 {
            let alpha = (velocities[i + 1] - velocities[i - 1]) / t;
            let at = radii[i] * alpha;
            let ac = radii[i] * velocities[i] * velocities[i];
            let resultant = f64::from(at * at + ac * ac).sqrt() as f32;
            tsc[AngularAcceleration][i] = f64::from(ctx.convert_angular_acceleration(alpha));
            tsc[TangentialAcceleration][i] = f64::from(ctx.convert_acceleration(at));
            tsc[CentripetalAcceleration][i] = f64::from(ctx.convert_acceleration(ac));
            tsc[ResultantLinearAcceleration][i] = f64::from(ctx.convert_acceleration(resultant));
        }
    }
    for key in [AngularAcceleration, TangentialAcceleration, CentripetalAcceleration, ResultantLinearAcceleration] {
        pad(&mut tsc[key], 2);
    }
}
