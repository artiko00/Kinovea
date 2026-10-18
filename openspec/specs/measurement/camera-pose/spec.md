# measurement/camera-pose Specification

## Purpose
Estimates where the camera stands relative to the calibration rectangle, used to display the camera position and to reason about perspective.

## Requirements

### Requirement: Camera position from plane calibration
Given the world calibration rectangle, its homography and the lens intrinsics, the system SHALL compute the camera 3D position in the rectangle's space with the geometric method of Wefelscheid (2011), as the original `CameraPoser.Compute`.

#### Scenario: Synthetic camera
- **WHEN** a 4×3 m rectangle is projected by a known camera (position (2, −5, 3), looking at the rectangle centre, fx = fy = 1000, centre (960, 540)) and the pose is estimated from the resulting homography
- **THEN** the estimated position equals (2, −5, 3) within 1e-2 m

#### Scenario: Oracle parity
- **WHEN** the cases of `tests/fixtures/calibration/pose/cases.csv` are replayed
- **THEN** the position matches within 1e-3
