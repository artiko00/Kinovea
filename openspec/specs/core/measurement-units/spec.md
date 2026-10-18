# core/measurement-units Specification

## Purpose
Provides the units in which Kinovea expresses lengths, speeds, accelerations, angles, angular rates and cadences, their abbreviations, and the conversions between them, so kinematics and exports report values in the user's units.

## Requirements

### Requirement: Unit enumerations and abbreviations
The system SHALL expose the length, speed, acceleration, angle, angular velocity, angular acceleration and cadence units of the original application, each with the same abbreviation string (for example `m`, `km/h`, `m/s²`, `°`, `rad/s`, `rpm`, `Hz`, `steps/min`). The percentage length unit SHALL use `%` and pixels SHALL use `px`.

#### Scenario: Abbreviations match the oracle table
- **WHEN** the abbreviation of every unit value is requested
- **THEN** it equals the `abbreviation` column of `tests/fixtures/units/conversions/abbreviations.csv`

### Requirement: Velocity conversion
The system SHALL convert a velocity expressed in length units per second into a speed unit: per-second units keep the magnitude, per-hour units and knots multiply by 3600 after converting the length to the target base, and pixel speeds pass through unchanged.

#### Scenario: Metres per second to kilometres per hour
- **WHEN** 10 m/s is converted with length unit metres and speed unit kilometres per hour
- **THEN** the result is 36 within 1e-4

#### Scenario: Fixture parity
- **WHEN** every row of `tests/fixtures/units/conversions/velocity.csv` is converted
- **THEN** the result equals the `output` column within a relative tolerance of 1e-6

### Requirement: Acceleration, angular and frequency conversions
The system SHALL convert accelerations between length-unit based and metric or imperial units, angular velocities from radians per second to degrees per second, radians per second or revolutions per minute, angular accelerations from radians to degrees per second squared, and frequencies from hertz into every cadence unit, matching the oracle.

#### Scenario: Fixture parity
- **WHEN** every row of `acceleration.csv`, `angular_velocity.csv`, `angular_acceleration.csv` and `frequency.csv` under `tests/fixtures/units/conversions/` is converted
- **THEN** the result equals the `output` column within a relative tolerance of 1e-6

### Requirement: Cross length-unit conversion from a speed unit
The system SHALL convert a value whose source length unit is implied by a speed unit into a target length unit by passing through metres, leaving pixel speeds unchanged.

#### Scenario: Kilometres per hour magnitude into centimetres
- **WHEN** 1 (km/h magnitude) is converted to centimetres
- **THEN** the result is 100 000 within 1e-3

### Requirement: Calibration precision text and significant digits
The system SHALL compute the "units per pixel" text used to describe calibration precision, switching metres to centimetres or millimetres and centimetres to millimetres when the value is below one unit, and SHALL round a value to the minimal digits rule (integer above 10, one decimal between 1 and 10, one significant digit below 1).

#### Scenario: Fixture parity
- **WHEN** every row of `tests/fixtures/units/conversions/pixel_size.csv` is evaluated
- **THEN** the produced text equals the `output` column exactly

#### Scenario: Minimal digits
- **WHEN** 12.34, 3.456 and 0.0234 are rounded
- **THEN** the results are 12, 3.5 and 0.02
