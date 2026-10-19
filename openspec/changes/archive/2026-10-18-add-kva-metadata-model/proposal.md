# Proposal

## Why

Every later change of the migration (headless renderer, importers and exporters, MCP server, desktop UI) manipulates annotations: keyframes, the 25 drawing kinds, styles, trackable points and the KVA file that stores them. Porting this model first, with KVA compatibility proven against the original implementation, unblocks the whole roadmap and gives the future MCP server its data model.

## What Changes

- Add the annotation model to `kinovea-metadata`: metadata (video context, keyframes, detached and singleton drawings), keyframes, all 23 KVA drawing kinds as data, fading, trackable points, content hashes.
- Add the style system: typed style elements bound to style data, defaults from `Tools/DrawingTools/Standard`.
- Add KVA 2.x reading and writing, including context remapping (image size, timestamps) and export flag subsets.
- Add in-Rust migration of KVA 1.3 to 1.5 files (replacing the XSLT) and rejection of 1.2 and older.
- Add undo history as serialized mementos.
- Add the generic posture engine: the 16 custom posture descriptions, constraints, impacts, options, including `OpenPoseBody25`.
- Extend the parity oracle: `kva` cases for every drawing kind, a `posture` area, a seeded fuzzer corpus.

No UI is added. No C# code changes except `Kinovea.FixtureGen`.

## Capabilities

### New Capabilities
- `metadata/drawing-model`: keyframes, drawings as data, fading, trackable points, content hashing, no UI dependency.
- `metadata/drawing-style`: style elements and style data, tool defaults, binding and presets.
- `metadata/kva-serialization`: read and write KVA 2.x with byte-level parity on the oracle fixtures and interoperability with the original application.
- `metadata/kva-migration`: load KVA 1.3 to 1.5 files, reject older ones, produce the same 2.x model as the original XSLT path.
- `metadata/undo-history`: memento-based undo and redo over the metadata.
- `metadata/generic-posture`: posture tool descriptions, constraint engine and OpenPose Body25 mapping.

### Modified Capabilities
- `tooling/parity-oracle`: fixture areas gain `posture` and a fuzzed KVA corpus; the kva area records per-drawing summaries.

## Impact

- **Assemblies touched**: `Kinovea.FixtureGen` only (new areas and cases). `Kinovea.ScreenManager` and `Kinovea.Services` are read, not modified.
- **Crates**: `kinovea-metadata` (depends on `kinovea-core`, `kinovea-geometry`). New third-party crates for XML, ids, property tests and snapshots are listed in design.md and require approval.
- **Files**: `rust/kinovea-metadata/**`, `tests/cases/{kva,posture}/**`, `tests/fixtures/**`, `Kinovea.FixtureGen/Areas/*.cs`.
- **Verification**: Rust verified on Linux (`cd rust && make verify`, CI `rust.yml`). FixtureGen verified by `tools/oracle/fixtures.sh --verify` on the Windows host and CI `build.yml`. Interoperability verified by loading Rust-written KVA through the oracle (cross-load case).

## Non-goals

- Painting drawings or hit testing against pixels (`add-headless-renderer`).
- Tracking, camera motion, lens distortion and kinematics maths; only their persisted state is modeled.
- Exporters, importers and `MeasuredData` (`add-importers-exporters`).
- Video filter state beyond storing its XML verbatim.
- Any Windows target or change to the C# application.
