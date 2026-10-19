## REMOVED Requirements

### Requirement: Format detection
**Reason**: It required rejecting 1.2 and older, which the original does not do: `MetadataConverter.Convert` logs that the format is too old and returns the document unchanged, and the loader reads it.
**Migration**: Replaced by "Format version handling" below.

## ADDED Requirements

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
