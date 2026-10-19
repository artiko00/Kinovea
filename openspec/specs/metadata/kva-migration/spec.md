# metadata/kva-migration Specification

## Purpose
Keeps analyses saved by old Kinovea versions (KVA 1.3 to 1.5) loadable by the Rust fork by converting them to the 2.x model in Rust, replacing the XSLT path, and rejecting formats that the original application also rejects.

## Requirements

### Requirement: Migration semantics match the XSLT
The migration SHALL reproduce `kva-1.5to2.0.xsl`: calibration help to line calibration with unit mapping, keyframes and drawings renamed, chronos and tracks moved to 2.x structure, and track points converted from relative to absolute time.

#### Scenario: Relative trajectories become absolute
- **WHEN** a 1.5 document with a track whose points carry relative times is migrated
- **THEN** every track point time equals the track start time plus the relative time

### Requirement: Format version handling
The loader SHALL read `FormatVersion` (as a descendant of the root) and select: 2.x direct reading; 1.3 to 1.5 migration followed by the relative trajectory fix before the post-import pass; 1.2 and older read as they are, as the original does after logging that the format is too old.

#### Scenario: Old file is migrated
- **WHEN** a KVA 1.5 document is loaded
- **THEN** the resulting model equals the model obtained by loading the XSLT output recorded in the `kva/migration` fixture

#### Scenario: Too old is read without conversion
- **WHEN** a KVA 1.2 document with a keyframe is loaded
- **THEN** loading succeeds, no conversion is applied and the keyframe is present

#### Scenario: Relative trajectories are fixed before labels
- **WHEN** a 1.5 document with a track and a keyframe inside the track span is loaded
- **THEN** the track keyframe label is attached to the point closest to the keyframe time in absolute time
