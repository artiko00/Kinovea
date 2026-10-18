# Spec Delta

## Purpose

Specifies the kinogram: a single image composed of crops of selected frames laid out in a grid, used to show a movement sequence at a glance.

## ADDED Requirements

### Requirement: Parameters
Kinogram parameters SHALL include tile count (default 18), rows (default 3), crop size (default 400x600), crop positions, manual position set, frame offsets, auto-interpolate (default true), left-to-right (default true), border colour and visibility, label type, colour and size, with the original serialization element names and a content hash.

#### Scenario: Round trip
- **WHEN** parameters with 12 tiles and 2 rows are written and read
- **THEN** all fields are unchanged and the hash is equal

### Requirement: Frame selection
Base frame indices SHALL be evenly spaced over the working zone frames for the tile count; each tile MAY apply an offset clamped so the index stays inside the zone and does not cross its neighbours.

#### Scenario: Even spacing
- **WHEN** 60 frames and 6 tiles are configured
- **THEN** the base indices are 0, 12, 24, 36, 48 and 59

#### Scenario: Offset clamped
- **WHEN** the first tile receives an offset of -5
- **THEN** its index stays 0

### Requirement: Grid layout
Tiles SHALL be laid out in the configured rows with columns = ceil(count / rows), filling left-to-right then top-to-bottom, or right-to-left when configured; the tile size SHALL be the output size divided by columns and rows.

#### Scenario: Right to left
- **WHEN** 6 tiles in 2 rows are laid out right-to-left in a 600x400 output
- **THEN** tile 0 occupies the top-right 200x200 cell

### Requirement: Crop positions and interpolation
Each tile crops its frame at its crop position with the crop size; when auto-interpolate is enabled, positions of tiles not manually placed SHALL be linearly interpolated between manually placed neighbours; changing the tile count SHALL re-interpolate positions.

#### Scenario: Interpolation
- **WHEN** tiles 0 and 4 are manually placed at x 0 and x 400 with 5 tiles
- **THEN** tiles 1, 2, 3 are at x 100, 200, 300

### Requirement: Composition
The composite SHALL draw each crop scaled into its cell, draw a border between tiles when visible, draw a label per tile (none, frame time, or elapsed time) at the configured size and colour, and SHALL be exportable as an image at the output size.

#### Scenario: Composite content
- **WHEN** frames whose pixels encode their index are composed with borders hidden and no labels
- **THEN** the center pixel of every cell decodes to that tile's frame index

### Requirement: Parity with the original kinogram
On the `kinogram` oracle cases the Rust composite SHALL match the recorded original image in every pixel outside label and border areas within a tolerance of 2 per channel.

#### Scenario: Fixture replay
- **WHEN** every `kinogram` case is replayed
- **THEN** the pixel comparison passes
