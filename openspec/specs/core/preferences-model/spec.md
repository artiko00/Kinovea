# core/preferences-model Specification

## Purpose
Holds the user preferences of the application (general, player, capture, file explorer) as typed values with the original defaults, and reads and writes them in the `Preferences.xml` format so an existing installation's settings remain readable.

## Requirements

### Requirement: Typed preferences with original defaults
The system SHALL expose the general, player, capture and file-explorer preference groups as typed values. A freshly created preferences object SHALL hold the same default values as the C# `PreferencesManager` on first run (for example classic time format, metres per second, degrees, filtering enabled).

#### Scenario: Defaults match the oracle file
- **WHEN** default preferences are written to XML
- **THEN** the document is element-wise equal to `tests/fixtures/preferences/defaults/Preferences.xml`, ignoring element order and the `FormatVersion` attribute's position

### Requirement: Preferences file round-trip
The system SHALL read a `Preferences.xml` of format version `3.0`, preserve every value it understands, ignore and log unknown elements, and write the file back with the same values.

#### Scenario: Round-trip of a modified file
- **WHEN** `tests/fixtures/preferences/modified/Preferences.xml` is read and written back
- **THEN** every element known to the model has the same value as in the input and the unknown element `FutureSetting` is dropped without error

#### Scenario: Missing file yields defaults
- **WHEN** a non-existent path is loaded
- **THEN** the defaults are returned and no error is raised

### Requirement: No implicit persistence
Setting a preference value SHALL NOT write to disk; persistence happens only through an explicit save call.

#### Scenario: Set without save
- **WHEN** a value is changed and no save is called
- **THEN** the file on disk is unchanged
