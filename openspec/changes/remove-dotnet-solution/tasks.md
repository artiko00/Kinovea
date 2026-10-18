# Tasks

## 1. Preconditions and parity checklist (human sign-off required before group 3)

- [ ] 1.1 Write `rust/kinovea-fixtures/tests/coverage_test.rs` asserting every `tests/fixtures/<area>` directory is read by at least one Rust test (areas listed in a `const` kept in sync) and verify it passes; any unlisted area blocks this change until the owning change is fixed.
- [ ] 1.2 Add `openspec/changes/remove-dotnet-solution/parity-checklist.md` listing, per area (core, geometry, kinematics, calibration, metadata, render, video, tracking, io, mcp, ui, capture, i18n, packaging): fixture tests green, `egui_kittest` suites green, one manual Linux scenario, Weblate switched to Fluent, `release.yml` produced a pre-release; verify the human has ticked every line (do not proceed otherwise).
- [ ] 1.3 Verify `openspec list --json` shows changes 01 to 19 archived and `openspec/roadmap.md` ticked through 19; record the outcome in `.agent/progress.md`.

## 2. Relocate assets and freeze the baseline

- [ ] 2.1 Move `Tools/XML/Schema/**` to `assets/schema/`, `Tools/XML/Kva to Kva/**` to `assets/schema/migrations/`, `Tools/DrawingTools/{Standard,Custom,Toolbars}` to `assets/drawing-tools/`; switch `kinovea-metadata` and `kinovea-ui` `include_dir!` paths; write `rust/kinovea-metadata/src/assets_test.rs` (schema count, migration XSLT count, 44 posture files, toolbars present) first; verify `cd rust && make verify`.
- [ ] 2.2 From a clean clone with `Tools/`, `Refs/` and all `Kinovea*` directories temporarily removed, run `make verify`, `kinovea --check-open assets/schema/samples/simple.kva` and `make -C packaging package smoke`; verify all exit 0 and record the commands in `.agent/progress.md`.
- [ ] 2.3 Add `baseline_commit:` to `tests/README.md` with the manifests' commit, extend `rust/kinovea-fixtures/tests/provenance_test.rs` to compare against it (test first), add `tools/fixtures-frozen.sh` and a lefthook `pre-commit` entry that fails on changes under `tests/fixtures/` or `tests/cases/`; verify the hook blocks a scratch edit and `cargo test -p kinovea-fixtures` passes.
- [ ] 2.4 Create and push annotated tag `legacy-dotnet-final` on the current `master` commit (requires approval); verify `git describe --tags` on that commit prints the tag and the retirement entry in `.agent/progress.md` names it.

## 3. Delete the .NET solution (requires approval; one commit per task)

- [ ] 3.1 Delete `.github/workflows/build.yml` and the `build` status references in `readme.md`; verify `rust.yml` still runs on the push and no other workflow references MSBuild.
- [ ] 3.2 Delete `Kinovea.FixtureGen/`, `tools/oracle/`, the `tools/oracle/*.sh` line of `.gitattributes`, and the "Fixtures" row of `AGENTS.md`; verify `cargo test -p kinovea-fixtures` passes (fixtures unchanged) and `npm run lint` is green.
- [ ] 3.3 Delete the 20 C# project directories, `Kinovea.Video.FFMpeg/`, `Kinovea.Tests/`, `VideoTests/`, `Kinovea.VS2019.sln`, `Kinovea.targets`, `Directory.Build.props`, `Installer/`, `Refs/`, remaining `Tools/`; verify `git ls-files | grep -E '\.(cs|csproj|sln|vcxproj|nsi|resx|targets)$|packages\.config'` prints nothing outside `openspec/changes/archive/`.
- [ ] 3.4 Remove the legacy entries from `package.json` and `testlens.json` excludes, prune `.gitignore` (Visual Studio, `bin/`, `obj/`, `packages/`), `.editorconfig` (`.cs/.cpp/.h`), `.gitattributes`; write `tools/lint-excludes-exist.sh` asserting every exclude entry exists and wire it into lefthook; verify `npm run lint`, the new script and `cd rust && make verify` exit 0.

## 4. Documentation and configuration

- [ ] 4.1 Rewrite `readme.md`, `architecture.md` (crate map, Mermaid diagram, replace `Tools/Architecture/deps-highlevel.png`), `localization.md` (already Fluent, drop `.resx` remnants), and `AGENTS.md` (commands table, stack, structure, rules 5 and 6, recurring errors) for the single stack; verify every command in the AGENTS.md table runs or prints usage on Linux (`tools/agents-commands-test.sh`).
- [ ] 4.2 Rewrite `openspec/config.yaml` context and rules for the Rust stack and run `openspec update`; verify `openspec validate --all --strict` passes for the archived changes and `openspec instructions proposal --change <scratch>` shows the new rules (delete the scratch change afterwards).
- [ ] 4.3 Append the retirement entry to `.agent/progress.md` (tag, baseline commit, last Windows CI run URL, deleted paths) and tick 20 in `openspec/roadmap.md`; verify `npm run lint` and `cd rust && make verify` exit 0 from a fresh clone.

## Workflow follow-up

- Archive the change; the roadmap is complete.
