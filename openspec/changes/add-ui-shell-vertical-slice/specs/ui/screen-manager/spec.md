# Spec Delta

## Purpose

Specifies how the main window hosts one or two screens, their layout and lifecycle, and the minimal player screen that proves the open, draw and save path on Linux.

## ADDED Requirements

### Requirement: One or two screens
`View > One player` and `View > Two players` SHALL set the number of hosted screens. With two screens the layout SHALL be side by side by default and `View > Vertical layout` SHALL stack them; the splitter ratio SHALL be adjustable by drag and persisted in the window descriptor.

#### Scenario: Add a second screen
- **WHEN** one video is loaded and the user picks `View > Two players`
- **THEN** a second empty player screen appears to the right and the first keeps its video and position

#### Scenario: Back to one screen
- **WHEN** two screens are loaded and the user picks `View > One player`
- **THEN** the application asks which screen to keep when both have content, and keeps only the chosen one

### Requirement: Swap and close
`View > Swap screens` (and the button between the screens) SHALL exchange the two screens, and each screen SHALL have a `Close` action (`Ctrl+F4`, screen context menu) that removes it, asking to save unsaved annotations first.

#### Scenario: Swap
- **WHEN** screen 1 shows `a.mp4` and screen 2 shows `b.mp4` and the user swaps
- **THEN** screen 1 shows `b.mp4` and screen 2 shows `a.mp4`, with their current positions preserved

#### Scenario: Close with unsaved annotations
- **WHEN** a screen has unsaved drawings and the user closes it
- **THEN** a dialog offers `Save`, `Don't save`, `Cancel`, and `Cancel` leaves the screen open

### Requirement: Active screen
Exactly one hosted screen SHALL be active at a time, marked visually; clicking inside a screen activates it; menus, hotkeys and the navigation pane act on the active screen.

#### Scenario: Activate by click
- **WHEN** two screens exist and the user clicks inside the second
- **THEN** the second becomes active and `File > Save` targets its annotations

### Requirement: Minimal player screen for the vertical slice
The player screen in this change SHALL display the current frame of the loaded video fitted to the viewport, a frame slider to seek, `Play/Pause`, a `Line` tool button and a pointer tool. Drawing a line on the current frame SHALL create a keyframe at that time holding the line; the line SHALL be selectable and movable with the pointer.

#### Scenario: Draw a line
- **WHEN** the user selects the `Line` tool and drags from point A to point B on the frame
- **THEN** a line from A to B is drawn with the default style, a keyframe exists at the current time and the screen is marked as modified

#### Scenario: Move the line
- **WHEN** the user selects the pointer tool and drags the line body
- **THEN** both endpoints translate by the drag vector and the keyframe keeps a single line

#### Scenario: Seek
- **WHEN** the user drags the frame slider to the middle
- **THEN** the displayed frame changes to the frame nearest to that timestamp and the line is only shown at its keyframe time

### Requirement: Save annotations as KVA
`File > Save` (`Ctrl+S`) on a modified screen SHALL write a KVA 2.0 file next to the video (`<video>.kva`) and `File > Save as…` SHALL ask for a path. The file SHALL be readable by the original Kinovea application and by the Rust metadata reader.

#### Scenario: Save produces a loadable KVA
- **WHEN** the user draws a line and presses `Ctrl+S`
- **THEN** `<video>.kva` exists, validates against `kva.2.0.xsd`, and reloading it in a new screen shows the same line at the same keyframe

#### Scenario: Reopen video with sidecar KVA
- **WHEN** a video with an existing `<video>.kva` is opened
- **THEN** its annotations are loaded automatically, as in the original application
