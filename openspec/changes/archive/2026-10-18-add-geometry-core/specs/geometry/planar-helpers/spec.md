# Spec Delta

## Purpose

Offers the planar geometry helpers used by every drawing and measurement: distances, oriented angles, rotations about a pivot, projections onto lines, quadrilateral manipulation, and circle and ellipse descriptions.

## ADDED Requirements

### Requirement: Distances, angles and rotations
The system SHALL compute the distance between two points, the oriented angle between vectors ab and cd (and ab, ac) in [−π, π] positive counter-clockwise, the middle point, a point at a given distance along a segment, and a point rotated about a pivot.

#### Scenario: Point at distance
- **WHEN** the point at distance 5 along (0,0)→(10,0) is requested
- **THEN** the result is (5, 0)

#### Scenario: Oriented angle
- **WHEN** the angle at origin (0,0) between a=(1,0) and c=(0,1) is computed
- **THEN** the result is +π/2; swapping a and c gives −π/2

### Requirement: Angular snapping and constructions
The system SHALL compute the point at the closest rotation step (with subdivisions, optionally cardinal), the point at a target angle, the point at an angle and distance from a leg, the projection of a point onto the parallel to a line, and linear mixing of scalars and points.

#### Scenario: Closest rotation step
- **WHEN** a point at 50° from the leg is snapped with 8 subdivisions
- **THEN** the result lies at 45° from the leg at the same distance within 1e-4

#### Scenario: Oracle parity
- **WHEN** every row of `tests/fixtures/geometry/helpers/angles.csv` and `points.csv` is evaluated
- **THEN** results match within 1e-4 (single precision)

### Requirement: Closest point on a line with allowed position
Given a line (a, b) and a point c, the system SHALL return the closest point on the line restricted by an allowed position (before a, on the segment, after b, or combinations) with a margin, exactly as the original enumerates it.

#### Scenario: Restricted to segment
- **WHEN** c projects beyond b and only the segment is allowed
- **THEN** the result is b

### Requirement: Quadrilaterals
A quadrilateral SHALL hold four vertices A, B, C, D (clockwise from top-left), SHALL report convexity and axis-aligned-rectangle status, SHALL support translate, scale, expand, make-rectangle and make-square from an anchor vertex, horizontal and vertical flips, 90° rotation, point containment, bounding box, emptiness and a stable hash, and SHALL provide the unit square, centred unit square and construction from width/height or a rectangle.

#### Scenario: Convexity
- **WHEN** a bow-tie quadrilateral (0,0),(2,2),(2,0),(0,2) is tested
- **THEN** it is not convex, and the unit square is convex

#### Scenario: Make square from anchor
- **WHEN** make-square is applied with anchor 0 to a 4×2 rectangle
- **THEN** A stays fixed and the quad becomes a square whose side equals the rectangle width within 1e-5

#### Scenario: Oracle parity
- **WHEN** the operations listed in `tests/fixtures/geometry/quads/ops.csv` are replayed
- **THEN** the resulting vertices match within 1e-4

### Requirement: Circles, ellipses and least-squares circle fit
The system SHALL provide circle and ellipse value types (centre, radius; centre, semi-axes, rotation, vertical flag) and SHALL fit a circle to at least three points by linear least squares; with fewer than three points or a singular system it SHALL return the empty circle.

#### Scenario: Exact fit
- **WHEN** 12 points on a circle of centre (3, −2) and radius 5 are fitted
- **THEN** the centre and radius are recovered within 1e-4

#### Scenario: Degenerate input
- **WHEN** two points or three collinear points are fitted
- **THEN** the empty circle is returned

#### Scenario: Oracle parity on noisy data
- **WHEN** the fit is applied to the trajectory of `tests/fixtures/kinematics/circular/linear.csv` (filtered X, Y)
- **THEN** centre and radius equal `best_fit_circle` in the sibling `filtering.json` within 1e-3
