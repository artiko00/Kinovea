# Design

## Context

See proposal.md. The C# geometry folder (`Kinovea.ScreenManager/Measurement/Geometry/`, ~3 300 lines) is pure math over `System.Drawing.PointF`/`RectangleF` with `float` storage. `ProjectiveMapper` keeps `double[,]` internally and exposes `float`. `Matrix3x3` embeds a copy of a Numerical-Recipes SVD (`SVD.cs`) used only by its own helpers. `CircleFitter` is the single user of `MathNet.Numerics` in this folder (a 3×3 normal-equation solve). `AngleHelper` and `SweepAngle` mix geometry with drawing and are deferred. The only existing test asset is `Kinovea.Tests/ProjectiveGeometry/LineClippingTester.cs`, which renders to a PNG for visual check.

## Goals / Non-Goals

**Goals:**
- Numeric parity with the C# code within single-precision tolerances, enforced by oracle fixtures and by algebraic property tests.
- A dependency-free crate (`kinovea-geometry` depends only on `kinovea-core` for nothing yet; it stays standalone) so `kinovea-metadata`, `-tracking` and `-measurement` can depend on it without pulling a linear algebra library.

**Non-Goals:**
- Generic linear algebra; only the operations the application uses.
- GPU or SIMD optimisation.

## Decisions

### D1. Own point and rect types instead of a third-party crate
`kinovea-geometry` defines `PointF {x: f32, y: f32}`, `SizeF`, `RectangleF` (left, top, width, height with right/bottom accessors) mirroring `System.Drawing`. Alternative: `euclid` or `glam`; rejected because the port needs exact `System.Drawing` semantics (e.g. `RectangleF.Right = Left + Width`) in dozens of places and a dependency would leak into every crate.

### D2. Port the SVD and matrix code verbatim, no `nalgebra`
`Matrix3x3` and `SVD.svdcmp` are translated line by line with `f32`/`f64` as in the source. `CircleFitter` replaces `MathNet` by solving the 3×3 normal equations with Cramer's rule in `f64`, returning the empty circle when the determinant is below 1e-12 (the original throws `InvalidOperationException` on singular systems). Alternative: `nalgebra` for both; rejected to keep the crate dependency-free and parity exact.

### D3. Property tests with `proptest` (approval required)
Add `proptest` as a workspace dev-dependency. It is used for: homography forward∘backward ≈ identity, rotation·inverse ≈ identity, clipping endpoints lie on the rectangle boundary, convexity invariance under translation/scale. Seeds are fixed through `proptest`'s persisted regressions file under `rust/kinovea-geometry/proptest-regressions/` (committed).

### D4. Module layout
`src/point.rs`, `rect.rs`, `vector.rs`, `vector3.rs`, `matrix3x3.rs` (+ `matrix3x3/rotations.rs`), `svd.rs`, `helpers.rs` (GeometryHelper + MathHelper), `quadrilateral.rs` (+ `quadrilateral/ops.rs`), `shapes.rs` (Circle, Ellipse, Segment, NamedPoint, PointLinePosition), `clipping.rs`, `projective.rs` (+ `projective/ellipse.rs`), `circle_fit.rs`. Each with `<file>_test.rs`.

### D5. Oracle area `geometry`
`Kinovea.FixtureGen/Areas/GeometryArea.cs` reads `tests/cases/geometry/*.json` and writes: `clipping/lines.csv` (the 10 hand-crafted lines of `LineClippingTester` plus N seeded random lines, columns `ax,ay,bx,by,mode,visible,ex,ey,xx,xy`), `homography/cases.csv` + `points.csv` (seeded plane/image quads, forward/backward of seeded points, ellipse parameters), `helpers/angles.csv` and `points.csv` (GeometryHelper calls), `quads/ops.csv` (make-rectangle/square/flip/rotate results), `svd/*.csv` (seeded matrices and singular values). `Oracle.R` formatting keeps full precision; Rust compares with 1e-3 (positions) and 1e-4 (angles).

### Legacy superseded
`Kinovea.ScreenManager/Measurement/Geometry/{Vector,Vector3,Matrix3x3,SVD,GeometryHelper,MathHelper,QuadrilateralF,SegmentF,Circle,Ellipse,NamedPoint,PointLinePosition,ClipResult,LiangBarsky,ProjectiveMapper}.cs`, `Measurement/Kinematics/CircleFitter.cs`, `Kinovea.Tests/ProjectiveGeometry/LineClippingTester.cs`. `AngleHelper.cs`, `SweepAngle.cs`, `AngleOptions.cs` move with the renderer. Deleted in `remove-dotnet-solution`.

## Risks / Trade-offs

- [`float` accumulation order differs between C# JIT and Rust] → tolerances are relative to single precision (1e-4 to 1e-3 in pixels); strict equality is never asserted on floats.
- [`ProjectiveMapper.Backward(Vector3)` in C# has a known typo using `mapMatrix[1,2]` instead of `unmapMatrix[1,2]`] → the port reproduces the behaviour the application ships, documents it in a test named `backward_vector3_reproduces_original_typo`, and leaves the fix for a later explicit change so fixtures stay valid.
- [`QuadrilateralF.GetHashCode` relies on `PointF.GetHashCode`] → the Rust hash uses the `f32` bit patterns xor-ed; equality of hashes with C# is not a requirement, stability within Rust is.
- [Circle fit on near-collinear points is ill-conditioned] → threshold documented; oracle `kinematics/circular` case gives a well-conditioned parity point.

## Migration Plan

Land in the order point/rect → vectors/matrix/SVD → helpers/quads/shapes → clipping → projective/circle-fit → oracle area and parity tests. No C# behaviour changes; rollback is deleting the crate contents.
