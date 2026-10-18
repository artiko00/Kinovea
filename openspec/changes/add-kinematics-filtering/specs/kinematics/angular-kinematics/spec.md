# Spec Delta

## Purpose

Derives angular position, velocity and the tangential, centripetal and resultant accelerations of an angle defined by three tracked points, as shown by Kinovea's angle tools.

## ADDED Requirements

### Requirement: Angular position from three trajectories
Given trajectories named origin, leg a and leg b of equal length, angle options (signed, counter-clockwise, supplementary) and a measurement context, the system SHALL compute per sample the oriented angle (legs reversed when not counter-clockwise, leg a reflected through the origin when supplementary, wrapped to [0, 2π) when unsigned), the displacement from the previous sample and from the first sample, each converted through the context's angle unit.

#### Scenario: Supplementary angle
- **WHEN** supplementary is enabled for legs at 0° and 90°
- **THEN** the stored position is the angle between leg b and the reflection of leg a, 90°

### Requirement: Angular velocity and accelerations
The system SHALL derive angular velocity by central difference over two frames (padded by 1), tangential velocity as radius × ω with radius the distance origin to leg b, angular acceleration from the velocities (padded by 1), tangential acceleration radius × α, centripetal acceleration radius × ω², and the resultant √(at² + ac²), converting through the context's angular and linear units.

#### Scenario: Oracle parity, circular case
- **WHEN** `tests/fixtures/kinematics/circular/` is replayed with options (signed, ccw, not supplementary)
- **THEN** every cell of `angular.csv` matches within 1e-6 relative and NaN positions coincide

#### Scenario: Unsigned wrap
- **WHEN** an angle of −π/2 is computed with signed disabled
- **THEN** the stored position is 3π/2

#### Scenario: Wrong number of trajectories
- **WHEN** fewer or more than three trajectories are supplied
- **THEN** an error is returned
