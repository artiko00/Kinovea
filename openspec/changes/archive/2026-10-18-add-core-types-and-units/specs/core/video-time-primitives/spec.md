# Spec Delta

## Purpose

Defines the small value types that the player, metadata and tracking layers exchange: timestamp sections, timed points, version numbers and timestamp mapping.

## ADDED Requirements

### Requirement: Video section semantics
A video section SHALL hold a start and an end timestamp. It is empty when the end is negative, wrapped when the end is before the start, has length `end - start` otherwise, and SHALL answer containment for a timestamp (inclusive bounds) and for another section (the other lies within or equals this one). The default section SHALL be `[0, 99]` and the empty section `[-1, -1]`.

#### Scenario: Containment
- **WHEN** the section `[10, 20]` is asked whether it contains 10, 20, 21 and the section `[12, 18]`
- **THEN** the answers are true, true, false, true

#### Scenario: Equality ignores empty sections
- **WHEN** two empty sections are compared for equality
- **THEN** they are not equal, matching the original behaviour

#### Scenario: Text form
- **WHEN** `[5, 2]` and `[0, 99]` are rendered as text
- **THEN** the results are `? --> 2] [5 --> ?` and `[0 --> 99]`

### Requirement: Timed point text round-trip
A timed point SHALL carry `x`, `y` (single precision), timestamp `t` and radius `r`, SHALL serialize as `x;y;t;r` with invariant formatting, SHALL parse the three-field form `x;y;t` with radius 0, and SHALL expose the same content hash rule (xor of the field hashes) as a stable identity for change detection.

#### Scenario: Four-field round-trip
- **WHEN** `1.5;-2.25;120;3` is parsed and written back
- **THEN** the output is `1.5;-2.25;120;3`

#### Scenario: Three-field legacy form
- **WHEN** `1;2;3` is parsed
- **THEN** the radius is 0

### Requirement: Three-part version ordering
A version SHALL parse `YYYY.N` or `YYYY.N.R` (missing revision is 0) and compare by major, then minor, then revision.

#### Scenario: Ordering
- **WHEN** `2023.1`, `2023.1.1` and `2024.1` are compared
- **THEN** `2023.1 < 2023.1.1 < 2024.1`

### Requirement: Timestamp mapper
The system SHALL define a timestamp mapper as a function from timestamp to timestamp and provide the identity mapper.

#### Scenario: Identity
- **WHEN** the identity mapper is applied to 42
- **THEN** the result is 42
