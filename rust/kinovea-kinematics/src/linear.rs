//! Linear kinematics of one trajectory (`LinearKinematics`): cumulative distance, displacements,
//! velocities and accelerations by central differences, with optional derivative smoothing.

mod smoothing;
#[cfg(test)]
mod smoothing_test;

use crate::component::{Component, Kinematics};
use crate::context::MeasurementContext;
use crate::time_series::{pad, TimeSeriesCollection};
use crate::trajectory::FilteredTrajectory;
use kinovea_geometry::helpers::distance;
use kinovea_geometry::PointF;
use Kinematics::*;

const VELOCITIES: [Kinematics; 3] = [LinearSpeed, LinearHorizontalVelocity, LinearVerticalVelocity];
const ACCELERATIONS: [Kinematics; 3] = [LinearAcceleration, LinearHorizontalAcceleration, LinearVerticalAcceleration];
const COMPONENTS: [Component; 3] = [Component::Magnitude, Component::Horizontal, Component::Vertical];

/// Builds the linear time series of `traj` in the units of `ctx`.
pub fn build_linear_kinematics(traj: &FilteredTrajectory, ctx: &dyn MeasurementContext) -> TimeSeriesCollection {
    let mut tsc = TimeSeriesCollection::new(traj.length());
    if traj.length() == 0 {
        return tsc;
    }
    tsc.set_times(traj.times.clone());
    tsc.add_component(XRaw, traj.raw_xs.clone());
    tsc.add_component(YRaw, traj.raw_ys.clone());
    tsc.add_component(X, traj.xs().to_vec());
    tsc.add_component(Y, traj.ys().to_vec());
    let coord = |i: usize| if traj.can_filter { traj.coordinates(i) } else { traj.raw_coordinates(i) };
    tsc.initialize_components(&[LinearDistance, LinearHorizontalDisplacement, LinearVerticalDisplacement]);
    tsc.initialize_components(&VELOCITIES);
    tsc.initialize_components(&ACCELERATIONS);
    compute_distances(&mut tsc, &coord);
    compute_velocities(&mut tsc, ctx, &coord);
    compute_accelerations(&mut tsc, ctx);
    tsc
}

fn compute_distances(tsc: &mut TimeSeriesCollection, coord: &dyn Fn(usize) -> PointF) {
    let o = coord(0);
    for i in 1..tsc.length() {
        let (a, b) = (coord(i - 1), coord(i));
        let previous = tsc[LinearDistance][i - 1];
        tsc[LinearDistance][i] = previous + f64::from(get_distance(a, b, Component::Magnitude));
        tsc[LinearHorizontalDisplacement][i] = f64::from(get_distance(o, b, Component::Horizontal));
        tsc[LinearVerticalDisplacement][i] = f64::from(get_distance(o, b, Component::Vertical));
    }
}

fn compute_velocities(tsc: &mut TimeSeriesCollection, ctx: &dyn MeasurementContext, coord: &dyn Fn(usize) -> PointF) {
    let n = tsc.length();
    if n > 2 {
        let t = ctx.frames_to_seconds(2);
        for i in 1..n - 1 {
            let (a, b) = (coord(i - 1), coord(i + 1));
            for (key, component) in VELOCITIES.iter().zip(COMPONENTS) {
                tsc[*key][i] = f64::from(ctx.convert_speed(get_distance(a, b, component) / t));
            }
        }
    }
    for key in VELOCITIES {
        pad(&mut tsc[key], 1);
    }
    if n > 2 && ctx.high_speed_smoothing_enabled() {
        smoothing::smooth(tsc, &VELOCITIES, ctx.capture_fps(), smoothing::VELOCITY_SPAN_MS, 1);
    }
}

fn compute_accelerations(tsc: &mut TimeSeriesCollection, ctx: &dyn MeasurementContext) {
    let n = tsc.length();
    if n > 4 {
        let t = f64::from(ctx.frames_to_seconds(2));
        for i in 2..n - 2 {
            for (key, velocity) in ACCELERATIONS.iter().zip(VELOCITIES) {
                let acceleration = (tsc[velocity][i + 1] - tsc[velocity][i - 1]) / t;
                tsc[*key][i] = f64::from(ctx.convert_acceleration_from_velocity(acceleration as f32));
            }
        }
    }
    for key in ACCELERATIONS {
        pad(&mut tsc[key], 2);
    }
    if n > 4 && ctx.high_speed_smoothing_enabled() {
        smoothing::smooth(tsc, &ACCELERATIONS, ctx.capture_fps(), smoothing::ACCELERATION_SPAN_MS, 2);
    }
}

/// Single precision distance between `a` and `b` along `component`.
pub fn get_distance(a: PointF, b: PointF, component: Component) -> f32 {
    match component {
        Component::Magnitude => distance(a, b),
        Component::Horizontal => b.x - a.x,
        Component::Vertical => b.y - a.y,
    }
}
