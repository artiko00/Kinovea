# Design

## Context

See proposal.md for motivation. Constraints:

- Every capability the server exposes already exists as a Rust function in a domain crate after change 12; the server must stay a thin adapter so parity remains verified where the logic lives.
- MCP clients expect JSON schemas for tool inputs, structured errors, progress notifications and cancellation. The official Rust SDK (`rmcp`) provides stdio and streamable HTTP transports, `#[tool]` macros that derive schemas with `schemars`, and runs on `tokio`.
- The domain crates are synchronous and single-threaded per document. Video decoding and tracking are CPU bound and can take seconds.
- The user's workflow: an external pose model produces OpenPose/TRC files; the agent opens the video, imports them, calibrates, tracks or computes kinematics, renders frames to look at, exports.

## Goals / Non-Goals

**Goals:**
- `kinovea mcp` works out of the box over stdio for Claude Desktop/Code style clients; HTTP for remote agents on the same machine.
- Tool names and parameters are stable and documented in one place (`rust/kinovea-mcp/TOOLS.md`, generated from the schemas).
- No business logic in the server: each tool is "look up session, call crate function, map result".

**Non-Goals:**
- Running pose estimation models.
- Concurrency across one session (calls on the same session are serialized).

## Decisions

### D1. New crate `kinovea-mcp`, binary subcommand `kinovea mcp`
Alternative: a separate `kinovea-mcp` binary. One binary keeps packaging (`add-linux-packaging`) and the CLI/MCP shared argument parsing (`clap`, from change 12) in one place. The crate is a library with `run_stdio()` and `run_http(addr)`; the binary only dispatches. New workspace member: approval required.

### D2. `rmcp` as the MCP implementation
Alternatives: hand-rolled JSON-RPC (reimplements the spec, no progress/cancellation plumbing), `mcp-sdk` community crates (less maintained). `rmcp` is the official SDK, tracks the spec revisions, and gives `#[tool_router]`/`#[tool]` with `schemars`-derived input schemas. Dependencies added (approval required): `rmcp` (features `server`, `transport-io`, `transport-streamable-http-server`), `tokio` (rt-multi-thread, io-std, macros), `schemars`, `base64`, `serde`/`serde_json` (already in the workspace).

### D3. Session model
`SessionStore: HashMap<SessionId, Arc<Mutex<Session>>>`; `Session { reader: Box<dyn VideoReader>, document: Document, history: HistoryStack, renderer: HeadlessRenderer, position: i64 }`. Tools lock the session for the duration of the call; long calls (`track`, `export_video`, `export_images`) run on `tokio::task::spawn_blocking` with a `CancellationToken` and a progress channel bridged to MCP progress notifications. Session ids are random 8-byte hex strings; `close_session` or process exit releases readers.

### D4. Coordinates and units
Tool inputs take image pixels (what pose models produce). Outputs of measurements are in session units after calibration, with the unit symbol in every numeric field's sibling `unit`. Times are milliseconds in inputs and outputs; `frame` is accepted as an alternative in `seek` and `add_drawing` and converted with the document's average timestamps per frame.

### D5. Frame transport
`render_frame` returns an MCP `image` content block (PNG, base64). Default scale 1; `scale` lets agents request thumbnails to save tokens. Alternative: writing to a temp file and returning a path; kept as `render_frame_to_file` for video-sized frames in long sessions.

### D6. Mapping to existing features
Tool inventory grounded in the original menus and dialogs: File > Open video / Load annotations / Save → `open_video`, `save_kva`; drawing toolbar (line, angle, marker, label, chrono, generic postures, track) → `add_drawing`; context menus Calibrate / Tracking / Kinematics / Angular kinematics → `calibrate`, `track`, `kinematics`, `angular_kinematics`; File > Export → `export_*`; Edit > Undo → `undo`/`redo`. Decorative tools are reachable only through KVA, by design.

### D7. End-to-end test
`rust/kinovea-mcp/tests/e2e_stdio_test.rs` spawns `kinovea mcp`, uses `rmcp`'s client over child stdio, and runs the user scenario: generate `clip.mp4` with `ffmpeg -f lavfi -i testsrc=duration=2:size=640x480:rate=25`; `open_video`; `import_skeleton` with the OpenPose fixture folder from change 12 (frame numbers remapped to the clip); `calibrate` a synthetic 200 px line as 1 m; `kinematics` on the neck joint; `render_frame` at 1000 ms compared to a snapshot PNG committed under `rust/kinovea-mcp/tests/snapshots/`; `save_kva` and reload with `kinovea-metadata`. Skipped with an explicit message when `ffmpeg` is absent.

### Legacy superseded
No C# file maps to this change; it replaces the interactive workflow (open, annotate, measure, export) for the headless case. The `Kinovea.ScreenManager/PlayerScreen` command handlers are superseded functionally by the tool set and will be removed with the UI changes.

## Risks / Trade-offs

- [`rmcp` API churn between spec revisions] → pin the version in `Cargo.toml`; the tool surface is defined by our schemas, not the SDK's.
- [Large PNGs blow up agent context] → `scale` parameter and `render_frame_to_file`; document defaults in `TOOLS.md`.
- [Blocking calls starve the async runtime] → all domain calls run in `spawn_blocking`; one tokio worker per core.
- [Pose model expectations differ (COCO vs BODY_25)] → the importer only accepts BODY_25 and HALPE_26; the error message names the accepted models and points to Sports2D's TRC output.
- [Snapshot test flakiness across font/raster differences] → the renderer tolerance from `add-headless-renderer` applies; snapshot generated on CI's image.

## Migration Plan

1. Crate skeleton, stdio transport, `open_video`/`describe_session`/`render_frame`/`save_kva` (vertical slice).
2. Annotation tools and undo.
3. Analysis tools.
4. Export tools, HTTP transport, `TOOLS.md`.
5. End-to-end test and CI.
Rollback: remove the crate and the subcommand; nothing else depends on it.

## Open Questions

- Bundling a pose estimator (ONNX Runtime with an RTMPose model) as `add-pose-estimation` so the agent can start from a bare video; deferred until the import path is proven.
- Whether to expose MCP prompts (guided analysis recipes) in addition to tools; does not change the tool set.
