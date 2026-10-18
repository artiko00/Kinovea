# Spec Delta

## Purpose

Produces versioned Linux release artifacts from a tag with one source of truth for the version number and automated smoke checks, replacing the Windows installer pipeline for the Rust fork.

## ADDED Requirements

### Requirement: Version single source
The version SHALL be defined once in the Rust workspace manifest and SHALL be the value reported by the binary, embedded in the bundles' metadata and used in artifact names.

#### Scenario: Consistent version
- **WHEN** the workspace version is `2026.2.0`
- **THEN** `kinovea --version` prints `kinovea 2026.2.0`, the AppImage is named `Kinovea-2026.2.0-x86_64.AppImage` and the AppStream release entry lists `2026.2.0`

#### Scenario: Tag mismatch rejected
- **WHEN** a release tag `v2026.2.1` is pushed while the workspace version is `2026.2.0`
- **THEN** the release workflow fails before building and names both values

### Requirement: Tag-driven release
Pushing a tag matching `v*` SHALL build the AppImage and the Flatpak bundle on Linux, run the smoke checks, and attach both artifacts plus checksums to a GitHub release for that tag.

#### Scenario: Successful release
- **WHEN** a tag is pushed and all checks pass
- **THEN** the GitHub release contains the AppImage, the `.flatpak` bundle and a `SHA256SUMS` file

#### Scenario: Smoke check failure blocks publication
- **WHEN** the AppImage fails to print its version in the smoke step
- **THEN** no release is created and the workflow is red

### Requirement: Packaging reproducible locally
A developer SHALL build the same AppImage locally with one command using the pinned packaging tool versions.

#### Scenario: Local build
- **WHEN** the developer runs the packaging command on Linux
- **THEN** an AppImage is produced under the packaging output directory and passes the same smoke checks CI runs
