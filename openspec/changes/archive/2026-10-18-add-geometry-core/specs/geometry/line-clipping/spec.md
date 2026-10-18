# Spec Delta

## Purpose

Clips segments, rays and infinite lines against an axis-aligned rectangle so drawings extending beyond the image can be rendered and hit-tested only where visible.

## ADDED Requirements

### Requirement: Liang-Barsky clipping
Given a rectangle and two points, the system SHALL return whether the primitive is visible and, if so, its entry and exit points. Segments use the parameter range [0, 1]; infinite lines use (−∞, +∞); rays use either [0, +∞) or (−∞, 1]. Primitives parallel to an edge and outside it are invisible.

#### Scenario: Diagonal through the window
- **WHEN** the line from (250, 1750) to (1750, 250) is clipped against the rectangle (500, 500, 1000×1000)
- **THEN** it is visible with endpoints (500, 1500) and (1500, 500) within 1e-3

#### Scenario: Line missing the window
- **WHEN** the segment (250, 250)–(500, 1750) is clipped against the same rectangle
- **THEN** it is reported invisible both as a segment and as an infinite line (the original tester labels it "line not visible")

#### Scenario: Segment partly outside
- **WHEN** the segment (250, 1500)–(1750, 0) is clipped against the same rectangle
- **THEN** it is visible as a segment and as an infinite line with the same clipped endpoints

#### Scenario: Parallel to an edge
- **WHEN** the vertical line x = 250 is clipped
- **THEN** it is invisible; the vertical line x = 500 (on the edge) is visible

#### Scenario: Oracle parity
- **WHEN** every case in `tests/fixtures/geometry/clipping/lines.csv` (the hand-crafted lines of the original `LineClippingTester` plus seeded random ones) is clipped
- **THEN** visibility and endpoints match within 1e-3
