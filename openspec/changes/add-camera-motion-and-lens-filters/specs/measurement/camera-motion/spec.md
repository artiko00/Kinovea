# Spec Delta

## Purpose

Specifies the estimation of global camera motion across the frames of a working zone and the transformation of point coordinates between frames, so drawings placed on the world stay attached when the camera moves.

## ADDED Requirements

### Requirement: Parameters
Camera motion parameters SHALL include step-by-step mode, feature type (SIFT by default, ORB), features per frame (default 2048), RANSAC reprojection threshold (default 1.25), distance threshold usage with a normalized threshold (default 0.1 of the image width), ratio test usage and minimum track length (default 6), with the exact serialization element names of the original.

#### Scenario: Defaults and round trip
- **WHEN** default parameters are serialized and read back
- **THEN** every field is unchanged and the element names match the original files

### Requirement: Feature detection and matching
With the OpenCV feature enabled, the estimator SHALL detect up to the configured number of features per frame (optionally restricted by a mask), match consecutive frames with a brute-force matcher using Hamming distance for ORB and L1 for SIFT with cross-check, and discard matches failing the ratio test or farther than the normalized distance threshold.

#### Scenario: Synthetic textured sequence
- **WHEN** a textured image rendered by formula is warped by known translations over 10 frames and features are detected and matched
- **THEN** each consecutive pair yields at least 50 matches whose median displacement is within 0.5 pixel of the known translation

### Requirement: Consecutive homographies
The estimator SHALL compute one homography per consecutive pair with RANSAC at the configured threshold, record inlier masks, and SHALL fail the pair (identity transform, flagged) when fewer than 4 inliers remain.

#### Scenario: Known homography recovered
- **WHEN** frames are related by a known perspective warp
- **THEN** the estimated homography maps the four image corners within 1 pixel of the known warp

#### Scenario: Untextured frame
- **WHEN** a frame is uniform
- **THEN** its pair is flagged failed and later frames are still processed

### Requirement: Point transformation between frames
Given consecutive transforms, transforming a point from a source time to a target time SHALL chain the homographies forward or the inverses backward through the frame indices; when the transformer is not initialized the point SHALL be returned unchanged.

#### Scenario: Forward and back
- **WHEN** a point is transformed from frame 0 to frame 5 and back to frame 0
- **THEN** the result is within 1e-3 pixel of the original

#### Scenario: Not initialized
- **WHEN** no motion data exists
- **THEN** the point is returned unchanged

### Requirement: Tracks
Building tracks SHALL chain inlier matches across frames into point tracks and keep only tracks at least as long as the minimum track length.

#### Scenario: Minimum length
- **WHEN** the minimum track length is 6 and a feature is visible for 4 frames
- **THEN** no track is produced for it

### Requirement: Serialization
Motion data SHALL be written as a `CameraMotion` element with attribute `type="Homographies"` and one `Frame` element per frame carrying `timestamp` and `index` attributes and the 3x3 matrix as text for all but the last frame, and SHALL be read back into an initialized transformer.

#### Scenario: Round trip
- **WHEN** motion data for 5 frames is written and read back
- **THEN** transforming a point through both instances gives identical results

### Requirement: Parity with the original estimator
On the `cameramotion` oracle cases the Rust estimator SHALL recover the known ground-truth translations within 0.5 pixel, and the recorded original homographies SHALL map corners within 2 pixels of the Rust ones.

#### Scenario: Fixture replay
- **WHEN** every `cameramotion` case is replayed with the OpenCV feature enabled
- **THEN** all tolerances hold
