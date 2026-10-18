# Spec Delta

## ADDED Requirements

### Requirement: Fixtures are an immutable baseline
Once generation is retired, fixtures and their input cases SHALL be treated as immutable: no fixture or case file may be edited, added or regenerated. Behavior that intentionally diverges from the original SHALL be recorded as explicit expectations in Rust tests next to the fixture comparison, never by altering the fixture.

#### Scenario: Fixture modification rejected
- **WHEN** a commit modifies a file under the fixtures or cases directories
- **THEN** a quality gate or CI check fails naming the file

#### Scenario: Intentional divergence
- **WHEN** a Rust test needs a different expected value than the fixture
- **THEN** the test asserts the new value explicitly with a comment referencing the decision, and the fixture file is unchanged

## MODIFIED Requirements

### Requirement: Each fixture records its provenance
Every fixture directory SHALL contain a manifest with the generator commit hash, the FFmpeg version used, and a checksum of its input case. After generation is retired, all manifests SHALL reference the same final generator commit, which is the tagged legacy commit, and that commit SHALL be recorded in the fixtures documentation.

#### Scenario: Manifest content
- **WHEN** a fixture is inspected
- **THEN** its manifest lists commit, FFmpeg version and case checksum, and the checksum matches the case file in the repository

#### Scenario: Stale fixtures are detected
- **WHEN** fixtures in the repository were generated from different commits
- **THEN** a Linux test fails and lists the fixtures whose commit differs from the majority

#### Scenario: Baseline commit is frozen
- **WHEN** the fixture tests run after generation was retired
- **THEN** every manifest commit equals the baseline commit named in the fixtures documentation and the test fails otherwise

## REMOVED Requirements

### Requirement: Fixtures are generated from the original implementation
**Reason**: The generator and the original implementation are deleted; the committed fixtures already satisfy this origin and are frozen by the new immutability requirement.
**Migration**: Fixtures stay in place unchanged; the legacy tag preserves the generator for audit.

### Requirement: Generation is deterministic
**Reason**: The generator no longer exists; fixtures are frozen, so determinism of generation is moot.
**Migration**: The committed fixtures were produced by a run verified with the generator's verify mode before retirement; the verification log is kept in the retirement entry of the progress log.

### Requirement: Oracle is driven from Linux with one command
**Reason**: `tools/oracle/fixtures.sh`, the Windows host and the generator are deleted.
**Migration**: None needed; tests read the committed fixtures only. Developers who need the legacy behavior check out the legacy tag.

### Requirement: Host build is idempotent and self-checking
**Reason**: There is no host build after the solution is removed.
**Migration**: The provisioning and build scripts remain available at the legacy tag.

### Requirement: Original application behavior is not altered
**Reason**: The original application no longer exists in the repository.
**Migration**: Behavioral parity is now asserted by the Rust tests against the frozen fixtures.
