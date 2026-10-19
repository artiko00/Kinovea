# Tasks

## 1. Parameters, timeline and pixel helpers

- [ ] 1.1 Add path dependencies to `rust/kinovea-tracking/Cargo.toml` (`kinovea-core`, `kinovea-geometry`, `kinovea-video`, `kinovea-metadata`; filters in-crate, see "Decisions taken at apply time"); write `src/pixels_test.rs` (BGRA `FrameBuffer` → gray/HSV, ROI crop clamped to the image, median blur, Sobel, dilate/erode as OpenCV with default borders) then implement `src/pixels.rs`; verify `cargo test -p kinovea-tracking` passes.
- [ ] 1.2 Check `kinovea_metadata::drawing::tracking_parameters` (`TrackingParameters`, `TrackingAlgorithm`, `HsvRange`) against the spec: defaults, clamping to 400, content hash, XML round trip with the exact element names, unknown element skipped; add the missing behavior there with tests; verify tests pass.
- [ ] 1.3 Write `src/timeline_test.rs` (insert, closest from, last, trim, clear with disposer, enumerate order) then implement `src/timeline.rs`; verify tests pass.

## 2. Template matching

- [x] 2.1 Write `src/ncc_test.rs`: correlation coefficient map equals a brute-force reference on random images, is 1.0 at the exact location, masked variant ignores excluded pixels; implement `src/ncc.rs` with integral images; verify tests pass.
- [x] 2.2 Write `src/refine_test.rs` (closed-form parabola vertex equals the three-point polynomial fit; border and 1.0 cases skip) then implement `src/refine.rs`; verify tests pass.
- [ ] 2.3 Write `tests/support/synthetic.rs` rendering frames by the same integer formula as the oracle (gradient background, square or disc, speeds, sub-pixel sinus, optional colour); write `src/template_test.rs` (readiness, translated target within 0.5 px, lost target keeps position with score 0, half-pixel motion within 0.15 px, template update policy, mask with changing background, trim and clear); implement `src/template.rs` (`TemplateTracker`, `TrackingTemplate`, `StepResult`, `Tracker` trait in `src/tracker.rs` with `tracker_test.rs`); verify tests pass.

## 3. Circle tracking

- [ ] 3.1 Write `src/hough_test.rs`: a single rendered disc is detected at the right center and radius within 1 px; two discs at different radii are separated; uniform image yields none; implement `src/hough.rs` (gradient Hough at a fixed radius with vote threshold); verify tests pass.
- [ ] 3.2 Write `src/circle_test.rs` (reference radius averaging bounded by the last reference time, descending vote threshold, failure keeps point and radius, radius stored on the point) then implement `src/circle.rs` (`CircleTracker`); verify tests pass.

## 4. Trackability

- [ ] 4.1 Write `src/trackable_point_test.rs` (reference set clears cache, camera transform computed once) then implement `src/trackable_point.rs` and the `CameraTransform` trait in `src/camera.rs` with a fake in tests; verify tests pass.
- [ ] 4.2 Write `src/drawing_tracker_test.rs` with a fake `Trackable`: object tracking precedence, fallback to camera, none, bind and unbind tracks, trim; implement `src/drawing_tracker.rs` and the `Trackable` trait; verify tests pass.
- [ ] 4.3 Write `src/manager_test.rs` (add, assign after load, forget changes the hash, any-tracking flag, trim) for the runtime over `kinovea_metadata::TrackabilityManager`; serialization is already covered by the KVA fixture tests of `kinovea-metadata` (check that the `Trackers` and `TrackerParameters` sections of `Tools/XML/Schema/samples/*.kva` round trip there, add a test if missing); implement `src/manager.rs`; verify tests pass.

## 5. Oracle parity

- [ ] 5.1 Add `tests/cases/tracking/{template-square,template-subpixel,template-lost,template-mask,circle-disc,circle-lost}.json` and document the frame formula in `tests/cases/tracking/README.md`; implement `Kinovea.FixtureGen/Areas/TrackingArea.cs` (formula renderer into locked bits, tracker runs, `track.csv`, `truth.csv`) and register it; run `tools/oracle/fixtures.sh --verify`; verify identical trees.
- [ ] 5.2 Write `tests/tracking_parity_test.rs` replaying each case on the Rust trackers with the tolerances of design D6; verify `cargo test -p kinovea-tracking` passes and the test fails when a tolerance is tightened to 0 in a scratch run.

## 6. Integration

- [ ] 6.1 Replace the placeholder in `rust/kinovea-tracking/src/lib.rs` with the module tree and docs; verify `cd rust && make verify` exits 0 and `npx testlens check` lists no untested Rust file.
- [ ] 6.2 Append the `.agent/progress.md` entry (Hough divergences observed, chosen filter crate) and tick `10` in `openspec/roadmap.md` after archive; verify `npm run lint` passes.

## Workflow follow-up

- Archive after review; sync creates the three `tracking/*` main specs.
