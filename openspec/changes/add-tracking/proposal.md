# Proposal

## Why

Object tracking is what turns a drawing into a measurement over time: a point placed on a joint follows it frame after frame and feeds kinematics. The original trackers (template matching through OpenCvSharp, circle tracking through the Hough transform) and the trackability layer that binds drawings, tracks and camera motion live in `Kinovea.ScreenManager/Tracking` (5.5k LOC). They are also the core the MCP server will drive when it tracks skeleton joints headlessly.

## What Changes

- Port the template-matching tracker to pure Rust in `kinovea-tracking`: normalized cross-correlation coefficient over a search window around the last position, similarity and template-update thresholds, optional HSV mask with dilate/erode, sub-pixel refinement by parabola fit, failure semantics that keep the previous position.
- Port the circle tracker: grayscale, median blur, Hough circle detection at the averaged reference radius with a descending vote threshold, failure semantics.
- Port the tracker contract (`AbstractTracker`): readiness, track step, create track point, reference point, trim, clear, update image; and `TrackingParameters` with its KVA serialization.
- Port the trackability layer: `TrackablePoint`, `DrawingTracker` (object tracking vs camera tracking vs none, per-point binding to tracks), `TrackabilityManager` (add, assign, forget, trim, update positions at a time, KVA `Trackers` section) and the time-indexed `Timeline<T>`.
- Add a `tracking` oracle area that renders synthetic frames by formula, runs the original trackers on them and records positions and scores per frame.

## Capabilities

### New Capabilities
- `tracking/template-matching`: template tracker behavior, parameters, thresholds, refinement and failure handling.
- `tracking/circle-tracking`: circle tracker behavior and radius handling.
- `tracking/trackability`: how drawings' trackable points get their position at any time from object tracks, camera motion or their reference value, and how that state is serialized.

### Modified Capabilities
None.

## Impact

- **Assemblies superseded**: `Kinovea.ScreenManager/Tracking/Tracking/{AbstractTracker, TrackStatus, TrackingSource, TemplateMatching/*, Circle/*}.cs`, `Tracking/Trackability/*.cs`, `Kinovea.Services/Types/{TrackingParameters, TrackingAlgorithm, TimedPoint}.cs` (the last two are shared with `add-core-types-and-units`), `Kinovea.ScreenManager/Metadata/Timeline.cs`. `DrawingTrack.cs` (the track drawing itself) belongs to `add-kva-metadata-model`; this change provides the algorithms it calls. `Kinovea.FixtureGen` gains the `tracking` area.
- **Rust**: `rust/kinovea-tracking` (depends on `kinovea-core`, `kinovea-geometry`, `kinovea-video` for `FrameBuffer`). Decision requiring approval: crate `imageproc` (median blur, Sobel gradients, morphology) or an in-crate implementation; see design.
- **KVA**: `TrackerParameters`, `TrackPointList`/`TrackPoint` (`x;y;t;r`) and `Trackers/TrackablePoint` elements keep their exact format; reading and writing them is covered by tests on the sample files.
- **Verification**: Linux (`cd rust && make verify`, CI); oracle via `tools/oracle/fixtures.sh --verify`. `build.yml` unaffected.
- No UI.

## Non-goals

- Drawing the tracker gizmo and the track path (`add-player-screen-rendering`, through the `Renderer` trait).
- The `DrawingTrack` drawing, labels and KVA `Track` element (`add-kva-metadata-model`).
- Camera motion estimation (`add-camera-motion-and-lens-filters`); this change only consumes a `CameraTransformer`-like trait.
- OpenCV: trackers are pure Rust by decision.
