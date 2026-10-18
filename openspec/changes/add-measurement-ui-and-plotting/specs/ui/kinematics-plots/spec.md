# Spec Delta

## Purpose

Specifies the data-analysis windows that plot kinematic series and diagrams for the drawings of a player screen, with series selection and graph/data export, replacing the OxyPlot-based forms.

## ADDED Requirements

### Requirement: Analysis windows and their entry points
`Tools > Linear kinematics`, `Tools > Angular kinematics`, `Tools > Angle-angle diagram`, `Tools > Scatter diagram` SHALL open the corresponding window for the active player; `Analysis…` on a track SHALL open the single-track analysis. Each window SHALL be non-modal and update when the player's drawings change.

#### Scenario: Open linear kinematics
- **WHEN** two tracks exist and the user opens `Linear kinematics`
- **THEN** the window lists both tracks as series with checkboxes and plots the selected component (default `Horizontal position`) against time

#### Scenario: No data
- **WHEN** no track exists and the user opens `Linear kinematics`
- **THEN** the window opens with an empty plot and the text `No trajectory to analyse`

### Requirement: Series, component and time model
Each window SHALL offer: series checkboxes (one per track, angle or point set), the kinematic component selector (position, displacement, speed, velocity components, acceleration, angle, angular velocity…), the time model (`Absolute`, `Relative` to series start, `Normalized`), and the filter cutoff plot of the Butterworth filter used (`Linear kinematics` only), with axis labels in the calibrated units.

#### Scenario: Switch to speed
- **WHEN** the user selects `Speed` in the component selector
- **THEN** the plot shows the speed series computed by the kinematics library with unit label `m/s` (or the configured speed unit) and the y-axis rescales

#### Scenario: Normalized time
- **WHEN** series of different durations are plotted with `Normalized`
- **THEN** both span 0 to 1 on the x-axis

### Requirement: Diagrams
`Angle-angle diagram` SHALL plot one angle series against another with time as colour gradient; `Scatter diagram` SHALL plot calibrated positions of all point-like drawings of selected keyframes, with the calibration plane overlaid when `Show calibration plane` is checked.

#### Scenario: Scatter with plane
- **WHEN** a plane is calibrated and three cross marks exist and the user checks `Show calibration plane`
- **THEN** the plot shows the plane rectangle in world units and the three points inside it

### Requirement: Graph and data export
Each window SHALL provide `Copy graph` and `Save graph…` (PNG at a user-chosen width and height, default 800×600) and `Copy data` and `Save data…` (CSV with a header row and one column per selected series, numbers in the invariant culture with the application's decimal precision).

#### Scenario: CSV parity
- **WHEN** the user exports the data of the oracle kinematics case `linear`
- **THEN** the CSV equals `tests/fixtures/kinematics/linear/export.csv` byte for byte

#### Scenario: PNG size
- **WHEN** the user saves the graph at 1200×800
- **THEN** the written PNG has those dimensions and contains the plotted series

### Requirement: Plot interaction
Plots SHALL support zoom by scroll and drag-rectangle, pan by drag, `Reset view` by double-click, and a hover tooltip with the series name, time and value.

#### Scenario: Hover tooltip
- **WHEN** the pointer hovers a sample of the `Speed` series
- **THEN** a tooltip shows `<series> t=<time> v=<value unit>`
