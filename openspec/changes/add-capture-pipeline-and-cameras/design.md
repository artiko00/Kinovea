# Design

## Context

See proposal.md for motivation. Constraints that shape this design:

- The original pipeline (`Kinovea.Pipeline`) is a Disruptor-style ring buffer: the producer claims a slot, copies the camera buffer, commits; consumers run on their own threads and spin or wait on the producer position. `ConsumerDelayer` owns the `Delayer` (frames by age) and the recorder; `ConsumerDisplay` hands the latest frame to the viewport. Drops are counted per consumer.
- Camera backends share `CameraManager` (discovery, summary, thumbnail, configure, create capture source) and `ICaptureSource` (prepare, start, stop, frame-produced event). Camera settings are persisted as "blurbs" in preferences and merged back on discovery.
- The capture screen (`CaptureScreen.cs`, 2300 lines) orchestrates camera load, pipeline connect, delay, recording modes (`CaptureRecordingMode.Delay|Scheduled`), triggers (audio via NAudio, UDP), dynamic path resolution and the captured-files strip.
- Prerequisites available when this change starts: `kinovea-video` writer for MJPEG and raw (change 08), the egui shell and player rendering (14, 15), the KVA model (05) and preferences model (01).
- Linux only: V4L2 for local cameras, Aravis for GenICam, ALSA/PipeWire through `cpal` for the audio trigger.

## Goals / Non-Goals

**Goals:**
- Same observable behavior for delay, recording modes, triggers and file naming as the original, verifiable without hardware.
- Backends are compile-time features; the generator backend is always present and deterministic.
- Pipeline and delay buffer are pure Rust with no UI dependency so the MCP server can reuse them later.

**Non-Goals:**
- Reproducing the lock-free micro-optimizations (cache-line padding) of the original before measurements show they are needed.
- Supporting cameras outside V4L2, HTTP and GenICam.

## Decisions

### D1. Crate split (requires approval: five new crates)
`kinovea-capture` holds the pipeline, delay buffer, recording consumer, triggers, file naming and the camera abstraction traits. Each backend is its own crate (`kinovea-camera-framegen`, `-http`, `-v4l2`, `-genicam`) depending only on `kinovea-capture`, so a backend with heavy native deps (Aravis) never slows down the others and can be excluded by feature. The capture screen UI lives in `kinovea-ui` (owned by change 14) under a `capture` module. Alternative: one crate with features; rejected because `cargo test -p kinovea-capture` would still compile backend deps.

### D2. Ring buffer
`kinovea-capture::pipeline::RingBuffer<Frame>` with a producer cursor (`AtomicU64`), per-consumer cursors and a `Vec<Frame>` of preallocated buffers sized from the image descriptor. Consumers wait with a `parking_lot` condvar after a short spin; the producer never waits (it overwrites the slot of the slowest consumer and increments that consumer's drop counter), matching the original policy. Alternative: `crossbeam` channels; rejected because frames must not be allocated per message and consumers need random access by position for the delay buffer.

### D3. Delay buffer
Port `Delayer` as `DelayBuffer`: capacity from memory budget (`Preferences > Capture > Memory`), `push`, `get_by_age`, `get_by_id`, trigger marks. Frames are stored in the pipeline format; rotation and mirror are applied by the renderer at display time, as in `GetWeakByAge`.

### D4. Recording
`RecordingConsumer` pulls from the delay buffer at the current age and feeds `kinovea_video::writer::FrameWriter` (MJPEG or raw, containers MP4/MKV/AVI from `VideoContainer`). Scheduled mode is a deadline checked per frame. The recorder backlog is reported so the screen can show "recording is falling behind" as the original does.

### D5. Camera abstraction
```rust
pub trait CameraBackend { fn kind(&self) -> &str; fn discover(&mut self, known: &[CameraBlurb]) -> Vec<CameraSummary>; fn thumbnail(&mut self, cam: &CameraSummary) -> Result<RgbImage>; fn open(&mut self, cam: &CameraSummary) -> Result<Box<dyn CaptureSource>>; fn configure(&mut self, cam: &mut CameraSummary, ui: &mut egui::Ui) -> bool; }
pub trait CaptureSource { fn prepare(&mut self) -> Result<ImageDescriptor>; fn start(&mut self, sink: FrameSink); fn stop(&mut self); fn stats(&self) -> SourceStats; }
```
`CameraBlurb` keeps the original XML shape inside `Preferences.xml` so settings migrate. Discovery polling stays at 1 s while the camera list is visible.

### D6. Backends
- FrameGenerator: port of `Kinovea.Camera.FrameGenerator` (frame counter drawn with `kinovea-render`, configurable size/fps, optional noise with a fixed seed). Used by every CI test of the pipeline and screen.
- HTTP: `ureq` (blocking, small) for MJPEG multipart parsing and JPEG polling; decoding with `image`/`turbojpeg` (decision for change 08 reused).
- V4L2: `v4l` crate, mmap streaming, formats MJPEG/YUYV/RGB24; conversion via `kinovea-video` color helpers. Tests run only when `/dev/video*` exists and are `#[ignore]`d otherwise; `v4l2loopback` documented for local runs.
- GenICam: `aravis` crate behind feature `genicam`, dynamic linking to `libaravis-0.8`. Covers exposure, gain, framerate, ROI, Mono8/Bayer8 like the original `CameraManagerGenICam`.

### D7. Triggers
Audio level via `cpal` (default input device, RMS over 20 ms windows, threshold in dB as in `AudioInputLevelMonitor`), UDP via `std::net::UdpSocket` on a background thread. `QuietPeriodHelper` port keeps the same semantics.

### D8. File naming
Port `DynamicPathResolver` and the variables repository (profile CSV) into `kinovea-capture::naming`. This is pure string logic and is the first module written, with a FixtureGen area `capture-naming` producing expected resolutions for fixed dates.

### D9. Legacy retirement in this change
The five machine-vision plugin projects are not referenced by `Kinovea.csproj`; they are built by the `.sln` only to be dropped in `Plugins/Camera`. After the GenICam spec exists, delete `Kinovea.Camera.Basler`, `Kinovea.Camera.Baumer`, `Kinovea.Camera.Daheng`, `Kinovea.Camera.IDS`, `Kinovea.Camera.GenICam`, remove them from `Kinovea.VS2019.sln`, remove their entries from the linter `exclude` lists, and confirm `build.yml` still passes.

## Legacy superseded

`Kinovea.Pipeline/*` (RingBuffer, FramePipeline, Frame, ImageDescriptor, Consumers/*), `Kinovea.Camera/*` (CameraManager, CameraSummary, CameraTypeManager, Finishline, LogarithmicMapper, IconLibrary, FormHandshakeResult), `Kinovea.Camera.FrameGenerator/*`, `Kinovea.Camera.HTTP/*`, `Kinovea.Camera.DirectShow/*`, `Kinovea.ScreenManager/CaptureScreen/*` (CaptureScreen, Delayer, ConsumerDelayer, ConsumerDisplay, PipelineManager, Trigger/*, CapturedFile(s), ViewportController, ImageManipulator, MetadataManipulator, MetadataRenderer), `Kinovea.ScreenManager/Variables/DynamicPathResolver.cs`, `VariablesRepository.cs`, `Kinovea.Services/Types/Capture/*`, `Kinovea.Services/Preferences/CapturePreferences.cs`. UI text is Fluent, not `.resx`; see `add-i18n-fluent`.

## Risks / Trade-offs

- [Aravis not installed on the developer machine] → feature off by default; CI builds with the feature on a job that installs `libaravis-0.8-dev` to catch compile errors only.
- [V4L2 behavior untestable in CI] → unit-test the format negotiation against recorded `VIDIOC_ENUM_FMT` tables; keep one ignored end-to-end test with `v4l2loopback`.
- [Encoder slower than camera] → same policy as the original: delay buffer absorbs the backlog, drops are counted and shown; documented in the recording spec.
- [Deleting plugin projects breaks the Windows CI build] → delete in a dedicated commit after `build.yml` passes on the preceding commit; revert if it fails.

## Migration Plan

1. Pipeline, delay buffer, naming (pure, fixture-tested). 2. Camera traits and FrameGenerator. 3. Recording consumer with the writer. 4. HTTP, V4L2, GenICam backends. 5. Capture screen in `kinovea-ui`. 6. Delete the five plugin projects. Rollback: each step is additive except step 6, which is one revertible commit.

## Open Questions

- Whether `turbojpeg` or `image` decodes MJPEG fast enough for 1080p60 on the target machines; measure in step 4 and switch if needed. Does not change specs or tasks.
