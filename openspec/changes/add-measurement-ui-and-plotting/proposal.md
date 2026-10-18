# Proposal

## Why

Changes 03, 04 and 10 port calibration, kinematics and tracking as libraries, but a desktop user still cannot calibrate a scene, configure a track or see a speed curve. This change adds the measurement dialogs and data-analysis windows, with `egui_plot` replacing OxyPlot.

## What Changes

- Calibration dialogs: `Calibrate line` (from a `Line` drawing context menu), `Calibrate plane` (from a `Plane`/`Grid` drawing), coordinate system origin and axis orientation, `Calibration validation` window (`Tools > Calibration validation`), lens distortion dialog (`Image > Lens calibration > Manual…`, open/save distortion files, Agisoft import).
- Tracking configuration: tracking parameters dialog for a track (search and template window sizes, similarity threshold, update strategy), `Configure trajectory display` dialog (marker, line, measurement label, visibility), start/stop tracking actions on tracks and trackable drawings, measurement label options per drawing (`Display measure` submenu: none, name, distance, speed, acceleration…).
- Data analysis windows replacing the OxyPlot forms: `Linear kinematics` (multi-trajectory time series with filter cutoff plot), `Angular kinematics`, `Angle-angle diagram`, `Scatter diagram` (points analysis with calibration plane overlay), `Track analysis` (one track, position/speed/acceleration), each with series selection, time model, and the four export actions (copy graph, save graph PNG at chosen size, copy data CSV, save data CSV).
- `Video > Configure timebase…` and `Time calibration` wiring to the calibration helper.

## Capabilities

### New Capabilities
- `ui/calibration-dialogs`: spatial calibration by line or plane, coordinate system, calibration validation, lens distortion calibration and file import/export dialogs.
- `ui/tracking-configuration`: dialogs and actions to configure, start and stop tracking of tracks and trackable drawings, and the trajectory display options.
- `ui/kinematics-plots`: the data-analysis windows with time-series and diagram plots, series selection, and graph/data export actions.

### Modified Capabilities
None.

## Impact

- **Rust**: `rust/kinovea-ui` gains `measurement/` (dialogs) and `analysis/` (plot windows) modules; consumes `kinovea-measurement` (calibration helper, unit helper, lens distortion), `kinovea-kinematics` (time series, filtering), `kinovea-tracking` (trackers, trackability), `kinovea-export` (CSV writer of change 12).
- **Dependencies (approval required)**: `egui_plot` for charts; PNG export of plots reuses the headless renderer of change 06 (`tiny-skia`) so no image library is added; clipboard through `arboard` (approval required, also used by change 15's `Copy image`; whichever lands first adds it).
- **Assemblies touched**: none on the C# side. Superseded: `Kinovea.ScreenManager/Measurement/Plotting/*`, `Measurement/LensDistortion/FormCalibrateDistortion.cs`, `PlayerScreen/Forms/FormCalibrate*`, `FormCalibrationValidation.cs`, `FormConfigureTrajectoryDisplay.cs`, tracking forms. OxyPlot NuGet becomes unused.
- **Verification**: Linux only. Dialog view-models unit-tested against oracle fixtures (calibration round-trips and kinematics series already exist from changes 03/04); `egui_kittest` drives dialogs; plot snapshots with tolerance; CSV export compared byte-wise with `Kinovea.FixtureGen` output of the original `CSVHelper`.
- **KVA format**: unchanged; calibration, tracking parameters and display options are already KVA 2.0 elements written by change 05.

## Non-goals

- Numeric algorithms (already in changes 03, 04, 10, 11): this change only exposes them.
- Spreadsheet/document exporters and their dialogs (change 12); the analysis windows only produce CSV through change 12's writer.
- Capture-side measurement, localisation (change 18).
- New plot types not present in the original application.
