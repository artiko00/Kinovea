# Spec Delta

## Purpose

Lets external 2D pose estimation results (OpenPose BODY_25 JSON, Sports2D TRC) become posture drawings and tracks in a Kinovea document, so skeletons can be reviewed, measured and exported without manual digitizing.

## ADDED Requirements

### Requirement: OpenPose keypoint folders import as one posture per person per frame
The system SHALL import an OpenPose `*_keypoints.json` file by scanning its directory for every file with the same prefix, sorted alphanumerically, taking the frame number from the token before `keypoints.json`, and creating per frame one keyframe at `first_timestamp + frame * average_timestamps_per_frame` with one `OpenPoseBody25` posture per BODY_25 person. Entry points: File > Load annotations (`*_keypoints.json`); `kinovea import <file>`; MCP tool `import_skeleton`.

#### Scenario: Low-confidence joint is hidden
- **WHEN** a person's right wrist has confidence below 0.25
- **THEN** the posture option `showRWrist` is false and the wrist point and its incoming segment are not rendered

#### Scenario: Folder with three frames and two people
- **WHEN** `clip_000000000000_keypoints.json`, `clip_000000000001_keypoints.json`, `clip_000000000002_keypoints.json` with two BODY_25 persons each are imported into an empty document with 400 timestamps per frame
- **THEN** the document has three keyframes at 0, 400 and 800 timestamps, each with two drawings named `OpenPose`, and the KVA written afterwards matches the oracle fixture after id normalisation

#### Scenario: Non BODY_25 person is skipped
- **WHEN** a person has 54 values (COCO model)
- **THEN** no drawing is created for that person and the other persons of the frame are still imported

#### Scenario: Frame number cannot be parsed
- **WHEN** a file in the folder has a non-numeric token before `keypoints.json`
- **THEN** import stops at that file, keeps what was imported so far and reports the offending file name

### Requirement: Sports2D TRC import creates one posture and one track per marker
The system SHALL parse a tab-separated TRC file with an invariant decimal point and create one `BodyWithFeet` posture named `Human` on a keyframe at the first data row plus one track `Human.<marker>` per marker, one timed point per row, coloured like the posture point. Entry points: File > Load annotations (`.trc`); `kinovea import <file>`; MCP `import_skeleton`.

#### Scenario: Header layout
- **WHEN** the file is parsed
- **THEN** line 3 supplies `DataRate`, `CameraRate`, `NumFrames`, `NumMarkers`, line 4 supplies the marker names after `Frame#` and `Time`, and data is read from line 6 on

#### Scenario: Valid Sports2D file
- **WHEN** a TRC with 26 markers and 50 rows is imported
- **THEN** the document has one keyframe with one posture drawing and 26 tracks of 50 points each, and the resulting KVA matches the oracle fixture

#### Scenario: Marker count mismatch
- **WHEN** line 4 does not contain exactly `NumMarkers + 2` labels
- **THEN** import fails with an invalid-data error naming the line and the document is unchanged

### Requirement: Imports merge into existing keyframes
When an imported frame falls on a timestamp that already has a keyframe, the system SHALL add the drawings to that keyframe instead of creating a second one.

#### Scenario: Import over an annotated frame
- **WHEN** a keyframe exists at timestamp 400 and an OpenPose frame maps to 400
- **THEN** the keyframe count does not change and the keyframe at 400 holds both the previous drawings and the imported postures
