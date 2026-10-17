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
