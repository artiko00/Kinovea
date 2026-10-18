# Proposal

## Why

Kinovea is being migrated in full from C# .NET Framework 4.8 to Rust so that it can be developed, tested and run on Linux, and later driven headlessly by an MCP server. Every later change needs two things that do not exist yet: a Rust workspace that compiles and tests on Linux, and a trustworthy source of reference outputs from the original C# implementation to drive TDD. This change creates both before any domain code is ported.

## What Changes

- Add a Cargo workspace at the repo root (`rust/`) with an initial crate layout mirroring the target architecture, shared lint configuration, and a CI job on `ubuntu-latest` running `cargo fmt --check`, `cargo clippy`, `cargo test`.
- Add the parity oracle: a C# console project `Kinovea.FixtureGen` that exercises the original logic (KVA serialization, kinematics, calibration, exporters) over versioned input cases and writes deterministic fixtures.
- Add oracle scripts: `tools/oracle/build.ps1` (runs on the Windows host: restore, fetch FFmpeg, MSBuild Release x64, run FixtureGen) and `tools/oracle/fixtures.sh` (runs on Linux: invokes the host over SSH, pulls results into `tests/fixtures/`).
- Provision the Windows host once with Visual Studio Build Tools 2022 (.NET desktop, .NET Framework 4.8 targeting pack, MSVC v143, C++/CLI) via `winget`. Requires explicit approval before execution.
- Point `testlens` at Rust and extend linter excludes for `rust/target`.
- Document the migration roadmap and the oracle workflow in `AGENTS.md` and `.agent/progress.md`.

No existing C# behavior changes. No user-facing feature is added.

## Capabilities

### New Capabilities
- `tooling/rust-workspace`: the Rust workspace builds, lints and tests on Linux with a single command, and CI enforces it on every push.
- `tooling/parity-oracle`: reference fixtures are produced deterministically from the original C# implementation on the Windows host and versioned in the repo, so Rust tests never depend on Windows at run time.

### Modified Capabilities
None. There are no existing specs.

## Impact

- **Assemblies touched**: a new C# project `Kinovea.FixtureGen` referencing `Kinovea.Services` and `Kinovea.ScreenManager`. It is added to `Kinovea.VS2019.sln`. Existing assemblies are not modified.
- **New top-level paths**: `rust/`, `tools/oracle/`, `tests/fixtures/`.
- **Workflows**: `.github/workflows/rust.yml` added. `build.yml` untouched except for the new project compiling as part of the solution.
- **Dependencies**: Rust toolchain stable (pinned via `rust-toolchain.toml`). Windows host gains Build Tools 2022.
- **Verification**: Rust side is fully verified on Linux (`cargo test`, CI). The C# side (FixtureGen) is verified by building and running it on the Windows host over SSH; GitHub Actions `build.yml` remains a secondary check that the solution still compiles.

## Non-goals

- Porting any domain logic to Rust (starts in `add-core-types-and-units`).
- Choosing or integrating the UI toolkit beyond reserving the crate name.
- Running the C# UI on Linux, or any Mono/Wine/Docker path.
- Windows as a target for the Rust build.
