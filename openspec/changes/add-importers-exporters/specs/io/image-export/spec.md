# Spec Delta

## Purpose

Saves rendered video frames with their annotations as image files: the current frame, every key image, a sampled sequence, or two synchronized players side by side.

## ADDED Requirements

### Requirement: Single frame export
The system SHALL render the frame at a given timestamp with all visible drawings through the headless renderer and save it in the format implied by the file extension (`png`, `jpg`, `bmp`). The suggested file name SHALL be `<video name>_<timecode>` using the current timecode format. Entry points: File > Export > Image > Image (also toolbar button); `kinovea export image --at <time>`; MCP `render_frame`.

#### Scenario: PNG of an annotated frame
- **WHEN** the frame at 1000 ms of a synthetic clip with one line drawing is exported as PNG
- **THEN** the file decodes to the video size and matches the renderer snapshot fixture within the renderer tolerance

### Requirement: Key images and image sequence export
The system SHALL export either every keyframe in the selection or one frame every N timestamps across the selection, naming each file `<base>_<timecode><ext>` in the chosen directory and reporting progress per frame. Entry points: File > Export > Image > Key images / Image sequence; `kinovea export key-images|sequence --interval <frames>`.

#### Scenario: Sequence every 10 frames
- **WHEN** a 100-frame selection is exported with interval 10
- **THEN** 10 files exist with increasing timecodes and the first equals the single-frame export of the selection start

#### Scenario: Cancellation
- **WHEN** the caller cancels after the third frame
- **THEN** exactly three files exist and the exporter returns a cancelled status

### Requirement: Side by side export
The system SHALL compose the current frames of two documents horizontally or vertically, each with its drawings, into one image whose suggested name joins both video names. Entry point: File > Export > Image > Side by side (dual player); `kinovea export side-by-side --horizontal|--vertical`.

#### Scenario: Horizontal composition
- **WHEN** two 640×480 frames are composed horizontally
- **THEN** the image is 1280×480 with the first document on the left
