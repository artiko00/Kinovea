# Proposal

## Why

Drawings, calibration, tracking and kinematics all rest on the same small 2D/3D geometry toolbox that the C# code keeps in `Kinovea.ScreenManager/Measurement/Geometry/`: vectors, a 3×3 matrix, a projective (homography) mapper, quadrilaterals, line clipping, angles and circle/ellipse helpers. Porting it as a self-contained crate with property-based and oracle tests gives every later change a verified mathematical base.

## What Changes

- Port `Vector`, `Vector3`, `Matrix3x3` (rotations, yaw/pitch/roll, rows/columns, multiply, determinant, inverse, SVD-based helpers) and the Numerical-Recipes `SVD.svdcmp` routine to `kinovea-geometry`.
- Port `GeometryHelper` (distances, angles, rotations, projections on lines, rotation steps, mixing, rectangle-centre intersection) and `MathHelper` (degree/radian conversion, rounding).
- Port `QuadrilateralF` (convexity, axis-aligned test, translate/scale/expand, make rectangle/square, flips, rotate 90, contains, bounding box) and `SegmentF`, `Circle`, `Ellipse`, `NamedPoint`, `PointLinePosition`, `ClipResult`.
- Port `LiangBarsky` segment/line/ray clipping.
- Port `ProjectiveMapper` (square-to-quad homography, forward/backward for points, quads and homogeneous vectors, ellipse of the mapped unit circle) and `CircleFitter` least-squares fit.
- Add the oracle area `geometry` to `Kinovea.FixtureGen` producing reference outputs for clipping, homographies, angles and circle fits over deterministic inputs.

## Capabilities

### New Capabilities
- `geometry/vector-and-matrix`: 2D/3D vector arithmetic, 3×3 matrix operations and singular value decomposition.
- `geometry/planar-helpers`: distances, angles, rotations, point projections, quadrilaterals, circles, ellipses and least-squares circle fitting.
- `geometry/line-clipping`: Liang-Barsky clipping of segments, rays and infinite lines against a rectangle.
- `geometry/projective-mapping`: homography between two quadrilaterals with forward and inverse mapping.

### Modified Capabilities
None.

## Impact

- **Assemblies touched**: none of the shipped C# assemblies change. `Kinovea.FixtureGen` gains `Areas/GeometryArea.cs`; `tests/cases/geometry/` and `tests/fixtures/geometry/` are added.
- **Rust**: `rust/kinovea-geometry` receives its modules. Candidate dev-dependency `proptest` for invertibility and convexity properties (approval needed, see design). No runtime dependency: the matrix code is small and ported verbatim rather than pulling `nalgebra`.
- **Verification**: Linux (`cd rust && make verify`, CI `rust.yml`). The new oracle area is verified with `tools/oracle/fixtures.sh --verify`; `build.yml` remains the secondary check that the solution compiles.

## Non-goals

- Anything that draws: `AngleHelper.DrawText`, `SweepAngle` hit-testing with viewport transforms and `CoordinateSystemGrid` belong to the renderer and player changes.
- Calibration semantics (`CalibratorPlane`, `CameraPoser`, lens distortion): `add-calibration-and-lens-distortion`.
- Replacing the Numerical-Recipes SVD with a library implementation; parity first.
