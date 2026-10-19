# Tasks

## 1. Core types

- [x] 1.1 Write `rust/kinovea-video/src/info_test.rs` for `VideoInfo` (empty constructor, single-frame detection, frame interval from fps) and `Fraction::is_empty`; implement `src/info.rs`; verify `cargo test -p kinovea-video` passes.
- [x] 1.2 Write `src/capabilities_test.rs` (bitflag combinations, convenience accessors, `can_switch_decoding_mode` matrix) then implement `src/capabilities.rs` with `VideoCapabilities`, `VideoDecodingMode`, `CacheLoadMode`; verify tests pass.
- [x] 1.3 Write `src/state_test.rs` for `PlayerState` (clone, invalid state, display) and `PlayerAction`, `WorkingZoneRequest`, `OpenVideoResult`, `RecordingResult`, `VideoExportResult`, `SeekProgress`; implement `src/state.rs`; verify tests pass.
- [x] 1.4 Write `src/frame_test.rs` for `FrameBuffer` (BGRA8, stride, `Arc` sharing, `pixel(x, y)`) and `VideoFrame::empty`; implement `src/frame.rs`; verify tests pass.
- [x] 1.5 Write `src/geometry_test.rs` for `VideoGeometry`, `VideoGeometryRequest`, reference size under rotation and `FitHelper` (fit a size into a box keeping ratio) then implement `src/geometry.rs`; verify tests pass and `cargo clippy -p kinovea-video --all-targets -- -D warnings` is clean.

## 2. Reader contract

- [x] 2.1 Write `src/reader_test.rs` with a `SyntheticReader` (configurable fps, frames, timestamps per frame; each frame paints its timestamp in the first pixels) and tests for `relate_timestamps` (all five relations plus unknown), `playback_timestamp`, `move_to`/`move_next`, end-of-zone; implement `src/reader.rs` (`VideoReader` trait with default methods, `RequestFulfilled` channel); verify tests pass.
- [x] 2.2 Write `src/enumerate_test.rs` for interval 0 and positive interval enumeration against `SyntheticReader`; implement `enumerate_frames` as an iterator in `src/reader.rs`; verify tests pass.
- [x] 2.3 Write `src/summary_test.rs` (thumbnail count and bound, companion KVA detection in a temp dir) then implement `src/summary.rs`; verify tests pass.
- [x] 2.4 Write `src/registry_test.rs` (case-insensitive lookup, wildcard fallback, `is_supported`, most-recent supported file in a temp dir) then implement `src/registry.rs`; verify tests pass.

## 3. Dense cache

- [x] 3.1 Write `src/cache_test.rs` covering add/duplicate, closest with ties, next at end, trim protecting current, clear, snapshot versions and working zone; implement `src/cache.rs` and the shared `src/closest.rs` helpers with their own `closest_test.rs`; verify tests pass.

## 4. Prebuffer

- [x] 4.1 Write `src/prebuffer_test.rs` for single-threaded behavior: capacity, retention on acquire closest and next, contiguity, relate timestamp, try acquire for the four player actions, purge, shutdown, snapshot; implement `src/prebuffer.rs` (`Mutex` + `Condvar`) porting `EvictBehind`, `EvictNonContiguous`, `EvictOne` arithmetic; verify tests pass.
- [x] 4.2 Write `tests/prebuffer_concurrency_test.rs`: producer thread blocked on a full buffer unblocks after consumer acquire; interrupt returns `Interrupted` within 1 s and the frame is absent; forced add never blocks; run with `cargo test -p kinovea-video --test prebuffer_concurrency_test -- --test-threads=1` ten times in a loop to check for flakiness; verify all runs pass.

## 5. Parity with the original containers

- [x] 5.1 Add `tests/cases/videocache/{cache-basic,prebuffer-eviction,prebuffer-tryacquire,prebuffer-forceadd}.json` with `inputs.container`, `inputs.capacity`, `inputs.retention`, `inputs.tsPerFrame`, `inputs.ops`; document the op grammar in `tests/README.md`; verify `openspec validate` is unaffected and `npm run lint` passes.
- [x] 5.2 Implement `Kinovea.FixtureGen/Areas/VideoCacheArea.cs` (no-op disposer, null bitmaps, `steps.csv` with `op,result,current,snapshot`), register it in `Program.cs`; run `tools/oracle/fixtures.sh --verify`; verify the four fixture directories appear with identical trees on the second run.
- [x] 5.3 Write `tests/videocache_parity_test.rs` that loads every `tests/fixtures/videocache/*/steps.csv` through `kinovea-fixtures`, replays the ops on the Rust containers and asserts each row exactly; verify `cargo test -p kinovea-video` passes and fails when one expected snapshot is edited in a temp copy.

## 6. Frame pipeline

- [x] 6.1 Write `src/pipeline/ring_buffer_test.rs` (power-of-two check, claim/commit, refused claim and drop count, inactive consumer, wait-for barrier with a thread) then implement `src/pipeline/ring_buffer.rs` with atomics only; verify tests pass.
- [x] 6.2 Write `src/pipeline/frequency_test.rs` (steady 50 Hz within 5 percent, window reset) then implement `src/pipeline/frequency.rs`; verify tests pass.
- [x] 6.3 Write `src/pipeline/consumer_test.rs` (started/active/position, deactivate and reactivate skipping slots, stop) then implement `src/pipeline/consumer.rs` (`FrameConsumer` trait + `BatchConsumer` runner) and `src/pipeline/producer.rs` (`FrameProducer` trait, `Frame`, `ImageDescriptor::compatible`); verify tests pass.
- [x] 6.4 Write `tests/pipeline_test.rs`: a scripted producer at 200 frames with a slow consumer and a fast consumer, asserting drop count, order of processed positions and measured frequency; implement `src/pipeline/mod.rs` (`FramePipeline::new`, `teardown`); verify the test passes.

## 7. Integration

- [x] 7.1 Replace `crate_name()` placeholder in `rust/kinovea-video/src/lib.rs` with the public module tree and crate docs listing the superseded C# files; update `rust/Cargo.toml` dependency of `kinovea-video` on `kinovea-core`; verify `cd rust && make verify` exits 0 and `npx testlens check` reports no untested Rust file.
- [x] 7.2 Append the session entry to `.agent/progress.md` (what was ported, fixture area added, flaky-test loop result) and tick `07` in `openspec/roadmap.md` after archive; verify `npm run lint` passes.

## Workflow follow-up

- Archive the change after review; the main specs `video/reader-contract`, `video/frame-cache`, `video/frame-pipeline` are created by the sync.
