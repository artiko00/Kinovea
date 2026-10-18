# Tasks

## 1. Primitives and dependencies

- [x] 1.1 `proptest` deferred (dependency approval pending, see design D3); property-style tests use the seeded `lib_test::Rng` instead. Added `kinovea-fixtures` as dev-dependency of `rust/kinovea-geometry/Cargo.toml`; verified `cargo build -p kinovea-geometry --tests` compiles.
- [x] 1.2 Write `src/point_test.rs` and `src/rect_test.rs` (right/bottom accessors, contains, empty) failing; implement `PointF`, `SizeF`, `RectangleF` in `point.rs`/`rect.rs`; verify tests pass.
- [x] 1.3 Write `src/vector_test.rs` (from two points, norm, normalized, zero vector NaN) and `src/vector3_test.rs` (dot, cross, min/max index); implement `vector.rs`, `vector3.rs`; verify `cd rust && make verify` is green.

## 2. Matrix and SVD

- [x] 2.1 Write `src/svd_test.rs` (reconstruction and orthonormality on a seeded 5×3 matrix, iteration cap error); port `SVD.svdcmp` to `svd.rs` operating on `Vec<Vec<f64>>`; verify tests pass.
- [x] 2.2 Write `src/matrix3x3_test.rs` (identity, determinant, yaw/pitch/roll round-trip, rotation·inverse, rows/columns/diagonal, multiply by vector) and a seeded check for rotation inverses; port `Matrix3x3` into `matrix3x3.rs` and `matrix3x3/rotations.rs`; verify tests pass and no file exceeds 150 lines (`npx linelens check`).

## 3. Planar helpers, quadrilaterals and shapes

- [x] 3.1 Write `src/helpers_test.rs` (distance, oriented angle signs, middle, point at distance, rotate, closest rotation step 50°→45°, point on parallel, mix, rectangle-centre intersection, closest point with each `PointLinePosition`); implement `helpers.rs` (GeometryHelper + MathHelper); verify tests pass.
- [x] 3.2 Write `src/quadrilateral_test.rs` (convexity of bow-tie vs unit square, axis-aligned test, translate/scale/expand, make-rectangle and make-square per anchor, flips, rotate90, contains, bounding box, hash stability) plus seeded convexity invariance under translation/scale; implement `quadrilateral.rs` and `quadrilateral/ops.rs`; verify tests pass.
- [x] 3.3 Write `src/shapes_test.rs` (Circle empty, Ellipse vertical flag, Segment empty, NamedPoint) and `src/circle_fit_test.rs` (exact fit of 12 points, degenerate inputs return empty); implement `shapes.rs`, `circle_fit.rs` with Cramer's rule; verify tests pass and `make verify` is green.

## 4. Clipping and projective mapping

- [x] 4.1 Write `src/clipping_test.rs` with the ten hand-crafted lines of `Kinovea.Tests/ProjectiveGeometry/LineClippingTester.cs` as segment, line and ray cases plus a seeded check that visible endpoints lie on the rectangle boundary or inside; implement `clipping.rs` (`clip_line(rect, a, b, t0, t1)`, `clip_segment`, `clip_infinite_line`); verify tests pass.
- [x] 4.2 Write `src/projective_test.rs` (corners map to corners, 100 seeded round-trips, degenerate quad leaves mapping unchanged, `backward_vector3_reproduces_original_typo`); implement `projective.rs` (square-to-quad, adjugate, forward/backward for point, quad, vector3); verify tests pass.
- [x] 4.3 Write `src/projective/ellipse_test.rs` (unit circle under identity → unit ellipse, under a known affine map → expected axes); implement `projective/ellipse.rs`; verify tests pass and `make verify` is green.

## 5. Oracle area and parity tests

- [x] 5.1 Add `tests/cases/geometry/{clipping,homography,helpers,quads,svd}.json` with seeds and counts, and `Kinovea.FixtureGen/Areas/GeometryArea.cs` writing the CSV files described in design D5; register in `Program.cs`; verify `tools/oracle/fixtures.sh --verify` is green and `tests/fixtures/geometry/` is committed.
- [x] 5.2 Write `rust/kinovea-geometry/tests/geometry_parity_test.rs` (dev-dependency `kinovea-fixtures`) asserting clipping (1e-3), homography points and ellipse (1e-3, rotation 1e-4), helper angles and points (1e-4), quad ops (1e-4), SVD singular values (1e-9); verify every fixture row passes.
- [x] 5.3 Write `rust/kinovea-geometry/tests/circle_fit_parity_test.rs` fitting the filtered X,Y of `tests/fixtures/kinematics/circular/linear.csv` and comparing to `filtering.json` `best_fit_circle` within 1e-3; verify it passes.

## 6. Integration

- [x] 6.1 Export the public API from `rust/kinovea-geometry/src/lib.rs`, document the module map in the crate root doc comment, update `tests/README.md` (area `geometry`) and `.agent/progress.md`; verify `cd rust && make verify`, `npm run lint` and `cargo test -p kinovea-fixtures` are green.
