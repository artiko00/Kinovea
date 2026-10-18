# Tasks

## 1. Crate and toolchain (approval checkpoints)

- [ ] 1.1 After approval of `ffmpeg-next` and libav dev packages: add `rust/kinovea-video-ffmpeg` to `rust/Cargo.toml` members with `ffmpeg-next` and a dependency on `kinovea-video`; write `src/version_test.rs` asserting the reported libav versions are at least the FFmpeg 6.1 ones; implement `src/version.rs`; verify `cargo test -p kinovea-video-ffmpeg` passes on the host (FFmpeg 9) and record the result; if the crate fails to build, stop and record the `rsmpeg` fallback decision in design.md before continuing.
- [ ] 1.2 After approval: add an apt step (`ffmpeg libavcodec-dev libavformat-dev libavutil-dev libswscale-dev libavfilter-dev libavdevice-dev pkg-config clang`) to `.github/workflows/rust.yml` and document the packages in `AGENTS.md` setup; verify the workflow passes on a branch push.
- [ ] 1.3 Write `tests/support/clip_test.rs` and implement `tests/support/clip.rs` (frame-index encoding, raw BGRA writer, `ffmpeg` CLI invocation for `ffv1`/MKV, lossless `libx264` or `ffv1` with GOP 12, rotated variant, PNG sequence; skip with message when `ffmpeg` is missing); verify the helper round-trips the index of frame 0 and frame N-1 through `ffprobe -show_frames` count.

## 2. Open and probe

- [ ] 2.1 Write `src/probe_test.rs` using the synthesized clips: fps, duration, first/last timestamp, size, rotation, SAR, missing-file failure; implement `src/probe.rs` (`open_format`, `select_video_stream`, `guess_frame_rate`, `compute_timing`, `read_rotation`); verify tests pass.
- [ ] 2.2 Write `src/geometry_test.rs` for `resolve_geometry` (aspect override, rotation transposition, even padding, pre-scaling decision, stabilization flag) then implement `src/geometry.rs`; verify tests pass.

## 3. Decode, scale, seek

- [ ] 3.1 Write `src/decoder_test.rs`: decode the first 5 frames of the `ffv1` clip and assert the pixel-encoded indices; implement `src/decoder.rs` (software decoder init with `skip_loop_filter`, `decode_next`, `best_effort_timestamp` mapping) and `src/scaler.rs` (BGRA conversion through `sws_scale_frame`, reconfigure on geometry change); verify tests pass.
- [ ] 3.2 Write `src/seek_test.rs`: request frames 17, 5, 49 and 0 in the GOP-12 clip and assert exact indices; implement `src/seek.rs` (index lookup, backward seek, flush, decode-until with tolerance and pre-roll); verify tests pass.
- [ ] 3.3 Write `src/on_demand_test.rs` implementing `VideoReader` in on-demand mode (`move_to`, `move_next`, step backward after a seek, end of zone); implement `src/reader.rs` with the `SingleFrame` container path; verify tests pass.

## 4. Prebuffering and caching

- [ ] 4.1 Write `src/plan_test.rs` for the pure job planner (relocation none/advance/seek from the cache relation, pending-frame resubmission, fulfilled-in-planning) then implement `src/plan.rs` ported from `PlanRelocation`; verify tests pass.
- [ ] 4.2 Write `tests/prebuffer_mode_test.rs`: start prebuffering on the 50-frame clip, request timestamp 20 then step forward 10 times, assert every delivered index and that the prebuffer snapshot stays contiguous; implement the decoder thread in `src/prebuffer_thread.rs` (job handoff, decode loop, store into `kinovea_video::PreBuffer`, interruption); verify the test passes 10 consecutive runs.
- [ ] 4.3 Write `tests/caching_mode_test.rs`: working zone of 20 frames fits a 64 MB budget and switches to caching; a 1 MB budget refuses and stays prebuffering; implement `working_zone_update_request`, `working_zone_memory_requirement` and `change_caching_mode`; verify the test passes.
- [ ] 4.4 Write `src/skipping_test.rs` for the frame-skipping policy state machine (lag thresholds → decode all / skip scaling / skip non-ref / seek ahead; disabled policy) then implement `src/skipping.rs` and wire it into the playback branch of the decoder thread; verify tests pass.

## 5. Summary and sequences

- [ ] 5.1 Write `src/summary_test.rs` (4 thumbnails from 100 frames in increasing order, reader unloaded afterwards, bounded size) then implement `extract_summary`; verify tests pass.
- [ ] 5.2 Write `src/sequence_test.rs` opening the PNG sequence pattern and asserting 10 frames at the configured fps; implement the image2 demuxer options in `src/probe.rs`; verify tests pass.

## 6. Recording

- [ ] 6.1 Write `src/writer_test.rs`: open MJPEG/MKV at 25 fps, save 25 raw frames, close, re-open with the reader and assert duration and frame count; uncompressed round trip asserts identical pixels; implement `src/writer.rs` (`VideoWriter`, container guess, MJPEG/raw encoder setup, BGRA→YUVJ420P conversion, timing); verify tests pass.
- [ ] 6.2 Write `src/writer_jpeg_test.rs` (JPEG passthrough: encode frames with the `image` crate's JPEG encoder in the test only, save, assert packet count) and `src/writer_rotation_test.rs` (rotation metadata read back as 90 degrees); implement `save_jpeg` and rotation side data; verify tests pass.

## 7. Integration

- [ ] 7.1 Implement `register(&mut ReaderRegistry)` for `*` and the sequence pattern with `src/lib_test.rs`; add a `--probe <file>` subcommand to `rust/kinovea/src/main.rs` with `src/main_test.rs` coverage of argument handling; verify `cd rust && make verify` exits 0.
- [ ] 7.2 Update `AGENTS.md` (tech stack line for libav, setup packages), `.agent/progress.md` (libav versions tested, encoder fallbacks observed) and tick `08` in `openspec/roadmap.md` after archive; verify `npm run lint` passes.

## Workflow follow-up

- Archive after review; sync creates `video/ffmpeg-decoding` and `video/video-recording` main specs.
