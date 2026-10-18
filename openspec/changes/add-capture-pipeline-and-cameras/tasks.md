# Tasks

## 1. Crates and pure modules

- [ ] 1.1 Add crate `rust/kinovea-capture` to the workspace (`rust/Cargo.toml`) with `lib.rs` and `src/lib_test.rs`; record the approval for the new crates in `.agent/progress.md`; verify `cd rust && make verify`.
- [ ] 1.2 Add FixtureGen area `capture-naming` (`Kinovea.FixtureGen/Areas/CaptureNamingArea.cs`, cases `tests/cases/capture-naming/*.json` with fixed date, camera alias, pattern, existing files) calling `DynamicPathResolver.Resolve` and `ComputeNextFilename`; run `tools/oracle/fixtures.sh --verify`; verify fixtures committed and smoke test green.
- [ ] 1.3 Write `src/naming_test.rs` reading the `capture-naming` fixtures, then implement `src/naming.rs` (variables, user profile CSV, counter increment); verify `cargo test -p kinovea-capture naming`.
- [ ] 1.4 Write `src/pipeline/ring_buffer_test.rs` (ordering, drop accounting with a slow consumer, descriptor compatibility), then implement `src/pipeline/ring_buffer.rs`, `frame.rs`, `descriptor.rs`; verify tests and `cargo test -- --test-threads=1` stability over 20 runs (`scripts` not needed, use a shell loop).
- [ ] 1.5 Write `src/delay_test.rs` (age lookup, beyond-content, capacity from budget, trigger marks), then implement `src/delay.rs`; verify `cargo test -p kinovea-capture delay`.
- [ ] 1.6 Write `src/pipeline/stats_test.rs` (frequency estimate, backlog) and implement `src/pipeline/stats.rs`; verify tests pass.

## 2. Camera abstraction and FrameGenerator

- [ ] 2.1 Write `src/camera/summary_test.rs` (blurb XML round-trip against `Preferences.xml` samples, merge of remembered settings on discovery) and implement `src/camera/{summary.rs,blurb.rs,backend.rs}` with the `CameraBackend`/`CaptureSource` traits; verify tests.
- [ ] 2.2 Add crate `rust/kinovea-camera-framegen`; write `src/generator_test.rs` (frame count at 60 fps for 1 s, counter drawn, configurable descriptor) then implement `src/generator.rs` using `kinovea-render`; verify `cargo test -p kinovea-camera-framegen`.
- [ ] 2.3 Write `rust/kinovea-capture/tests/pipeline_e2e_test.rs` wiring FrameGenerator → ring buffer → display and delay consumers for 300 frames and asserting delay-by-age; verify it passes in under 5 s.

## 3. Recording and triggers

- [ ] 3.1 Write `src/recording/recorder_test.rs` (Delay mode first frame age, Scheduled stop, MJPEG MKV reopened by `kinovea-video` with frame count and fps) then implement `src/recording/recorder.rs` over `kinovea_video::writer`; verify tests.
- [ ] 3.2 Write `src/recording/snapshot_test.rs` (JPEG/PNG/BMP of the delayed frame) and implement `src/recording/snapshot.rs`; verify tests.
- [ ] 3.3 Write `src/trigger/quiet_period_test.rs` and `src/trigger/udp_test.rs` (datagram on loopback triggers once, quiet period respected), implement `src/trigger/{quiet_period.rs,udp.rs}`; verify tests.
- [ ] 3.4 Add `cpal` (approval noted) and implement `src/trigger/audio.rs` with `src/trigger/audio_test.rs` testing the RMS/threshold logic on synthetic PCM buffers (no device); verify tests and `make verify`.
- [ ] 3.5 Write `src/recording/captured_files_test.rs` (append on finish, thumbnail path) and implement `src/recording/captured_files.rs`; verify tests.

## 4. HTTP, V4L2 and GenICam backends

- [ ] 4.1 Add crate `rust/kinovea-camera-http`; write `tests/mjpeg_test.rs` serving a multipart MJPEG stream from a local `std::net::TcpListener` and a JPEG snapshot endpoint, then implement `src/{mjpeg.rs,snapshot.rs,backend.rs}` with basic auth; verify tests and unreachable-URL error path.
- [ ] 4.2 Add crate `rust/kinovea-camera-v4l2` with dependency `v4l`; write `src/formats_test.rs` on recorded enumeration tables (identifier from bus path, format/resolution/fps selection persisted), implement `src/{formats.rs,backend.rs,source.rs}`; add `tests/loopback_test.rs` marked `#[ignore]` documenting `v4l2loopback`; verify `cargo test -p kinovea-camera-v4l2`.
- [ ] 4.3 Add crate `rust/kinovea-camera-genicam` behind feature `genicam` with dependency `aravis`; write `src/convert_test.rs` (Mono8 and BayerRG8 to RGB on synthetic buffers) and implement `src/{convert.rs,backend.rs}`; add a CI job step installing `libaravis-0.8-dev` and running `cargo check --features genicam` (workflow edit requires approval); verify both default and feature builds compile.
- [ ] 4.4 Register backends in `kinovea` bin behind features `camera-v4l2`, `camera-http`, `camera-genicam` (framegen always on); verify `kinovea --list-cameras` prints the generator camera in CI.

## 5. Capture screen (in `kinovea-ui`)

- [ ] 5.1 Write `rust/kinovea-ui/src/capture/screen_test.rs` with `egui_kittest`: open generator camera from the camera list, header shows alias and fps; implement `src/capture/screen.rs`; verify test.
- [ ] 5.2 Write `src/capture/delay_control_test.rs` (slider sets 90 frames at 30 fps and 3 s) and implement `src/capture/delay_control.rs`; verify test.
- [ ] 5.3 Write `src/capture/controls_test.rs` (record toggle state and elapsed time, snapshot, arm trigger, shortcuts from Preferences > Keyboard) and implement `src/capture/controls.rs`; verify test.
- [ ] 5.4 Write `src/capture/annotations_test.rs` (line drawn over live image saved as KVA 2.0 next to the recording when enabled) and implement reuse of the player drawing tools over the live frame; verify the produced KVA validates against `kva.2.0.xsd`.
- [ ] 5.5 Write `src/capture/dual_test.rs` (joint record starts both screens within one frame interval) and implement joint controls for dual capture; verify test.
- [ ] 5.6 Add Preferences > Capture tabs (Paths, Recording, Trigger, Memory) in `src/preferences/capture.rs` with `capture_test.rs` round-tripping to the preferences model; add English Fluent strings for new UI text per `add-i18n-fluent`; verify `make verify`.

## 6. Legacy retirement and integration

- [ ] 6.1 Confirm no project outside the five machine-vision plugins references them (`grep` over `*.csproj`, `Kinovea.targets`, `Installer/kinovea.nsi`); verify the grep output is empty except the plugin folders themselves.
- [ ] 6.2 Delete `Kinovea.Camera.Basler`, `Kinovea.Camera.Baumer`, `Kinovea.Camera.Daheng`, `Kinovea.Camera.IDS`, `Kinovea.Camera.GenICam`, remove their entries from `Kinovea.VS2019.sln`, from `package.json` and `testlens.json` excludes, and from `architecture.md`; verify `npm run lint` is green and the Windows `build.yml` passes on the push.
- [ ] 6.3 Update `AGENTS.md` (structure, features, new crates) and append a `.agent/progress.md` entry; verify `cd rust && make verify` and `npm run lint` both exit 0.

## Workflow follow-up

- Archive the change after review; tick 17 in `openspec/roadmap.md`.
