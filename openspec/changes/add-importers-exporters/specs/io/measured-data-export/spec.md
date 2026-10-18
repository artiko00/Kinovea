# Spec Delta

## Purpose

Collects every measurement of a document (key images, positions, distances, angles, chronometer sections, tracked time series) into one model and writes it as CSV, TXT, JSON, XLSX or ODS with the same values the original application produces.

## ADDED Requirements

### Requirement: Measured data is collected in a fixed order with the configured units
The system SHALL build the measured-data model from a document: header (producer, file names, image size, frame rates, units), keyframes, positions, distances, angles, times and time series, in the same order and with the same values as the original collector. Lengths SHALL be in world units when the export space preference is world space, in pixels otherwise.

#### Scenario: Ordering rules
- **WHEN** the model is built
- **THEN** positions (cross marks, postures), distances (lines, postures) and angles (angle tools, postures) are sorted by name within each enabled keyframe and stamped with its time, times are sorted by first section start, and time series by first timestamp

#### Scenario: Complete sample document
- **WHEN** the model is built from `Tools/XML/Schema/samples/complete.kva` with the fixture preferences
- **THEN** the counts and values of keyframes, positions, distances, angles, times and timeseries equal the oracle `measured.json` fixture within 1e-4

#### Scenario: Disabled keyframe
- **WHEN** a keyframe is disabled
- **THEN** none of its drawings contributes measurements

### Requirement: CSV trajectory export
The system SHALL write a header row `"Time"`, `"<series>/<point>/X"`, `"<series>/<point>/Y"` per tracked point, then one row per distinct time in ascending order with empty cells where a series has no sample; all cells double-quoted. Nothing is written when there is no series. Entry points: File > Export > Spreadsheet > CSV trajectory; `kinovea export csv-trajectory`; MCP `export_measurements`.

#### Scenario: Separators follow the preference
- **WHEN** the decimal separator preference is point, comma or system
- **THEN** numbers use that separator and the list separator is `;` when the decimal separator is `,`, otherwise `,`

#### Scenario: Two tracks with different time bases
- **WHEN** the complete sample is exported with point decimal separator
- **THEN** the file equals the oracle `trajectory.csv` byte for byte

### Requirement: CSV chronometer export
The system SHALL write a header of all distinct section names in order of first appearance, then per chronometer a `(Cumulative)` row when the cumulative column is visible, a `(Duration)` row always, and a `(Tag)` row when the chronometer has tags and the tag column is visible, with empty cells for sections the chronometer does not have. Entry point: File > Export > Spreadsheet > CSV chronometer; `kinovea export csv-chrono`.

#### Scenario: Chronometers from the sample
- **WHEN** the complete sample is exported
- **THEN** the file equals the oracle `chrono.csv`

### Requirement: TXT trajectory export
The system SHALL write, per series, a `# <name>` header, one space-separated row per sample (`time x y x y ...`, no quotes, configured decimal separator) and a blank line. Entry point: File > Export > Spreadsheet > TXT trajectory; `kinovea export txt-trajectory`.

#### Scenario: Sample export
- **WHEN** the complete sample is exported
- **THEN** the file equals the oracle `trajectory.txt`

### Requirement: JSON export follows the Kinetics Toolkit layout
The system SHALL write an indented JSON object with `metadata` and `data` (keyframes, positions, distances, angles, times, timeseries) using the same property names, order and unit fields as the original. Entry point: File > Export > Spreadsheet > JSON; `kinovea export json`; MCP `export_measurements`.

#### Scenario: Comments and time series shape
- **WHEN** a keyframe comment is RTF and a track has samples
- **THEN** the comment is written as plain text and the series as a `time` array plus a `data` object holding one `[x, y]` array per point

#### Scenario: Sample export
- **WHEN** the complete sample is exported
- **THEN** parsing the Rust output and the oracle `measured.json` yields equal documents with numbers compared at 1e-4

### Requirement: XLSX export reproduces the sheet layout
The system SHALL write one worksheet with, starting at row 2, the blocks Key images, Positions, Distances, Angles, Times and Time series in that order, each with a merged title cell, a header row with unit symbols, data rows, and the number formats `0.00` for values and a time format derived from the timecode preference (`0` for frames, `0.000` for classic time, `0.###` otherwise). Entry point: File > Export > Spreadsheet > XLSX; `kinovea export xlsx`; MCP `export_measurements`.

#### Scenario: Cell values match the oracle
- **WHEN** the complete sample is exported and the Rust and oracle workbooks are read back
- **THEN** every non-empty cell has the same address, value and number format, and the merged ranges are equal

### Requirement: ODS export reproduces the sheet layout
The system SHALL write a valid OpenDocument spreadsheet (`mimetype` first and uncompressed, `META-INF/manifest.xml`, `content.xml`, `styles.xml`, `meta.xml`, `settings.xml`) with the same blocks, header texts and values as the XLSX export. Entry point: File > Export > Spreadsheet > ODS; `kinovea export ods`.

#### Scenario: Content matches the oracle
- **WHEN** the complete sample is exported
- **THEN** the table cells of `content.xml` have the same positions, types and values as the oracle fixture, and the file opens in LibreOffice without repair

### Requirement: Exports use invariant formatting unless asked otherwise
Numbers SHALL never depend on the process locale: the decimal separator comes only from the export preference, and `NaN` values are written as empty cells.

#### Scenario: Comma preference
- **WHEN** the decimal separator preference is comma
- **THEN** CSV cells use `,` as decimal separator and `;` as list separator
