# tooling/rust-workspace Specification

## Purpose
Guarantees that the Rust implementation of Kinovea can be built, linted and tested on a Linux developer machine and in CI with one command, so every later migration change has a green baseline to extend with TDD.

## Requirements

### Requirement: Workspace builds on Linux with the pinned toolchain
The Rust workspace SHALL compile on Linux x86_64 with the toolchain version pinned in the repository, without requiring any Windows component.

#### Scenario: Fresh clone builds
- **WHEN** a developer clones the repository on Linux, installs the pinned Rust toolchain and runs the workspace build command
- **THEN** the build completes with exit code 0 and produces a `kinovea` binary that prints its version and exits 0

#### Scenario: Toolchain drift is rejected
- **WHEN** the installed toolchain differs from the pinned version
- **THEN** the toolchain manager selects or installs the pinned version instead of building with the local default

### Requirement: Single-command verification
The workspace SHALL expose a single verification entry point that runs formatting check, lints with warnings treated as errors, and the full test suite.

#### Scenario: Verification passes on a clean tree
- **WHEN** the verification command runs on an unmodified checkout
- **THEN** formatting, lints and tests all pass and the command exits 0

#### Scenario: Verification fails on a lint warning
- **WHEN** a crate contains code that triggers a lint warning
- **THEN** the verification command exits non-zero and reports the file and line

### Requirement: Tests run without external hosts or network
Every test in the workspace SHALL pass on Linux using only the repository contents and the pinned toolchain.

#### Scenario: Offline test run
- **WHEN** the test suite runs with network access disabled and the Windows oracle host unreachable
- **THEN** all tests pass

#### Scenario: Fixtures are read from the repository
- **WHEN** a test needs reference data
- **THEN** it reads it from the versioned fixtures directory and fails with a clear message naming the missing file if absent

### Requirement: Continuous integration enforces verification on Linux
A CI workflow SHALL run the verification command on Linux for every push and pull request to the default branch.

#### Scenario: Pull request with failing test
- **WHEN** a pull request introduces a failing test
- **THEN** the Linux CI job fails and the failure is visible on the pull request

#### Scenario: Existing Windows build unaffected
- **WHEN** the Linux CI job runs
- **THEN** the existing Windows build workflow still runs independently and its outcome does not depend on the Linux job

### Requirement: Crate layout mirrors the target architecture
The workspace SHALL contain one crate per planned architectural boundary, each with at least one test, so later changes add behavior to an existing crate rather than creating structure.

#### Scenario: Every crate participates in tests
- **WHEN** the test suite runs
- **THEN** each workspace crate reports at least one executed test

### Requirement: Quality gates cover Rust sources
The repository quality gates SHALL analyze Rust sources and ignore build output.

#### Scenario: Untested Rust source is reported
- **WHEN** a Rust source file without an associated test is added
- **THEN** the test-coverage gate lists that file

#### Scenario: Build output is ignored
- **WHEN** the workspace has been built locally
- **THEN** no gate reports files under the build output directory
