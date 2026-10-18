# measurement/spatial-calibration Specification

## Purpose
Converts positions, lengths and times measured on the image into real-world values by a user-defined line or plane calibration, keeping the coordinate system origin, offset and units that the original stores in the KVA file.

## Requirements

### Requirement: Default calibration
After initialization for an image size, the calibration SHALL be a line calibration of 100 units along a horizontal line starting at the image centre, pixel length unit, no distortion, so that world coordinates equal (x − cx, −(y − cy)).

#### Scenario: Default mapping
- **WHEN** a 1920×1080 calibration is initialized and the point (1060, 440) is converted
- **THEN** the world point is (100, 100) within 1e-4

### Requirement: Line calibration
Given a world length, two image points and an axis mode (horizontal, vertical, image axes), the system SHALL build a square calibration quad as the original (`MakeQuad`), map the line to the +X or +Y axis accordingly, and SHALL update the mapping when the line moves.

#### Scenario: Horizontal line
- **WHEN** a line of 2 m from (100, 500) to (300, 500) is used and the point (200, 400) is converted
- **THEN** the world point is (1, 1) within 1e-4

#### Scenario: Vertical line
- **WHEN** the same length is set with vertical axis from (100, 500) to (100, 300) and (200, 400) is converted
- **THEN** the world point is (1, 1) within 1e-4

### Requirement: Plane calibration
Given a world rectangle size and an image quadrilateral, the system SHALL build the homography, set the origin at vertex D, report validity (convex quad), transform image points to world and back, project world points to homogeneous image coordinates, and SHALL recompute on update of the image quadrilateral.

#### Scenario: Corners of the plane
- **WHEN** a 4×3 m plane is calibrated on a perspective quad and the four image vertices are converted
- **THEN** D maps to (0, 0), C to (4, 0), B to (4, 3), A to (0, 3) within 1e-3

#### Scenario: Non-convex quad
- **WHEN** the image quad is non-convex
- **THEN** the calibration is marked invalid and points still transform through the last homography

### Requirement: Origin and offset
The system SHALL allow moving the coordinate system origin to any image point, resetting it to vertex D, and adding a world offset; world coordinates are `(grid.x − origin.x + offset.x, −grid.y + origin.y + offset.y)`.

#### Scenario: Move origin
- **WHEN** the origin is set to the image point that mapped to world (1, 1) and the same point is converted
- **THEN** the result is (0, 0)

### Requirement: Time-aware conversion with tracked calibration
Given callbacks providing the calibration object's image quad and the origin at a timestamp, the system SHALL convert a point at that time using the tracked quad (origin forced to its D vertex) when the calibration drawing is tracked, or the static calibration with the tracked origin otherwise, matching `GetPointAtTime`.

#### Scenario: Tracked plane
- **WHEN** the quad callback returns a translated quad at time 10 and a point is converted at time 10
- **THEN** the result equals the conversion with a static calibration built from the translated quad

### Requirement: Scalars, times, units and texts
The system SHALL convert an image length to a world length by mapping two points, frames to seconds by the capture frame rate, speeds, accelerations and angles through the selected units, and SHALL format the point text `x ; y`, the length text and the circumference text with the unit abbreviation when requested, as the original.

#### Scenario: Oracle parity
- **WHEN** every row of `tests/fixtures/calibration/transforms/*.csv` is replayed (default, line, plane cases)
- **THEN** numeric outputs match within 1e-3 and texts match exactly

### Requirement: KVA calibration fragment
The system SHALL write the calibration as `CalibrationLine` or `CalibrationPlane` (size or length, quadrilateral or segment, origin, offset, axis), `CalibrationDrawingId`, `Unit` with abbreviation attribute and `CameraCalibration`, and SHALL read the same fragment applying an image scale to the quad.

#### Scenario: Byte-identical fragment
- **WHEN** the calibrations of `tests/fixtures/calibration/kva/` are written
- **THEN** the XML equals the fixture byte for byte

#### Scenario: Round-trip with scale
- **WHEN** a fragment is read with scale (0.5, 0.5) and written back
- **THEN** quad coordinates are halved and all other values are preserved
