# Tasks

Every task writes the failing test first (`src/<file>_test.rs` with `#[cfg(test)] mod <file>_test;`, or `tests/<name>_test.rs`), then the code, and ends with `cd rust && make verify` and `npm run lint` green. Strings go through `kinovea-ui/src/strings.rs`; no `.resx` is edited (change 18 localises). Dependencies `egui_plot` and `arboard` need approval before 1.3 and 3.5.

## 1. Oracle fixtures and shared models

- [ ] 1.1 Extend `KinematicsArea` in `Kinovea.FixtureGen` to write `export.csv` through the original `CSVHelper` for `linear`, `circular`, `winter`, and add a `CalibrationArea` emitting `plane.csv` (image quad + real size → world coordinates of sample points, camera position) and `line.csv`; add `tests/cases/calibration/*.json`; run `tools/oracle/fixtures.sh`; commit. Verify `--verify` identical.
- [ ] 1.2 `rust/kinovea-ui/src/analysis/model.rs`: `AnalysisKind`, `SeriesEntry`, `TimeModel`, `PlotSpec`, `AnalysisModel::rebuild` from `kinovea_kinematics` for all five kinds; tests build the model from the `linear` and `circular` fixtures and assert series count, component switch, normalized time range. Verify.
- [ ] 1.3 Add `egui_plot` to the workspace (after approval); `src/analysis/plot_view.rs` maps `PlotSpec` to `egui_plot` items with zoom/pan/reset/tooltip; `egui_kittest` test opens the linear window and asserts legend labels. Verify.
- [ ] 1.4 `src/analysis/plot_raster.rs`: render `PlotSpec` to PNG via `kinovea_render` tiny-skia backend at given size; snapshot test `tests/plot_raster_test.rs` (tolerance) and a size assertion. Verify.

## 2. Calibration dialogs

- [ ] 2.1 `src/measurement/calibrate_line.rs`: view-model (length, unit list from `kinovea_measurement::UnitHelper`, axis), `validate`, `apply` returning a memento; tests apply then measure a second line against `line.csv`. Verify.
- [ ] 2.2 `src/measurement/calibrate_plane.rs`: fields, convexity validation, flips, offset; `apply`; tests against `plane.csv` world coordinates and the non-convex rejection. Verify.
- [ ] 2.3 `src/measurement/preview.rs`: frame + drawing preview using `EguiRenderer`; `egui_kittest` snapshot of the plane preview. Verify.
- [ ] 2.4 Dialog shells for line and plane in `src/measurement/dialogs.rs` wired to the `Calibrate…` context-menu entry of `Line`/`Plane`/`Grid` drawings; `egui_kittest` test enters `100 cm` and asserts the label of a second line. Verify.
- [ ] 2.5 `src/measurement/coordinate_system.rs`: `Tools > Coordinate system` toggle, origin drag, axis options, grid; test moving the origin changes a cross-mark position label to `(0, 0)`. Verify.
- [ ] 2.6 `src/measurement/calibration_validation.rs`: table model (points, computed, expected, error), camera position via `kinovea_measurement::CameraPoser`, CSV copy through `kinovea_export::csv`; tests on the plane fixture; `egui_kittest` edits an expected value. Verify.
- [ ] 2.7 `src/measurement/lens_distortion.rs`: parameters model, `Open…` (Kinovea/Agisoft via `kinovea_measurement::lens::import`), `Save…`, `Calibrate from grid` using `CameraCalibrator`, live grid preview; tests round-trip a Kinovea lens file and an Agisoft file from the lens fixtures of change 04. Verify.
- [ ] 2.8 `src/measurement/time_calibration.rs`: `Configure video timing…` (capture fps, user fps) with slow-motion factor label; tests on the `30/240` and `25/1000` cases. Verify.

## 3. Tracking configuration

- [ ] 3.1 `src/tracking/actions.rs`: `Track path` creation, start/stop/restart/delete-after, `Start all tracking` with progress reporting through `kinovea_tracking::TrackabilityManager`; tests on the synthetic moving-object clip (change 10 fixture) assert the track follows within tolerance. Verify.
- [ ] 3.2 `src/tracking/parameters.rs`: tracking parameters model with preview rectangles, persisted in the KVA track element; test round-trips parameters through `kinovea_metadata`. Verify.
- [ ] 3.3 `src/tracking/trajectory_display.rs`: marker, line style, extent, measurement label, keyframe markers; `apply` undoable; `egui_kittest` picks `Speed` and asserts the label text at a frame. Verify.
- [ ] 3.4 Trackable drawings: `Track` toggle on drawings exposing trackable points, manual re-seed on drag; test tracks an angle 20 frames on the synthetic clip. Verify.
- [ ] 3.5 `Display measure` submenu from `Drawing::supported_measures()`, persisted `MeasureLabelType`; `arboard` clipboard adapter with in-memory fake for tests (after approval); test the line length label under a metre calibration. Verify.

## 4. Analysis windows

- [ ] 4.1 `src/analysis/windows.rs`: non-modal window host, one per `AnalysisKind`, refresh on metadata change; `Tools` menu and track `Analysis…` entries enabled; `egui_kittest` opens each window with and without data (`No trajectory to analyse`). Verify.
- [ ] 4.2 Series selection, component selector, time model, cutoff plot (linear only) bound to `AnalysisModel`; tests for `Speed` unit label and normalized range. Verify.
- [ ] 4.3 Diagrams: angle-angle with time gradient segments, scatter with calibration-plane overlay; snapshot tests. Verify.
- [ ] 4.4 Export actions: `Copy graph`, `Save graph…` (size dialog, `plot_raster`), `Copy data`, `Save data…` via `kinovea_export::csv`; test CSV equals `tests/fixtures/kinematics/linear/export.csv` byte for byte and PNG has the requested size. Verify.

## 5. Integration and documentation

- [ ] 5.1 Integration test `tests/measurement_flow_test.rs`: open synthetic clip, calibrate a plane, track the object, open `Linear kinematics`, export CSV and PNG, reload KVA and assert calibration and track parameters persisted. Verify in CI.
- [ ] 5.2 Manual check on the `windows` host: open the KVA from 5.1 in the original app, confirm calibration, track and display options; record in `.agent/progress.md`. Verify entry present.
- [ ] 5.3 Update `AGENTS.md`, `architecture.md` (measurement UI and plotting in Rust, OxyPlot superseded), `openspec/roadmap.md` tick 16; verify `npm run lint` and `make verify`.
