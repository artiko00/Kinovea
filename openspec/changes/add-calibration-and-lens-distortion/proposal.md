# Proposal

## Why

Every measurement Kinovea reports in real-world units passes through the calibration layer: a line or plane calibration maps rectified image points to world coordinates, lens distortion parameters rectify the image, and the camera pose is derived from both. The kinematics crate already consumes this through the `MeasurementContext` trait; this change provides the real implementation and the KVA calibration fragment, so analyses on calibrated videos become possible on Linux.

## What Changes

- Port `CalibratorPlane` (plane and line calibration, world/grid/image transforms with origin and offset, quad construction from a line and axis, clone/update) and `CalibrationHelper` (calibrator type, image size, unit selection, point/scalar/time conversions, tracked calibration object and origin at a given time, length/circumference text, ellipse of a world circle) to `kinovea-measurement`, implementing `kinovea_kinematics::MeasurementContext`.
- Port lens distortion: `DistortionParameters` (intrinsics, Brown–Conrady coefficients, defaults from sensor width), `DistortionHelper` distort/undistort of points, lines and quads in pure Rust, `DistortionSerializer` and the Kinovea and Agisoft XML importers/exporter.
- Port `CameraPoser` (camera 3D position from the calibration rectangle, Wefelscheid 2011).
- Read and write the KVA `Calibration` fragment (`CalibrationPlane`/`CalibrationLine`, `CalibrationDrawingId`, `Unit`, `CameraCalibration`) byte-identical to the original.
- Add the oracle area `calibration` to `Kinovea.FixtureGen`.

## Capabilities

### New Capabilities
- `measurement/spatial-calibration`: mapping between image and world coordinates through a line or plane calibration, with coordinate system origin, offset, units and the KVA calibration fragment.
- `measurement/lens-distortion`: radial and tangential distortion model, point rectification and distortion, parameter files in Kinovea and Agisoft formats.
- `measurement/camera-pose`: estimation of the camera position relative to the calibration rectangle.

### Modified Capabilities
None.

## Impact

- **Assemblies touched**: none of the shipped C# assemblies change. `Kinovea.FixtureGen` gains `Areas/CalibrationArea.cs`; `tests/cases/calibration/` and `tests/fixtures/calibration/` are added.
- **Rust**: `rust/kinovea-measurement` depends on `kinovea-core`, `kinovea-geometry`, `kinovea-kinematics` and, for the XML fragment, the XML backend chosen in `add-core-types-and-units` (`quick-xml`). No OpenCV dependency: point undistortion is reimplemented (see design).
- **KVA format**: no change; the fragment is reproduced exactly and verified against the oracle.
- **Verification**: Linux (`cd rust && make verify`, CI `rust.yml`); fixtures via `tools/oracle/fixtures.sh --verify`; `build.yml` as secondary compile check.

## Non-goals

- Camera intrinsics calibration from checkerboard images (`CameraCalibrator`, `cv::calibrateCamera`): arrives with the OpenCV-backed features in `add-camera-motion-and-lens-filters`.
- Distortion map and grid bitmaps (`GetDistortionMap`, `GetDistortionGrid`) and `FormCalibrateDistortion`: renderer and UI changes.
- `CoordinateSystemGrid` computation (drawing-side, `add-kva-metadata-model`) and capture frame-rate replacement (`add-capture-pipeline-and-cameras`).
- Any dialog: calibration UI comes with `add-measurement-ui-and-plotting`.
