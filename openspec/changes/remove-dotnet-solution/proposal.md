# Proposal

## Why

Once changes 01 to 19 are archived the Rust fork covers playback, annotation, measurement, tracking, import/export, the MCP server, capture, localization and packaging on Linux. Keeping the .NET solution alive after that costs a Windows CI job, 21 MB of vendored SDKs in `Refs/`, a Windows oracle host, and two code bases that drift. This change retires the C# and C++/CLI code so the repository holds a single stack, while freezing the parity fixtures as the permanent regression baseline.

## What Changes

- Delete every C# project (`Kinovea`, `Kinovea.ScreenManager`, `Kinovea.Services`, `Kinovea.FileBrowser`, `Kinovea.Pipeline`, `Kinovea.Camera*`, `Kinovea.Video*`, `Kinovea.Updater`, `Kinovea.Tests`), the C++/CLI project `Kinovea.Video.FFMpeg`, `Kinovea.VS2019.sln`, `Kinovea.targets`, `Directory.Build.props`, `packages.config` files, `VideoTests/`, `Installer/`, `Refs/`, legacy `Tools/` (after moving `Tools/XML/Schema` and `Tools/DrawingTools` data to `assets/`), and `.github/workflows/build.yml`.
- Retire the parity oracle generator: delete `Kinovea.FixtureGen`, `tools/oracle/`, and the Windows host provisioning. `tests/cases/` and `tests/fixtures/` stay as the frozen baseline; the `tooling/parity-oracle` spec is amended so fixtures are immutable and manifests still enforce a single commit.
- Remove the legacy entries from the linter `exclude` lists so the quality gates cover the whole repository, and drop the `.resx` and `.cs` related rules from `AGENTS.md`, `openspec/config.yaml`, `.editorconfig`, `.gitattributes`, `.gitignore`.
- Rewrite `readme.md`, `architecture.md` and `localization.md` for the Rust application; `AGENTS.md` loses the Windows commands.

## Capabilities

### New Capabilities
- `tooling/single-stack-repository`: the repository contains one buildable stack and the gates, docs and CI reflect it.

### Modified Capabilities
- `tooling/parity-oracle`: generation requirements are removed; fixtures become an immutable baseline with provenance still verified.
- `tooling/rust-workspace`: the quality-gates requirement widens from "Rust sources" to the whole repository with no legacy exclusions.

## Impact

- **Assemblies touched**: all 22 projects of the solution are deleted. No C# code is modified. After this change nothing in the repository compiles on Windows, and the only CI is `rust.yml` plus the release workflow from `add-linux-packaging`.
- **Verification**: `make verify`, `npm run lint`, the packaging smoke checks, and the parity checklist signed off by the human before deletion. There is no C# build to verify anymore; that is the point.
- **KVA**: unchanged. The schemas move to `assets/schema/` and the Rust metadata crate's embedded copies are the reference.
- **Approval**: every deletion in this change is destructive and irreversible on `master`; the change requires explicit approval of the parity checklist and of each workflow and `.gitignore` edit before execution.

## Non-goals

- Rewriting git history or removing the legacy code from past commits; it remains reachable through tags (`legacy-dotnet-final` is created first).
- Supporting Windows with the Rust build.
- Changing any Rust behavior; this change is removal and documentation only.
