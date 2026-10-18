# core/timecode-formatting Specification

## Purpose
Renders a time position of a video as text in any of the timecode formats the original application offers, so that measurements, labels and exports show the same strings as the C# version.

## Requirements

### Requirement: Timecode formats are enumerated and classified
The system SHALL expose the nine timecode formats `ClassicTime`, `Frames`, `Milliseconds`, `Microseconds`, `TenThousandthOfHours`, `HundredthOfMinutes`, `TimeAndFrames`, `Normalized`, `Timestamps`, and SHALL classify each as a number type or a string type exactly as the original: `Frames`, `Milliseconds`, `Microseconds`, `HundredthOfMinutes`, `TenThousandthOfHours` and `Timestamps` are numbers, the others are strings.

#### Scenario: Classification matches the original
- **WHEN** the classification of every format is requested
- **THEN** the result equals the table above

#### Scenario: Format names round-trip as text
- **WHEN** a format is written as its name and parsed back
- **THEN** the same format is obtained, and an unknown name yields a parse error rather than a default

### Requirement: Time string rendering matches the oracle
Given a frame rate, a frame count, a duration in milliseconds, a timestamp count, the total duration in timestamps and total frames, a format and a symbol flag, the system SHALL produce the same string as `TimeHelper.GetTimestring` for every row of `tests/fixtures/timecode/formats/timecodes.csv`.

#### Scenario: Every fixture row matches
- **WHEN** each row of the fixture is rendered with its inputs
- **THEN** the output equals the `output` column byte for byte, including the ` ms` and ` µs` suffixes when the symbol flag is set

#### Scenario: Precision follows frame rate magnitude
- **WHEN** the classic time of 1500 ms is rendered at 25 fps and at 1000 fps
- **THEN** the results are `1.50` and `1.500` respectively

### Requirement: Millisecond to timecode conversion handles sign, carry and precision
The system SHALL convert a signed millisecond value and a precision into `[h:][mm:]ss.xx[x]`, prefixing `- ` for negative values, carrying centiseconds into seconds, minutes and hours when rounding reaches 100, and truncating (not rounding) the fraction when precision is three or more.

#### Scenario: Fixture parity
- **WHEN** each row of `tests/fixtures/timecode/formats/ms_to_timecode.csv` is converted
- **THEN** the output equals the `output` column

#### Scenario: Carry at the centisecond boundary
- **WHEN** 59 995 ms is converted with precision 2
- **THEN** the result is `1:00.00`
