# Proposal

## Why

The end goal of the migration is an agent that receives a video, marks the people's skeletons and answers analysis questions on Linux without a screen. With the domain crates in place (changes 01 to 12), what is missing is a discoverable protocol in front of them: the Model Context Protocol, which agents already speak.

## What Changes

- Add the `kinovea-mcp` crate: an MCP server over stdio (optionally streamable HTTP) started with `kinovea mcp`.
- Session tools: open a video (or video plus KVA), describe it (size, frame rate, duration, keyframes, drawings), seek, render a frame to PNG with drawings, close.
- Annotation tools: import skeleton keypoints (OpenPose BODY_25, Sports2D TRC) and subtitles; add, update, delete core drawings (line, angle, cross mark, chrono, label, posture, track); set keyframe names and comments; undo.
- Analysis tools: calibrate by line or plane, set the origin, track a point or posture over a range, compute linear and angular kinematics, read measurements, export measurements (CSV, JSON, XLSX), reports (Markdown), video and image sequences.
- Resources: the current KVA document as `kinovea://session/<id>/document.kva`, the measured data as JSON, the list of installed posture tools.
- Load and save KVA, so a session is resumable and its result is a normal Kinovea file.
- Headless end-to-end test: an MCP client over stdio against a synthetic clip and a fixed OpenPose fixture, asserting the saved KVA and a rendered PNG.

The KVA format does not change; every tool is a thin wrapper over functions of the domain crates.

## Capabilities

### New Capabilities
- `mcp/server-transport`: process lifecycle, stdio and HTTP transports, capability negotiation, error and progress reporting.
- `mcp/video-session-tools`: opening videos and documents, inspecting them, seeking and rendering frames, saving.
- `mcp/annotation-tools`: importing skeletons and subtitles, creating and editing drawings and keyframes.
- `mcp/analysis-tools`: calibration, tracking, kinematics, measurement reads and exports.

### Modified Capabilities
None.

## Impact

- **Rust crates**: new `kinovea-mcp` (approval required) depending on `kinovea-metadata`, `kinovea-render`, `kinovea-video`, `kinovea-tracking`, `kinovea-kinematics`, `kinovea-measurement`, `kinovea-export`; `kinovea` binary gains the `mcp` subcommand.
- **New dependencies** (approval required, see design): `rmcp` (official Rust MCP SDK), `tokio`, `schemars`, `base64`.
- **Assemblies touched**: none in C#. No new fixture area: the server composes already-verified crates; its tests are Rust integration tests plus a snapshot.
- **Verification**: Linux only (`cd rust && make verify`, CI `rust.yml`); the end-to-end test needs the system `ffmpeg` CLI, already required by `add-importers-exporters`.

## Non-goals

- Pose estimation. The server consumes keypoints produced by an external model (OpenPose, RTMPose/Sports2D); bundling an estimator is a possible later change (`add-pose-estimation`), recorded as an open question.
- Camera capture, live streams, the desktop UI.
- Authentication or multi-tenant hosting for the HTTP transport; the server is a local tool.
- Any drawing type beyond the core list above; the remaining decorative tools (arrows, pencil, grid, spotlight...) stay accessible through KVA load/save only.
