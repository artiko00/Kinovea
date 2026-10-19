# Spec Delta

## Purpose

Specifies the circle tracker used for balls and markers: find the circle closest to the expected radius near the last position and follow it.

## ADDED Requirements

### Requirement: Reference radius
The expected radius SHALL be the average of the radii of the most recent tracked points, at most the averaging window and never including points before the last user-set reference.

#### Scenario: User-adjusted reference
- **WHEN** the user sets a reference point with radius 12 after automatic points with radius 10
- **THEN** the next step searches around radius 12

### Requirement: Detection
A step SHALL convert the search region to grayscale, apply a median blur only when region blurring is enabled (off by default, as in the original), and detect circles with a gradient Hough transform at the reference radius, starting from a high vote threshold and lowering it by steps until exactly one circle passes or the minimum is reached; zero votes SHALL mean failure.

#### Scenario: Synthetic disc
- **WHEN** a filled disc of radius 10 rendered by formula moves 4 pixels between frames
- **THEN** the step succeeds and the center is within 1 pixel of the true center and the radius within 1 pixel

#### Scenario: No circle
- **WHEN** the search region contains only uniform background
- **THEN** the step fails and the point keeps the previous location and radius

### Requirement: Track point carries the radius
Points produced by the circle tracker SHALL store the detected radius so it can be serialized and used as the next reference.

#### Scenario: Radius serialized
- **WHEN** a tracked point with radius 10.5 is written as a track point
- **THEN** the text is `x;y;t;10.5` in invariant culture

### Requirement: Parity with the original circle tracker
On the `circle-*` cases of the `tracking` oracle area the Rust tracker SHALL match the recorded centers within 1 pixel and radii within 1 pixel with identical success flags.

#### Scenario: Fixture replay
- **WHEN** every `circle-*` case is replayed
- **THEN** all rows satisfy the tolerances
