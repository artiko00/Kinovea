# Tasks

## 1. Crate and transport (approval: new crate, `rmcp`, `tokio`, `schemars`, `base64`)

- [ ] 1.1 Add `rust/kinovea-mcp` to the workspace with the dependencies pinned in `rust/Cargo.toml`, `src/lib.rs` exposing `run_stdio()` and `run_http(addr, allow_remote)`, and `src/lib_test.rs` asserting the server info (name `kinovea`, crate version); verify `cd rust && make verify`.
- [ ] 1.2 Write `rust/kinovea-mcp/tests/transport_test.rs` driving an in-process `rmcp` client over duplex pipes: `initialize` lists `tools` and `resources` capabilities, `tools/list` returns a non-empty list with schemas, EOF on the input closes the server; implement `src/server.rs` with the tool router skeleton; verify the test passes.
- [ ] 1.3 Write `src/error_test.rs` for `McpError` mapping (`invalid_params`, `not_found`, `unsupported_media`, `external_tool_missing`, `internal`) and panic capture via `catch_unwind` in the dispatcher; implement `src/error.rs`; verify.
- [ ] 1.4 Add the `mcp` subcommand (`--http <addr>`, `--allow-remote`) to `rust/kinovea/src/cli.rs` with tests in `cli_test.rs` (non-loopback without flag → exit 2); verify `make verify`.

## 2. Sessions and video tools

- [ ] 2.1 Write `src/session_test.rs` for `SessionStore` (create, lookup error, close releases the reader, ids unique) using a fake `VideoReader` from `kinovea-video`; implement `src/session.rs`; verify.
- [ ] 2.2 Write `src/tools/video_test.rs` for `open_video` (sidecar KVA detection, `unsupported_media`), `describe_session`, `list_drawings` (time filter), `seek` (clamping), `close_session` against a synthetic reader and `Tools/XML/Schema/samples/complete.kva`; implement `src/tools/video.rs`; verify.
- [ ] 2.3 Write `src/tools/render_test.rs` for `render_frame` (PNG size, `scale`, `with_drawings=false`) and `render_frame_to_file`, comparing with the `kinovea-render` snapshot helper; implement `src/tools/render.rs`; verify.
- [ ] 2.4 Write `src/tools/save_test.rs` for `save_kva` (default sidecar path, explicit path, reload round trip through `kinovea-metadata`); implement `src/tools/save.rs`; verify.
- [ ] 2.5 Write `src/resources_test.rs` for `kinovea://session/{id}/document.kva`, `.../measurements.json` and `kinovea://tools/postures` (lists `OpenPoseBody25` with 25 points, `BodyWithFeet` with 26); implement `src/resources.rs`; verify.

## 3. Annotation tools

- [ ] 3.1 Write `src/tools/import_test.rs` for `import_skeleton` (openpose, trc, wrong format → `invalid_params`) and `import_subtitles` using `tests/cases/import/data/`; implement `src/tools/import.rs` over `kinovea-export`; verify.
- [ ] 3.2 Write `src/tools/drawing_test.rs` for `add_drawing` (each core type, keyframe creation, partial posture points, unknown type lists supported ones), `update_drawing`, `delete_drawing` (`not_found`), `set_keyframe`; implement `src/tools/drawing.rs`; verify.
- [ ] 3.3 Write `src/tools/history_test.rs` for `undo`/`redo` across import and drawing mutations; implement `src/tools/history.rs` over the document history stack; verify.

## 4. Analysis tools

- [ ] 4.1 Write `src/tools/calibration_test.rs` for `calibrate` line and plane and `set_origin` (200 px as 1 m → 100 px line measures 0.5 m; plane centre maps to (2, 1)); implement `src/tools/calibration.rs` over `kinovea-measurement`; verify.
- [ ] 4.2 Write `src/tools/tracking_test.rs` for `track` on the synthetic moving-disc clip helper of `kinovea-tracking` (frames tracked, lost frames, progress notifications, cancellation); implement `src/tools/tracking.rs` with `spawn_blocking` and a `CancellationToken`; verify.
- [ ] 4.3 Write `src/tools/kinematics_test.rs` for `kinematics` and `angular_kinematics` against `tests/fixtures/kinematics/{linear,circular}/` with the fixture tolerances; implement `src/tools/kinematics.rs`; verify.
- [ ] 4.4 Write `src/tools/measurements_test.rs` for `get_measurements` against `tests/fixtures/export/complete/measured.json` (all and per keyframe); implement `src/tools/measurements.rs`; verify.
- [ ] 4.5 Write `src/tools/export_test.rs` for `export_measurements` (json equals the resource, csv-trajectory, xlsx), `export_report`, `export_images` and `export_video` (ffprobe frame count; skipped without `ffmpeg`), including cancellation removing the partial file; implement `src/tools/export.rs`; verify.

## 5. HTTP transport, documentation, end to end

- [ ] 5.1 Write `tests/http_test.rs` binding `127.0.0.1:0`, completing `initialize` over streamable HTTP, and asserting the refusal of `0.0.0.0` without `--allow-remote`; implement `run_http`; verify.
- [ ] 5.2 Add `rust/kinovea-mcp/src/bin/gen_tools_md.rs` (or a test) that renders every tool name, description and input schema into `rust/kinovea-mcp/TOOLS.md` and a test asserting the committed file is up to date; verify `make verify` fails when a tool is added without regenerating.
- [ ] 5.3 Write `rust/kinovea-mcp/tests/e2e_stdio_test.rs` per design D7 (ffmpeg testsrc clip, open, import OpenPose fixture, calibrate, kinematics, render snapshot `tests/snapshots/frame_1000ms.png`, save and reload) spawning the real `kinovea mcp` binary; verify it passes locally and in `rust.yml` (ffmpeg already installed by change 12).
- [ ] 5.4 Document the server in `AGENTS.md` (critical command `kinovea mcp`, structure entry for `rust/kinovea-mcp/`), add a `docs/mcp.md` quick start with a Claude Code `mcpServers` snippet, append the `.agent/progress.md` entry; verify `npm run lint` and `make verify`.

## Workflow follow-up

- Archive after CI is green; tick 13 in `openspec/roadmap.md`.
- Consider `add-pose-estimation` once the import path has been exercised by a real agent session.
