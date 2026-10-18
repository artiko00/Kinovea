# Tasks

## 1. Context trait and crate wiring

- [x] 1.1 Add `kinovea-core` and `kinovea-geometry` as dependencies and `kinovea-fixtures` as dev-dependency in `rust/kinovea-kinematics/Cargo.toml`; write `src/context_test.rs` (PixelContext maps the image centre to (0,0), inverts Y, `frames_to_seconds(2)` at 100 fps is 0.02, conversions are identity); implement `context.rs`; verify tests pass.

## 2. Filtering

- [x] 2.1 Write `src/stats_test.rs` (Durbin-Watson of a constant residual is NaN, of alternating ±1 is 4/… as computed by the formula, `subtract`) and `src/filtering_result_test.rs`; implement `stats.rs`, `filtering_result.rs`; verify tests pass.
- [x] 2.2 Write `src/butterworth_test.rs` (error on ≤10 samples, reflected padding values on a known ramp, coefficients for fs=100 fc=10 against hand-computed values, normalized score 0 at dw=2); implement `butterworth.rs` and `butterworth/padding.rs`; verify tests pass.
- [x] 2.3 Write `src/moving_average_test.rs` (span below resolution returns input, window size `span/(2·interval)`, padding capped by sentinels); implement `moving_average.rs`; verify tests pass and `cd rust && make verify` is green.
- [x] 2.4 Write `tests/filtering_parity_test.rs` filtering `XRaw`/`YRaw` of `tests/fixtures/kinematics/{linear,circular,winter}/linear.csv` at the fixture fps and asserting cutoff indices from `filtering.json` and filtered `X`/`Y` within 1e-6; verify it passes.

## 3. Trajectory and time series

- [x] 3.1 Write `src/component_test.rs` (enum names round-trip as the CSV headers) and `src/time_series_test.rs` (add/get component, padding of 3 samples by 2 is all NaN, padding of 10 by 2 leaves 6); implement `component.rs`, `time_series.rs`; verify tests pass.
- [x] 3.2 Write `src/trajectory_test.rs` (world mapping through PixelContext, `can_filter` false with 10 samples or filtering disabled, coordinates beyond length return last, best-fit circle on a synthetic circle); implement `trajectory.rs` using `kinovea_geometry::circle_fit`; verify tests pass.

## 4. Linear and angular kinematics

- [x] 4.1 Write `src/linear_test.rs` (2-sample trajectory: distances defined, velocities/accelerations NaN; constant velocity line gives constant speed and zero acceleration in the unpadded interior without smoothing); implement `linear.rs` with distance/velocity/acceleration passes; verify tests pass.
- [x] 4.2 Write `src/linear/smoothing_test.rs` (smoothing enabled changes interior velocities on a noisy series and leaves a 25 fps series unchanged); implement `linear/smoothing.rs`; verify tests pass.
- [x] 4.3 Write `src/angular_test.rs` (error unless 3 trajectories, unsigned wrap −π/2 → 3π/2, supplementary reflection, constant angular speed gives constant ω and zero α interior); implement `angular.rs`; verify tests pass and `make verify` is green.

## 5. Oracle cases and parity

- [x] 5.1 Extend `Kinovea.FixtureGen/Areas/KinematicsArea.cs` with inputs `smoothing` and `filtering` (set and restore the two player preferences) and add `tests/cases/kinematics/{linear-unsmoothed,short,nofilter}.json`; run `tools/oracle/fixtures.sh --verify`; verify the three pre-existing fixture directories are byte-identical (`git diff --stat tests/fixtures/kinematics/{linear,circular,winter}` empty) and the new ones are committed.
- [x] 5.2 Write `tests/kinematics_parity_test.rs` replaying `linear`, `winter`, `linear-unsmoothed`, `short`, `nofilter` through `LinearKinematics` with a `PixelContext` (fps from `filtering.json`, flags from the case) and asserting every cell of `linear.csv` within 1e-6 relative with NaN positions coinciding; verify it passes.
- [x] 5.3 Extend the parity test with `circular/angular.csv` through `AngularKinematics` (legs `o`, `a`, `b` rebuilt from `truth.csv` centre/radius and `linear.csv` raw samples as the generator does, options signed/ccw) within 1e-6 relative; verify it passes.

## 6. Integration

- [x] 6.1 Export the API from `rust/kinovea-kinematics/src/lib.rs`, document the `MeasurementContext` contract for change 04 in the crate doc, update `tests/cases/kinematics/README.md` (new inputs) and `.agent/progress.md`; verify `cd rust && make verify`, `npm run lint` and `cargo test -p kinovea-fixtures` are green.
