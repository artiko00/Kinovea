# Design

## Context

See proposal.md. Current state:

- `CalibrationHelper` (765 lines) owns one `CalibratorPlane` regardless of type: a line calibration is a square plane built by `MakeQuad`. It reads units from `PreferencesManager`, receives three callbacks at `Initialize` (origin at time, calibration quad at time, has tracking data) and raises `CalibrationChanged`. It also computes the `CoordinateSystemGrid` (drawing-side) and the capture frame-rate replacement (preferences-side), both out of scope.
- `DistortionHelper.Undistort` is the only OpenCV call in the layer (`Cv2.UndistortPoints`); `Distort` is already closed-form C#. `CameraCalibrator` wraps `Cv2.CalibrateCamera` and is deferred.
- The KVA fragment is written by `CalibrationHelper.WriteXml`/`CalibratorPlane.Write{Plane,Line}Xml`/`DistortionSerializer`; the existing `kva` fixtures (`roundtrip.kva`) already contain it for the `complete` sample.
- `kinovea-kinematics` defines `MeasurementContext`; the fixtures of change 03 use the pixel context.

## Goals / Non-Goals

**Goals:**
- `CalibrationHelper` as a plain struct implementing `MeasurementContext`, with units injected, no global preferences and no events (callers observe changes through return values; the UI layer adds notification later).
- Exact XML fragment parity, since the fragment lives inside every calibrated KVA file.
- No OpenCV dependency in this crate.

**Non-Goals:**
- Camera intrinsics calibration (needs `cv::calibrateCamera`; feature-gated in change 11).
- Rendering of distortion grids/maps.

## Decisions

### D1. Undistortion reimplemented, not wrapped
OpenCV's `undistortPoints` without termination criteria runs 5 fixed-point iterations: starting from the normalized distorted point, repeatedly compute the radial/tangential correction at the current estimate and re-apply. The port implements exactly that in `f64` and verifies against the oracle within 1e-3 px. Alternative: `opencv` crate behind a feature; rejected here because it would make the first calibrated measurement depend on a native library on Linux, which the roadmap reserves for change 11.

### D2. Callbacks become a trait
`Initialize(imageSize, getCalibrationOrigin, getCalibrationQuad, hasTrackingData)` becomes a `TrackedCalibrationSource` trait with the three methods; a `StaticSource` implementation (no tracking) serves tests and headless use. Alternative: keep three boxed closures; a trait is cheaper to implement from `kinovea-metadata` later.

### D3. Units are injected
`CalibrationHelper::new(image_size, Units { length, speed, acceleration, angle, angular_velocity, angular_acceleration }, fps)`; `refresh_units(Units)` replaces `RefreshUnits()`. Length unit is still owned by the helper because the KVA fragment stores it.

### D4. XML through the shared backend
The fragment reader/writer uses `quick-xml` events (not serde) because element order and the `Abbreviation` attribute must be byte-identical and parsing must tolerate unknown elements. `XmlHelper` formatting helpers for floats, points and sizes (`WriteFloat`, `WritePointF`, `WriteSizeF`, parsers) are ported into `kinovea-core::xml` in this change as they are first needed here; `add-kva-metadata-model` reuses them.

### D5. Module layout
`rust/kinovea-measurement/src/`: `calibrator_plane.rs` (+ `calibrator_plane/xml.rs`, `calibrator_plane/quad.rs` for `MakeQuad`), `calibration_helper.rs` (+ `calibration_helper/convert.rs`, `calibration_helper/text.rs`, `calibration_helper/xml.rs`), `calibrator_type.rs`, `calibration_axis.rs`, `distortion/{parameters,helper,undistort,serializer,import_agisoft}.rs`, `camera_poser.rs`, `source.rs` (trait + StaticSource). `rust/kinovea-core/src/xml.rs` added. Sibling tests throughout; parity tests in `tests/calibration_parity_test.rs`.

### D6. Oracle area `calibration`
`Kinovea.FixtureGen/Areas/CalibrationArea.cs` writes: `transforms/{default,line-h,line-v,plane,plane-offset}.csv` (image→world, world→image, scalar, lengths, texts for seeded points), `kva/{line,plane,plane-distortion}.xml` (the exact `WriteXml` fragment), `distortion/points.csv` (distort and `Cv2.UndistortPoints` results), `distortion/lines.csv`, `distortion/kinovea.xml` (exported file), `distortion/agisoft.csv` (parameters imported from `tests/cases/calibration/agisoft.xml`), `pose/cases.csv` (CameraPoser on seeded quads with default intrinsics). Cases live in `tests/cases/calibration/*.json`.

### Legacy superseded
`Kinovea.ScreenManager/Measurement/Calibration/{CalibrationHelper,CalibratorPlane,CalibratorType,CalibrationAxis,CameraPoser}.cs`, `Measurement/LensDistortion/{DistortionParameters,DistortionHelper,DistortionSerializer,DistortionImporterKinovea,DistortionImporterAgisoft}.cs`, `Kinovea.Services/XmlHelper.cs` (numeric/point/size parts; colour and bitmap helpers move with the metadata model). `CameraCalibrator.cs` and `FormCalibrateDistortion.*` are superseded by changes 11 and 16. Deleted in `remove-dotnet-solution`.

### Deviations found while implementing
- `kinovea-core::xml` returns points and sizes as tuples because `kinovea-core` does not depend on `kinovea-geometry`; the integer `Size` type lives in `kinovea-geometry::point`. `format_f64_invariant` (15 significant digits) joins the 7-digit `float` formatter, both built on one `format_significant` helper.
- The XML backend is the element tree of `kinovea-core::preferences::dom` (already byte-exact for `Preferences.xml`) rather than raw `quick-xml` events; the KVA fragment and the Kinovea lens file are produced through it.
- The two player preferences do not belong to the calibration: `CalibrationHelper::with_preferences(filtering, smoothing)` returns the `CalibratedContext` that implements `MeasurementContext`.
- `CalibrationHelper` owns a `Box<dyn TrackedCalibrationSource>` (the three callbacks of `Initialize`); `without_tracking` uses `StaticSource`. No `CalibrationChanged` event and no coordinate-system grid (renderer side).
- Reading a `CalibrationLine` scales the origin but not the segment, as the original `ParseSegment` ignores its scale argument; reading a `CameraCalibration` without `ImageSize` keeps the intrinsics unscaled. Both are reproduced and tested.
- OpenCV returns the undistorted normalized coordinates as `float` before scaling back; the port narrows at the same point. The 5-iteration inverse is not exact for strong coefficients at the image border, so the round-trip test uses the realistic parameters of `tests/cases/calibration/distortion.json`.
- `import_kinovea` records the file path in `path` like the original; the byte-for-byte round trip clears it before exporting.
- `CalibratorPlane::clone_calibrator` reproduces the original `Clone` (offset dropped, origin and axis kept). Cutoff of invalid plane updates (`size.IsEmpty`) is `width <= 0 || height <= 0`.
- The oracle `transforms` cases use `Guid.Empty` as drawing id so `GetPointText` follows the static path; with a real id and the static callbacks the original would place the origin at image (0, 0).
- `CsvRows` (FixtureGen) centralizes the CSV cell builders previously private to `GeometryArea`.

## Risks / Trade-offs

- [OpenCV's iteration count or stopping rule differs from the assumed 5 iterations in the vendored version 4.10] → the parity test on `distortion/points.csv` fails loudly; the iteration count is a constant to adjust once, documented with the OpenCV source reference.
- [`float` arithmetic in `CalibratorPlane` transforms vs `double` in `ProjectiveMapper`] → mirror widths exactly (`PointF` f32 in/out, f64 inside the mapper); tolerance 1e-3 px / 1e-4 world units.
- [`GetEllipseFromCircle` depends on `ProjectiveMapper.Ellipse`] → covered by change 02 parity; here only the composition is tested.
- [Byte-identical XML requires the same float formatting as `XmlHelper.WriteFloat` (`float.ToString(CultureInfo.InvariantCulture)`)] → implement `format_f32_invariant` (shortest round-trip in .NET Core 3+ semantics) and test it on the fixture values; note that the oracle runs .NET Framework 4.8, whose `float.ToString()` uses 7 significant digits, so the helper reproduces that rule.

## Migration Plan

Land in order: core XML helpers → distortion → calibrator plane → helper and `MeasurementContext` impl → camera pose → KVA fragment → oracle area and parity. No user-visible C# change.

## Open Questions

- Whether `CalibrationChanged` notifications are needed outside the UI; decide in `add-player-screen-rendering`.
