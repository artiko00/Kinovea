# Proposal

## Why

Kinovea opens still images, animated GIFs and SVG drawings as if they were videos so users can annotate photos and diagrams with the same tools. Those three small readers (`Kinovea.Video.Bitmap`, `Kinovea.Video.Gif`, `Kinovea.Video.SVG`, 1.3k LOC) depend on GDI+ and SharpVectors; porting them gives the player and the MCP server a decoder-free source for tests and demos long before cameras exist.

## What Changes

- Add crate `kinovea-video-image` implementing the `VideoReader` trait for:
  - still images (`.jpg`, `.jpeg`, `.png`, `.bmp`): a frame generator exposing the image at any timestamp, with a synthetic 10 s at 25 fps timeline, EXIF orientation applied, user rotation by re-generation and thumbnails;
  - animated GIFs: every frame decoded into the dense cache with the per-frame delay, 100 timestamps per second, frame count and duration derived from the file;
  - SVG files (`.svg`): rendered at the requested output size so zooming re-rasterizes instead of scaling pixels, with the same synthetic timeline as images.
- Register the three readers in the reader registry by extension.
- Add an `imagesource` oracle area recording the video info the original readers compute for generated test images, so timing constants are checked against the C# code rather than copied by hand.

## Capabilities

### New Capabilities
- `video/still-image-source`: a single image exposed as a fixed-length video with orientation handling.
- `video/gif-source`: animated GIF frames exposed with their native delays.
- `video/svg-source`: vector drawings rasterized on demand at the presentation size.

### Modified Capabilities
None.

## Impact

- **Assemblies superseded** (deleted in `remove-dotnet-solution`): `Kinovea.Video.Bitmap`, `Kinovea.Video.Gif`, `Kinovea.Video.SVG` and the SharpVectors NuGet reference. `Kinovea.FixtureGen` gains the `imagesource` area; no other C# file changes.
- **Rust**: new workspace member `rust/kinovea-video-image` (decision requiring approval: crates `image` with `png`, `jpeg`, `bmp`, `gif` features, `kamadak-exif` for orientation, `resvg`/`usvg`/`tiny-skia` for SVG). Depends on `kinovea-video`.
- **Tests**: images, GIFs and SVGs are generated in test setup with the same crates (encoders) or as literal SVG text; nothing binary is committed except the small oracle `info.json` files.
- **Verification**: Linux (`cd rust && make verify`, CI). Oracle area verified with `tools/oracle/fixtures.sh --verify`. `build.yml` keeps compiling the untouched readers.
- No UI, no KVA format impact.

## Non-goals

- Image sequences as video (handled by the FFmpeg reader's pattern support).
- Editing or exporting images (`add-importers-exporters`).
- Animated SVG or SVG scripting; only static rendering.
- HEIC/AVIF/WebP: can be added to the still-image reader later by enabling `image` features, not part of this change.
