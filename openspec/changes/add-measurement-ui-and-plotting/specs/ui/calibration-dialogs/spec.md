# Spec Delta

## Purpose

Specifies the dialogs through which users calibrate space and time in a player screen and inspect or correct lens distortion, exposing the calibration library with the same entry points as the original application.

## ADDED Requirements

### Requirement: Calibrate by line
The context menu of a `Line` drawing SHALL offer `Calibrate…` opening a dialog with the real length value, a unit selector (millimetres, centimetres, metres, inches, feet, yards, percentage, custom) and axis alignment (`Horizontal`, `Vertical`, `None`). `OK` SHALL set the line calibration of the metadata so measurements on the screen use it.

#### Scenario: Calibrate a known length
- **WHEN** the user draws a line over a 1 m rod, opens `Calibrate…`, enters `100` with unit `cm` and presses `OK`
- **THEN** a second line of the same pixel length shows the measure `100 cm` when its `Display measure > Length` is on

#### Scenario: Cancel keeps previous calibration
- **WHEN** a plane calibration exists and the user opens line calibration and presses `Cancel`
- **THEN** the plane calibration remains active

### Requirement: Calibrate by plane
The context menu of a `Plane` or `Grid` drawing SHALL offer `Calibrate…` opening a dialog with real width and height, unit, offset of the origin, flip horizontal/vertical and a preview of the quadrilateral. `OK` SHALL set the plane calibration; the dialog SHALL refuse non-convex quadrilaterals with a message.

#### Scenario: Calibrate a plane
- **WHEN** the user places a plane over a 2 m × 1 m mat, enters `200`, `100`, unit `cm` and presses `OK`
- **THEN** the coordinate system drawing shows axes aligned with the plane and a point at the far corner reads `(200, 100)` cm

#### Scenario: Non-convex plane rejected
- **WHEN** the quadrilateral is self-intersecting
- **THEN** `OK` is disabled and the dialog shows `The plane must be convex`

### Requirement: Coordinate system
`Tools > Coordinate system` SHALL toggle the coordinate-system drawing; dragging its origin SHALL move the origin of the calibration; its context menu SHALL offer axis orientation options and `Show grid`.

#### Scenario: Move origin
- **WHEN** the user drags the coordinate-system origin to a point P
- **THEN** a cross mark at P displays `(0, 0)` and other measures shift accordingly

### Requirement: Calibration validation window
`Tools > Calibration validation` SHALL open a window listing every point-like drawing with its image coordinates, computed world coordinates, and editable expected coordinates per axis; it SHALL compute camera position from the plane calibration and show per-point errors, with `Copy` producing CSV.

#### Scenario: Validation errors
- **WHEN** three cross marks are placed and the user enters expected coordinates for two of them
- **THEN** the table shows the error for those two in the calibration unit and `Copy` places a CSV with the table on the clipboard

### Requirement: Lens distortion calibration
`Image > Lens calibration` SHALL offer `None`, `Open…` (Kinovea or Agisoft XML), `Manual…` and `Save…`. `Manual…` SHALL open a dialog with the distortion parameters (fx, fy, cx, cy, k1, k2, k3, p1, p2), a `Calibrate from grid` action using the `Distortion grid` drawings on the current video, a live preview of the distortion grid over the image, and `OK` applies the parameters to the metadata.

#### Scenario: Open Agisoft file
- **WHEN** the user opens an Agisoft lens XML
- **THEN** the parameters are converted and the distortion grid preview updates

#### Scenario: Calibrate from grids
- **WHEN** two `Distortion grid` drawings exist and the user presses `Calibrate from grid`
- **THEN** parameters are estimated with the calibration library, the error is displayed and the preview follows the new parameters

### Requirement: Time calibration
`Video > Configure video timing…` SHALL set the capture frame rate and user frame rate (timebase) used by the calibration helper so kinematics use real time; the dialog SHALL show the resulting slow-motion factor.

#### Scenario: High-speed timing
- **WHEN** the user sets capture rate 1000 fps on a 25 fps file
- **THEN** the dialog shows `40× slow motion` and exported speeds use 1 ms frame interval
