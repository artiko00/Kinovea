# Proposal

## Why

Real videos enter Kinovea through the C++/CLI project `Kinovea.Video.FFMpeg` (5.5k lines over FFmpeg 8.1.2). It is the only component that cannot be compiled on Linux at all, and every later change (tracking, player, MCP server) needs frames from actual files. Porting it to Rust over the system libav libraries unblocks the whole pipeline on Linux.

## What Changes

- Add crate `kinovea-video-ffmpeg` implementing the `VideoReader` trait from `add-video-reader-and-cache` with libav: open and probe (size, rotation from the display matrix, sample aspect ratio quirks, frame rate estimation, start time, duration, last timestamp), software decoding, scaling and pixel conversion to BGRA, and the three decoding modes (on demand, prebuffering on a decoder thread with job relocation plans and frame-skipping policy, full caching of the working zone within a memory budget).
- Exact seeking: seek to the keyframe at or before the target, decode forward until the target timestamp, honouring the pre-roll so a few frames behind the target are available for stepping back.
- Video geometry: output size from the presentation request, rotation, aspect ratio forcing, deinterlacing, demosaicing, optional pre-scaling, stabilization offset from a point list.
- Summary extraction (thumbnails at evenly spaced seeks) and image-sequence opening through a numbered pattern.
- Recording: port `MJPEGWriter` as a `VideoWriter` that muxes MJPEG or raw frames with a frame interval, rotation metadata and quality, used by capture and export.
- CI installs the Linux libav development packages and the `ffmpeg` CLI, which test setup uses to synthesize clips.

## Capabilities

### New Capabilities
- `video/ffmpeg-decoding`: opening, probing, exact seeking, decoding modes and geometry for file-based videos.
- `video/video-recording`: writing frame sequences to a video container with configurable codec, interval, quality and rotation.

### Modified Capabilities
None.

## Impact

- **Assemblies superseded**: `Kinovea.Video.FFMpeg` (C++/CLI: `VideoReaderFFMpeg.cpp/.h`, `MJPEGWriter.cpp/.h`, `SavingContext.h`, `ReadResult.h`). The C# solution is not modified.
- **Rust**: new workspace member `rust/kinovea-video-ffmpeg` (decision requiring approval: crate `ffmpeg-next` and native `libavcodec`, `libavformat`, `libavutil`, `libswscale`, `libavfilter` development packages). `kinovea-video` is unchanged except for registering the reader for `*`.
- **Workflows**: `.github/workflows/rust.yml` gains an apt step for libav and `ffmpeg` (requires approval).
- **Tests**: clips are generated at test time from Rust-synthesized frames encoded losslessly with the `ffmpeg` CLI; no binary video is committed. No oracle fixture: decoder output depends on the libav build.
- **Verification**: Linux only (`cd rust && make verify`, CI). The C++/CLI project is not touched and keeps compiling in `build.yml`.
- No UI, no KVA format impact.

## Non-goals

- Hardware decoding (CUDA/D3D11VA in the original). VAAPI is an optional feature left for a later change.
- Audio.
- Camera capture formats and live streams (`add-capture-pipeline-and-cameras`).
- Exporting videos with drawings burnt in (`add-importers-exporters` composes frames and calls the writer defined here).
