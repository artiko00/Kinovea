# Spec Delta

## Purpose

Lets an agent measure: calibrate the image, track points over time, compute kinematics and read or export the resulting numbers, matching what the measurement menus and the kinematics dialogs produce.

## ADDED Requirements

### Requirement: calibrate by line or plane and set the origin
Tool `calibrate {session, mode: line|plane, drawing_id, length?|size?, unit}` SHALL calibrate from an existing line drawing (real length) or plane/grid drawing (real width and height) with the given length unit, and `set_origin {session, point}` SHALL move the coordinate origin. Subsequent measurements SHALL be expressed in that unit. Equivalent of the line and plane context menu Calibrate and the coordinate system tool.

#### Scenario: Line calibration
- **WHEN** a 200 px line is calibrated as 1 m and a second 100 px line is measured
- **THEN** the second line's length is 0.5 m within 1e-3

#### Scenario: Plane calibration
- **WHEN** a plane drawing is calibrated as 4 m × 2 m
- **THEN** `describe_session.calibration` reports mode `plane`, the size and the unit, and a point at the plane centre maps to (2, 1)

### Requirement: track runs the tracker over a range
Tool `track {session, drawing_id, point?, from, to, parameters?}` SHALL run template-matching tracking for a track drawing or for a named point of a trackable drawing (posture joint, line end) from `from` to `to`, reporting progress, and return the number of frames tracked and the frames where tracking lost the target. Equivalent of the track tool and the Tracking context menu.

#### Scenario: Synthetic moving disc
- **WHEN** `track` follows a disc moving 3 px per frame over 50 frames of a synthetic clip
- **THEN** 50 frames are tracked and the final position is within 2 px of the ground truth

#### Scenario: Target leaves the frame
- **WHEN** the target leaves the image at frame 30
- **THEN** the result lists frames 30 and later as lost and the track keeps the 29 valid points

### Requirement: kinematics and angular kinematics
Tool `kinematics {session, drawing_id, point?, range?, filter=true}` SHALL return the linear time series of a track or tracked point, and `angular_kinematics {session, drawing_id, angle?}` the angular series of an angle or posture angle, both in session units and matching the fixtures of `add-kinematics-filtering`. Equivalent of the Kinematics and Angular kinematics dialogs.

#### Scenario: Returned columns
- **WHEN** either tool succeeds
- **THEN** `kinematics` returns time, x, y, distance, speed, horizontal and vertical velocity and acceleration, and `angular_kinematics` returns angle, angular velocity and angular acceleration, each column with its unit symbol

#### Scenario: Linear series parity
- **WHEN** `kinematics` is called on the track built from `tests/fixtures/kinematics/linear/truth.csv`
- **THEN** the returned columns equal `linear.csv` within the tolerance declared by that fixture

### Requirement: get_measurements
Tool `get_measurements {session, time?}` SHALL return the measured-data model (positions, distances, angles, chrono sections, time series summary) in session units, restricted to the keyframe at `time` when given. Equivalent of the values shown on the drawings and in the measurement side panel.

#### Scenario: Complete sample
- **WHEN** `get_measurements` is called on a session opened with `complete.kva`
- **THEN** the counts and values equal `tests/fixtures/export/complete/measured.json`

### Requirement: export tools
Tools `export_measurements {session, format: csv-trajectory|csv-chrono|json|xlsx, path}`, `export_report {session, path, images=true}`, `export_video {session, path, from?, to?, profile?}` and `export_images {session, dir, mode: key-images|sequence, interval?}` SHALL call the exporters of the input/output layer and return the written paths. Equivalent of File > Export.

#### Scenario: JSON export equals the resource
- **WHEN** `export_measurements` writes JSON
- **THEN** the file content equals the `measurements.json` resource of the session

#### Scenario: Video export with skeletons
- **WHEN** `export_video` runs on a session with imported skeletons
- **THEN** `ffprobe` reports the selection frame count and the first frame matches `render_frame` at the selection start
