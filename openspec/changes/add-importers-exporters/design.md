# Design

## Context

See proposal.md for motivation. Constraints:

- The C# layer is thin and well separated: `Metadata.CollectMeasuredData()` produces `MeasuredData` (plain lists), and each exporter serializes that model. Importers are static classes that mutate `Metadata` through `MergeInsertKeyframe`. The Rust port keeps that shape.
- Everything user facing in the C# exporters (save dialogs, `BackgroundWorker`, `FormProgressBar`) is UI and is out of scope; the Rust exporters take a settings struct and an optional cancellation token and report progress through a callback.
- Image and video export need rendered frames: they depend on `kinovea-render` (`add-headless-renderer`) and on `kinovea-video` with the FFmpeg reader (`add-ffmpeg-video-reader`). Importers and spreadsheet/JSON/Markdown exports need only `kinovea-metadata` and `kinovea-kinematics`.
- Oracle outputs for xlsx/ods/docx are zip containers whose binary bytes are not reproducible across libraries; comparison is on content.
- The parity fixtures for `kva` already fix preferences (invariant culture, `Kinovea.0.0` producer). Export fixtures need the same plus explicit export preferences.

## Goals / Non-Goals

**Goals:**
- One `kinovea-export` crate with pure functions `collect_measured_data(&Document, &ExportPrefs) -> MeasuredData` and `write_<format>(&MeasuredData, &mut impl Write)`, testable against fixtures without files.
- Importers as `import_openpose(&mut Document, path)`, `import_trc`, `import_srt` returning an `ImportReport` (created keyframes, drawings, tracks, skipped items).
- The `kinovea` binary exposes the behaviour (`kinovea import`, `kinovea export`) so the MCP change only wraps it.

**Non-Goals:**
- Reproducing xlsx/ods styling bytes; only cells, formats and merges.
- Streaming exports larger than memory.

## Decisions

### D1. Importers live in `kinovea-export`, not `kinovea-metadata`
Alternative: put them in `kinovea-metadata` next to the KVA serializer, as the C# `MetadataSerializer.Load` dispatches on extension. Keeping `kinovea-metadata` limited to the document model and KVA keeps that crate free of format zoo code; `kinovea-export` becomes the "foreign formats" crate (name kept to avoid renaming a workspace member; documented in `lib.rs`). Dispatch by extension (`.kva`, `.trc`, `.srt`, `_keypoints.json`) is a function in `kinovea-export` that the CLI and the MCP call.

### D2. Spreadsheet writers
- CSV/TXT: hand-written like `CSVHelper` (quoting and separator rules are bespoke; the `csv` crate would fight them). No dependency.
- JSON: `serde_json` (workspace dependency) with a hand-ordered `serde_json::Map` so property order matches the oracle.
- XLSX: `rust_xlsxwriter` (new dependency, approval required). Alternatives: `umya-spreadsheet` (reads and writes, heavier), hand-written OOXML (more code than the ODS one). `rust_xlsxwriter` covers merged cells, number formats, fills and borders used by `ExporterXLSX`.
- ODS: hand-written XML through `quick-xml` into a `zip` archive, mirroring `ExporterODS` (634 lines of XML writing; no maintained Rust ODS writer exists). `zip` and `quick-xml` are new dependencies (approval required); `quick-xml` is expected to be adopted by `add-kva-metadata-model` anyway.

### D3. Documents through pandoc, as the original
Markdown is written natively. ODT/DOCX call `pandoc` (`std::process::Command`) with the same arguments as `DocumentExporter.RunPandoc`. Alternative `docx-rs`: would diverge from the oracle output and add a large dependency for one format; rejected. Tests mark the pandoc path `#[ignore]` unless `pandoc` is on `PATH`; CI installs it with `apt`.

### D4. RTF comments
Keyframe comments are RTF in KVA 2.0 (`RichTextBox.Rtf`). A minimal RTF-to-text extractor (control words, groups, `\par`, `\'hh` escapes, `\u` unicode) lives in `kinovea-export/src/rtf.rs`; fixtures from the samples drive it. Full RTF rendering stays with the UI changes.

### D5. Video writing through the `ffmpeg` CLI
Mirrors `WriterFFMpegCLI`: `ffmpeg` child process, raw BGR24 frames piped to stdin, arguments built by a pure function `ffmpeg_args(&VideoExportSettings) -> Vec<String>` that is unit tested against the strings the C# helper produces. Alternative: encode in-process with `ffmpeg-next`; deferred because the CLI path is what the original ships and keeps `kinovea-export` free of libav linkage. The MJPEG in-process writer of `add-ffmpeg-video-reader` is unaffected.

### D6. Frame source abstraction for image and video export
`trait FrameSource { fn frame_at(&mut self, ts: i64) -> Result<Rgba8Image>; fn key_timestamps(&self) -> Vec<i64>; }` implemented by the player document (video reader + renderer) and by a synthetic source in tests. Side-by-side composition is a pure function over two images.

### D7. CLI subcommands
`clap` (new dependency, approval required; derive API) replaces the hand-rolled `--version` parsing in `rust/kinovea/src/main.rs`. Subcommands: `kinovea import <file> --kva <doc> [--out <doc>]`, `kinovea export <format> --kva <doc> --video <file> --out <path> [--decimal point|comma] [--space world|pixels]`.

### D8. Oracle `export` area
`Kinovea.FixtureGen/Areas/ExportArea.cs` loads a sample KVA with the same context as `KvaArea`, sets `PreferencesManager.PlayerPreferences` export fields from the case (`exportSpace`, `csvDecimalSeparator`, `timecodeFormat`, `exportImagesInDocuments=false`), then runs the six spreadsheet exporters and `ExporterMarkdown` (no images) into `<out>/export/<case>/`. For xlsx and ods it also extracts `xl/worksheets/sheet1.xml` + `xl/sharedStrings.xml` and `content.xml` into sibling `.xml` files so review diffs are readable; Rust compares against those XML parts after parsing. Importers get fixtures too: `tests/cases/import/{openpose,trc,srt}.json` point to small hand-written inputs under `tests/cases/import/data/`; the oracle imports them with `MetadataSerializer.Load` and saves the KVA (ids normalised like `KvaArea`).

### Legacy superseded
`Kinovea.ScreenManager/Importers/**` (OpenPose, SRT, TRC), `Exporters/Spreadsheet/*.cs`, `Exporters/Document/{DocumentExporter,ExporterMarkdown,DocumentExportFormat}.cs`, `Exporters/Images/{ExporterImage,ExporterImageSequence,ExporterImageSideBySide,ImageExporter,ImageExportFormat}.cs`, `Exporters/Video/{ExporterVideo,ExporterVideoSideBySide,VideoExporter,VideoExportFormat,WriterFFMpegCLI,WriterFFMpegCliHelper}.cs`, `Metadata/Serialization/MeasuredData*.cs`, `Metadata.CollectMeasuredData` and the `CollectMeasuredData` methods on drawings. The `FormConfigureExport*` dialogs and `ControlEncodingSettings` are superseded by the UI changes.

## Deviations recorded during apply

- Oracle: the xlsx and ods containers embed their creation time, so only the sheet XML (`measured.xlsx.sheet1.xml`, `measured.xlsx.strings.xml`, `measured.ods.content.xml`) is stored. Import inputs live in `tests/data/import` because the generator treats every `.json` under `tests/cases` as a case. The Markdown keyframe timecode comes from the document timecode builder; the oracle's builder returns the raw timestamp, and the Rust writer takes the formatter as an argument.
- The TRC case has 22 markers (the `BodyWithFeet` tool) and 7 data rows with `NumFrames` 10: the original bounds its data loop by `NumFrames` as a line count, so 5 rows are imported; Rust reproduces it. Each TRC track starts with its first point twice, as `DrawingTrack(name, points)` does.
- `rtf_to_text` drops the final paragraph mark, as `RichTextBox.Text`; `\ansicpg` is not read (code page 1252 always).
- Choosing the same video context checks first that the current video exists and is not empty (`MetadataSerializer.IsSameContext`); Rust used names first, which only showed on the `all-drawings` export case.
- `HighSpeedFactor` is a field of the video context (default 1, not stored in KVA); the original derives it in `PostSetupVideo`, which the oracle does not call, so the fixtures use 1.
- Not yet ported: trackability timelines in the measured data (with `add-tracking`), counter and cadence (TODO in the original), calibration driven by tracking (static calibration is used), culture-aware sorting (ordinal in Rust, stable sorts).

## Risks / Trade-offs

- [`rust_xlsxwriter` cell styles differ from SpreadsheetLight] → the spec compares values, number formats and merges, not fills or fonts.
- [RTF edge cases in comments] → extractor covers the subset `RichTextBox` emits; unknown control words are dropped; fixtures from all three samples plus a hand-written RTF case.
- [`ffmpeg`/`pandoc` absent on the developer machine] → tests needing them are skipped with a visible reason; CI installs both.
- [Oracle `ExporterMarkdown` needs a `RichTextBox`, which needs a message loop] → FixtureGen already runs on the desktop host; it works in a console with `[STAThread]`, as `Kinovea.Tests` does.
- [Time series collection depends on trackability manager internals (`trackabilityManager.CollectMeasuredData`)] → that part is ported in `add-tracking`; here the collector reads whatever `kinovea-metadata` exposes and the fixture from the samples (which contain tracks but no trackable timelines) stays green; a follow-up case is added in `add-tracking`.

## Migration Plan

1. Fixtures first: `export` and `import` areas, regenerate, commit.
2. Collector + CSV/TXT/JSON/Markdown + importers (no new deps except `quick-xml`).
3. XLSX and ODS (deps approved).
4. CLI.
5. Image and video export once 06 and 08 are archived.
Rollback: the crate is additive; nothing in C# changes except FixtureGen.

## Open Questions

- Whether `kinovea-export` should be renamed `kinovea-io` when the workspace is next reorganised; cosmetic, deferred.
- ODS export could later be replaced by `pandoc`-style delegation to LibreOffice headless; not needed for parity.
