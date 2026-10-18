# Proposal

## Why

Three "video filters" of the original operate on the whole working zone rather than on a frame: camera motion estimation (so drawings stay attached to the world when the camera pans), lens calibration from a chessboard (feeding the distortion model of `add-calibration-and-lens-distortion`), and the kinogram (a single composite image of selected frames). They are the last consumers of OpenCV in the code base and the last analysis features the MCP server needs to answer questions on moving-camera footage.

## What Changes

- Port `CameraTracker` and `CameraTransformer` to `kinovea-measurement`: feature detection (ORB or SIFT), brute-force matching with cross-check, ratio and distance tests, RANSAC homographies between consecutive frames, optional bundle-adjustment rotation model, track building, point transformation between any two frames, and the KVA `CameraMotion` serialization (`type="Homographies"`, one `Frame` element per frame).
- Port lens calibration: chessboard corner detection on sampled frames, `calibrateCamera` with the rational model, reprojection error, output as distortion parameters consumed by the existing distortion helper.
- Port the kinogram filter model to `kinovea-render`: frame selection (tile count, offsets), grid layout (rows, left-to-right), crop size and positions with interpolation, border and labels, composition into an image, KVA parameters serialization and image export.
- Introduce the `opencv` cargo feature on `kinovea-measurement`: feature detection, matching, RANSAC homography, chessboard detection and camera calibration call the OpenCV crate when enabled; point transforms, track building, serialization and the kinogram are pure Rust and always built.
- Add oracle areas `cameramotion`, `lenscalib` and `kinogram` on synthetic inputs rendered by formula.

## Capabilities

### New Capabilities
- `measurement/camera-motion`: estimating global camera motion over a working zone and transforming points across frames.
- `measurement/lens-calibration`: estimating camera intrinsics and distortion from chessboard views.
- `video/kinogram`: composing selected frames into a tiled image with configurable layout and labels.

### Modified Capabilities
None.

## Impact

- **Assemblies superseded**: `Kinovea.ScreenManager/Measurement/CameraMotion/*.cs`, `Measurement/LensDistortion/CameraCalibrator.cs`, `VideoFilters/{IVideoFilter, VideoFilterType, VideoFilterInfo, VideoFilterFactory}.cs`, `VideoFilters/CameraMotion/VideoFilterCameraMotion.cs`, `VideoFilters/LensCalibration/VideoFilterLensCalibration.cs`, `VideoFilters/Kinogram/VideoFilterKinogram.cs`, `Kinovea.Services/Types/{CameraMotionParameters, CameraMotionFeatureType, LensCalibrationParameters, KinogramParameters}.cs`. The configuration forms (`FormConfigure*`, `FormExportKinogram`) belong to `add-measurement-ui-and-plotting`. `Kinovea.FixtureGen` gains three areas.
- **Rust**: `rust/kinovea-measurement` (feature `opencv`, decision requiring approval: crate `opencv` and native OpenCV 4 development packages, installed in CI), `rust/kinovea-render` (kinogram module). Depends on `kinovea-geometry` (homography application), `kinovea-tracking` (`CameraTransform` trait), `kinovea-video`.
- **Workflows**: `rust.yml` gains `libopencv-dev` and runs the suite with and without the feature (requires approval).
- **KVA**: `CameraMotion` and `Kinogram` sections keep their format; round trips are tested on sample files.
- **Verification**: Linux (`cd rust && make verify` plus `cargo test --features opencv`), oracle via `tools/oracle/fixtures.sh --verify`. `build.yml` unaffected.

## Non-goals

- Filter menus, dialogs and interactive tile dragging (`add-measurement-ui-and-plotting`, `add-player-screen-rendering`).
- COLMAP import (`ParseColmap`, experimental in the original).
- A pure-Rust feature detector; recorded as a later option.
- Video export of the kinogram.
