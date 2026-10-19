# metadata/drawing-model Specification

## Purpose
Defines the annotation data model of a Kinovea analysis: keyframes, every kind of drawing as plain data, fading, trackable points and identity, independent of any rendering or UI technology, so it can be used by tests, the MCP server and the desktop UI alike.

## Requirements

### Requirement: Metadata holds the video context and all annotation containers
The metadata SHALL hold the video context (image size, average timestamps per frame, first timestamp, selection start and end, time origin, capture and video framerates, aspect, rotation, mirror, demosaicing, deinterlacing, background color, stabilization track id) and the containers: an ordered list of keyframes, detached drawings (chronos, tracks), and the singleton drawings (coordinate system, test grid, spotlights, number sequence).

#### Scenario: Keyframes stay sorted by timestamp
- **WHEN** keyframes are added in arbitrary timestamp order
- **THEN** iterating keyframes yields them by ascending timestamp and `keyframe_index(timestamp)` returns the matching index or none

#### Scenario: Drawing lookup across containers
- **WHEN** a drawing id exists in a keyframe, in the detached chronos or tracks, or as a singleton
- **THEN** `find_drawing(id)` returns it along with its manager id, and returns none for an unknown id

### Requirement: Every KVA drawing kind is representable as data
The model SHALL represent the following drawing kinds with the fields the original application persists: Line, Angle, CrossMark, Circle, Rectangle, Pencil, Polyline, Label (text), Plane, DistortionGrid, TestGrid, CoordinateSystem, Spotlights with Spotlight items, AutoNumbers with AutoNumber items, Chrono, ChronoMulti, Counter, TimeSegment, TimePolyline, Track, GenericPosture, Bitmap, SVG. Each drawing SHALL expose an id, a name, a reference timestamp, its capabilities set and a content hash.

#### Scenario: Content hash is stable and sensitive
- **WHEN** a drawing is cloned and one persisted field is changed on the clone
- **THEN** both content hashes are equal before the change and differ after it

#### Scenario: No UI types in the model
- **WHEN** the metadata crate is compiled
- **THEN** it depends on no windowing, GPU or widget crate, and the crate's public API exposes no type from one

### Requirement: Fading computes opacity from time
A drawing's fading information (enabled, use default, always visible, fading frames, opaque frames, reference timestamp, average timestamps per frame, master factor) SHALL produce an opacity factor in [0, 1] for a given timestamp, with the same values as the original application.

#### Scenario: Opacity matches the oracle
- **WHEN** the opacity factor is evaluated at the timestamps listed in the `kva/fading` fixture
- **THEN** every value equals the fixture value within 1e-6

#### Scenario: Always visible overrides fading
- **WHEN** fading is enabled with always visible set
- **THEN** the opacity factor is 1 for any timestamp

### Requirement: Trackable drawings expose named points
Drawings that the original application tracks (Line, Angle, CrossMark, Circle, Rectangle, Plane, Polyline, DistortionGrid, GenericPosture and others declared trackable) SHALL expose their trackable points by name and accept a value for a point at a tracking timestamp.

#### Scenario: Setting a tracked value moves the drawing
- **WHEN** a trackable point value is set for a named point at a timestamp
- **THEN** the drawing's geometry reflects the new point and the trackability record stores the reference value and timestamp

### Requirement: Measured values follow the calibration
Drawings with measurements (Line length, Angle value, CrossMark position, Circle radius, Plane, Track) SHALL compute their displayed value through a calibration mapping provided by the caller, defaulting to pixels.

#### Scenario: Pixel calibration yields raw values
- **WHEN** no calibration is provided
- **THEN** a Line from (0,0) to (3,4) reports a length of 5 pixels
