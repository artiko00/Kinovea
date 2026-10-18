# measurement/lens-distortion Specification

## Purpose
Models the camera lens distortion so that image points can be rectified before calibration and drawn back in distorted space, and exchanges the parameters with files produced by Kinovea and Agisoft Lens.

## Requirements

### Requirement: Distortion parameters
The system SHALL hold intrinsics fx, fy, cx, cy, coefficients k1, k2, k3, p1, p2 and pixels-per-millimetre, SHALL build defaults for an image size (focal 2.77 mm, sensor width 6.16 mm, centre at the image centre, zero coefficients), and SHALL expose a stable content hash.

#### Scenario: Defaults
- **WHEN** defaults are built for 1920×1080
- **THEN** pixels per millimetre is 1920 / 6.16, fx = fy = 2.77 × that value, and (cx, cy) = (960, 540)

### Requirement: Distort and undistort points
The system SHALL distort a rectified point with the radial (k1, k2, k3) and tangential (p1, p2) model and SHALL undistort a point by the iterative inverse used by OpenCV's `undistortPoints` (5 fixed-point iterations), returning the input unchanged when no parameters are set.

#### Scenario: Round-trip
- **WHEN** 200 seeded image points are distorted then undistorted with the parameters of `tests/cases/calibration/distortion.json`
- **THEN** each returns within 1e-3 px of the original

#### Scenario: Oracle parity
- **WHEN** every row of `tests/fixtures/calibration/distortion/points.csv` is distorted and undistorted
- **THEN** results match the oracle (`Cv2.UndistortPoints`) within 1e-3 px

### Requirement: Distorted lines and quads
The system SHALL sample a rectified segment into the original number of points and distort each, SHALL distort a rectified line segment given in distorted endpoints, and SHALL undistort the four vertices of a quadrilateral.

#### Scenario: Oracle parity
- **WHEN** the segments of `tests/fixtures/calibration/distortion/lines.csv` are processed
- **THEN** the point lists match within 1e-3 px

### Requirement: Parameter files
The system SHALL read and write the Kinovea `CameraCalibration` XML (with optional `ImageSize`, rescaling intrinsics when the target image size differs and defaulting pixels-per-millimetre from the sensor width) and SHALL import Agisoft Lens XML (`/calibration/{width,height,fx,fy,cx,cy,k1,k2,k3,p1,p2}`) with the same rescaling.

#### Scenario: Kinovea file round-trip
- **WHEN** `tests/fixtures/calibration/distortion/kinovea.xml` is imported for its own image size and exported
- **THEN** the output equals the input byte for byte

#### Scenario: Rescaled import
- **WHEN** a file saved for 1920×1080 is imported for 960×540
- **THEN** fx, fy, cx, cy are halved and the coefficients unchanged

#### Scenario: Agisoft import
- **WHEN** `tests/cases/calibration/agisoft.xml` is imported
- **THEN** the parameters equal those in `tests/fixtures/calibration/distortion/agisoft.csv`

#### Scenario: Missing or malformed file
- **WHEN** a missing path or malformed XML is imported
- **THEN** the result is an error naming the path, not a crash or silent defaults
