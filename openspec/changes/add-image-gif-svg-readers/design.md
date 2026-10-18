# Design

## Context

See proposal.md. Observed in the C# source:

- `VideoReaderBitmap` wraps an `IFrameGenerator` (`FrameGeneratorImageFile`) and fakes a timeline of 250 frames at 25 fps with 1 timestamp per frame (`SetupVideoInfo`); `MoveRequest(next)` delivers frame 0 the first time; rotation re-opens the file and applies `RotateFlip`; EXIF orientation is read from property 0x0112 and mapped to `ImageRotation` ignoring mirroring.
- `VideoReaderGIF` decodes every frame into a `Cache` with `AverageTimeStampsPerSeconds = 100`, interval from `PropertyTagFrameDelay` (0x5100), `DurationTimeStamps = count * interval`, `FramesPerSeconds = 100 / interval`; summary thumbnails every `ceil(count / requested)` frames.
- `VideoReaderSVG` wraps `FrameGeneratorSVG` (SharpVectors) with the same fake timeline, re-renders at `Generate(timestamp, maxSize)`.
- All three implement `VideoReader` without prebuffering and declare `CanChangeWorkingZone | CanChangeImageRotation` (images) or no extra capability (GIF).

## Goals / Non-Goals

**Goals:**
- Readers implemented purely on top of `kinovea-video` traits and containers, so they also serve as the first realistic `VideoReader` implementations in tests of the player.
- Timing constants validated against the original through the oracle instead of being copied.

**Non-Goals:**
- Progressive or lazy GIF decoding (files are small; the original decodes everything).
- Colour management.

## Decisions

### D1. Crate `kinovea-video-image` (requires approval)
Dependencies: `image` (features `png`, `jpeg`, `bmp`, `gif`), `kamadak-exif` (orientation), `resvg` + `usvg` + `tiny-skia` (SVG). Alternatives: `zune-image` (fewer formats), `librsvg` bindings (native GNOME dependency; rejected to stay pure Rust). `tiny-skia` is also the planned backend of `add-headless-renderer`, so it adds no new native code. The `image` crate's GIF decoder composites frames with disposal applied, matching the GDI+ behavior observed in the original.

### D2. One `FrameGenerator` trait for images and SVG
Port `IFrameGenerator` as `trait FrameGenerator { open, close, original_size, reference_size, original_rotation, set_rotation, generate(timestamp), generate_sized(timestamp, max_size) }` and one `GeneratorReader<G: FrameGenerator>` implementing `VideoReader` with the shared fake timeline, replacing the two near-identical C# readers. GIF uses `kinovea_video::Cache` directly.

### D3. Timeline constants come from a shared `still_timeline()` helper
Returns the `VideoInfo` for a still source (250 frames, 25 fps, 1 ts/frame). The oracle `imagesource` area records the same fields from the original readers so the constants are asserted, not assumed.

### D4. Orientation mapping ported from `FrameGeneratorImageFile`
EXIF 1→0°, 3→180°, 6→90°, 8→270°; mirrored values (2,4,5,7) map to their rotation counterpart. User rotation composes with EXIF rotation; the generator re-decodes and rotates on `set_rotation` as the original does (simplicity over caching).

### D5. SVG sizing
`usvg::Tree::size()` gives the document size (width/height, else viewBox); fallback 800x600 when the tree has no size. Rendering uses `resvg::render` into a `tiny_skia::Pixmap` of the requested size with a white background, converted to BGRA `FrameBuffer`.

### D6. Oracle area `imagesource`
`Kinovea.FixtureGen/Areas/ImageSourceArea.cs` generates in memory a PNG (`inputs.width x height`), a JPEG with an EXIF orientation tag (`inputs.orientation`) and a GIF (`inputs.frames`, `inputs.delayHundredths`) using System.Drawing, opens each with the original readers and writes `info.json` per source (fps, interval, timestamps per second, average timestamps per frame, duration, first timestamp, original size, rotation, working zone, summary image flag). The generated images are not written to the fixture; the Rust test regenerates equivalents with the same dimensions and delays.

## Legacy superseded

`Kinovea.Video.Bitmap/VideoReaderBitmap.cs`, `FrameGeneratorImageFile.cs`; `Kinovea.Video.Gif/VideoReaderGif.cs`; `Kinovea.Video.SVG/VideoReaderSVG.cs`, `FrameGeneratorSVG.cs`; their `.csproj` files and the SharpVectors packages.

## Risks / Trade-offs

- [GDI+ and `image` differ on GIF edge cases (disposal, local palettes)] → spec requires composited frames; tests use GIFs generated with explicit disposal modes; divergences are recorded as documented differences rather than reproduced.
- [System.Drawing cannot write EXIF orientation directly] → the FixtureGen area sets the property item on a JPEG `Bitmap` before saving, which GDI+ supports; if the tag is dropped, the orientation case is limited to the Rust-side test and noted in the fixture README.
- [`resvg` output differs from SharpVectors] → no pixel parity is claimed; only size, timeline and re-rasterization behavior are specified.

## Migration Plan

Additive crate; registry entries are added in the `kinovea` binary. Rollback: remove the member.
