# Spec Delta

## Purpose

Lets an agent put annotations on the video: skeletons from external pose data, subtitles, and the core measuring drawings a user would place with the drawing toolbar.

## ADDED Requirements

### Requirement: import_skeleton and import_subtitles
Tool `import_skeleton {session, path, format: openpose|trc}` SHALL run the corresponding importer of the input/output layer and return an `ImportReport` (keyframes created, drawings created, tracks created, skipped items with reasons); tool `import_subtitles {session, path}` SHALL do the same for SRT. Equivalent of File > Load annotations.

#### Scenario: OpenPose folder
- **WHEN** `import_skeleton` points to the first `*_keypoints.json` of the import fixture folder
- **THEN** the report lists 3 keyframes and 6 drawings and `list_drawings` shows `OpenPoseBody25` postures named `OpenPose`

#### Scenario: Wrong format
- **WHEN** `format` is `trc` but the file is JSON
- **THEN** the error code is `invalid_params` and the document is unchanged

### Requirement: add_drawing covers the core measuring tools
Tool `add_drawing {session, time, type, points, name?, style?}` SHALL create a drawing at the keyframe for `time`, creating the keyframe if needed, and return the drawing id. Points are image pixels. Unknown types SHALL fail with `invalid_params` listing the supported types. Equivalent of the drawing toolbar tools of the same names.

#### Scenario: Supported types and point counts
- **WHEN** `type` is `line` (2 points), `angle` (3: origin, leg a, leg b), `cross_mark` (1), `label` (1 + text), `chrono` (start and stop times), `posture` (tool name + optional point values) or `track` (1 point, starts a track)
- **THEN** the drawing is created with those inputs and any other point count fails with `invalid_params`

#### Scenario: Angle on an empty frame
- **WHEN** `add_drawing` creates an `angle` at 2000 ms with three points
- **THEN** a keyframe exists at 2000 ms, `list_drawings` shows the angle and `get_measurements` reports its value

#### Scenario: Posture with partial points
- **WHEN** a `posture` of tool `BodyWithFeet` is added with values for 10 of 26 points
- **THEN** the remaining points keep the tool's default positions

### Requirement: update_drawing, delete_drawing, set_keyframe
Tool `update_drawing {session, id, points?, name?, style?, options?}` SHALL move points, rename or restyle a drawing and toggle posture options; `delete_drawing {session, id}` SHALL remove it; `set_keyframe {session, time, name?, comment?, color?}` SHALL set keyframe metadata. Comments are plain text stored as the KVA expects. Equivalent of the drawing context menu and the keyframe side panel.

#### Scenario: Move a line end
- **WHEN** `update_drawing` moves the second point of a line
- **THEN** `get_measurements` returns the new length and the KVA contains the new coordinates

#### Scenario: Delete unknown id
- **WHEN** `delete_drawing` is called with an id that does not exist
- **THEN** the error code is `not_found`

### Requirement: undo reverts the last mutation
Tool `undo {session, steps=1}` SHALL revert the last `steps` mutating tool calls using the document history, and `redo` SHALL reapply them. Equivalent of Edit > Undo / Redo.

#### Scenario: Undo an import
- **WHEN** `import_skeleton` is followed by `undo`
- **THEN** the keyframe and drawing counts equal those before the import
