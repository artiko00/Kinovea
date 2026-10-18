# Proposal

## Why

In the original application every drawing paints itself directly on a GDI+ `Graphics` and tests itself for hits with GDI+ paths, so the model cannot be exercised without a Windows UI. The migration needs drawings to be rendered and picked without any window: for parity tests, for the MCP server that must return annotated frames, and later as the single code path the `egui` desktop UI reuses.

## What Changes

- Add a `Renderer` trait in `kinovea-render` with the primitives the 23 drawing kinds use (lines, curves, ellipses, arcs, rectangles, paths, text, images, clipping, transforms) plus pens, brushes and fonts derived from style and opacity.
- Add the painter: one `paint` per drawing kind ported from each `Draw` override, driven by the viewport transform, timestamp, selection, and optional distortion and camera-motion transforms from later changes.
- Add scene composition: paint a video frame plus the metadata in the original z-order (detached chronos, tracks, singletons, then attached keyframe drawings by fading z-order or current keyframe).
- Add a CPU backend on `tiny-skia` producing RGBA buffers and PNG files, used by tests and by the MCP server.
- Add hit testing as geometry: handle hits, path hits with expansion, line hits, rounded-rectangle and bounding-box handles, matching the original tolerances in viewport pixels.
- Extend the parity oracle with a `render` area: GDI+ rasterizations of each case KVA at fixed sizes and timestamps, plus hit-test tables.

No UI is added. No C# code changes except `Kinovea.FixtureGen`.

## Capabilities

### New Capabilities
- `render/renderer-trait`: backend-neutral drawing primitives and style-to-paint mapping that any backend (CPU, GPU, UI toolkit) implements.
- `render/headless-snapshot`: composition of a frame and its annotations into an image buffer or PNG with no window, with declared tolerance against the original rendering.
- `render/hit-testing`: picking of drawings and handles at a point, in image coordinates, with viewport-scale tolerances.

### Modified Capabilities
- `tooling/parity-oracle`: new `render` fixture area (PNG rasterizations and hit tables).

## Impact

- **Assemblies touched**: `Kinovea.FixtureGen` only (new `RenderArea`). `Kinovea.ScreenManager` is read, not modified.
- **Crates**: `kinovea-render` (depends on `kinovea-metadata`, `kinovea-geometry`, `kinovea-core`). New crates `tiny-skia`, `png`, a font rasterizer and a bundled font are decisions in design.md requiring approval.
- **Files**: `rust/kinovea-render/**`, `tests/cases/render/**`, `tests/fixtures/render/**`, `Kinovea.FixtureGen/Areas/RenderArea.cs`.
- **Verification**: Rust verified on Linux (`cd rust && make verify`, CI `rust.yml`). FixtureGen verified on the Windows host with `tools/oracle/fixtures.sh --verify` and CI `build.yml`. Pixel parity is a tolerance comparison (text rasterization differs between GDI+ and any other engine), stated per fixture.

## Non-goals

- A GPU or `egui` backend (`add-player-screen-rendering` implements the trait on `egui`).
- The magnifier, keyframe thumbnails, timeline and toolbar painting.
- Video decoding; frames are given as RGBA buffers.
- Distortion and camera-motion maths; the painter accepts optional transforms and this change provides identity implementations only.
- Interactive editing (moving handles) beyond reporting which handle is hit.
