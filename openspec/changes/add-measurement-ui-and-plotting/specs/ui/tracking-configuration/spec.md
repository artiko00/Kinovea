# Spec Delta

## Purpose

Specifies how users start, stop and configure tracking of tracks and trackable drawings, and how trajectories and measurement labels are displayed, exposing the tracking library from the player screen.

## ADDED Requirements

### Requirement: Track lifecycle from the player
Right-clicking a point of the image with the pointer tool SHALL offer `Track path`, which creates a track at that point and enters tracking mode; a track's context menu SHALL offer `Start tracking`/`Stop tracking`, `Restart tracking from here`, `Delete points after`, `Configure…`, `Display…`, `Analysis…`, `Delete`. `Start all tracking` (`Ctrl+Shift+T`) SHALL run every open track to the end of the working zone with a progress bar.

#### Scenario: Track a moving object
- **WHEN** the user picks `Track path` on the synthetic moving object and presses play
- **THEN** the track follows the object frame by frame and shows its trajectory; pressing `Stop tracking` freezes it

#### Scenario: Start all tracking
- **WHEN** two tracks are open and the user presses `Ctrl+Shift+T`
- **THEN** both tracks are extended to the end of the working zone and the progress bar closes

### Requirement: Tracking parameters dialog
`Configure…` on a track or the `Tracking` side-panel tab SHALL expose the tracking parameters (search window and template window sizes in pixels, similarity threshold, template update threshold, tracker type when more than one is available) with a preview rectangle on the viewport; values persist in the KVA track element.

#### Scenario: Change search window
- **WHEN** the user sets search window 80×80 and template 30×30 and tracks 10 frames
- **THEN** the KVA track element holds those parameters and the tracker used them (observable through the tracking fixture test harness)

### Requirement: Trajectory display dialog
`Display…` SHALL open the trajectory display dialog: marker shape, line style and colour, trajectory extent (`Complete`, `One second`, `Label`, `Trailing`), measurement label (`None`, `Name`, `Position`, `Total distance`, `Speed`, `Horizontal/Vertical velocity`, `Acceleration`, …) and `Show keyframe markers`; changes preview live and are undoable.

#### Scenario: Show speed label
- **WHEN** the user picks `Speed` as measurement label on a tracked trajectory
- **THEN** the label near the current point displays the speed in the calibrated unit at every frame

### Requirement: Trackable drawings
Drawings with trackable points (angles, lines, markers, postures…) SHALL offer `Track` in their context menu to toggle tracking of their points; while tracking is on, moving a point by hand SHALL re-seed the tracker at that frame.

#### Scenario: Track an angle
- **WHEN** the user enables `Track` on an angle and plays 20 frames
- **THEN** the three angle points follow the tracked features and the angle value updates per frame

### Requirement: Measurement display per drawing
Every measurable drawing SHALL offer a `Display measure` submenu listing the measures the drawing supports (`None`, `Name`, `Length`, `Angle`, `Position`, `Speed`…) and the selection SHALL persist in the KVA.

#### Scenario: Line length in calibrated unit
- **WHEN** a plane calibration in metres is active and the user enables `Length` on a line
- **THEN** the label shows the length in metres with the precision configured in preferences
