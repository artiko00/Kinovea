# kinematics/linear-kinematics Specification

## Purpose
Derives the linear kinematic quantities Kinovea shows for a tracked point (distance, displacement, velocity, acceleration) from a filtered trajectory, in the user's units.

## Requirements

### Requirement: Time series collection
A time series collection SHALL hold the sample times and one `f64` series per kinematic component (`X`, `Y`, `XRaw`, `YRaw`, the nine linear and the nine angular components), SHALL be addressable by component, and SHALL pad series with NaN at both ends by a given count (whole series when shorter than twice the padding).

#### Scenario: Padding a short series
- **WHEN** a 3-sample series is padded by 2
- **THEN** all three values are NaN

### Requirement: Linear kinematics from a trajectory
Given a filtered trajectory and a measurement context, the system SHALL produce times, raw and filtered coordinates, cumulative distance, horizontal and vertical displacement from the first point, speed and axis velocities by central difference over two frames (padded by 1), accelerations as the central difference of the speeds (padded by 2), converting through the context's speed and acceleration units; filtered coordinates are used when the trajectory can filter, raw ones otherwise.

#### Scenario: Oracle parity, linear case
- **WHEN** `tests/fixtures/kinematics/linear/` is replayed with a pixel context at 100 fps
- **THEN** every non-NaN cell of `linear.csv` matches within 1e-6 relative and NaN cells coincide

#### Scenario: Oracle parity, Winter gait data
- **WHEN** `tests/fixtures/kinematics/winter/` is replayed at 69.9 fps
- **THEN** every cell of `linear.csv` matches within 1e-6 relative

#### Scenario: Short trajectories
- **WHEN** a trajectory of 2 samples is processed
- **THEN** velocities and accelerations are all NaN and distances are defined

### Requirement: Optional high-speed derivative smoothing
When the context enables high-speed derivative smoothing (the original default), the system SHALL apply the moving average with a 40 ms span and 1 sentinel to the three velocity series and a 50 ms span and 2 sentinels to the three acceleration series after padding. When disabled, the central-difference values are returned as is.

#### Scenario: Oracle parity without smoothing
- **WHEN** `tests/fixtures/kinematics/linear-unsmoothed/` (same inputs as `linear` with `smoothing: false`) is replayed with smoothing disabled
- **THEN** every cell matches within 1e-6 relative, and its velocity columns differ from `linear/linear.csv`
