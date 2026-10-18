# Design

## Context

See proposal.md. Constraints observed in the original code:

- Each drawing's `Draw(Graphics, DistortionHelper, CameraTransformer, IImageToViewportTransformer, bool selected, long timestamp)` and `HitTest(PointF, long, DistortionHelper, IImageToViewportTransformer)` live inside the drawing class. GDI+ primitives used across `Metadata/Drawings` and `DrawingTrack`: DrawLine (41), FillEllipse (30), DrawString (17), DrawEllipse (17), DrawCurve (12), DrawRectangle (8), FillPath (6), DrawLines (4), DrawImage (3), DrawArc (3), FillPie (2), squiggly lines (3), DrawPath (2), FillRectangle (1), FillPolygon (1), plus `Matrix`/Translate/Rotate transforms and `GraphicsPath` for hit testing.
- Helpers: `ArrowHelper`, `RoundedRectangle`, `BoundingBox`, `MiniLabel`, `GraphicsExtensions.DrawSquigglyLine`, `HitTester` (point box 10 px, path widen by line size + 10 px, `Region.IsVisible`).
- Composition order is in `PlayerScreenUserInterface2.FlushOnGraphics` and `FlushDrawingsOnGraphics`.
- Text uses GDI+ `Arial` (bold for labels). No cross-platform renderer reproduces GDI+ text antialiasing exactly; parity must be tolerance based.
- `DistortionHelper` (04) and `CameraTransformer` (11) are not available yet; the painter must accept optional implementations of two small traits and run with identity.

## Goals / Non-Goals

**Goals:**
- Drawings paint through one trait; the CPU backend is the reference implementation and the only one in this change.
- Pixel snapshots and hit tables compared against the oracle with explicit tolerances.
- Zero dependency on display, GPU or system fonts.

**Non-Goals:**
- Performance tuning for real-time playback (the egui backend in 15 will own that).
- Magnifier, thumbnails, UI chrome.

## Decisions

### D1. `Renderer` trait with resource-less primitives
`trait Renderer { fn stroke_line(&mut self, a, b, &Pen); fn fill_ellipse(...); fn draw_text(&mut self, text, &Font, &Brush, at) -> Size; fn measure_text(...); fn draw_image(&mut self, &ImageRef, src, dst, mirrored); fn push_transform(&mut self, Transform); fn pop_transform(); fn clip(rect); ... }`. `Pen`, `Brush`, `Font` are plain value structs (color with alpha, width, dash pattern, cap, join; font family tag, size, bold). Alternative: a retained scene graph (`Vec<Shape>`) consumed by backends. The immediate trait mirrors the C# code one-to-one, which keeps the port auditable; a recording backend (`RecordingRenderer`) gives tests the scene-graph view where useful.

### D2. Painter as functions over `Drawing`, not methods on the model
`kinovea-render::paint::paint_drawing(&Drawing, &PaintContext, &mut dyn Renderer)` with one module per drawing kind, matching `match` arms of the metadata enum. `PaintContext { transformer: &ImageToViewport, timestamp, selected, distorter: Option<&dyn Distorter>, camera: Option<&dyn CameraTransform> }`. Keeps `kinovea-metadata` free of rendering.

### D3. Viewport transform lives here
`ImageToViewport { location, scale }` ports `ImageToViewportTransformer`. Hit testing uses `untransform(distance)` for tolerances.

### D4. CPU backend on `tiny-skia`, text via `fontdue`, bundled font
`tiny-skia` for anti-aliased paths, strokes, dashes, clipping and transforms; `fontdue` to rasterize glyphs of a bundled open font metrically close to Arial (Liberation Sans, SIL OFL) into `tiny-skia` pixmaps; `png` for encoding. Alternatives: `raqote` (less maintained), `skia-safe` (heavy C++ build), `cosmic-text` (more than needed). Dependencies requiring approval: `tiny-skia`, `fontdue`, `png`, and the Liberation Sans TTF added under `rust/kinovea-render/assets/` with its license file.

### D5. Hit testing as geometry, not pixel regions
`HitTester` ports: `hit_point` (box of `max(1, untransform(10))`), `hit_path` (polyline widened by `line_size + untransform(10)`: distance from point to segments, with the single-point ellipse special case), `hit_line` with optional distorted curve, filled-area hits via point-in-polygon on the flattened path. `GraphicsPath.Widen + Region.IsVisible` is approximated by distance-to-segment; the mismatch tolerance (2 px at boundaries) is declared in the spec.

### D6. Scene composition
`compose::render_frame(frame: &RgbaImage, &Metadata, &SceneOptions { size, timestamp, keyframe_index, default_fading, mirrored, background }, &mut dyn Renderer)` ports `FlushOnGraphics` (image placement, mirror, background) and `FlushDrawingsOnGraphics` (z-order). `Metadata::keyframes_z_order(timestamp)` is implemented in 05 or here if missing.

### D7. Parity tolerance
Oracle PNGs come from GDI+ with `SmoothingMode.AntiAlias` and `TextRenderingHint.AntiAlias`. Comparison: mean absolute difference per channel and a count of pixels differing by more than 64. Tolerances stored in `tests/fixtures/render/<case>/manifest.json` (`mad_max`, `outliers_max`) and chosen per case when the fixture is created: tight for geometry-only cases (lines, circles, planes), looser for text-heavy cases. Text differences are the accepted trade-off.

### Legacy superseded
Painting and hit-testing parts of `Kinovea.ScreenManager/Metadata/Drawings/*.cs`, `Metadata/Drawings/Helpers/{ArrowHelper,BoundingBox,GraphicsExtensions,RoundedRectangle}.cs`, `Metadata/{HitTester,MiniLabel}.cs`, `IImageToViewportTransformer.cs`, `ImageToViewportTransformer.cs`, and the flush methods of `PlayerScreen/Controls/PlayerScreenUserInterface2.cs` (`FlushOnGraphics`, `FlushDrawingsOnGraphics`, `GetFlushedImage`). Deleted in `remove-dotnet-solution`.

## Risks / Trade-offs

- [GDI+ text cannot be matched pixel-exactly] → tolerance-based comparison; geometry-only cases keep tight tolerances so regressions in shapes are still caught.
- [Cardinal spline (`DrawCurve`, tension 0.5 default and 0.9 for squiggles) differs from Bezier] → port GDI+ cardinal-to-Bezier conversion explicitly; test on the Pencil and Polyline cases.
- [Dash patterns and caps differ subtly] → compare on dashed line cases with slightly looser `outliers_max`.
- [Hit testing approximation] → probe grid of 1 px step around each drawing; spec accepts 1 percent boundary mismatches.
- [Oracle rendering depends on host DPI and fonts] → `RenderArea` paints onto an in-memory `Bitmap` with explicit 96 DPI and `Arial`, independent of the screen.

## Migration Plan

1. Oracle `render` area and fixtures first.
2. Trait, viewport, recording backend, painter for geometric drawings.
3. CPU backend, snapshots for geometric cases.
4. Text, labels, chronos, tracks, postures; composition; hit testing.
Rollback: additive crate; no consumer until 13 and 15.

## Open Questions

- Whether `Bitmap`/`SVG` drawings are rasterized here or stubbed with a placeholder until `add-image-gif-svg-readers`; stubbing is the default (a grey rectangle with the file name), decided when 09 lands.
