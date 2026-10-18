# Design

## Context

See proposal.md. State at the start of this change: changes 01 to 19 archived; `rust.yml` and `release.yml` green; `build.yml` still builds the .NET solution on `windows-latest`; `tests/fixtures/` holds fixtures for every ported area, all from one generator commit; `package.json`/`testlens.json` exclude 25 legacy directories; `AGENTS.md`, `architecture.md`, `readme.md`, `localization.md`, `openspec/config.yaml` describe both stacks. The Rust build embeds locales (18) but still reads `Tools/XML/Schema` and `Tools/DrawingTools` from their legacy paths (05, 14).

## Goals / Non-Goals

**Goals:**
- Removal is mechanical: each design "Legacy superseded" section of changes 01 to 19 enumerates what goes; this change only sums them up.
- Nothing that Rust tests or the application read at run time is lost.
- The legacy state stays one `git checkout` away.

**Non-Goals:**
- History rewriting, LFS migration, or repository renaming.

## Decisions

### D1. Parity checklist before any deletion (requires human sign-off)
A checklist in `tasks.md` (task 1.2) lists per area: fixture tests green, manual scenario run on Linux, and for the UI the `egui_kittest` suites. The human ticks it; deletion tasks are blocked until then. Alternative: delete progressively per change; rejected because the oracle needs the whole solution compiling (see roadmap "Legacy retirement").

### D2. Tag `legacy-dotnet-final` on the last commit containing the solution
Annotated tag pushed before the deletion commit. The retirement entry in `.agent/progress.md` and `tests/README.md` names the tag and the fixture baseline commit. Alternative: a `legacy` branch; a tag is immutable and sufficient.

### D3. Assets relocation
`Tools/XML/Schema/**` → `assets/schema/`, `Tools/XML/Kva to Kva/**` (migration XSLT) → `assets/schema/migrations/`, `Tools/DrawingTools/{Standard,Custom,Toolbars}` → `assets/drawing-tools/`. The Rust crates switch to `include_dir!` on the new paths in the same commit; FixtureGen case files that reference `Tools/XML/Schema/samples/*.kva` stay valid because `tests/cases/` is frozen and the smoke test hashes the case JSON, not the referenced path (the samples are copied, not moved, until the oracle is deleted in the same change; after deletion the path in the case JSON is informational only).

### D4. Fixture immutability gate
Add a lefthook/CI check `tools/fixtures-frozen.sh`: fails if `git diff --name-only <base>` touches `tests/fixtures/` or `tests/cases/`. The `kinovea-fixtures` provenance test gains the baseline commit constant read from `tests/README.md` front-matter (`baseline_commit:`).

### D5. Deletion order (one commit each, revertible)
1. assets relocation + Rust path switch; 2. tag; 3. delete workflows `build.yml`; 4. delete `Kinovea.FixtureGen`, `tools/oracle/`; 5. delete the 20 C# projects, C++/CLI project, `.sln`, `.targets`, `Directory.Build.props`, `VideoTests/`, `Installer/`, `Refs/`, remaining `Tools/`; 6. linter excludes, `.gitattributes` (drop `tools/oracle/*.sh`), `.gitignore` (drop `bin/`, `obj/`, `packages/`, `*.suo`, Visual Studio entries), `.editorconfig` (drop `.cs/.cpp/.h`), `openspec/config.yaml` context; 7. docs.

### D6. OpenSpec config
`openspec/config.yaml` context rewritten for the Rust stack; rules change: `.resx` task rule becomes "add English Fluent strings"; "CI build on Windows" statements become "`make verify` and packaging smoke". Run `openspec update` afterwards.

## Legacy superseded

Everything: the union of the "Legacy superseded" sections of changes 01 to 19, plus `Kinovea.FixtureGen/*`, `tools/oracle/*`, `Kinovea.Tests/*`, `VideoTests/*`, `Tools/{Architecture,UnusedResources,explorer-harness,fxcop,helptools,remote,tests-measurement}`, `Refs/*`, `Kinovea.VS2019.sln`, `Kinovea.targets`, `Directory.Build.props`, `.github/workflows/build.yml`.

## Risks / Trade-offs

- [A fixture area was never ported and nobody noticed] → task 1.1 cross-checks every `tests/fixtures/<area>` against a Rust test that reads it; unported areas block the checklist.
- [Run-time asset missed by the relocation] → task 2.2 runs the application's `--check-open` and the full `egui_kittest` suites from a clean clone with `Tools/` removed before the real deletion commit.
- [Contributors with open PRs touching C#] → announce the tag and the freeze date in the retirement entry; PRs against legacy code are closed with a pointer to the tag.
- [Weblate still pointed at `.resx`] → precondition from change 18's follow-up; verified in task 1.2.

## Migration Plan

Sequence D5; each commit passes `make verify`, `npm run lint` and the packaging smoke before the next. Rollback: revert the last commit; the tag guarantees the full legacy tree is recoverable at any time.

## Open Questions

None. Everything deferred (Flathub, Windows) is out of scope by decision.
