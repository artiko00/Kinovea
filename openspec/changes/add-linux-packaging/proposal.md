# Proposal

## Why

Until now the Rust fork is only runnable from a `cargo` checkout. The original ships as an NSIS installer and a portable zip, both Windows artifacts produced by `build.yml`, with an in-app updater polling `kinovea.org`. For Linux users to install the fork, and for the Rust version to become the release artifact, the project needs Linux packages, a release pipeline and a single source for the version number, which today is duplicated between the two workflows, the NSIS script and `AssemblyInfo.cs`.

## What Changes

- Add Linux packaging: an AppImage built from the release binary, and a Flatpak manifest (`org.kinovea.Kinovea`) with desktop entry, icon set rendered from the existing 256 px icon, MIME type for `.kva` files and AppStream metadata.
- Single-source the version in `rust/Cargo.toml` (`workspace.package.version`), read by the binary (`kinovea --version`), by the packaging scripts and by CI; remove the `VERSION` env duplication from `rust.yml`.
- Add a release workflow triggered by tags `v*` that builds the AppImage and Flatpak bundle on `ubuntu-latest`, runs smoke checks and attaches them to a GitHub release (workflow addition requires approval).
- Replace the updater: on startup, when enabled in Preferences > General, query the GitHub releases feed of the fork and show a non-blocking notice with a link; no download or self-update.
- Add packaging checks that run in CI: `desktop-file-validate`, `appstreamcli validate`, `kinovea --version` executed from the AppImage, opening a sample `.kva` by file association.

## Capabilities

### New Capabilities
- `packaging/linux-bundles`: installable AppImage and Flatpak with desktop integration and `.kva` association.
- `packaging/release-pipeline`: version single-sourcing, tag-driven release builds and artifact smoke checks.
- `packaging/update-notice`: opt-in notification of newer releases.

### Modified Capabilities
None.

## Impact

- **Assemblies superseded**: `Installer/*` (NSIS, signing scripts), `Kinovea.Updater/*`. Neither is modified here; they are removed in `remove-dotnet-solution`. `build.yml` is untouched until then.
- **New files**: `packaging/appimage/`, `packaging/flatpak/`, `packaging/share/` (desktop, icons, MIME, metainfo), `.github/workflows/release.yml`, `rust/kinovea/src/update.rs`.
- **Dependencies**: `linuxdeploy` and `appimagetool` downloaded in CI; `flatpak-builder` with the `org.freedesktop.Platform` runtime; `ureq` already present from the HTTP camera backend for the release feed. Decisions requiring approval: the Flatpak application id, the runtime version, the workflow.
- **Verification**: Linux only, in CI and locally with `make package`. No Windows artifact is produced by this change.
- **KVA**: unchanged; the MIME type registers the existing `.kva` extension and XML root element.

## Non-goals

- Code signing or notarization of Linux bundles.
- Distribution packages (`.deb`, `.rpm`, AUR); Flatpak and AppImage cover the targets first.
- Self-updating binaries or delta updates.
- Publishing to Flathub (needs the upstream project's agreement; documented as a follow-up).
