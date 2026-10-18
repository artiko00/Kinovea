# Spec Delta

## Purpose

Turns noisy digitised coordinates into filtered trajectories suitable for differentiation, using the same filter design, padding and automatic cutoff selection as the original so derived values stay comparable with existing Kinovea analyses.

## ADDED Requirements

### Requirement: Butterworth low-pass filtering with automatic cutoff
Given at least 11 samples and a sampling frequency, the system SHALL filter the series at cutoff frequencies spanning 0.5 Hz to the Nyquist frequency in a configurable number of steps (default 100), using a second-order forward then backward pass with correction factor for two passes and 10 reflected padding points on each side, SHALL keep every filtered series with its cutoff and normalized Durbin-Watson score, and SHALL report the index of the series with the lowest score.

#### Scenario: Oracle cutoff selection
- **WHEN** the raw X and Y columns of `tests/fixtures/kinematics/linear/linear.csv` are filtered at 100 Hz with 100 steps
- **THEN** the selected indices equal `x_cutoff_index` and `y_cutoff_index` of the sibling `filtering.json`, and the selected series equal the fixture columns `X` and `Y` within 1e-6

#### Scenario: Too few samples
- **WHEN** 10 or fewer samples are filtered
- **THEN** an error is returned instead of a result

#### Scenario: Residual score
- **WHEN** residuals with Durbin-Watson statistic 2 are scored
- **THEN** the normalized score is 0, and a statistic of 0 or 4 yields 1

### Requirement: Moving average smoothing
Given samples, a sampling frequency, a span in milliseconds and a sentinel count, the system SHALL return the input unchanged when half the span does not exceed one sample interval, and otherwise SHALL average over a centred window of `span / (2·interval)` samples each side with up to 50 reflected padding samples, matching the original.

#### Scenario: Span below resolution
- **WHEN** a 40 ms span is applied at 25 fps (40 ms interval)
- **THEN** the output equals the input

#### Scenario: Oracle parity
- **WHEN** the `LinearSpeed` series of `tests/fixtures/kinematics/linear/linear.csv` (generated with the default smoothing enabled) is recomputed
- **THEN** it matches the fixture within 1e-6

### Requirement: Filtered trajectory
A filtered trajectory SHALL be built from timed points and a measurement context: it stores times, raw world coordinates, filtered coordinates per axis when filtering is enabled and the sample count exceeds 10, the chosen cutoff indices (−1 when not filtered), and the least-squares best-fit circle of its coordinates. Coordinate access beyond the last index SHALL return the last sample.

#### Scenario: Oracle best-fit circle
- **WHEN** the trajectory of `tests/fixtures/kinematics/circular/` is built
- **THEN** `can_filter` is true, the cutoff indices match `filtering.json`, and the circle centre and radius match within 1e-3

#### Scenario: Filtering disabled
- **WHEN** the context reports filtering disabled
- **THEN** filtered coordinates equal raw coordinates and cutoff indices are −1
