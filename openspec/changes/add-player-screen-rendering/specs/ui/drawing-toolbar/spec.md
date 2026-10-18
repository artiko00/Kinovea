# Spec Delta

## Purpose

Specifies the drawing toolbar and direct manipulation of drawings on the viewport, driven by the same toolbar and tool XML files the original application ships, so every drawing type is reachable with the same grouping and defaults.

## ADDED Requirements

### Requirement: Toolbar built from the toolbar definition
The toolbar SHALL be generated from `Tools/DrawingTools/Toolbars/player.xml`: each `entry` is a button, each `subentry` an item of its drop-down (separators honoured), in file order, preceded by the pointer tool and followed by the colour profile button.

#### Scenario: Toolbar matches the XML
- **WHEN** the toolbar is built from the shipped `player.xml`
- **THEN** the buttons are `Pointer, Label, Pencil, HumanModel, Line, Arrow, CrossMark, Angle, ChronoMulti, Counter, Plane, Spotlight, Colour profile` and `Line` has sub-tools `Curve, Polyline, Rectangle, Circle, HorizontalDistance`

#### Scenario: Custom tool
- **WHEN** a generic-posture XML is placed in the custom tools directory
- **THEN** it appears under `HumanModel` after restart

### Requirement: Tool selection and creation gestures
Selecting a tool SHALL arm it; a click or drag on the viewport SHALL create the drawing according to the tool's creation rule (single click: `CrossMark`, `Label`, `Counter`, `Spotlight`, postures; drag: `Line`, `Arrow`, `Rectangle`, `Circle`, `Angle`, `Plane`; freehand: `Pencil`; multi-click with `Enter`/double-click to finish: `Polyline`, `Curve`, `TimePolyline`). After creation the pointer tool SHALL be reselected unless the tool is sticky (`Pencil`, `CrossMark`).

#### Scenario: Create a polyline
- **WHEN** the user selects `Polyline`, clicks three points and double-clicks
- **THEN** a polyline with three points exists on the current keyframe and the pointer tool is active

#### Scenario: Sticky tool
- **WHEN** the user creates a `CrossMark`
- **THEN** the tool stays armed for the next click

### Requirement: Manipulation and hit testing
With the pointer tool, hovering a drawing SHALL show its handles; dragging a handle SHALL resize/move that point, dragging the body SHALL move the drawing; `Delete` removes the selection; `Ctrl+X/C/V` cut, copy and paste (`Ctrl+Shift+V` paste in place); every manipulation SHALL be undoable with `Ctrl+Z` / `Ctrl+Y`.

#### Scenario: Hit test parity
- **WHEN** the pointer is at a position that the headless renderer reports as hitting handle 2 of an angle
- **THEN** the egui backend reports the same handle for the same scene and zoom

#### Scenario: Undo a move
- **WHEN** the user moves a rectangle and presses `Ctrl+Z`
- **THEN** the rectangle returns to its previous position and `Ctrl+Y` reapplies the move

### Requirement: Style editing and presets
Right-clicking a drawing SHALL offer `Configure…` opening the style dialog (colour, line size, style, font size, arrows, decoration, fading/visibility), `Set as default style`, and the toolbar's `Colour profile` button SHALL open the tool presets dialog that edits default styles per tool and saves them to the colour profile file.

#### Scenario: Default style applied to new drawings
- **WHEN** the user sets a red `Line` as default in tool presets and draws a new line
- **THEN** the new line is red while existing lines keep their colour

### Requirement: Drawing visibility across time
Each drawing SHALL follow its fading configuration (always visible, visible on its keyframe only, fade in/out over configurable frames, custom range) when rendered at times other than its keyframe; `Toggle drawings visibility` (`Ctrl+D`) hides all drawings.

#### Scenario: Fading
- **WHEN** a line with default fading is viewed 5 frames after its keyframe with a 10-frame fade
- **THEN** it is drawn at 50% opacity
