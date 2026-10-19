# Tasks

## 1. Oracle coverage for annotations

- [x] 1.1 Add `tests/cases/kva/all-drawings.json` and a hand-written `tests/cases/kva/inputs/all-drawings.kva` containing one instance of every drawing element (Angle, Bitmap, Chrono, ChronoMulti, Circle, CoordinateSystem, Counter, CrossMark, DistortionGrid, GenericPosture, Label, Line, AutoNumbers, Pencil, Plane, Polyline, Rectangle, Spotlights, SVG, TestGrid, TimePolyline, TimeSegment, Track); extend `Kinovea.FixtureGen/Areas/KvaArea.cs` to write `drawings.json` (element name, id, name) per keyframe (deviation: `ContentHash` is not deterministic across processes in the original, so it is not part of the fixture); verify `tools/oracle/fixtures.sh --verify` is deterministic and `cargo test -p kinovea-fixtures` passes.
- [x] 1.2 Add `Kinovea.FixtureGen/Areas/KvaFuzzArea.cs` linking `Kinovea.Tests/Metadata/KVAFuzzer.cs` with seeded `Random`, case `tests/cases/kva/fuzz.json` (seed, count 20); write `fuzz/<n>.kva` plus round-trip; verify two runs are byte-identical with `fixtures.sh --verify`. Deviation: the legacy `KVAFuzzer.cs` targets the 2.0 shapes and is not deterministic, so `Kinovea.FixtureGen/KvaFuzz*.cs` is a seeded rewrite for 2.1.
- [x] 1.3 Add `tests/cases/kva/remap.json` (file 1280×720 loaded into 640×360 context, different first timestamp) and `tests/cases/kva/fading.json` (opacity per timestamp table); extend `KvaArea.cs`; verify fixtures regenerate and `provenance_test` passes.
- [x] 1.4 Add `tests/cases/kva/migration.json` with 1.3 and 1.5 inputs under `tests/cases/kva/inputs/legacy/` and make `KvaArea.cs` write the XSLT output (`converted.kva`) and the loaded round trip; verify fixtures exist for each legacy file.
- [x] 1.5 Add `Kinovea.FixtureGen/Areas/PostureArea.cs` and cases `tests/cases/posture/<name>.json` for the 16 custom postures (summary of counts) plus `tests/cases/posture/moves.json` (handle moves per constraint and impact type) writing `moves.csv`; verify `fixtures.sh --verify` and commit `tests/fixtures/`. Moves are generated per tool (`posture/<tool>/moves.csv`) instead of one `moves` case.

## 2. Crate skeleton and dependencies

- [x] 2.1 After approval, add `quick-xml`, `roxmltree`, `uuid` (features v4) to `rust/kinovea-metadata/Cargo.toml` and `proptest` as dev-dependency; depend on `kinovea-core`, `kinovea-geometry`, `kinovea-fixtures` (dev); verify `cd rust && make verify` passes with the existing placeholder test. Deviation: only `uuid` was added; XML goes through the existing `kinovea-core` DOM and properties use seeded loops, so `quick-xml`, `roxmltree` and `proptest` were not needed (see design D3, D5, D10).
- [ ] 2.2 Create modules `rust/kinovea-metadata/src/{ids,xml,fading,style,drawing,keyframe,metadata,kva,migration,history,posture}.rs` with `lib.rs` re-exports and one `*_test.rs` per module asserting module presence; verify `npx testlens check --fail` is green.

## 3. Ids, fading and style

- [x] 3.1 Write `src/ids_test.rs` (parse and display match `Guid.ToString()`, fixture normalization helper maps ids by order of appearance) then implement `src/ids.rs`; verify `cargo test -p kinovea-metadata ids`.
- [x] 3.2 Write `src/fading_test.rs` against `tests/fixtures/kva/fading/*.csv` and always-visible scenario, then port `InfosFading` to `src/fading.rs` including `read_kva`/`write_kva`; verify the fixture test passes within 1e-6.
- [x] 3.3 Write `src/style_test.rs`: round trip of every `DrawingStyle` block extracted from fixture KVAs, unknown element preserved, memorize/restore, binding; then implement `src/style.rs` (`StyleElement` enum, `StyleElements`, `StyleData`); verify tests pass.
- [x] 3.4 Embed `Tools/DrawingTools/Standard/*.xml` with `include_str!` in `src/style/tools.rs`, write `tools_test.rs` asserting 26 tools load with drawing kind and defaults; verify `make verify`.

## 4. Drawing model

- [x] 4.1 Write `src/drawing_test.rs` for the common header, `Drawing` enum exhaustiveness (one variant per KVA element name) and `content_hash` stability; implement `src/drawing.rs` with the header struct and enum; verify tests pass.
- [x] 4.2 Implement simple geometric drawings in `src/drawing/{line,angle,crossmark,circle,rectangle,pencil,polyline,label}.rs` each with `*_test.rs` covering fields, trackable points, measured value in pixels, and KVA read/write round trip of the matching fragments from `all-drawings.kva`; verify tests pass.
- [x] 4.3 Implement `plane`, `distortion_grid`, `test_grid`, `coordinate_system` with tests on quadrilateral persistence and read/write parity; verify tests pass.
- [x] 4.4 Implement time drawings `chrono`, `chrono_multi`, `counter`, `time_segment`, `time_polyline` and multi-drawings `spotlights`, `auto_numbers` with item structs; tests cover section lists, visibility timestamps and round trip; verify tests pass.
- [x] 4.5 Implement `track` (timed points, tracker parameters block kept verbatim, labels, keyframe labels), `bitmap`, `svg` (path or embedded payload) with read/write tests; verify tests pass.
- [x] 4.6 Write `src/keyframe_test.rs` and `src/metadata_test.rs` (sorted keyframes, `find_drawing`, managers, singletons, trackability records) then implement `src/keyframe.rs` and `src/metadata.rs`; verify tests pass and `cargo clippy` is clean.

## 5. KVA 2.x reader and writer

- [x] 5.1 Write `tests/kva_roundtrip_test.rs`: for every `tests/fixtures/kva/*/roundtrip.kva` load the case input, write with producer `Kinovea.0.0`, normalize ids, compare bytes; implement `src/kva/{reader,writer}.rs` general information, calibration block (stored verbatim as `CalibrationData` until 04 is integrated), keyframes and drawings dispatch; verify the minimal and simple cases pass.
- [x] 5.2 Extend the reader and writer for chronos, tracks, spotlights, number sequence, coordinate system, test grid, trackability, video filters (verbatim), camera motion (verbatim); verify `complete` and `all-drawings` cases pass byte for byte.
- [x] 5.3 Implement context remapping (`scaling`, `remap_timestamp`, same-context detection) in `src/kva/context.rs` with `context_test.rs` against the `kva/remap` fixture; verify within 1e-4.
- [x] 5.4 Implement export flags (`Basics`, `Drawings`, `Calibration`, `VideoSpecific`, capture recording) with a test comparing the capture-recording output to a fixture produced by a new `tests/cases/kva/capture.json` case; verify byte parity.
- [ ] 5.5 Add `tests/kva_fuzz_test.rs` loading the 20 fuzzed files and checking round trip against their fixture; add `proptest` property read(write(d)) == d for every drawing kind in `tests/kva_property_test.rs`; verify `make verify`.

## 6. Migration

- [x] 6.1 Write `tests/migration_test.rs` comparing `migrate(legacy)` output model to the model loaded from the fixture `converted.kva`, and rejection of 1.2; implement `src/migration.rs` (format detection, 1.5→2.x renames, unit mapping, relative track times); verify tests pass.

## 7. Undo history

- [ ] 7.1 Write `src/history_test.rs` (push clears redo, undo/redo of each memento kind restores KVA serialization and content hash); implement `src/history.rs` with `HistoryStack` and `Memento`; verify tests pass.

## 8. Generic posture

- [x] 8.1 Write `src/posture_test.rs` loading the 16 custom files embedded via `include_str!` and comparing counts to `tests/fixtures/posture/<name>/summary.json`, plus Body25 index-to-name assertions; implement `src/posture/{model,parser}.rs`; verify tests pass.
- [x] 8.2 Write `src/posture/engine_test.rs` against `tests/fixtures/posture/moves/moves.csv`; port constraints and impacts into `src/posture/engine.rs`; verify within 1e-3.
- [x] 8.3 Implement the `GenericPosture` drawing variant read/write and add it to the all-drawings round trip; verify byte parity.

## 9. Integration

- [x] 9.1 Write Rust-produced KVA files for the three samples into `tests/cases/kva/crossload/` via a `cargo run --example write_samples -p kinovea-metadata`, add `tests/cases/kva/crossload.json`, extend `KvaArea.cs` to load them and write `summary.json` with counts and hashes; run `fixtures.sh`; add `tests/crossload_test.rs` asserting equality with Rust counts; verify it passes.
- [ ] 9.2 Update `AGENTS.md` structure line for `kinovea-metadata`, append the `.agent/progress.md` entry, tick the roadmap item; verify `npm run lint` and `cd rust && make verify` are green and CI `rust.yml` passes on the branch.
