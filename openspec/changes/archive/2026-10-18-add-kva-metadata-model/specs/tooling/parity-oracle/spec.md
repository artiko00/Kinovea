# Spec Delta

## ADDED Requirements

### Requirement: Oracle covers annotations and postures
The oracle SHALL provide fixture areas for the annotation model: `kva` with per-drawing content hashes and remapping cases, `posture` with description summaries and handle-move results, and a fuzzed KVA corpus generated with fixed seeds.

#### Scenario: Fuzzed corpus is deterministic
- **WHEN** the fuzzed corpus is generated twice with the same seeds
- **THEN** both trees are byte-identical

#### Scenario: Cross-load check
- **WHEN** a KVA written by the Rust implementation is placed under `tests/cases/kva/crossload/`
- **THEN** the oracle loads it and writes a summary with counts and content hashes that Rust tests compare against their own
