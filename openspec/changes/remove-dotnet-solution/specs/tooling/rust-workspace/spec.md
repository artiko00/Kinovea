# Spec Delta

## MODIFIED Requirements

### Requirement: Quality gates cover Rust sources
The repository quality gates SHALL analyze every source directory of the repository, exclude only build output, dependencies, fixtures and generated locale files, and SHALL contain no exclusion naming an application source directory.

#### Scenario: Untested Rust source is reported
- **WHEN** a Rust source file without an associated test is added
- **THEN** the test-coverage gate lists that file

#### Scenario: Build output is ignored
- **WHEN** the workspace has been built locally
- **THEN** no gate reports files under the build output directory

#### Scenario: No legacy exclusions
- **WHEN** the gate configuration is inspected
- **THEN** no exclude entry refers to a directory that does not exist in the repository
