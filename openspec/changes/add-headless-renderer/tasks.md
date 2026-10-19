# Tasks

## 1. Oracle render area

- [x] 1.1 Add `Kinovea.FixtureGen/Areas/RenderArea.cs`: load the case KVA (same context as `KvaArea`), create a `Bitmap` of the case size at 96 DPI filled with the case color, call the original drawing `Draw` methods in the `FlushDrawingsOnGraphics` order via a small port of that method, save `<timestamp>.png` per declared timestamp; add `tests/cases/render/{simple,complete,all-drawings}.json` (size, background, timestamps, defaultFading); verify `tools/oracle/fixtures.sh --verify` is byte-identical. Cases: `geometry` (tight), `geometry-x2` (viewport scale 2), `simple`, `complete`, `all-drawings`. Deviation: tolerances `mad_max`/`outliers_max` live in the case JSON (reviewable, covered by `case_sha256`) instead of `manifest.json`.
- [x] 1.2 Extend `RenderArea.cs` to write `hits.csv` (drawing id, probe x, probe y, result) for a probe grid (step 4 px over the drawing bounds expanded by 16 px) at viewport scales 0.5, 1 and 2, plus `manifest.json` extras `mad_max` and `outliers_max`; verify regenerated fixtures and `cargo test -p kinovea-fixtures` pass. Deviation: probes cover the whole image at `hitStep` and only non-miss rows are written; the probe time is the keyframe time (detached drawings: `hitTimestamp`), with a frame painted first because drawings compute screen boxes while drawing; a drawing whose original `HitTest` throws (non-perspective plane with a sliding line) gets one `exception` row.

## 2. Crate skeleton and dependencies

- [ ] 2.1 After approval, add `tiny-skia`, `fontdue`, `png` to `rust/kinovea-render/Cargo.toml`, bundle `assets/LiberationSans-Regular.ttf`, `assets/LiberationSans-Bold.ttf` and their license, depend on `kinovea-metadata`, `kinovea-geometry`, `kinovea-core`, `kinovea-fixtures` (dev); verify `cd rust && make verify`.
- [x] 2.2 Create modules `src/{renderer,paint,viewport,style,hit,compose,backend/recording,backend/cpu}.rs` with `*_test.rs` placeholders; verify `npx testlens check --fail`. The crate depends only on internal crates until the external ones are approved.

## 3. Trait, viewport and recording backend

- [x] 3.1 Write `src/viewport_test.rs` (transform/untransform round trip, distance and rectangle transforms vs values computed from `ImageToViewportTransformer.cs`), implement `src/viewport.rs`; verify tests pass.
- [x] 3.2 Write `src/renderer_test.rs` for `Pen`, `Brush`, `Font` value types and `src/style_test.rs` for style-to-paint mapping (width scaling, dash patterns, alpha from opacity, font scaling); implement `src/renderer.rs` (trait) and `src/style.rs`; verify tests pass.
- [x] 3.3 Implement `src/backend/recording.rs` (logs every call) with `recording_test.rs`; verify tests pass.

## 4. Painter

- [x] 4.1 Write `src/paint/line_test.rs` etc. using the recording backend (expected primitive sequence for straight, dashed, squiggly, arrows) then port `Line`, `Polyline`, `Pencil`, `Rectangle`, `Circle`, `CrossMark` painting including `ArrowHelper` and squiggly lines into `src/paint/*.rs`; verify tests pass.
- [x] 4.2 Port `Angle` (pie, arc, arrow), `Plane`, `DistortionGrid`, `TestGrid`, `CoordinateSystem` painting with recording tests; verify tests pass.
- [x] 4.3 Port `MiniLabel`, `RoundedRectangle`, `BoundingBox` helpers and `Label` painting with text measure and draw; recording tests; verify tests pass.
- [x] 4.4 Port `Chrono`, `ChronoMulti`, `Counter`, `TimeSegment`, `TimePolyline`, `Spotlights`, `AutoNumbers`, `Track` (trajectory, points, labels), `GenericPosture`, `Bitmap` and `SVG` placeholders; recording tests; add the primitive-coverage test over the `all-drawings` fixture; verify tests pass. Deviations: Bitmap/SVG are placeholders until image readers land; counter cadence/period texts and the time-segment fraction formatting are simplified (recorded for the snapshot round).

## 5. CPU backend and snapshots

- [ ] 5.1 Write `src/backend/cpu_test.rs` (solid shapes rasterize with expected coverage, PNG encode and decode round trip, determinism) then implement `src/backend/cpu.rs` on `tiny-skia` with cardinal-spline to Bezier conversion and dash support; verify tests pass.
- [ ] 5.2 Implement text rendering with `fontdue` and the bundled fonts in `src/backend/cpu/text.rs`, test measuring and drawing a label; verify tests pass headless (`DISPLAY=` unset).
- [ ] 5.3 Write `tests/snapshot_test.rs`: render each `tests/cases/render/*` case over a flat frame at each timestamp, compare with fixture PNGs using `mad_max` and `outliers_max`, save the diff image under `target/` on failure; implement `src/compose.rs`; verify all cases pass.

## 6. Hit testing

- [x] 6.1 Write `src/hit_test.rs` (point box, path widening, single-point ellipse case, scale dependence) then implement `src/hit.rs`; verify tests pass.
- [x] 6.2 Implement per-drawing `hit_test(&Drawing, point, &HitContext) -> Hit` in `src/hit/*.rs` with unit tests per kind; add `tests/hits_test.rs` over `hits.csv` with the 99 percent and 2 px boundary rule; verify passes. Two explicit, staleness-checked lists (text metrics pending a font rasterizer; capped Pencil/posture deviations) are recorded in the design.

## 7. Integration

- [ ] 7.1 Add `cargo run --example snapshot -p kinovea-render -- <kva> <out.png>` example used by later changes and by the MCP server; verify it writes a PNG for the `complete` sample.
- [ ] 7.2 Update `AGENTS.md` (crate line, fixture areas), append `.agent/progress.md` entry with the tolerances chosen, tick the roadmap item; verify `npm run lint`, `cd rust && make verify` and CI `rust.yml` are green.
