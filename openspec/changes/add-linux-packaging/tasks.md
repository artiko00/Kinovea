# Tasks

## 1. Version single source and assets

- [ ] 1.1 Add `packaging/version.sh` (reads `workspace.package.version` via `cargo metadata`) and `packaging/version_test.sh` (asserts it equals `kinovea --version` output); record the packaging tool decisions in `.agent/progress.md`; verify `bash packaging/version_test.sh` exits 0.
- [ ] 1.2 Write `rust/kinovea/src/version_test.rs` (binary prints `kinovea <workspace version>`; `--check-open <kva>` exits 0 on `Tools/XML/Schema/samples/simple.kva` and 1 on a missing file) then implement `--check-open` in `rust/kinovea/src/main.rs` using the headless renderer; verify `cargo test -p kinovea`.
- [ ] 1.3 Add `packaging/share/` (desktop entry, MIME xml, metainfo, scalable SVG icon and `icons.sh` rendering PNG sizes from `Kinovea/Resources/kinovea-256x256.ico`); add `packaging/share_test.sh` running `desktop-file-validate`, `appstreamcli validate --no-net` and `xmllint --noout` on the MIME file; verify the script exits 0 (tools installed locally with the distro package manager).

## 2. AppImage

- [ ] 2.1 Add `packaging/appimage/tools.lock` (URLs + sha256 for `linuxdeploy`, `appimagetool`) and `packaging/appimage/fetch-tools.sh` verifying checksums; verify a tampered checksum makes the script exit non-zero.
- [ ] 2.2 Add `packaging/Makefile` target `package` building `cargo build --release`, assembling the AppDir with the share assets, and producing `dist/Kinovea-<version>-x86_64.AppImage`; add `packaging/smoke.sh` (`--version` string check, `--check-open` on the sample KVA, desktop/metainfo validators inside the AppDir); verify `make -C packaging package smoke` exits 0 locally.
- [ ] 2.3 Add `packaging/appimage/Dockerfile` (`ubuntu:22.04` build image) and a `make package-in-container` target; verify the AppImage built there passes `smoke.sh` on a fresh `ubuntu:22.04` container with no extra packages.

## 3. Flatpak

- [ ] 3.1 Add `packaging/flatpak/org.kinovea.Kinovea.yml`, generate `cargo-sources.json` with `flatpak-cargo-generator` (script `packaging/flatpak/update-sources.sh`), set finish-args per design D2; verify `flatpak-builder --user --install build-dir packaging/flatpak/org.kinovea.Kinovea.yml` succeeds and `flatpak run org.kinovea.Kinovea --version` matches the workspace version.
- [ ] 3.2 Add `make -C packaging flatpak-bundle` producing `dist/Kinovea-<version>.flatpak` and extend `smoke.sh` to run the installed Flatpak's `--check-open`; verify both commands exit 0.
- [ ] 3.3 Write `packaging/flatpak/permissions_test.sh` asserting the manifest's finish-args equal the documented list (no `--filesystem=home` write); verify it fails when an extra permission is added and passes on the committed manifest.

## 4. Release workflow (requires approval before 4.1)

- [ ] 4.1 Add `.github/workflows/release.yml` on tags `v*`: version/tag mismatch gate, `make package-in-container`, Flatpak bundle, `smoke.sh`, `SHA256SUMS`, `softprops/action-gh-release`; verify by pushing a pre-release tag `v0.1.0-rc1` on a scratch branch and observing the release with both artifacts and checksums, then deleting the tag and release.
- [ ] 4.2 Add a `packaging` job to `.github/workflows/rust.yml` running `make -C packaging package smoke` on pull requests (no upload) and `bash packaging/share_test.sh`; verify the job is green on the branch.

## 5. Update notice

- [ ] 5.1 Write `rust/kinovea/src/update_test.rs` (component comparison incl. leading `v` and pre-release suffix, newer/equal/older cases, 24 h cache file) then implement `rust/kinovea/src/update.rs` with `ureq` and a 3 s timeout; verify `cargo test -p kinovea update`.
- [ ] 5.2 Write `rust/kinovea-ui/src/update_notice_test.rs` with `egui_kittest` (preference off: no request made against a local mock server; on: notice shown with version and link; offline: no dialog, Help > Check for updates reports unreachable) and implement the notice and the Help menu entry; add the English Fluent strings; verify test.
- [ ] 5.3 Add "Check for updates at startup" to Preferences > General (`rust/kinovea-ui/src/preferences/general.rs`, test round-trip in `general_test.rs`); verify `make verify`.

## 6. Documentation

- [ ] 6.1 Write `packaging/README.md` (install from AppImage/Flatpak, local build, release procedure, Flathub follow-up) and update `readme.md`, `AGENTS.md` (commands table: `make -C packaging package`, release workflow) and `.agent/progress.md`; verify `npm run lint` and `cd rust && make verify` exit 0.

## Workflow follow-up

- Human: approve `release.yml`, decide the Flatpak id if upstream answers, bump `workspace.package.version` to the first Kinovea-style release number.
- Archive the change; tick 19 in `openspec/roadmap.md`.
