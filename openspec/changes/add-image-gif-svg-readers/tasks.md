# Tasks

## 1. Crate and shared timeline

- [ ] 1.1 After approval of the dependencies: add `rust/kinovea-video-image` to the workspace with `image`, `kamadak-exif`, `resvg`, `usvg`, `tiny-skia` and `kinovea-video`; write `src/timeline_test.rs` (250 frames, 25 fps, interval 40 ms, working zone 0..249) then implement `src/timeline.rs::still_timeline`; verify `cargo test -p kinovea-video-image` passes.
- [ ] 1.2 Write `src/generator_test.rs` with a fake `FrameGenerator` covering first move-next → frame 0, move-to inside and outside the zone, rotation swaps reference size, summary flags image; implement `src/generator.rs` (`FrameGenerator` trait, `GeneratorReader<G>`); verify tests pass.

## 2. Still images

- [ ] 2.1 Write `tests/support/images.rs` generating PNG/JPEG/BMP files in a temp dir with the `image` encoders and a JPEG with EXIF orientation 6 (write the APP1 segment with `kamadak-exif`-compatible bytes); write `src/raster_test.rs` (open each format, size, EXIF transposed size, user rotation, corrupt file failure, thumbnail 100x75); implement `src/raster.rs` (`RasterGenerator` with orientation mapping and rotation); verify tests pass.
- [ ] 2.2 Write `src/orientation_test.rs` for the EXIF→rotation table including mirrored values then implement `src/orientation.rs`; verify tests pass.

## 3. GIF

- [ ] 3.1 Extend `tests/support/images.rs` to write GIFs (3 frames at 10 hundredths, zero-delay variant, partial-update second frame with disposal, one-frame GIF) with the `image` GIF encoder; write `src/gif_test.rs` (timing fields, composited frame 1, closest navigation at 15, single-frame summary flag, thumbnail spacing 0 and 3) then implement `src/gif.rs` on `kinovea_video::Cache`; verify tests pass.

## 4. SVG

- [ ] 4.1 Write `src/svg_test.rs` with literal SVG strings (width/height, viewBox-only, no size, malformed, 100x100 diagonal line rendered at 400x400 and checked for the absence of 4x4 constant blocks along the edge, white background); implement `src/svg.rs` (`SvgGenerator` over `usvg`/`resvg`); verify tests pass.

## 5. Oracle parity

- [ ] 5.1 Add `tests/cases/imagesource/{png-640x480,jpeg-exif6,gif-3x10,gif-zero-delay,gif-single}.json`; implement `Kinovea.FixtureGen/Areas/ImageSourceArea.cs` writing `info.json` per source and register it; run `tools/oracle/fixtures.sh --verify`; verify fixtures are identical across the two runs.
- [ ] 5.2 Write `tests/imagesource_parity_test.rs` regenerating equivalent images and asserting every `info.json` field through `kinovea-fixtures`; verify `cargo test -p kinovea-video-image` passes.

## 6. Integration

- [ ] 6.1 Implement `register(&mut ReaderRegistry)` for `.jpg .jpeg .png .bmp .gif .svg` with `src/lib_test.rs`, wire it in `rust/kinovea/src/main.rs` next to the FFmpeg registration; verify `cd rust && make verify` exits 0 and `npx testlens check` lists no untested file.
- [ ] 6.2 Update `AGENTS.md` structure line for the new crate, append the `.agent/progress.md` entry (GDI+ vs `image` GIF differences found) and tick `09` in `openspec/roadmap.md` after archive; verify `npm run lint` passes.

## Workflow follow-up

- Archive after review; sync creates `video/still-image-source`, `video/gif-source`, `video/svg-source`.
