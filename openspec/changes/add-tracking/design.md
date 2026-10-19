# Design

## Context

See proposal.md. Observed in `Kinovea.ScreenManager/Tracking`:

- `TrackerTemplateMatching.MatchTemplate` crops the search window around the last point, converts an optional HSV mask to gray, runs `Cv2.MatchTemplate(..., CCoeffNormed, mask)`, falls back to the unmasked call, takes `MinMaxLoc` max, rejects below `SimilarityThreshold`, refines with `RefineLocationParabola` (MathNet `Fit.Polynomial` degree 2 on 3 samples per axis) unless the peak is on the border or equals 1.0. `CreateTrackPoint` updates the template when `similarity > TemplateUpdateThreshold || similarity < SimilarityThreshold`.
- `TrackerCircle.MatchCircle`: ROI → gray → `MedianBlur` → `FindBestCircleAtRadius` looping `Cv2.HoughCircles(HoughModes.Gradient, ...)` from `maxGuessVoteThreshold = 50` down by `guessVoteThresholdStep = 2`; radius averaged over `averagingWindow` samples after the last reference time.
- `TrackabilityManager` / `DrawingTracker` / `TrackablePoint`: dictionary of trackers by drawing id, per-point `TrackablePoint` with a camera-track cache, KVA `Trackers` element.
- `TimedPoint` serializes as `x;y;t;r`; `TrackingParameters.WriteXml/ReadXml` element names `TrackingAlgorithm`, `SearchWindow`, `BlockWindow`, `SimilarityThreshold`, `TemplateUpdateThreshold`, `UseMask`, `HSVRange`, `Dilate`, `Erode`.
- `Metadata.PerformTracking` converts the frame to a `Mat` once per frame and calls every active track.

## Goals / Non-Goals

**Goals:**
- Pure Rust trackers with results within declared tolerances of OpenCV on synthetic data.
- Trackers independent of drawings: they take `&FrameBuffer`, `&[TimedPoint]` and return `TimedPoint`s, so the MCP server can drive them without metadata.
- KVA elements byte-identical on round trip.

**Non-Goals:**
- Matching OpenCV bit for bit (different blur and Hough implementations).
- Multi-threaded tracking; one frame at a time as the original.

## Decisions

### D1. Pure Rust, no OpenCV (confirmed hybrid decision)
NCC (`CCOEFF_NORMED`) is implemented in-crate with integral images for means and variances (exact formula of OpenCV, which makes parity tight). Hough gradient circles are implemented in-crate following OpenCV's `HOUGH_GRADIENT` (Sobel gradients, Canny-like edge threshold, accumulator at a fixed radius, non-maximum suppression, vote threshold). Alternative: `opencv` crate; rejected for the trackers because they are the hot path of the headless use case and must build everywhere. Dependency decision requiring approval: `imageproc` for median blur, Sobel, morphology and HSV conversion versus in-crate implementations; the design prefers `imageproc` (already pure Rust, `image`-based) to avoid re-implementing filters, with the `image` conversion isolated in `src/pixels.rs`.

### D2. Tracker trait
`trait Tracker { fn parameters(&self); fn is_ready(&self, last: &TimedPoint) -> bool; fn track_step(&mut self, timeline: &[TimedPoint], time: i64, image: Option<&FrameBuffer>) -> StepResult; fn create_reference_point(&mut self, point: TimedPoint, image: &FrameBuffer); fn trim(&mut self, time: i64); fn clear(&mut self); fn update_image(...) }`. `StepResult { point: TimedPoint, matched: bool, score: f32 }`. Drawing of gizmos is not in the trait: the player renders from `TrackerDebugInfo` (search and block rectangles) returned on demand.

### D3. Parabola refinement without MathNet
A closed form for the vertex of a parabola through (-1, a), (0, b), (1, c): `dx = (a - c) / (2 (a - 2b + c))`. Identical to the degree-2 polynomial fit on three points.

### D4. Timeline and trackability
`Timeline<T>` as a `BTreeMap<i64, T>` with `closest_from`, `last`, `trim`, `clear_with`. `TrackabilityManager` owns `HashMap<Uuid, DrawingTracker>`; drawings are referenced through a `trait Trackable { id, reference_timestamp, trackable_points, set_trackable_point_value, after_all_points_set }` implemented by the metadata crate later. `CameraTransform` is a trait (`initialized`, `transform(source_t, target_t, point)`) implemented in `add-camera-motion-and-lens-filters`.

### D5. Oracle area `tracking`
`Kinovea.FixtureGen/Areas/TrackingArea.cs` renders frames by formula directly into locked bitmap bits (no GDI+ drawing): background gradient `inputs.background`, a square or disc of `inputs.size` moving with `MovingObject` speeds and optional sinusoidal sub-pixel motion, optional colour for the mask case. It instantiates `TrackerTemplateMatching`/`TrackerCircle` with case parameters, creates the reference point at frame 0 and calls `TrackStep` per frame, writing `track.csv` (`frame,t,x,y,r,score,matched`) and `truth.csv`. Rust reproduces the frames with the same integer formula so inputs are identical.

### D6. Tolerances
Template: 0.5 px positions, 0.02 similarity, exact flags. Circle: 1 px center and radius, exact flags. Recorded in `tests/README.md`.

## Decisions taken at apply time

- D1 filters: written in-crate (gray and HSV conversion, median blur, Sobel, dilate and erode in `src/pixels*`), no `imageproc`/`image`. The design allowed either; no dependency approval was available and the specs do not change.
- D4 reuse: `kinovea-metadata` (change 05) already owns `TrackingParameters`, `HsvRange`, `TrackablePoint`, `DrawingTracker`, `TrackabilityManager` and their KVA reading and writing, with byte parity on the fixtures. `kinovea-tracking` depends on `kinovea-metadata` and reuses those types instead of declaring a second set; it adds the algorithms, the `Tracker` trait, `Timeline<T>`, the camera-transform trait and the trackability runtime (positions at a time from tracks, camera motion or the reference value). Missing behavior of the shared types (clamping, content hash) is added in `kinovea-metadata`.

## Legacy superseded

`Kinovea.ScreenManager/Tracking/Tracking/AbstractTracker.cs`, `TrackStatus.cs`, `TrackingSource.cs`, `TemplateMatching/TrackerTemplateMatching.cs`, `TemplateMatching/TrackingTemplate.cs`, `TemplateMatching/TemplateMatchResult.cs`, `Circle/TrackerCircle.cs`; `Tracking/Trackability/TrackablePoint.cs`, `DrawingTracker.cs`, `TrackabilityManager.cs`; `Kinovea.ScreenManager/Metadata/Timeline.cs`; `Kinovea.Services/Types/TrackingParameters.cs`, `TrackingAlgorithm.cs`, `HSVRange.cs`.

## Risks / Trade-offs

- [Hough circle re-implementation diverges from OpenCV on noisy inputs] → parity is specified on synthetic clean discs with 1 px tolerance; noisy behavior is tested for robustness only (no crash, failure flag).
- [HSV conversion differences shift mask edges] → mask tests use saturated colours far from the range boundaries.
- [NCC cost on 400x400 windows] → integral-image formulation is O(search area × template area) like OpenCV's naive path; acceptable for the original window limits; FFT path deferred.

## Migration Plan

Additive in `kinovea-tracking`. Rollback: delete modules.

## Open Questions

- Whether `imageproc` is accepted or filters are written in-crate (task 1.1 is the checkpoint); either choice keeps the specs and the remaining tasks unchanged.
