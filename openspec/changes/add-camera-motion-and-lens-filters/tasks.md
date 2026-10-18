# Tasks

## 1. Parameters and pure camera-motion model

- [ ] 1.1 Write `rust/kinovea-measurement/src/camera_motion/parameters_test.rs` (defaults, round trip with the original element names, content hash) then implement `parameters.rs` (`CameraMotionParameters`, `FeatureType`); verify `cargo test -p kinovea-measurement` passes.
- [ ] 1.2 Write `src/camera_motion/transformer_test.rs` (forward/backward chaining within 1e-3, not initialized returns input, frame index lookup by timestamp, `CameraTransform` trait from `kinovea-tracking` implemented) then implement `transformer.rs` over `kinovea-geometry` homographies; verify tests pass.
- [ ] 1.3 Write `src/camera_motion/xml_test.rs` (`CameraMotion type="Homographies"` round trip, sample files with a `CameraMotion` section if any, last frame without matrix) then implement `xml.rs`; verify tests pass.
- [ ] 1.4 Write `src/camera_motion/tracks_test.rs` (chain scripted inlier matches, minimum length filter) then implement `tracks.rs`; verify tests pass.
- [ ] 1.5 Write `src/camera_motion/estimator_test.rs` with a `FakeBackend` (step-by-step pipeline: features → matches → homographies → tracks, failed pair flagged with identity, mask passed through, ratio and distance filters applied) then implement `estimator.rs` and the `MotionBackend` trait in `backend.rs`; verify tests pass.

## 2. OpenCV backend (approval checkpoint)

- [ ] 2.1 After approval of the `opencv` crate and `libopencv-dev`: add the `opencv` feature to `rust/kinovea-measurement/Cargo.toml`, add `libopencv-dev clang libclang-dev` and a second `cargo test --workspace --features kinovea-measurement/opencv` step to `.github/workflows/rust.yml`, document packages in `AGENTS.md`; verify both CI jobs pass on a branch.
- [ ] 2.2 Write `tests/support/warp.rs` (textured image by formula from a seed, inverse-mapped nearest-neighbour warp by a homography) and `src/camera_motion/cv_test.rs` behind the feature (ORB and SIFT detection counts, matches median displacement within 0.5 px, RANSAC homography maps corners within 1 px, uniform frame fails) then implement `cv.rs` (`OpenCvBackend`); verify `cargo test -p kinovea-measurement --features opencv` passes.

## 3. Lens calibration

- [ ] 3.1 Write `src/lens/parameters_test.rs` (defaults, round trip) then implement `src/lens/parameters.rs`; write `src/lens/sampling_test.rs` (even sampling of at most N frames over a zone) then implement `sampling.rs`; verify tests pass.
- [ ] 3.2 Write `src/lens/calibration_test.rs` with a `FakeBackend` (skipped frames counted, refusal under 3 views with the count in the error, diagnostics sets equal counts, result mapped to `DistortionParameters` from `add-calibration-and-lens-distortion`) then implement `calibration.rs` and the `CalibrationBackend` trait; verify tests pass.
- [ ] 3.3 Write `tests/support/chessboard.rs` (board rendered by per-pixel inverse mapping through pose, intrinsics and k1) and `src/lens/cv_test.rs` behind the feature (corners within 0.5 px, fx/fy within 1 percent, cx/cy within 2 px, k1 within 0.02) then implement `src/lens/cv.rs`; verify the feature test passes.

## 4. Kinogram

- [ ] 4.1 Write `rust/kinovea-render/src/kinogram/parameters_test.rs` (defaults, round trip, hash) then implement `parameters.rs`; verify `cargo test -p kinovea-render` passes.
- [ ] 4.2 Write `src/kinogram/layout_test.rs` (base indices 0,12,24,36,48,59; offset clamping; cells left-to-right and right-to-left; tile size; interpolation 100/200/300; padding on tile count change) then implement `layout.rs`; verify tests pass.
- [ ] 4.3 Write `src/kinogram/compose_test.rs` (index-encoded frames: center pixel of each cell decodes to the tile frame; border drawn when visible; label drawn when enabled) then implement `compose.rs` on the `Renderer` trait and `export_png`; verify tests pass.

## 5. Oracle parity

- [ ] 5.1 Add `tests/cases/cameramotion/{translate-orb,perspective-orb}.json`, `tests/cases/lenscalib/{k1-neg02,k1-zero}.json`, `tests/cases/kinogram/{grid-3x6-borders,grid-2x6-plain}.json`; implement `Kinovea.FixtureGen/Areas/CameraMotionArea.cs`, `LensCalibArea.cs`, `KinogramArea.cs` per design D5 and register them; run `tools/oracle/fixtures.sh --verify`; verify identical trees and that `kinogram.png` is under 100 KB.
- [ ] 5.2 Write `rust/kinovea-measurement/tests/cameramotion_parity_test.rs` and `tests/lenscalib_parity_test.rs` (feature-gated, tolerances of D6) and `rust/kinovea-render/tests/kinogram_parity_test.rs` (pixel comparison outside border/label masks, exact `layout.json`); verify both feature configurations pass.

## 6. Integration

- [ ] 6.1 Add `--camera-motion <video>` and `--kinogram <video> <out.png>` subcommands to `rust/kinovea/src/main.rs` with `src/main_test.rs` coverage of argument parsing and the disabled-feature message; verify `cd rust && make verify` exits 0 in both configurations.
- [ ] 6.2 Update `openspec/roadmap.md` (dependency 06 → 11, tick `11` after archive), `AGENTS.md` (feature flag command), `.agent/progress.md` (OpenCV versions tested, SIFT availability); verify `npm run lint` passes.

## Workflow follow-up

- Archive after review; sync creates `measurement/camera-motion`, `measurement/lens-calibration`, `video/kinogram`.
