# Tasks

## 1. Oracle fixtures

- [ ] 1.1 Add `tests/cases/export/{minimal,simple,complete}.json` (inputs: kva path, `exportSpace`, `csvDecimalSeparator: point`, `timecodeFormat: ClassicTime`) and `Kinovea.FixtureGen/Areas/ExportArea.cs` running the six spreadsheet exporters plus `ExporterMarkdown` without images into `<out>/export/<case>/{measured.json,trajectory.csv,chrono.csv,trajectory.txt,report.md,measured.xlsx,measured.ods}`; register the area in `Program.cs`; verify `tools/oracle/fixtures.sh --verify` reports identical trees.
- [ ] 1.2 Extend `ExportArea` to extract `xl/worksheets/sheet1.xml`, `xl/sharedStrings.xml` (as `measured.xlsx.sheet1.xml`, `measured.xlsx.strings.xml`) and `content.xml` (as `measured.ods.content.xml`) next to the binaries; verify the extracted XML is stable across two runs and committed under `tests/fixtures/export/`.
- [ ] 1.3 Add `tests/cases/import/{openpose,trc,srt}.json` with small hand-written inputs in `tests/cases/import/data/` (3 OpenPose frames × 2 persons incl. one low-confidence joint and one COCO person; a 26-marker TRC with 5 rows; an SRT with 3 cues incl. tags and one empty block) and `Kinovea.FixtureGen/Areas/ImportArea.cs` that imports them into an empty document (1920×1080, 400 ts/frame) and saves `imported.kva` with ids normalised as in `KvaArea`; verify fixtures regenerate deterministically.

## 2. Measured data collector and text exports

- [ ] 2.1 Write failing tests in `rust/kinovea-export/src/measured_data_test.rs` loading `tests/fixtures/export/complete/measured.json` and asserting counts and values of each list; implement `rust/kinovea-export/src/measured_data.rs` (`MeasuredData` structs, `collect_measured_data(&Document, &ExportPrefs)`); verify `cargo test -p kinovea-export` green and `cd rust && make verify`.
- [ ] 2.2 Write `rust/kinovea-export/src/rtf_test.rs` (cases: plain, `\par`, `\'e9`, `\u233?`, nested groups) then `src/rtf.rs` `rtf_to_text`; verify tests pass and the keyframe comments of the three samples equal the `comment` fields of `measured.json`.
- [ ] 2.3 Write `src/csv_test.rs` comparing `write_csv_trajectory` and `write_csv_chrono` output to `trajectory.csv` and `chrono.csv` byte for byte, plus a comma-separator case; implement `src/csv.rs` (`CsvFormat { decimal, list_separator }`, quoting, NaN → empty); verify tests pass.
- [ ] 2.4 Write `src/txt_test.rs` against `trajectory.txt`, implement `src/txt.rs`; verify.
- [ ] 2.5 Write `src/json_test.rs` parsing both Rust output and `measured.json` into `serde_json::Value` and asserting equality with 1e-4 numeric tolerance and identical key order; implement `src/json.rs`; verify.
- [ ] 2.6 Write `src/markdown_test.rs` against `report.md` (no images) and a case with three image paths; implement `src/markdown.rs`; verify.

## 3. Importers

- [ ] 3.1 Write `src/openpose_test.rs` (folder scan with prefix and alphanumeric sort, frame token parse failure, 75-value filter, confidence options) asserting the saved KVA equals `tests/fixtures/import/openpose/imported.kva` after id normalisation; implement `src/openpose.rs` using the `OpenPoseBody25` tool from `kinovea-metadata`; verify.
- [ ] 3.2 Write `src/trc_test.rs` (header parsing, marker mismatch error, posture + 26 tracks) against `tests/fixtures/import/trc/imported.kva`; implement `src/trc.rs`; verify.
- [ ] 3.3 Write `src/srt_test.rs` (block split, `,`/`.` millis, tag stripping, CRLF) against `tests/fixtures/import/srt/imported.kva`; implement `src/srt.rs`; verify.
- [ ] 3.4 Write `src/import_test.rs` for `import_file(&mut Document, path)` dispatch (`.kva` → metadata loader, `.trc`, `.srt`, `_keypoints.json`, unknown → error) and `ImportReport`; implement `src/import.rs`; verify `make verify`.

## 4. Spreadsheet containers (approval for `rust_xlsxwriter`, `zip`, `quick-xml`)

- [ ] 4.1 Add the three dependencies to `rust/Cargo.toml` `[workspace.dependencies]` and `rust/kinovea-export/Cargo.toml`; verify `cargo build -p kinovea-export` and `make verify` still pass with no new clippy warnings.
- [ ] 4.2 Write `src/xlsx_test.rs` that writes the workbook to a `Vec<u8>`, opens it with `zip`, parses `sheet1.xml` and `sharedStrings.xml` with `quick-xml` into `(cell, value, style number format)` and merged ranges, and compares to the oracle `measured.xlsx.sheet1.xml`/`strings.xml` parsed the same way; implement `src/xlsx.rs` with the six blocks and the time format rule; verify.
- [ ] 4.3 Write `src/ods_test.rs` comparing table cells of `content.xml` with `measured.ods.content.xml` and asserting `mimetype` is the first, stored entry; implement `src/ods.rs` (manifest, content, styles, meta, settings); verify, and open the file once with `libreoffice --headless --convert-to csv` to confirm it loads.

## 5. CLI (approval for `clap`)

- [ ] 5.1 Add `clap` to the workspace; write `rust/kinovea/src/cli_test.rs` for argument parsing (`--version`, `import`, `export <format>` with `--decimal`, `--space`, unknown format → exit 2) keeping the existing `main_test.rs` and `tests/main_test.rs` green; implement `rust/kinovea/src/cli.rs` and route `main.rs` through it; verify `make verify`.
- [ ] 5.2 Write `rust/kinovea/tests/export_cli_test.rs` running `kinovea export json --kva Tools/XML/Schema/samples/complete.kva --out <tmp>` and comparing with the fixture, and `kinovea import` on the OpenPose fixture folder; verify both pass and document the commands in `tests/README.md`.

## 6. Image and video export (after `add-headless-renderer` and `add-ffmpeg-video-reader` are archived)

- [ ] 6.1 Write `src/frame_source_test.rs` with a synthetic `FrameSource` (solid frames with a timestamp label) and tests for `export_image`, `export_key_images`, `export_sequence` (interval, naming `<base>_<timecode>`, cancellation after N) and `compose_side_by_side` (horizontal/vertical sizes); implement `src/frame_source.rs`, `src/image_export.rs`; verify.
- [ ] 6.2 Write `src/ffmpeg_args_test.rs` asserting `ffmpeg_args` equals the argument strings of `WriterFFMpegCliHelper` for MJPEG, H264 and H265 profiles with and without scaling; implement `src/ffmpeg_args.rs` and `src/video_export.rs` (child process, stdin pipe, duplication rule ≥ 8 fps, slideshow, with-pauses, side by side, refuse output == source); verify unit tests pass.
- [ ] 6.3 Write `rust/kinovea-export/tests/video_export_test.rs` that generates a 25-frame clip with `ffmpeg -f lavfi -i testsrc` in a temp dir (skipped with a message when `ffmpeg` is absent), exports MJPEG and slideshow, and asserts frame counts and frame rate via `ffprobe -show_streams -count_frames`; add `ffmpeg` and `pandoc` installation to `.github/workflows/rust.yml` (workflow edit, approval required); verify CI green.
- [ ] 6.4 Write `src/document_export_test.rs` for `export_document(Markdown|Odt|Docx)`: Markdown with rendered key images in `images/`, pandoc missing → error before writing, pandoc present → zip contains `word/document.xml` / `content.xml` (ignored when pandoc absent); implement `src/document_export.rs`; verify.
- [ ] 6.5 Wire `kinovea export image|key-images|sequence|side-by-side|video|slideshow|with-pauses|video-side-by-side|markdown|odt|docx` in `rust/kinovea/src/cli.rs` with tests in `cli_test.rs`; update `AGENTS.md` structure line for `kinovea-export` (imports and exports) and append the `.agent/progress.md` entry; verify `make verify` and `npm run lint`.

## Workflow follow-up

- Archive after `rust.yml` is green and fixtures are committed from one oracle commit.
- `add-mcp-server` wraps `kinovea-export` functions; `add-tracking` adds the trackable-timeline export case.
