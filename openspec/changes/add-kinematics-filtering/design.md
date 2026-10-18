# Design

## Context

See proposal.md. Facts that shape the port:

- `FilteredTrajectory.Initialize` maps every sample through `CalibrationHelper.GetPointAtTime`, so the trajectory already holds world coordinates; filtering happens in world space. `CanFilter = EnableFiltering && count > 10`.
- `ButterworthFilter` keeps all 100 filtered series (the UI lets the user pick another cutoff); `FilteringResult.DurbinWatson` stores the normalized score `|2 − dw| / 2`.
- `LinearKinematics` and `AngularKinematics` read two preferences (`EnableFiltering`, `EnableHighSpeedDerivativesSmoothing`, both default `true`) and six unit conversions through `CalibrationHelper`. The existing fixtures were produced with the defaults, so they include the moving-average pass at 100 fps and 60 fps (Winter at 69.9 fps too: 40 ms span vs 14.3 ms interval).
- The default calibration used by the fixtures is a 100 px horizontal line at the image centre with pixel units, i.e. world = (x − cx, −(y − cy)) and all conversions are identity.
- `MathNet` appears only in `CircleFitter`, which `add-geometry-core` already ports.

## Goals / Non-Goals

**Goals:**
- Bit-level reproducibility of the C# algorithms within 1e-6 relative on the fixtures.
- Decouple kinematics from calibration through a small trait so this change does not wait for `add-calibration-and-lens-distortion`.

**Non-Goals:**
- Choosing cutoffs by any other criterion; the user override is a UI concern.
- Vectorised or parallel filtering.

## Decisions

### D1. `MeasurementContext` trait defined here
```rust
pub trait MeasurementContext {
    fn point_at_time(&self, p: PointF, t: i64) -> PointF;   // image → world
    fn frames_to_seconds(&self, frames: i32) -> f32;
    fn capture_fps(&self) -> f64;
    fn convert_speed(&self, v: f32) -> f32;
    fn convert_acceleration(&self, a: f32) -> f32;
    fn convert_acceleration_from_velocity(&self, a: f32) -> f32;
    fn convert_angle(&self, rad: f32) -> f32;
    fn convert_angular_velocity(&self, rad_s: f32) -> f32;
    fn convert_angular_acceleration(&self, rad_s2: f32) -> f32;
    fn filtering_enabled(&self) -> bool;
    fn high_speed_smoothing_enabled(&self) -> bool;
}
```
`PixelContext { image_size, fps, filtering, smoothing }` implements it for tests and for the MCP's uncalibrated mode. `CalibrationHelper` (change 04) implements it later. Alternative: depend on `kinovea-measurement` now; rejected because it inverts the roadmap dependency and blocks this change.

### D2. Preferences become explicit inputs
The two `PreferencesManager` reads move into the trait (`filtering_enabled`, `high_speed_smoothing_enabled`). The caller (player, MCP) passes preferences in; the crate has no global state.

### D3. Float widths mirror the original
Series are `Vec<f64>`; per-sample arithmetic in `LinearKinematics` goes through `f32` (`PointF`, `GetSpeed`, `GetTime`) and casts to `f64` exactly where C# does. `AngularKinematics` keeps `positions`, `velocities`, `radii` as `f32`. This is what makes 1e-6 relative parity achievable.

### D4. Module layout
`rust/kinovea-kinematics/src/`: `butterworth.rs` (+ `butterworth/padding.rs`), `moving_average.rs`, `stats.rs` (Durbin-Watson, `subtract`), `filtering_result.rs`, `trajectory.rs` (FilteredTrajectory), `component.rs` (Kinematics enum, Component), `time_series.rs` (collection + padder), `linear.rs` (+ `linear/smoothing.rs`), `angular.rs`, `context.rs` (trait + PixelContext). Sibling `<file>_test.rs` for each; parity tests in `tests/kinematics_parity_test.rs`.

### D5. Fixture additions
`tests/cases/kinematics/linear-unsmoothed.json` (`smoothing: false`), `short.json` (`frames: 3`, expecting NaN derivatives) and `nofilter.json` (`filtering: false`). `KinematicsArea.cs` gains the inputs `smoothing` and `filtering`, setting `PreferencesManager.PlayerPreferences.EnableHighSpeedDerivativesSmoothing` / `EnableFiltering` before building (and restoring after). Existing fixtures are regenerated and must be byte-identical (checked by `fixtures.sh --verify` and by `git diff` being empty for the three existing directories).

### Legacy superseded
`Kinovea.ScreenManager/Measurement/Kinematics/{Butterworth,MovingAverage,StatsHelper,FilteringResult,FilteredTrajectory,Kinematics,Component,TimeSeriesCollection,TimeSeriesPadder,LinearKinematics,AngularKinematics}.cs`, `Extensions.Subtract` in `Kinovea.ScreenManager/Extensions.cs`, `Kinovea.Tests/Kinematics/{KinematicsTestData,MovingObject,VideoSynthesizer}.cs` (data stays as a Rust test asset copied from the Winter fixture). Deleted in `remove-dotnet-solution`.

## Risks / Trade-offs

- [`Math.Pow(x, 2)` vs `x*x` and `Math.Tan` differences across runtimes] → use `powi(2)` and std `tan`; both are correctly rounded enough for 1e-6 relative; the fixtures are the arbiter.
- [The cutoff loop `for (fc = min; fc < max; fc += step)` accumulates floating error and may yield 100 or 101 iterations] → port the loop literally (accumulating `fc`), and assert the number of results against the fixture `x_cutoff_index` range.
- [Smoothing at low frame rates returns the input unchanged, hiding bugs] → the `winter` fixture (69.9 fps) exercises the smoothing path; `short` and `nofilter` cover the early returns.

## Migration Plan

Land filter → moving average → trajectory → time series → linear → angular → fixtures and parity. No C# change ships to users; `KinematicsArea.cs` only gains optional inputs.
