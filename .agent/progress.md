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
- Pending: see tasks.md of the change for CI run (needs push) and fresh-clone check.
- Next: `add-core-types-and-units`, starting with `tests/fixtures/timecode/formats/*.csv` as red tests.
