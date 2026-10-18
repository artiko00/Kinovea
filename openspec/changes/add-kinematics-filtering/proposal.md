# Proposal

## Why

Kinovea's analytical value lies in the kinematics it derives from tracked points: filtered trajectories, linear and angular time series. The C# implementation (`Kinovea.ScreenManager/Measurement/Kinematics/`) is pure numeric code with a published method (Winter's Butterworth filter, Challis' automatic cutoff) and already has oracle fixtures for three trajectories. Porting it now gives the MCP server and the plots their data layer.

## What Changes

- Port the zero-lag fourth-order Butterworth filter with reflected padding and automatic cutoff selection by Durbin-Watson residual autocorrelation (`ButterworthFilter`, `StatsHelper`, `FilteringResult`).
- Port the moving-average smoother used for high-speed derivative smoothing (`MovingAverage`).
- Port `FilteredTrajectory` (raw and filtered coordinates per axis, cutoff indices, best-fit circle) and the `TimeSeriesCollection` with `Kinematics` component keys and NaN padding.
- Port `LinearKinematics` (distance, displacements, speeds, accelerations by central differences, optional second-pass smoothing) and `AngularKinematics` (angular position/displacement, velocity, tangential, centripetal and resultant accelerations) over a measurement context trait.
- Parity tests against `tests/fixtures/kinematics/{linear,circular,winter}` with declared tolerances, plus new cases for the smoothing pass and for short trajectories.

## Capabilities

### New Capabilities
- `kinematics/trajectory-filtering`: low-pass filtering of sampled coordinates with automatic cutoff selection, moving-average smoothing, and the filtered trajectory model.
- `kinematics/linear-kinematics`: time series of distance, displacement, velocity and acceleration derived from one trajectory.
- `kinematics/angular-kinematics`: time series of angle, angular velocity and accelerations derived from three trajectories (origin and two legs).

### Modified Capabilities
None.

## Impact

- **Assemblies touched**: none of the shipped C# assemblies change. `Kinovea.FixtureGen/Areas/KinematicsArea.cs` gains inputs for the smoothing preference and for short trajectories; `tests/cases/kinematics/` gains cases.
- **Rust**: `rust/kinovea-kinematics` depends on `kinovea-geometry` (points, angles, circle fit) and `kinovea-core` (units, `TimedPoint`). No third-party runtime dependency. Defines the `MeasurementContext` trait that `add-calibration-and-lens-distortion` implements.
- **Verification**: Linux (`cd rust && make verify`, CI `rust.yml`); regenerated fixtures verified with `tools/oracle/fixtures.sh --verify`; `build.yml` as secondary compile check.

## Non-goals

- Calibration itself (image to world mapping, unit conversion tables): `add-calibration-and-lens-distortion`; this change consumes them through a trait and tests with a pixel-space context.
- Plotting, CSV export and the kinematics dialogs: `add-measurement-ui-and-plotting` and `add-importers-exporters`.
- Tracking (producing the trajectories): `add-tracking`.
