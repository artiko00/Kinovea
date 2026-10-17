# Tasks

## 1. Rust workspace

- [x] 1.1 Create `rust/Cargo.toml` workspace, `rust/rust-toolchain.toml` pinned to stable 1.93.1, `rust/rustfmt.toml`, `[workspace.lints]` denying clippy warnings; verify `cargo metadata` lists the workspace and `cargo build` succeeds on an empty workspace.
- [x] 1.2 Add the library crates `kinovea-core`, `kinovea-geometry`, `kinovea-kinematics`, `kinovea-measurement`, `kinovea-metadata`, `kinovea-render`, `kinovea-video`, `kinovea-tracking`, `kinovea-export`, each with a `lib.rs` exposing a `crate_name()` function and one unit test asserting it; verify `cargo test --workspace` reports one passing test per crate.
- [x] 1.3 Add the `kinovea` binary crate with `--version` output taken from `Cargo.toml` and an integration test in `rust/kinovea/tests/cli.rs` that spawns the binary and checks exit 0 and the version string; verify the test passes.
- [x] 1.4 Write `kinovea-fixtures` with `fixtures_root()`, `Manifest` (commit, ffmpeg_version, case_sha256) parsed from `manifest.json`, `approx_eq(a, b, tol)`; write failing tests first for manifest parsing and for a missing-fixture error message naming the path, then implement; verify `cargo test -p kinovea-fixtures` passes.
- [x] 1.5 Add `rust/justfile` (or `Makefile`) target `verify` running `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`; verify `just verify` exits 0 on the clean tree and exits non-zero after injecting an unused variable in a scratch test.
- [ ] 1.6 Commit `rust/Cargo.lock` and add `rust/target/` to `.gitignore`; verify `git status` shows no build output.

## 2. Continuous integration and quality gates

- [ ] 2.1 Add `.github/workflows/rust.yml` on `ubuntu-latest` with `dtolnay/rust-toolchain@stable` reading `rust/rust-toolchain.toml`, `Swatinem/rust-cache`, and the three verify commands with `working-directory: rust`; verify with `act` locally or by pushing a branch and observing the job pass.
- [x] 2.2 Switch `testlens.json` language to `rust`, add `rust/target` and `tests/fixtures` to the exclude lists of the four linters in `package.json` and `testlens.json`; verify `npx testlens check` reports no Rust file (all have tests) and `npx linelens check` scans no file under `rust/target`.
- [x] 2.3 Add a test in `kinovea-fixtures` that asserts every `tests/fixtures/**/manifest.json` shares the same `commit`; verify it passes with zero fixtures and fails when two manifests with different commits are placed in a temp dir.

## 3. Oracle host provisioning (requires explicit approval before 3.2)

- [x] 3.1 Write `tools/oracle/provision.ps1` that checks `vswhere` for MSBuild and the C++/CLI component and, if missing, prints the `winget install Microsoft.VisualStudio.2022.BuildTools --override "..."` command with the four components from design D2 without running it unless `-Install` is passed; verify by running it over `ssh windows` in check mode and reading the printed command.
- [x] 3.2 After approval, run `provision.ps1 -Install` on the host; verify `vswhere -products * -requires Microsoft.Component.MSBuild -find MSBuild\**\Bin\MSBuild.exe` returns a path and `MSBuild.exe -version` prints 17.x.
- [x] 3.3 Write `tools/oracle/build.ps1`: clone or fetch the repository into `C:\oracle\Kinovea` at a given commit, `nuget restore`, run `Refs/get-dependencies.ps1` only when `Refs/FFmpeg` is absent, `msbuild Kinovea.VS2019.sln /p:Platform=x64 /p:Configuration=Release`; verify it exits 0 twice in a row on the unchanged solution and the second run skips the FFmpeg download.

## 4. FixtureGen (C#)

- [x] 4.1 Add `Kinovea.FixtureGen/Kinovea.FixtureGen.csproj` (console, net48, x64, log4net) to `Kinovea.VS2019.sln` referencing `Kinovea.Services` and `Kinovea.ScreenManager`, with `Program.cs` parsing `--cases <dir> --out <dir> [--verify]` and failing on unknown area; verify `build.ps1` compiles it and `Kinovea.FixtureGen.exe --cases <empty> --out <tmp>` exits 0.
- [x] 4.2 Implement the case format `tests/cases/<area>/<name>.json` with `area`, `seed`, `inputs`, and the `manifest.json` writer (commit from `git rev-parse HEAD`, FFmpeg version from `Refs/FFmpeg` folder name, case SHA-256), forcing `CultureInfo.InvariantCulture` and LF endings; verify a run on a sample case writes the manifest with the three fields and that a second run yields an identical tree (`--verify` mode compares and exits non-zero on diff).
- [x] 4.3 Implement area `kva`: load each of `Tools/XML/Schema/samples/*.kva` through `MetadataSerializer`, re-serialize, and write `roundtrip.kva`; add three cases; verify fixtures exist for `minimal`, `simple`, `complete` and the round trip is deterministic.
- [x] 4.4 Implement area `kinematics`: link `Kinovea.Tests/Kinematics/*.cs` via `<Compile Include>` to synthesize trajectories with given seed, run `LinearKinematics` and `AngularKinematics` with Butterworth filtering, write `series.csv` with `R` formatting; add two cases (linear, circular); verify output columns are documented in `tests/cases/kinematics/README.md` and the run is deterministic.
- [x] 4.5 Implement area `timecode`: format a fixed list of timestamps with every `TimecodeFormat` and frame rates 25, 29.97, 60, write `timecodes.csv`; verify the fixture matches the expectations encoded in `Kinovea.Tests/Time/TimecodeFormatTest.cs`.
- [ ] 4.6 Confirm the Windows CI build still passes with the new project and that `Installer/kinovea.nsi` and the portable archive exclude `Kinovea.FixtureGen.exe`; verify by reading the NSIS file list and the workflow run artifacts.

## 5. Linux driver and documentation

- [x] 5.1 Write `tools/oracle/fixtures.sh` with `ORACLE_HOST` (default `windows`), steps: ssh connectivity check, `build.ps1` at the current local commit, run FixtureGen, `scp -r` into `tests/fixtures/`, optional `--verify`; verify a full run from Linux populates `tests/fixtures/kva/*/manifest.json` and that an unreachable host exits non-zero without touching `tests/fixtures/`.
- [x] 5.2 Write Rust tests in `kinovea-fixtures/tests/smoke.rs` that load every committed fixture manifest and check the case checksum against `tests/cases`; verify `cargo test --workspace` passes offline with the committed fixtures.
- [x] 5.3 Update `AGENTS.md` (Critical commands: `just verify`, `tools/oracle/fixtures.sh`; Structure: `rust/`, `tests/`, `tools/oracle/`; Boundaries: never hand-edit `tests/fixtures/`), append a `.agent/progress.md` entry with the migration roadmap and oracle decisions, and add `tests/README.md` describing cases, fixtures, manifests and tolerances; verify `AGENTS.md` stays under 150 lines and `npm run lint:secrets` passes.

## 6. Integration check

- [ ] 6.1 From a fresh clone on Linux run `cd rust && just verify` and `tools/oracle/fixtures.sh --verify`; verify both exit 0 and `git status` is clean afterwards.

## Workflow follow-up

- Archive the change once CI is green on the branch and fixtures are committed.
- Start `add-core-types-and-units` using the `timecode` fixtures as its first red tests.
