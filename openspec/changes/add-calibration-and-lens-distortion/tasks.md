# Tasks

## 1. Shared XML helpers (`rust/kinovea-core/src/xml.rs`)

- [ ] 1.1 Write `rust/kinovea-core/src/xml_test.rs` (parse/write float, PointF `x;y`, SizeF, Size, boolean, 7-significant-digit float formatting cases such as `0.1`, `100`, `1234.5678` → `1234.568`); implement `xml.rs` with `parse_*`/`write_*` and `format_f32_invariant`; verify tests pass and `cd rust && make verify` is green.

## 2. Lens distortion

- [ ] 2.1 Add dependencies (`kinovea-core`, `kinovea-geometry`, `kinovea-kinematics`, `quick-xml`; dev `kinovea-fixtures`) to `rust/kinovea-measurement/Cargo.toml`; write `src/distortion/parameters_test.rs` (defaults for 1920×1080, content hash stability, camera matrix/coefficient vector layout); implement `distortion/parameters.rs`; verify tests pass.
- [ ] 2.2 Write `src/distortion/helper_test.rs` (identity when uninitialized, distort of the principal point is itself, 200 seeded round-trips within 1e-3) and `src/distortion/undistort_test.rs` (5-iteration inverse on a pure-radial case against the closed form solved numerically); implement `distortion/helper.rs` (distort, distort_line, distort_rectified_line, undistort quad) and `distortion/undistort.rs`; verify tests pass.
- [ ] 2.3 Write `src/distortion/serializer_test.rs` (write with/without `ImageSize`, read with rescale, default pixels-per-mm) and `src/distortion/import_agisoft_test.rs` (valid file, missing path error, malformed XML error); implement `distortion/serializer.rs` (quick-xml events) and `distortion/import_agisoft.rs`; verify tests pass and `make verify` is green.

## 3. Calibrator plane

- [ ] 3.1 Write `src/calibrator_plane/quad_test.rs` (MakeQuad for the three axes on known lines); implement `calibrator_plane/quad.rs`; verify tests pass.
- [ ] 3.2 Write `src/calibrator_plane_test.rs` (plane corners map D→(0,0), C→(4,0), B→(4,3), A→(0,3); line horizontal/vertical scenarios; non-convex marks invalid; origin move and reset; offset; project to homogeneous; clone keeps origin); implement `calibrator_plane.rs`; verify tests pass.
- [ ] 3.3 Write `src/calibrator_plane/xml_test.rs` (write plane and line fragments equal to hand-built expected strings, read with scale halves the quad, unknown elements ignored); implement `calibrator_plane/xml.rs`; verify tests pass.

## 4. Calibration helper

- [ ] 4.1 Write `src/source_test.rs` (StaticSource returns empty origin, empty quad, no tracking) and `src/calibration_axis_test.rs`/`src/calibrator_type_test.rs` (names round-trip); implement `source.rs`, `calibration_axis.rs`, `calibrator_type.rs`; verify tests pass.
- [ ] 4.2 Write `src/calibration_helper_test.rs` (default mapping (1060,440)→(100,100) at 1920×1080; initialize by line and by plane; `get_point`, `get_image_point`, `get_scalar`, `get_image_length`, `get_time`, reset); implement `calibration_helper.rs` with `Units` injection; verify tests pass.
- [ ] 4.3 Write `src/calibration_helper/convert_test.rs` (speed, acceleration, acceleration-from-velocity, angle, angular velocity/acceleration against `kinovea-core` conversions; `MeasurementContext` impl returns the same values as a `PixelContext` for the default calibration); implement `calibration_helper/convert.rs` and the trait impl; verify tests pass and the change-03 parity test still passes when run with `CalibrationHelper` in place of `PixelContext`.
- [ ] 4.4 Write `src/calibration_helper/text_test.rs` (point text `x ; y` with and without abbreviation, length text, circumference text, ellipse from circle through the mapper) and tracked-source tests (`get_point_at_time` with a translated quad equals a static calibration on that quad; tracked origin only); implement `calibration_helper/text.rs` and the time-aware path; verify tests pass.
- [ ] 4.5 Write `src/calibration_helper/xml_test.rs` (full fragment for line, plane and plane+distortion against expected strings; read applies scale and sets the type); implement `calibration_helper/xml.rs`; verify tests pass and `make verify` is green.

## 5. Camera pose

- [ ] 5.1 Write `src/camera_poser_test.rs` (synthetic camera at (2, −5, 3) recovered within 1e-2 from a homography built by projecting the 4×3 rectangle with fx = fy = 1000); implement `camera_poser.rs`; verify the test passes.

## 6. Oracle area and parity

- [ ] 6.1 Add `tests/cases/calibration/{transforms,kva,distortion,pose}.json` and `tests/cases/calibration/agisoft.xml`, and `Kinovea.FixtureGen/Areas/CalibrationArea.cs` producing the files listed in design D6; register in `Program.cs`; verify `tools/oracle/fixtures.sh --verify` is green and `tests/fixtures/calibration/` is committed.
- [ ] 6.2 Write `rust/kinovea-measurement/tests/calibration_parity_test.rs` asserting transforms (1e-3 numeric, exact text), KVA fragments byte for byte, distortion points and lines (1e-3 px), `kinovea.xml` round-trip, Agisoft parameters, and camera pose (1e-3); verify every fixture passes.

## 7. Integration

- [ ] 7.1 Export the API from `rust/kinovea-measurement/src/lib.rs`, update `tests/README.md` (area `calibration`) and `.agent/progress.md`; verify `cd rust && make verify`, `npm run lint` and `cargo test -p kinovea-fixtures` are green.
