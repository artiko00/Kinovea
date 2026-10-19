# Design

## Context

See proposal.md for motivation. Constraints:

- The C# layer is `Kinovea.Video` (3.7k LOC: `VideoReader`, `Cache`, `PreBuffer2`, `VideoTypeManager`, geometry and state types) and `Kinovea.Pipeline` (1.1k LOC: `RingBuffer`, `FramePipeline`, `AbstractConsumer`). Frames are `System.Drawing.Bitmap` in `Format32bppPArgb`; the FFmpeg plugin keeps native buffers alive through the bitmap `Tag`.
- `PreBuffer2` is filled by the decoder thread and read by the player thread; it uses a lock, `Monitor.Wait/Pulse` for blocking adds and a volatile snapshot for lock-free reads of the cached sections.
- `kinovea-core` (change 01) provides `VideoSection`, `TimedPoint`, `ImageRotation`, `ImageAspectRatio`, `Demosaicing`, `Fraction`.
- The oracle cannot run real decoders deterministically across machines, but the containers are pure logic and can be replayed from scripts.

## Goals / Non-Goals

**Goals:**
- A `VideoReader` trait that the FFmpeg, image and synthetic readers implement without the player knowing which one is behind.
- Containers whose behavior is byte-for-byte equivalent to `Cache` and `PreBuffer2` on scripted sequences.
- Frame buffers that later changes can hand to the renderer and to FFmpeg without copies.

**Non-Goals:**
- GPU buffers, zero-copy from hardware decoders.
- The player clock loop itself (lives in the player screen change).

## Decisions

### D1. Frame image type
`kinovea-video::FrameBuffer { width, height, stride, data: Arc<[u8]> }` in BGRA8, row-major, top-down. Alternative: `image::RgbaImage` (forces the `image` dependency into every crate and RGBA ordering). BGRA matches `Format32bppArgb` and FFmpeg `AV_PIX_FMT_BGRA`, so the decoder writes directly and the renderer (`tiny-skia`, `egui`) reads with one swizzle at most. Premultiplication is dropped: the C# used PArgb only for GDI+ speed. The disposer delegate becomes `Arc` drop.

### D2. `VideoReader` is a trait with default methods
Abstract members of the C# class become required trait methods; the concrete helpers (`relate_timestamps`, `playback_timestamp`, `can_switch_decoding_mode`, `enumerate_frames`, `move_to`, `move_next`) become default methods over an `info()` accessor. The `RequestFulfilled` event becomes a `std::sync::mpsc::Sender<RequestFulfilled>` the player installs through `set_request_sink`. Alternative: async trait; rejected because the C# model is thread plus callbacks and the egui UI is synchronous.

### D3. Containers mirror the C# split
`Cache` (dense, single-threaded, `BTreeMap<i64, VideoFrame>`) and `PreBuffer` (`Mutex<State>` + `Condvar` for blocking adds, `ArcSwap`-free snapshot via `Mutex<Arc<CacheSnapshot>>` read with a short lock). Both implement `FramesContainer` (the C# `IVideoFramesContainer`). Shared helpers (`find_closest`, `lower_bound`, `relate_timestamp`) live in one module instead of being duplicated as in C#. `Shutdown` on `Cache` is `NotImplemented` in C#; in Rust it clears.

### D4. Eviction rules are ported verbatim
`evict_behind`, `evict_non_contiguous`, `evict_one` keep the exact pivot and retention arithmetic of `PreBuffer2` so the `videocache` fixtures match. Defaults: capacity 32, retention 8, tolerance and far-ahead threshold set by the reader from `average_timestamps_per_frame`.

### D5. Pipeline without `unsafe`
`RingBuffer` slots are `Vec<Frame>` behind `Arc`; producer position is an `AtomicI64`; consumer positions are `AtomicI64` read through the `FrameConsumer` trait. `FrameProducer` becomes a trait with a `FrameProduced` channel. The benchmark modes (`Bradycardia`, `FrameDrops`) are not ported. The frequency counter is a small ring of `Instant`s.

### D6. Registry
`ReaderRegistry` holds `Vec<(Vec<String>, Box<dyn Fn() -> Box<dyn VideoReader>>)>` registered explicitly at startup (the C# reflection scan is already obsolete there). Extension matching is case-insensitive; `*` is the wildcard.

### D7. Oracle area `videocache`
`Kinovea.FixtureGen/Areas/VideoCacheArea.cs` builds `Cache`/`PreBuffer2` with a no-op `VideoFrameDisposer` and null bitmaps, replays `inputs.ops` (`add t`, `forceAdd t`, `acquireClosest t`, `acquireNext`, `tryAcquire action ref`, `trim a b`, `purge`, `interrupt`) and writes `steps.csv` (`op, result, current, snapshot`). Blocking adds are scripted only when the buffer is not full so the replay stays single-threaded.

### D8. Dependencies
None beyond `std` and workspace crates. `arc-swap` was considered for the snapshot and rejected: a `Mutex` held for a pointer copy is sufficient. Any future addition is a decision requiring approval.

## Deviations recorded during apply

- Prebuffer frames live in a sorted `Vec` (not a `BTreeMap`): the eviction arithmetic indexes like `SortedList`.
- `PreBuffer::clear` empties the buffer where `PreBuffer2.Clear` throws `NotImplementedException`, like `Cache::shutdown` (D3); the fixture replay reports the original exception. Producers blocked in `add` are woken only where the original pulses (add, acquire with eviction, interrupt, shutdown), not after purge, forced add or try-acquire.
- The replay maps a blocking `add` to `try_add` (no state change when it would wait); the oracle records `Blocked` once its worker sits in `Monitor.Wait`. The blocking path itself is covered by the concurrency tests.
- `playback_timestamp` takes the elapsed time instead of reading a stored epoch; companion KVA detection is case-sensitive on Linux; `most_recent_supported` breaks mtime ties by name (the order of `GetFiles` is unspecified).
- `EvictNonContiguous` is not ported: its only call is commented out in `PreBuffer2.TryAcquire`.
- Two pipeline scenarios were corrected to the original: a consumer that has read nothing is at -1, and the measured frequency needs more than 48 intervals.

## Legacy superseded

`Kinovea.Video/VideoReaders/VideoReader.cs`, `VideoTypeManager.cs`, `VideoCapabilities.cs`, `SupportedExtensionsAttribute.cs`, `IFrameGenerator.cs`, `CapabilityNotSupportedException.cs`; `Kinovea.Video/Caching/*` (`Cache.cs`, `PreBuffer2.cs`, `SingleFrame.cs`, `VideoFrame.cs`, `PlayerState.cs`, `PlayerAction.cs`, `CacheSnapshot.cs`, `TryAcquireResult.cs`, `DecodingJobPlan.cs`, `DecoderRelocation.cs`, `CacheTimestampRelation.cs`, `FrameSkippingPolicy.cs`, `VideoDecodingMode.cs`, `CacheLoadMode.cs`, `SeekProgress.cs`, `ThreadCanceler.cs`, `IVideoFramesContainer.cs`, `IWorkingZoneContainer.cs`, `RequestFulfilledEventArgs.cs`, `CacheAddResult.cs`); `Kinovea.Video/VideoGeometry/*`, `VideoInfo.cs`, `VideoSummary.cs`, `Fraction.cs`, `FitHelper.cs`, `WorkingZoneRequest.cs`, `OpenVideoResult.cs`, `VideoExportResult.cs`, `VideoExportSettings.cs`, `RecordingResult.cs`, `RecordingSettings.cs`, `Delegates.cs`; all of `Kinovea.Pipeline/`.

## Risks / Trade-offs

- [Deadlock between blocking add and interrupt] → `Condvar::wait_timeout` with a generous timeout in tests and a loom-free but stress-tested interrupt path; a test spawns a producer and interrupts from the consumer side.
- [Snapshot read contention on the player thread] → snapshot is an `Arc` swapped under the lock; the player clones the `Arc` only.
- [Fixture replay cannot cover the blocking paths] → those are covered by Rust concurrency tests; the fixture covers the deterministic state machine.
- [BGRA assumption leaks into `kinovea-render`] → the renderer accepts `FrameBuffer` through one conversion function so a future pixel format change touches one place.

## Migration Plan

Additive: the crate grows from its placeholder. No behavior of the C# application changes. Rollback is deleting the new modules.

## Open Questions

- Whether `FrameBuffer` should carry a pixel-format tag now; deferred until the FFmpeg reader needs anything other than BGRA8.
