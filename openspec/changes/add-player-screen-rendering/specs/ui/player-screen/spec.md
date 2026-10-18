# Spec Delta

## Purpose

Specifies the full player screen: playback and navigation, speed control, working zone, keyframes, image transforms and side panels, matching the behaviour users know from the original `PlayerScreenUserInterface`.

## ADDED Requirements

### Requirement: Playback and frame navigation
The player screen SHALL provide play/pause (`Space`, play button), first/previous/next/last frame (`Home`, `Left`, `Right`, `End`), large and small jumps (`PageUp/PageDown`, `Shift+PageUp/PageDown`), previous/next keyframe (`Ctrl+Left/Right`) and looping within the working zone during playback.

#### Scenario: Step one frame
- **WHEN** the video is paused at frame 10 and the user presses `Right`
- **THEN** frame 11 is displayed and the time label shows its timecode in the current format

#### Scenario: Loop at working-zone end
- **WHEN** playback reaches the end of the working zone
- **THEN** playback continues from the working-zone start without pausing

### Requirement: Speed control with high-speed support
A speed slider SHALL map slider input to nominal speed with the original non-linear mapping (`TimeMapper`), show the real speed when a capture frame rate differs from the file frame rate (`Video > Configure video timing…`), and support `Up/Down` (10% steps) and `Shift+Up/Down` (1% steps) with stickiness around 100%.

#### Scenario: Slow motion from high-speed file
- **WHEN** a 240 fps capture stored at 30 fps is configured with capture rate 240 and the slider is at 100%
- **THEN** the label reads `100% (12.5% real)` and playback advances 30 frames per second of wall clock

#### Scenario: Speed parity with the original mapping
- **WHEN** the slider input values of the oracle fixture `tests/fixtures/player/timemapper/*.csv` are applied
- **THEN** the nominal and real speed factors match the fixture within 1e-9

### Requirement: Time display and time origin
The info bar SHALL show current time, duration and selection in the active timecode format (`Options > Time format`), and `Mark as time origin` (viewport context menu) SHALL make the current frame time zero so earlier frames display negative times.

#### Scenario: Set time origin
- **WHEN** the user marks frame 100 as time origin and steps to frame 90
- **THEN** the time label shows `- 0.40` at 25 fps in classic format

### Requirement: Image transforms
The `Image` menu SHALL provide `Mirror`, `Rotation` (0/90/180/270), `Aspect ratio` (Auto/4:3/16:9), `Deinterlace`, `Demosaic` (None/RGGB/BGGR/GRBG/GBRG), `Background` colour; transforms SHALL apply to display and to drawing coordinates so drawings stay attached to image features.

#### Scenario: Rotate with drawings
- **WHEN** a line is drawn and the user selects `Rotation > 90°`
- **THEN** the line appears rotated with the image and its stored image-space coordinates are unchanged in the KVA

### Requirement: Zoom, pan and magnifier
The viewport SHALL zoom with `Ctrl+wheel`, `Ctrl++`, `Ctrl+-`, reset with `Ctrl+0` or `Escape` (reset viewport), pan by dragging with the pointer tool when zoomed, offer `Stretch`/`Fill` toggle, and a magnifier tool with 1.5× to 4× factors movable over the image.

#### Scenario: Zoom at cursor
- **WHEN** the user zooms with `Ctrl+wheel` over a point of the image
- **THEN** that image point stays under the cursor after the zoom

#### Scenario: Magnifier follows the pointer
- **WHEN** the magnifier tool is active and the user moves the source region
- **THEN** the inset shows the region magnified by the chosen factor and drawings are rendered inside it

### Requirement: Keyframes
`Add keyframe` (`Insert`, keyframe button) SHALL create a keyframe at the current frame; the keyframe strip SHALL show thumbnail, title (editable with `F2`) and comments; `Delete keyframe` (`Shift+Delete`) SHALL remove it; presets `1..0` hotkeys SHALL apply the configured keyframe presets.

#### Scenario: Keyframe strip navigation
- **WHEN** the user clicks a keyframe thumbnail
- **THEN** the player seeks to that keyframe and the keyframe is highlighted in the strip and on the timeline

### Requirement: Side panel
The screen SHALL offer a collapsible side panel with `Keyframes`, `Drawing` (name, style, visibility/fading, tracking setup) and `Tracking` tabs, bound to the selected drawing; its visibility and splitter ratio persist in the screen descriptor.

#### Scenario: Edit drawing style from the side panel
- **WHEN** a line is selected and the user changes its colour in the `Drawing` tab
- **THEN** the line redraws with the new colour and the change is undoable

### Requirement: Annotations file menu
The viewport context menu and `File` menu SHALL offer `Save annotations` (`Ctrl+S`), `Save annotations as…`, `Load annotations…` (merge into current), `Unload annotations`, `Save as default player annotations`, `Reload default annotations`, and `Copy image` (`Ctrl+Shift+C`) / `Copy time`.

#### Scenario: Load merges
- **WHEN** a video has two keyframes and the user loads a KVA with one keyframe at a different time
- **THEN** the screen holds three keyframes and is marked modified
