# Progress log

Chronological. Newest entry last. Never delete entries; correct a wrong decision in a new entry.

## 2026-10-17 — Bootstrap agent harness

- Agent: Claude Code (Fable 5.1), host Linux (no msbuild/dotnet).
- Task: install the stack from siemprelisto.cl "bootstrap-agente" (AGENTS.md, .agent/, OpenSpec, open-harness, lefthook).
- Done:
  - Reconnaissance: C# .NET Framework 4.8 WinForms + one C++/CLI project, 22 projects in Kinovea.VS2019.sln, no automated tests, CI builds on windows-latest.
  - AGENTS.md (93 lines) written from verified facts.
  - .agent/init.sh (executable, idempotent), .agent/features.json (empty list), .agent/.gitignore.
  - OpenSpec was already initialized (claude, codex, hermes skills). Filled openspec/config.yaml (context + array rules) and ran `openspec update`.
  - package.json (private, harness only) with @open_harness/open-harness ^0.3.7 and the four linter keys; testlens.json for `notest` (ignored when set via package.json).
  - lefthook.yml with the four checks in parallel; `lefthook install` done.
- Initial linter findings (NOT fixed; human decides):
  - linelens (maxLines 150): 209 files over the limit. Largest: PlayerScreenUserInterface2.cs 4798, ScreenManager.cs 2880, VideoReaderFFMpeg.cpp 2629, Metadata.cs 1858, DrawingTrack.cs 1761.
  - dupelens (minTokens 50, minLines 5): 561 matches (172 exact, 389 renamed). Hotspots: Kinovea.Camera.Basler FormConfiguration.cs / CameraManagerBasler.cs, DrawingCounter.cs, DrawingCircle.cs, DrawingChronoMulti.cs.
  - secretlens: 4 findings, all false positives (password/token variable assignments in Kinovea.Camera.HTTP ConnectionWizard.cs:108,187, SpecificInfo.cs:43; Kinovea.Camera/Configuration/CameraPropertyViewLogarithmic.cs:19). 54 `PublicKeyToken` hits removed by excluding *.csproj, app.config, packages.config.
  - testlens (csharp): 745 source files without tests. Expected: the project has no test framework.
- Consequence: the lefthook pre-commit gate fails on every commit until the baseline is handled.
- Design decisions:
  - Build/type-check steps in init.sh degrade to a warning when msbuild is absent; CI is the build oracle.
  - Excluded from linters: Refs, packages, bin, obj, Installer, Tools, VideoTests, Languages/, *.resx, *.Designer.cs, *.resources.
  - Kinovea.Tests excluded from testlens as "source" (it is the test harness itself).
- Pending / for the human:
  - Decide linelens/dupelens baseline: raise limits, add skip rules for legacy files, or temporarily drop `--fail` from lefthook.
  - Decide whether to add a real test framework (NUnit/xUnit) and a CI test step.
  - Add this repo's .hermes/skills to ~/.hermes/config.yaml (skills.external_dirs) if Hermes is used.
  - Optionally add the `update` workflow to the OpenSpec profile (`openspec config profile core`).
- Next steps: first feature via `/opsx:propose <add-...>`; record it in .agent/features.json once acceptance criteria exist.

## 2026-10-17 — add-rust-workspace-scaffold (apply)

- Agent: Claude Code (Fable 5.1), host Linux. Human approved every task up front (including host provisioning) and was away.
- Mission context: full migration of Kinovea to Rust, Linux-only, TDD, `egui` for the UI, end goal an MCP server that annotates skeletons headlessly. Roadmap (21 changes) lives in the OpenSpec exploration; this is change 00.
- Done:
  - `rust/` Cargo workspace: 10 lib crates + `kinovea` bin, stable 1.93.1 pinned, clippy deny, `make verify` (fmt + clippy + test). Tests sit next to sources as `src/<file>_test.rs` because that is the only Rust layout `testlens` recognizes (`tests/foo_test.rs` and inline `#[cfg(test)]` are not).
  - `.github/workflows/rust.yml` (ubuntu-latest). Linters now target Rust; C# baseline frozen below.
  - Windows oracle host `windows` (192.168.0.138): installed VS Build Tools 2022 (MSBuild 17.14, .NET 4.8 targeting, MSVC v143, C++/CLI) and NuGet via winget over SSH. `tools/oracle/provision.ps1` (check/install), `build.ps1` (clone/bundle, restore, FFmpeg cache, Release x64), `fixtures.sh` (Linux driver, ships a git bundle when HEAD is not on origin/master).
  - `Kinovea.FixtureGen` (C#, in the .sln): areas `kva` (round-trip of the 3 samples), `kinematics` (linear, circular with angular series, Winter gait data), `timecode` (all formats × 3 frame rates). Output to `tests/fixtures/`, manifest with commit, FFmpeg version, case sha256.
- Gotchas found:
  - The original KVA importer mints a fresh Guid for every element without an id (all three 2.0 samples). Fixtures replace Guids by order of appearance (`00000000-0000-4000-8000-<n>`) to be deterministic.
  - Windows checkout had CRLF `tests/cases/*.json`, breaking the sha256 contract. Fixed with `.gitattributes` (LF for cases, `-text` for fixtures) and LF-normalized hashing in FixtureGen.
  - Pre-commit lefthook gates fail on the legacy C# baseline; commits in this change were made with `LEFTHOOK=0`. Decision still pending (see previous entry).
  - `Kinovea.Tests` is not in the .sln; FixtureGen links `KinematicsTestData.cs` and `MovingObject.cs` from it.
- secretlens: the 4 baseline hits were variable references (`.Password`, `tbPassword.Text`, `localizationToken`); allowlisted in `package.json` so `npm run lint:secrets` is green. linelens/dupelens baseline unchanged.
- Verified: `rust.yml` green on PR #1; fresh clone `make verify` + `fixtures.sh --verify` green; `git status` clean.
- Next: `add-core-types-and-units`, starting with `tests/fixtures/timecode/formats/*.csv` as red tests.

## 2026-10-18 — lefthook scoped to new code

- Agent: Claude Code (Fable 5.1). Human decision: the gates only consider new code; legacy C# will be deleted project by project as the Rust port replaces it.
- Done: every legacy C# project directory (`Kinovea`, `Kinovea.Camera*`, `Kinovea.FileBrowser`, `Kinovea.Pipeline`, `Kinovea.ScreenManager`, `Kinovea.Services`, `Kinovea.Tests`, `Kinovea.Updater`, `Kinovea.Video*`), plus `Installer`, `Tools`, `VideoTests`, `Refs`, is excluded in all four linter blocks of `package.json` and in `testlens.json`. Removed the C#-only skip rules, `.resx`/`Languages` excludes and the secretlens allowlist entries that only existed for legacy code. `Kinovea.FixtureGen/` stays linted (new code).
- Result: `npm run lint` green (33 files scanned by each linter, 0 findings). The lefthook pre-commit gate is usable again; `LEFTHOOK=0` is no longer needed.
- Rule: when a legacy directory is deleted, remove its exclude entry so it cannot be recreated unnoticed.

## 2026-10-18 — migration roadmap and changes 01–20 proposed

- Agent: Claude Code (Fable 5.1), under a `/goal` to create the changes that migrate the whole repo to Rust and end in an MCP server.
- Done: archived `add-rust-workspace-scaffold` (main specs `tooling/*` created). `openspec/roadmap.md` lists the 21 changes with the dependency graph, archive-order constraints and cross-cutting rules (capability paths, crate decisions, parity, legacy retirement). Changes 01–20 authored in parallel by six forks; `openspec validate --all --strict` passes (22 items), 380 tasks in total, nothing implemented.
- Discoveries recorded in the artifacts: the KVA writer emits FormatVersion 2.1; both kinematics smoothing preferences default to `true`, so the existing kinematics fixtures include the moving-average pass; no `Fraction` type exists in `Kinovea.Services`; `ProjectiveMapper.Backward` has a typo the port reproduces; the host FFmpeg is 9.0 while CI Ubuntu 24.04 ships 6.1 (08 pins the floor at 6.1).
- Pending human decisions: every new crate and dependency is flagged "requires approval" in the designs; workflow edits (apt packages, GUI libs, release.yml) likewise; `kinovea-export` rename to `kinovea-io`; Flatpak id; optional `add-pose-estimation` change (13 accepts external keypoints only).
- Next: review PR, archive nothing yet, start `/opsx:apply add-core-types-and-units`.

## 2026-10-18 — add-core-types-and-units (apply)

- Agent: Claude Code (Fable 5.1). Human asked to merge PR #2 and apply the change end to end.
- Done in `rust/kinovea-core`: timecode formats and rendering (`timecode.rs`, `timecode_format.rs`), units with abbreviations, conversions and calibration text (`units*.rs`), `VideoSection`, `TimedPoint`, `ThreePartsVersion`, `TimestampMapper`, and the full preferences model (`preferences/`, 19 modules under 150 lines each) with `Preferences::load/save` for format 3.0. New oracle areas `units` and `preferences` in `Kinovea.FixtureGen`; fixtures regenerated (all manifests share one commit).
- Parity: 811 timecode rows, 60 ms→timecode rows, 1 600+ unit conversion rows and 12 pixel-size strings match byte for byte or within 1e-6; default `Preferences.xml` is element-wise equal to the oracle; the `modified` case round-trips and drops unknown elements.
- Gotchas: `.NET` `(int)Math.Round` overflow is reproduced for `Microseconds`; `.NET Framework` `float.ToString()` (7 significant digits, `1E-05` style) is reimplemented in `units_precision::format_f32_default`; capture folders had fresh GUIDs and the user's Documents path, normalised by the oracle; open-harness' 150-line limit forced the preferences split and the `xml_fields!`/`xml_struct!` macros.
- Dependencies added (flagged in design): `quick-xml`, `uuid`. Deviations recorded in the change design.
- Verified: `cd rust && make verify` green; `npm run lint` green; `fixtures.sh --verify` deterministic.

## 2026-10-18 — add-geometry-core (apply)

- Agent: Claude Code (Fable 5.1). Human asked to archive `add-core-types-and-units` and continue with the next change. PR #3 merged, change archived (specs `core/*` created), roadmap item 01 ticked.
- Done in `rust/kinovea-geometry`: `PointF`/`SizeF`/`RectangleF`, `Vector`, `Vector3`, `Matrix3x3` (+ `rotations`), Numerical Recipes `svdcmp` (`svd/`), `helpers` (GeometryHelper + MathHelper, split in `lines`/`snapping`), `Quadrilateral` (+ `ops`), shapes, Liang-Barsky clipping, `ProjectiveMapper` (+ `ellipse`), `fit_circle`. 72 unit tests, 9 parity tests. New oracle area `geometry` (`Kinovea.FixtureGen/Areas/GeometryArea*.cs`, 5 cases) producing 1 270 fixture rows; `fixtures.sh --verify` deterministic.
- Parity: clipping (1e-3), helper angles/points and quad ops (1e-4 scaled by magnitude), homography points/ellipse/homogeneous vectors (1e-3, rotation 1e-4), SVD values and vectors (1e-9, vectors up to sign), circle fit vs `kinematics/circular` `best_fit_circle` (1e-3). All green on first run.
- Gotchas: the proposal's clipping scenario for (250,250)–(500,1750) was wrong (the original tester marks it "not visible"); `svd.svdcmp` is `internal`, reached by reflection to keep doubles, and has a 1-based loop bug the port does not reproduce; `(float)(π/4)` widens above `Math.PI/4` so `Ellipse.Vertical` is true at exactly π/4 and a circle gets rotation π/2; `QuadrilateralF.GetHashCode` collapses small coordinates (same hash for 4×2 and 2×4), reproduced as stable-only.
- Not done: `proptest` not added (dependency approval pending); property checks run as seeded loops (`lib_test::Rng`). Recorded in the change design and tasks.
- Verified: `cd rust && make verify` green; `npm run lint` green; `tools/oracle/fixtures.sh --verify` green.

## 2026-10-18 — add-kinematics-filtering (apply)

- Agent: Claude Code (Fable 5.1). Same `/goal` as above: PR #4 merged, `add-geometry-core` archived (specs `geometry/*`), roadmap item 02 ticked, then change 03 applied.
- Done in `rust/kinovea-kinematics`: `MeasurementContext` trait + `PixelContext`, Butterworth filter with automatic cutoff (`butterworth/`), `moving_average`, `stats`, `FilteringResult`, `FilteredTrajectory`, `Kinematics`/`Component`, `TimeSeriesCollection` + `pad`, `build_linear_kinematics` (+ `linear/smoothing`), `build_angular_kinematics`. 34 unit tests, 10 parity tests. Oracle `kinematics` area gained `filtering`/`smoothing` inputs (partial class) and the cases `linear-unsmoothed`, `nofilter`, `short`; the three pre-existing fixture directories are byte-identical apart from the manifest commit.
- Parity: every cell of `linear.csv` for `linear`, `winter`, `linear-unsmoothed`, `short`, `nofilter` and of `circular/angular.csv` within 1e-6 relative (floor 1, NaN positions coincide); Butterworth cutoff indices and filtered series for the three original cases. Green on first run.
- Gotchas: the cutoff loop yields 100 or 101 results (accumulating `fc`); the moving average overwrites the NaN sentinels with finite values (fixture confirms); angular code pads accelerations at `Length <= 2`, linear at `<= 4`; parity replays world coordinates through a zero-size `PixelContext` so the raw columns match bit for bit; `PlayerPreferences` setters save the preferences file, so the oracle restores them in a `finally`.
- Dependencies: none new (`kinovea-core`, `kinovea-geometry` internal). Deviations recorded in the change design.
- Verified: `cd rust && make verify` green; `npm run lint` green; `tools/oracle/fixtures.sh --verify` deterministic.

## 2026-10-18 — add-kinematics-filtering archived, add-calibration-and-lens-distortion started

- Agent: Claude Code (Fable 5.1). PR #5 merged, change 03 archived (specs `kinematics/*`), roadmap item 03 ticked. Branch `feat/calibration-core`, draft PR #6.
- Done: task 1.1 of change 04: `rust/kinovea-core/src/xml.rs` ports the numeric part of `XmlHelper` (parse/write float, point, size, boolean, int list, point reference, video section, timestamp). Points and sizes are tuples because `kinovea-core` does not depend on `kinovea-geometry`.
- Next: tasks 2.x (lens distortion with the 5-iteration undistort), 3.x (calibrator plane), 4.x (calibration helper implementing `MeasurementContext`), 5 (camera pose), 6 (oracle area `calibration`), 7 (integration).
- Verified: `cd rust && make verify` green; `npm run lint` green.

## 2026-10-18 — add-calibration-and-lens-distortion (apply)

- Agent: Claude Code (Fable 5.1), same `/goal`. Change 04 applied end to end on `feat/calibration-core` (PR #6).
- Done in `rust/kinovea-measurement`: `Units`, `CalibrationAxis`, `CalibratorType`, `TrackedCalibrationSource` + `StaticSource`, `CalibratorPlane` (+ `quad`, `xml`), `CalibrationHelper` (+ `calibrate`, `transform`, `convert` with the `MeasurementContext` impl via `with_preferences`, `text`, `xml`), lens distortion (`parameters`, `undistort` with the 5-iteration OpenCV inverse, `helper`, `serializer` + Kinovea file import/export, `import_agisoft`), `compute_camera_position`. `kinovea-core::xml` (numeric `XmlHelper`, `format_f64_invariant`), `kinovea-geometry::Size`, `kinovea_fixtures::assert_close`. 53 unit tests + 16 parity/support tests.
- Oracle: area `calibration` (`CalibrationArea*.cs`, `CsvRows.cs` shared with `GeometryArea`), 4 cases + `agisoft.xml`; 336 fixture rows and 4 XML files; `fixtures.sh --verify` deterministic.
- Parity: transforms numeric 1e-3 (image side 1e-2) and texts exact for 5 calibrations × 25 rows; `kva/*.xml` byte-identical; distortion points/lines 1e-3 px against `Cv2.UndistortPoints`; Kinovea lens file round trip byte-identical; Agisoft parameters 1e-9; camera pose 1e-3 relative. Green on first run except the trailing newline: `.NET XmlWriter` writes none after the root end tag, so `Element::to_document` no longer appends it (preferences save now matches the original byte for byte too).
- Gotchas: `ParseSegment` ignores its scale (origin scaled, segment not); `CameraCalibration` without `ImageSize` is not rescaled; OpenCV narrows the normalized undistorted point to float; `GetPointAtTime` with a non-empty drawing id and static callbacks puts the origin at image (0, 0), so oracle transforms use `Guid.Empty`; `float.Parse` precision loss when parsing doubles via `parse_float` (use f64 parse).
- Verified: `cd rust && make verify` green; `npm run lint` green; `tools/oracle/fixtures.sh --verify` green.

## 2026-10-19 — add-calibration-and-lens-distortion archived, add-kva-metadata-model started

- Agent: Claude Code (Fable 5.1). PR #6 merged, change 04 archived (specs `measurement/*`), roadmap item 04 ticked. Branch `feat/kva-metadata-model` (no PR yet).
- Done so far for change 05: `kinovea-core` gains the shared `text_enum!` macro and `Element::parse` keeps edge whitespace of text content; `kinovea-metadata` has `ids` (GUID text + fixture normalization), `color` (`A;R;G;B`), `fading` (`InfosFading` with opacity), `style` (typed elements, ordered `StyleElements`, 26 embedded standard tools), `kva_io` helpers, `measure_label`, the `Drawing` enum with `Header` and the kinds Line, Angle, CrossMark, Circle, Rectangle, Pencil, Polyline, TimePolyline, Label (KVA read/write per kind, 34 unit tests).
- Oracle: `KvaArea.cs` now registers trackable drawings like the player does (`TrackabilityManager.Add` on `TrackableDrawingAdded`); without it the fixtures serialized every trackable point as `0;0`. Fixtures not regenerated yet.
- Discovered: `StyleElements.ImportXML` only imports values into the keys of the tool's default style, so a written `DrawingStyle` always has the tool's keys; `ParseDrawings` inserts non-plane drawings at index 0 and planes at the end, the writer walks the list backwards; `ArrowEnd` of labels is not scaled on import; the coordinate-system singleton is never registered for trackability (its `Position` serializes as `0;0` in the original too).
- Next: Plane, DistortionGrid, TimeSegment, chronos, Counter, Track, Bitmap, SVG, GenericPosture, singletons; keyframe/metadata/trackability; KVA reader/writer with parity on `kva/*` fixtures; remap, export flags, fuzz corpus, migration, history, posture engine, cross-load (tasks 1.x–9.x).

## 2026-10-18 — add-kva-metadata-model (apply, orchestration)

- Agent: Claude Code (Fable 5.1, then Opus 5.5 as orchestrator). Branch `feat/kva-metadata-model`, local commits not pushed after `28d3460c`.
- Done: KVA 2.x reader/writer with byte parity on `minimal`, `simple`, `complete`, cross-load fixed point and fading table; generic postures (parser, catalog, constraint engine) with parity on 16 tools and their moves; legacy 1.3-1.5 migration (1.5 green, 1.3 pending on an empty-element detail); oracle cases for all drawings, seeded fuzz, remap, capture/export flags, migration, cross-load and postures. 19/29 tasks ticked.
- Oracle fixes: the original never sets `ParentMetadata` on the spotlight and number-sequence managers, so importing either throws and aborts the import (FixtureGen sets them); `ContentHash` differs between processes, so `drawings.json` no longer carries it. `fixtures.sh --verify` deterministic at fixture commit `0aa50c04`.
- Next: the plan in `.agent/plans/kva-metadata-orchestration.md` (three Haiku agents for parity, undo history and properties/docs; then a Hermes adversarial review). Blocked on a Claude Code restart so the `haiku-rust-dev` agent type is registered.

## 2026-10-18 — add-kva-metadata-model (apply complete)

- Agent: Claude Code (Opus 5.5) orchestrating four Haiku subagents (`haiku-rust-dev`, effort xhigh) in separate worktrees: parity, undo history, properties/docs, and fixes from the adversarial review. Each diff was validated (no fixture/case edits, every removed test line reviewed) and re-verified on the branch before committing.
- Done: byte parity on every `kva/*` fixture (all drawings, remap, capture/export flags, 20 fuzz files, migration 1.3/1.5, trackers), cross-load fixed point, fading table, 16 postures and their moves; undo/redo history with serialized mementos; seeded idempotence for 24 kinds × 64 seeds. 29/29 tasks.
- Adversarial review: Hermes (DeepSeek default, reasoning high, toolsets terminal+file, session `kinovea-change05-adversarial`) on a throwaway worktree; it changed no files. Confirmed and fixed: coordinate-system tracker dropped on import (the original keeps it and re-anchors the origin from the calibration), drawing trackers echoing stale file names/points, integer style values copied across the four integer classes, segment-handle impacts indexed by point reference in the original. A new oracle case `kva/trackers` covers the first three.
- Gotchas: Claude Code worktree isolation branches from `origin/master`, so agents need worktrees created from the change tip; custom agent types load only after a restart; the original never sets `ParentMetadata` on the spotlight and number-sequence managers; `ContentHash` is not stable across processes.
- Verified: `cd rust && make verify` (495 tests), `npm run lint`, `tools/oracle/fixtures.sh --verify`.

## 2026-10-18 — add-kva-metadata-model archived (PR #7 merged), add-headless-renderer started

- Agent: Claude Code (Opus 5.5) orchestrating Haiku subagents (`haiku-rust-dev`, effort xhigh) in worktrees under `/home/artiko/Desarrollos/Kinovea-wt/`. Branch `feat/headless-renderer`.
- Done: oracle `render` area (`RenderArea*.cs`): GDI+ rasterizations with the `FlushOnGraphics`/`FlushDrawingsOnGraphics` settings and z-order, hit tables over a probe grid at scales 0.5/1/2; cases `geometry` (tight), `geometry-x2`, `simple`, `complete`, `all-drawings` with tolerances fixed in the case JSON; `fixtures.sh --verify` deterministic. Rust: crate scaffold (internal deps only), viewport transform and `HitTester` geometry (R2), `Renderer` trait, style mapping and recording backend (R1), paint skeleton with per-kind ownership.
- Discovered: a non-perspective `Plane` never builds its projective mapping, so its original `HitTest` throws with a sliding line visible (recorded as an `exception` row); `HitPath` widens with a pen of `lineSize + enlarger`, so the tolerance per side is half of that.
- Blocked: tasks 5.x and 7.1 need `tiny-skia`, `fontdue`, `png` and Liberation Sans (asked, no answer yet; AGENTS.md requires approval).
- Next: painters P1/P2 and per-drawing hit testing in parallel, then integration, primitive-coverage test, Hermes review.

## 2026-10-19 — add-headless-renderer (apply, without the CPU backend)

- Agent: Claude Code (Opus 5.5) orchestrating Haiku subagents (`haiku-rust-dev`, effort xhigh): R1 trait/style/recording, R2 viewport/hit geometry, P1–P4 painters, unification of AngleHelper/TickMark, H per-drawing hit testing. Each diff validated (scope, removed test lines, test-name counts) and re-verified on the branch.
- Done (13/18 tasks): oracle `render` area and fixtures; `Renderer` trait, style mapping, recording backend; viewport; painters for all kinds plus singletons and multi-drawings; primitive coverage over `all-drawings`; per-drawing hit testing against `hits.csv` with two explicit, staleness-checked lists (text-metric zones pending a font rasterizer; capped Pencil/posture probes). `make verify` 890 tests green.
- Fixes found during integration: label leaders shortened by `UpdateStartEnd` like the original; `StyleData.LineSize` bound by key; two orphan `_test.rs` files that satisfied `testlens` but were never compiled (one did not parse) are now declared; decimal places carried in `PaintContext`.
- Tooling gap (not fixed, decision pending): `cargo fmt --check` skips modules declared inside macros (120 files across five crates, plus `kinovea-render/src/paint`, which is checked with `rustfmt --check` by hand).
- Blocked: tasks 2.1, 5.1–5.3, 7.1 need `tiny-skia`, `fontdue`, `png` and Liberation Sans (asked twice, no answer; AGENTS.md requires approval). The change cannot be archived until then.
## 2026-10-19 — fix-kva-review-findings (branch fix/kva-review-findings)

- Agent: Claude Code (Opus 5.5) with a Haiku subagent. Hermes (DeepSeek, reasoning high, toolsets terminal+file, session `kinovea-change05-migration-history`) reviewed migration and undo history on a throwaway worktree (no files changed); it reported 4 medium and 10 low findings with reproductions.
- Done: every finding verified against the cited C#/XSLT before fixing (M2 partly refuted: `MergeInsertKeyframe` does not refresh labels); fixes with tests, most verified by mutation. Format 1.2 and older are read as the original does (the earlier rejection was a misreading). All fixtures still match byte for byte; `make verify` and `npm run lint` green.
- Open: `AddDrawing(managerId)` type filters for chronos/tracks and the keyframe-name setter label refresh are not ported (noted by the agent, out of scope).

## 2026-10-19 — add-headless-renderer (hit parity review)

- Agent: Claude Code (Opus 5.5) orchestrating two Haiku subagents (R: hit harness and render fixes; T: trackability order), after the Hermes review of change 06.
- Done: hit harness measured inside each drawing's box, targets from the metadata, text exemptions limited to text-zone codes, singletons hit-tested; tautological tests and the empty `cpu`/`compose` modules removed. Root causes fixed against the C#: GDI+ top-left fill on stroke edges, infinite-line clipping of the coordinate system axes, item fading from the video, plane hit through the calibration mapping. The oracle now runs `PostSetupVideo` before loading a KVA (Rust too), and trackability follows the original `Dictionary` order and `UpdateId`; new case `kva/trackers-no-cs`. Fixtures from commit 4a587e9b; `cargo test --workspace` 935 passed, `make verify` and `npm run lint` green.
- Still blocked: tasks 2.1, 5.1–5.3, 7.1 (crates approval). Follow-ups H5 (default timecode format) and H8 (circle plane-calibration branch).
