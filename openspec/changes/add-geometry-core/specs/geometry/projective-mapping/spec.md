# Spec Delta

## Purpose

Maps points between a user-defined plane quadrilateral and its image, the core of plane calibration and perspective grids, with a forward and an inverse homography.

## ADDED Requirements

### Requirement: Homography between two quadrilaterals
The system SHALL compute the homography that maps a plane quadrilateral to an image quadrilateral (square-to-quad composition with adjugate inverse), SHALL update it in place, and SHALL map points, quadrilaterals and homogeneous 3-vectors forward (plane to image) and backward (image to plane).

#### Scenario: Corners map to corners
- **WHEN** the mapper is built from plane (0,0),(4,0),(4,3),(0,3) and image (100,80),(500,60),(520,400),(90,380)
- **THEN** forward of each plane vertex equals the corresponding image vertex within 1e-3 and backward of each image vertex equals the plane vertex within 1e-3

#### Scenario: Round-trip of interior points
- **WHEN** 100 seeded random plane points are mapped forward then backward
- **THEN** each returns within 1e-3 of the original

#### Scenario: Degenerate image quadrilateral
- **WHEN** the image quadrilateral has three collinear vertices
- **THEN** the update leaves the previous mapping unchanged and reports the failure

### Requirement: Mapped unit circle ellipse
The system SHALL compute the ellipse (centre, semi-major, semi-minor, rotation) that the homography produces from the unit circle of the plane.

#### Scenario: Oracle parity
- **WHEN** the mappers of `tests/fixtures/geometry/homography/cases.csv` are built and their ellipse computed
- **THEN** centre and axes match within 1e-3 and rotation within 1e-4 radians

### Requirement: Oracle parity of point mapping
For each case in `tests/fixtures/geometry/homography/`, the forward and backward mapping of the listed points SHALL equal the fixture within 1e-3.

#### Scenario: Fixture rows
- **WHEN** each `points.csv` row is mapped
- **THEN** the output columns match within 1e-3
