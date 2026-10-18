# tooling/parity-oracle Specification

## Purpose
Produces reference outputs from the original C# Kinovea implementation so the Rust port can be tested against real behavior, while keeping the Windows host out of the test loop by versioning the results in the repository.

## Requirements

### Requirement: Fixtures are generated from the original implementation
The oracle SHALL generate reference fixtures by executing the original C# code over versioned input cases, never by hand-writing expected outputs.

#### Scenario: Case produces a fixture
- **WHEN** an input case exists under the cases directory and the oracle runs
- **THEN** a fixture directory with the same case name appears under the fixtures directory containing the generated outputs and a manifest

#### Scenario: Case without supported area is rejected
- **WHEN** an input case declares an area the generator does not implement
- **THEN** the generator exits non-zero and names the unsupported area

### Requirement: Generation is deterministic
Running the oracle twice on the same commit and the same cases SHALL produce byte-identical fixtures.

#### Scenario: Repeated run
- **WHEN** the oracle runs twice without changes to cases or source
- **THEN** the verify mode reports no differences between the two output trees

#### Scenario: Culture independence
- **WHEN** the Windows host uses a locale with comma decimal separator
- **THEN** numeric values in fixtures use a period as decimal separator and invariant formatting

### Requirement: Each fixture records its provenance
Every fixture directory SHALL contain a manifest with the generator commit hash, the FFmpeg version used, and a checksum of its input case.

#### Scenario: Manifest content
- **WHEN** a fixture is generated
- **THEN** its manifest lists commit, FFmpeg version and case checksum, and the checksum matches the case file in the repository

#### Scenario: Stale fixtures are detected
- **WHEN** fixtures in the repository were generated from different commits
- **THEN** a Linux test fails and lists the fixtures whose commit differs from the majority

### Requirement: Oracle is driven from Linux with one command
A developer on Linux SHALL be able to build the original solution, run the generator and retrieve fixtures with a single command over SSH.

#### Scenario: Successful round trip
- **WHEN** the developer runs the oracle command with the Windows host reachable
- **THEN** the solution is built in Release x64 on the host, the generator runs, and the resulting fixtures are copied into the repository fixtures directory

#### Scenario: Host unreachable
- **WHEN** the Windows host does not answer over SSH
- **THEN** the command exits non-zero with a message naming the host alias tried and no fixture file is modified

#### Scenario: Host alias override
- **WHEN** the developer sets the oracle host environment variable to another SSH alias
- **THEN** the command uses that alias instead of the default

### Requirement: Host build is idempotent and self-checking
The host-side build script SHALL verify prerequisites before building and reuse cached dependencies.

#### Scenario: Missing build tools
- **WHEN** MSBuild or the C++/CLI toolset is not installed on the host
- **THEN** the script exits non-zero and prints the exact provisioning command without attempting to install anything

#### Scenario: Cached FFmpeg
- **WHEN** the FFmpeg dependency folder already exists on the host
- **THEN** the script skips the download and proceeds to build

### Requirement: Original application behavior is not altered
Adding the generator SHALL NOT change the behavior or the build output of the existing Kinovea executable.

#### Scenario: Existing build still passes
- **WHEN** the Windows CI build workflow runs after the generator project is added to the solution
- **THEN** the workflow succeeds and the produced installer and portable archive contain no generator binaries
