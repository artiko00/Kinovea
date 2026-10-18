# Proposal

## Why

Analysis in Kinovea starts with importers (OpenPose keypoints, Sports2D TRC, SRT) and ends with exports (spreadsheets, documents, images, videos). The MCP server (`add-mcp-server`) needs both without a UI. This change ports that input/output layer to Rust on Linux with the original C# output as the oracle.

## What Changes

- Port the three importers into `kinovea-export` (scope: import and export): OpenPose BODY_25 `*_keypoints.json` folders → one `OpenPoseBody25` posture per person per frame; Sports2D TRC (`BodyWithFeet`) → one posture plus one track per marker; SRT → one timed text drawing per cue.
- Port the measured-data collector (`CollectMeasuredData`) and the six spreadsheet exporters: CSV trajectory, CSV chronometer, TXT trajectory, JSON (Kinetics Toolkit layout), XLSX and ODS.
- Port document export: Markdown with key images; ODT/DOCX by delegating to an installed `pandoc`, as the original does.
- Port image export (current frame, key images, sequence, side by side) and video export (video, slideshow, with pauses, side by side) on top of the headless renderer and the FFmpeg reader, piping raw frames to the `ffmpeg` CLI like `WriterFFMpegCLI`.
- Add `kinovea import <file>` and `kinovea export <format>` subcommands so the behaviour is reachable without the UI.
- Add the `export` fixture area to `Kinovea.FixtureGen`: the C# exporters produce golden JSON, CSV, TXT, Markdown, xlsx and ods per sample KVA; Rust compares values (xlsx/ods on extracted sheet XML, not bytes).

No KVA format change. The C# code is not modified except for the new fixture area.

## Capabilities

### New Capabilities
- `io/skeleton-import`: importing external 2D pose data (OpenPose JSON, Sports2D TRC) into posture drawings and tracks.
- `io/subtitle-import`: importing SRT subtitles as timed text annotations.
- `io/measured-data-export`: collecting measurements from a document and writing them as CSV, TXT, JSON, XLSX, ODS.
- `io/document-export`: writing a Markdown report with key images, optionally converted with pandoc.
- `io/image-export`: saving rendered frames as single image, key images, sequence or side by side.
- `io/video-export`: encoding rendered frame ranges to video files through the system `ffmpeg`.

### Modified Capabilities
None.

## Impact

- **Rust crates**: `kinovea-export` (all logic), `kinovea` (CLI subcommands). Depends on `kinovea-metadata` (05), `kinovea-kinematics` (03), `kinovea-render` (06), `kinovea-video` (07/08). Image and video export tasks cannot start before 06 and 08 are archived.
- **New dependencies** (approval required, see design): `rust_xlsxwriter`, `zip`, `quick-xml`, `clap`. External tools at run time: `ffmpeg`, optional `pandoc`.
- **Assemblies touched**: `Kinovea.FixtureGen` only (new `ExportArea`). `Kinovea.ScreenManager` is read, not modified.
- **Verification**: Rust side fully on Linux (`cd rust && make verify`, CI `rust.yml`). The fixture area is compiled and run on the Windows host via `tools/oracle/fixtures.sh`; `build.yml` only confirms the solution still compiles.

## Non-goals

- Any UI (dialogs `FormConfigureExport*`, progress bars, save-file dialogs); that is `add-measurement-ui-and-plotting` and `add-player-screen-rendering`.
- Pose estimation: the importers consume keypoints produced elsewhere.
- Kinogram export and video-filter custom exports (`add-camera-motion-and-lens-filters`).
- Clipboard copy of CSV.
- Counter and cadence drawings in measured data (not exported by the original either).
