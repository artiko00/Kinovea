# Spec Delta

## Purpose

Keeps analyses saved by old Kinovea versions (KVA 1.3 to 1.5) loadable by the Rust fork by converting them to the 2.x model in Rust, replacing the XSLT path, and rejecting formats that the original application also rejects.

## ADDED Requirements

### Requirement: Format detection
The loader SHALL read `FormatVersion` from the document and select: 2.x direct reading; 1.3 to 1.5 migration; 1.2 and older rejection with an error naming the version.

#### Scenario: Old file is migrated
- **WHEN** a KVA 1.5 document is loaded
- **THEN** the resulting model equals the model obtained by loading the XSLT output recorded in the `kva/migration` fixture

#### Scenario: Too old is rejected
- **WHEN** a KVA 1.2 document is loaded
- **THEN** loading fails with an error that includes `1.2` and no partial metadata is produced

### Requirement: Migration semantics match the XSLT
The migration SHALL reproduce `kva-1.5to2.0.xsl`: calibration help to line calibration with unit mapping, keyframes and drawings renamed, chronos and tracks moved to 2.x structure, and track points converted from relative to absolute time.

#### Scenario: Relative trajectories become absolute
- **WHEN** a 1.5 document with a track whose points carry relative times is migrated
- **THEN** every track point time equals the track start time plus the relative time
