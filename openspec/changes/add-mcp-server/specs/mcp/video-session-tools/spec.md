# Spec Delta

## Purpose

Lets an agent open a video with or without annotations, inspect it, move through it, see frames and persist the result, the way a user opens a file in the player screen and saves a KVA.

## ADDED Requirements

### Requirement: open_video creates a session
Tool `open_video {path, kva?}` SHALL open the file with the Rust video reader, load the sidecar `<video>.kva` when present or the given KVA, and return a session id plus `info` (width, height, frame count, frame rate, duration ms, average timestamps per frame, keyframe count, drawing count, calibration summary). Opening a path that is not a supported video SHALL fail with `unsupported_media`. Several sessions MAY be open at once. Equivalent of File > Open video and File > Load annotations.

#### Scenario: Video with sidecar
- **WHEN** `open_video` is called on `clip.mp4` next to `clip.kva` with two keyframes
- **THEN** `info.keyframes` is 2 and the drawings of the KVA are present

#### Scenario: Unsupported file
- **WHEN** `open_video` is called on a text file
- **THEN** the error code is `unsupported_media`

### Requirement: describe_session and list_drawings
Tool `describe_session {session}` SHALL return the same `info` as `open_video` plus the current position; tool `list_drawings {session, time?}` SHALL return every keyframe with its drawings (id, type, name, visible range) and the detached chronos and tracks, filtered to those visible at `time` when given.

#### Scenario: Drawings at a time
- **WHEN** `list_drawings` is called with a time where one line is fading out and one track is active
- **THEN** both are returned with `visible: true` and drawings of other keyframes are absent

### Requirement: seek and render_frame
Tool `seek {session, time|frame}` SHALL position the session and return the actual timestamp reached; tool `render_frame {session, time?, with_drawings=true, scale?}` SHALL return the frame as PNG (base64 in an MCP image content block) with drawings rendered through the headless renderer exactly as the player would show them at that time. Equivalent of the player timeline and of File > Export > Image.

#### Scenario: Rendered frame with a line
- **WHEN** `render_frame` is called at 1000 ms on a synthetic clip with one line drawing
- **THEN** the PNG decodes to the video size and matches the renderer snapshot fixture within tolerance

#### Scenario: Seek outside the video
- **WHEN** `seek` is called past the last frame
- **THEN** the position clamps to the last frame and the returned timestamp is the last frame's

### Requirement: save_kva and close_session
Tool `save_kva {session, path?}` SHALL write the document as KVA 2.0 (default: sidecar path) and return the path; tool `close_session {session}` SHALL release the reader. The saved file SHALL reload in the Rust loader and in the original application without warnings.

#### Scenario: Round trip
- **WHEN** a session with imported skeletons is saved and reopened with `open_video`
- **THEN** `describe_session` reports the same keyframe and drawing counts

### Requirement: Document resources
Resource `kinovea://session/{id}/document.kva` SHALL return the current KVA text; `kinovea://session/{id}/measurements.json` the measured-data JSON export; `kinovea://tools/postures` the installed generic posture tools (name, id, point names).

#### Scenario: Posture tool list
- **WHEN** the client reads `kinovea://tools/postures`
- **THEN** `OpenPoseBody25` and `BodyWithFeet` are listed with 25 and 26 point names
