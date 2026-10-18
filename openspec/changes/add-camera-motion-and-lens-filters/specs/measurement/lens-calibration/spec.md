# Spec Delta

## Purpose

Specifies the estimation of camera intrinsics and lens distortion coefficients from chessboard views spread over a video, producing the parameters the distortion model uses to undistort drawings.

## ADDED Requirements

### Requirement: Parameters
Lens calibration parameters SHALL include maximum images (default 12), pattern size in inner corners (default 10x7) and maximum iterations (default 30), with the original serialization element names.

#### Scenario: Round trip
- **WHEN** parameters with pattern 9x6 are written and read
- **THEN** the pattern size is 9x6

### Requirement: Frame sampling and corner detection
The filter SHALL sample at most the configured number of frames evenly over the working zone, convert each to grayscale and detect chessboard inner corners of the pattern size; frames without a detected pattern SHALL be skipped and counted.

#### Scenario: Synthetic boards
- **WHEN** 12 frames showing a chessboard rendered by formula under known poses are sampled
- **THEN** corners are found in all 12 and each detected corner is within 0.5 pixel of its projected position

### Requirement: Calibration
With at least 3 frames with corners the filter SHALL compute the camera matrix and distortion coefficients with the rational model within the iteration limit, SHALL report the RMS reprojection error, and SHALL expose the result as distortion parameters (fx, fy, cx, cy, k1, k2, k3, p1, p2) for the image size.

#### Scenario: Known intrinsics recovered
- **WHEN** boards are rendered with focal length 1000, center at the image center and k1 -0.2
- **THEN** fx and fy are within 1 percent, cx and cy within 2 pixels and k1 within 0.02

#### Scenario: Too few views
- **WHEN** fewer than 3 frames have corners
- **THEN** calibration is refused with a typed error naming the count

### Requirement: Reprojection diagnostics
For each used frame the filter SHALL provide the detected and reprojected corners so a renderer can display them.

#### Scenario: Diagnostics available
- **WHEN** calibration succeeds on 12 frames
- **THEN** 12 sets of detected and reprojected corners are available with equal counts

### Requirement: Parity with the original calibration
On the `lenscalib` oracle cases the Rust intrinsics SHALL be within 1 percent (focal, center) and 0.02 (k1, k2) of the recorded original values.

#### Scenario: Fixture replay
- **WHEN** every `lenscalib` case is replayed with the OpenCV feature enabled
- **THEN** all tolerances hold
