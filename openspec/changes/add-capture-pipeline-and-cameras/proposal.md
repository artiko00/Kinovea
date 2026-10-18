# Proposal

## Why

Capture is the half of Kinovea that is entirely Windows-bound today: DirectShow for webcams, five vendor SDKs for machine-vision cameras, NAudio for the audio trigger, and a WinForms capture screen. None of it compiles or runs on Linux. The player side (changes 01 to 16) gives the Rust fork a working viewer; this change makes the fork able to acquire video, so that the Linux build covers the full capture-then-analyze workflow Kinovea is used for and the MCP server can later record from a live camera.

## What Changes

- Add the capture pipeline in Rust: a lock-free ring buffer with one producer and N consumers, the display consumer, the delay buffer (`Delayer`) and the recording consumer writing MJPEG or raw frames through the writer introduced in `add-ffmpeg-video-reader`.
- Add the camera abstraction (summary, discovery, thumbnail, configuration, capture source) and four backends: FrameGenerator (deterministic, used by tests), HTTP MJPEG/JPEG, V4L2 (replaces DirectShow) and GenICam through Aravis (replaces Basler, Baumer, Daheng, IDS and GenICam plugins).
- Add the capture screen on top of the `egui` shell from `add-ui-shell-vertical-slice` and `add-player-screen-rendering`: live view, delay slider, recording controls, snapshot, dynamic file naming with variables, audio and UDP triggers, scheduled and delay recording modes.
- Retire the five machine-vision C# plugin directories, which are leaves of the solution, and remove their linter exclude entries.

## Capabilities

### New Capabilities
- `capture/frame-pipeline`: ring buffer, consumers, drop accounting and delay buffer behavior.
- `capture/camera-backends`: camera discovery, configuration and frame production across FrameGenerator, HTTP, V4L2 and GenICam.
- `capture/recording`: recording modes, triggers, snapshot, file naming and capture folders.
- `capture/capture-screen`: the capture screen UI and its controls.

### Modified Capabilities
None.

## Impact

- **Assemblies superseded**: `Kinovea.Pipeline`, `Kinovea.Camera`, `Kinovea.Camera.DirectShow`, `Kinovea.Camera.HTTP`, `Kinovea.Camera.FrameGenerator`, `Kinovea.ScreenManager/CaptureScreen/*`. Deleted in this change: `Kinovea.Camera.Basler`, `Kinovea.Camera.Baumer`, `Kinovea.Camera.Daheng`, `Kinovea.Camera.IDS`, `Kinovea.Camera.GenICam` (not referenced by the `.sln` build of the main executable; verify before deleting).
- **New crates** (decision requiring approval): `kinovea-capture`, `kinovea-camera-framegen`, `kinovea-camera-http`, `kinovea-camera-v4l2`, `kinovea-camera-genicam`. New dependencies: `v4l`, `aravis` (feature-gated), `cpal` for the audio trigger, an HTTP client.
- **Verification**: Rust side on Linux with `make verify`; CI uses FrameGenerator and a local HTTP test server. V4L2 is exercised only by an ignored test that needs `v4l2loopback`. GenICam has no CI coverage beyond unit tests of its mapping layer. The C# CI build must still pass after deleting the plugin projects.
- **KVA**: unchanged. Capture metadata (linked annotations) continue to be saved as KVA 2.0 next to recordings.

## Non-goals

- Windows backends (DirectShow, vendor SDKs) or Windows as a target.
- Audio recording (the original records video only).
- Hardware-accelerated encoding; MJPEG and raw only, as in the original.
- Camera plugin loading from disk at run time; backends are compiled in behind feature flags.
