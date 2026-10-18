# Design

## Context

See proposal.md. The original is `Kinovea.Video.FFMpeg/PlayerServer/VideoReaderFFMpeg.cpp` (4k lines) plus `MJPEGWriter.cpp` (0.6k). Relevant facts observed in the source:

- Timestamps are in the stream time base (`AverageTimeStampsPerSeconds = time_base.den / time_base.num`); `FirstTimeStamp` comes from the container start time, `DurationTimeStamps` from the stream duration, else the container duration, else from `nb_frames`; frame rate from `avg_frame_rate` then `r_frame_rate`, else measured.
- Decoded timestamps use `best_effort_timestamp`; seeking uses `avformat_seek_file` with `AVSEEK_FLAG_BACKWARD` after consulting the index (`avformat_index_get_entry_from_timestamp`) and flushes the codec.
- Rotation is `av_display_rotation_get` on the display matrix side data, rounded to 90 degrees.
- Scaling and conversion use the dynamic `sws_scale_frame` API into `AV_PIX_FMT_BGRA`; deinterlacing and demosaicing adjust the source format; a hardware filter graph (`scale_cuda`) exists only for CUDA.
- Frame skipping toggles `skip_frame` between `AVDISCARD_DEFAULT` and `AVDISCARD_NONREF` and seeks ahead when the lag exceeds a threshold; `skip_loop_filter = AVDISCARD_ALL` is always set.
- Three containers (`SingleFrame`, `PreBuffer2`, `Cache`) are selected by `ChangeCachingMode`; the prebuffer thread works from a `DecodingJobPlan` with a relocation (`None`, `Advance`, `Seek`) and a pre-roll of `RetentionWindow + 2` frames.
- The writer muxes `AV_CODEC_ID_MJPEG` or `AV_CODEC_ID_RAWVIDEO`, container guessed from the extension, incoming frames `BGRA` or `RGB24`, encoder `YUV420P`/`YUVJ420P`, rotation written as stream metadata.

Linux host today: FFmpeg 9.0 (libavcodec 63). Ubuntu 24.04 (CI runner) ships FFmpeg 6.1 (libavcodec 60). The C# app bundled 8.1.2.

## Goals / Non-Goals

**Goals:**
- Same observable timing model as the original so KVA files keep referring to the same timestamps.
- Exactness of seeks verified against synthesized clips, independent of the libav build.
- The reader works with the libav versions shipped by current Linux distributions.

**Non-Goals:**
- Zero-copy GPU paths; hardware decoding is behind a future `vaapi` feature.
- Re-implementing demuxing or decoding.

## Decisions

### D1. Binding crate: `ffmpeg-next` (requires approval)
Alternatives: `rsmpeg` (thinner, tracks FFmpeg releases quickly, but less idiomatic), raw `ffmpeg-sys-next` (maximum control, all `unsafe`). `ffmpeg-next` is the most used safe wrapper and exposes `format`, `codec`, `software::scaling`, `filter`. Risk: it may lag the newest FFmpeg major; see R1. The `unsafe_code = "warn"` workspace lint stays; the crate isolates required `unsafe` blocks in `src/sys.rs` with `#[allow]` and a safety comment each.

### D2. Minimum libav: FFmpeg 6.1 APIs; verified against 6.1 and the host 9.0
The code uses only APIs present since 6.1 (`sws_scale_frame`, `avformat_index_get_entry_from_timestamp`, `av_display_rotation_get`). CI pins Ubuntu 24.04 packages; a local run on the Arch host covers the newest release. A compile-time check through `ffmpeg_next::util::version()` fails early with a message when the runtime is older.

### D3. Threading model mirrors the C++ prebuffer
One decoder thread owns the libav contexts; the player thread posts `PlayerState` jobs through a `Mutex<Option<Job>>` + `Condvar`. The thread computes a `DecodingJobPlan` (relocation none/advance/seek, pending frame resubmission) exactly as the original, then loops decode → scale → store into the `PreBuffer` from `kinovea-video`. On-demand and caching modes run decode on the caller thread. Alternative: `tokio`; rejected, no async anywhere else.

### D4. Exact seek algorithm
Seek target = requested timestamp minus pre-roll; consult the index for the keyframe at or before it, call `avformat_seek_file(..., BACKWARD)`, flush, then decode until `best_effort_timestamp >= target - tolerance`. Store frames once within `pre_roll` of the target so stepping back works (ported from `ReadFrameAtKeyframe`/`DecodeUntil`).

### D5. Geometry resolution ported as a pure function
`resolve_geometry(info, request) -> VideoGeometry` is pure and unit-tested (reference size from aspect ratio override and rotation, padding to even sizes, pre-scaling decision). The scaler is reconfigured only when the resolved geometry changes.

### D6. Test clips are synthesized, never committed
`tests/support/clip.rs` renders N frames where pixel (0,0..3) encodes the frame index in the blue, green and red channels, writes raw BGRA, and invokes the `ffmpeg` CLI (`-f rawvideo -pix_fmt bgra`) to encode: `ffv1` in MKV (lossless, exact pixels), `libx264 -qp 0 -pix_fmt yuv444p -g 12` in MP4 (lossless with GOPs, exact seek test), the same with `-metadata:s:v rotate=90` through a display matrix (`-display_rotation 90` on FFmpeg ≥ 6.1), and a PNG sequence for the pattern test. Tests skip with an explicit message when `ffmpeg` is absent; CI installs it. Alternative: encoding through the crate itself; rejected because it would test the writer with the reader and hide bugs common to both.

### D7. Writer
`VideoWriter::open(RecordingSettings)`, `save_jpeg(&[u8])`, `save_raw(&FrameBuffer)`, `close()`. Container from extension via `format::output`; MJPEG encoder `yuvj420p`, raw video keeps the incoming format; rotation set through the `rotate` stream metadata and a display matrix side data entry so both old and new readers agree.

### D8. Registry wiring
`kinovea-video-ffmpeg::register(&mut ReaderRegistry)` registers for `*` and for the image-sequence pattern. The crate is a workspace member and a dependency of the `kinovea` binary only, so pure domain crates never link libav.

## Legacy superseded

`Kinovea.Video.FFMpeg/PlayerServer/VideoReaderFFMpeg.cpp`, `VideoReaderFFMpeg.h`, `MJPEGWriter.cpp`, `MJPEGWriter.h`, `SavingContext.h`, `ReadResult.h`, `AssemblyInfo.cpp`, the `.vcxproj` and `Refs/FFmpeg/*` (deleted in `remove-dotnet-solution`).

## Risks / Trade-offs

- [R1 `ffmpeg-next` does not build against FFmpeg 9 on the developer host] → task 1.1 verifies compilation on both targets before any port work; fallback is `rsmpeg` with the same internal `sys.rs` boundary, decided then and recorded in the design.
- [Frame-skipping and prebuffer timing are hard to test deterministically] → the plan/relocation logic is a pure state machine tested with scripted inputs; real playback tests only assert ordering and completeness with skipping disabled.
- [Lossless x264 may not be available on the CI `ffmpeg` build] → fall back to `ffv1` with `-g 12` (FFV1 supports GOP size) when `libx264` is missing; the clip helper probes encoders first.
- [libav version drift breaks CI] → the apt step pins package names only; the version check in D2 makes failures explicit.

## Migration Plan

Additive crate. The `kinovea` binary gains an `--probe <file>` command printing the info, used for manual checks. Rollback: remove the workspace member.

## Open Questions

- Whether to expose VAAPI decoding later; no spec or task depends on it.
