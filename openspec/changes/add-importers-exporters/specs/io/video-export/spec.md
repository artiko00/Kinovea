# Spec Delta

## Purpose

Encodes a range of rendered frames, with annotations burned in, into a video file so analyses can be shared as video.

## ADDED Requirements

### Requirement: Frames are encoded through the system ffmpeg
The system SHALL encode by spawning the `ffmpeg` executable, feeding raw BGR24 frames on stdin, with arguments equivalent to the original command line. If `ffmpeg` is not on `PATH` the export SHALL fail with a message naming it. The output path SHALL never equal the source video path.

#### Scenario: Argument list
- **WHEN** the command is built for a given profile
- **THEN** it contains `-f rawvideo -pixel_format bgr24 -video_size WxH -framerate <1000/interval> -i pipe:0 -an`, a bicubic `scale` filter only when the output size differs, the codec arguments of the profile (MJPEG `-q:v`, or libx264/libx265 with `-pix_fmt yuv420p`, `-preset` and quality), the container format and the output path last

#### Scenario: MJPEG export of a 25-frame selection
- **WHEN** a synthetic 25-frame clip is exported with the MJPEG profile
- **THEN** `ffprobe` reports 25 frames, codec `mjpeg`, the selection frame rate, and the first frame matches the renderer snapshot within tolerance

#### Scenario: Export onto the source file
- **WHEN** the output path equals the source path
- **THEN** the export is refused and nothing is written

### Requirement: Playback-rate and slideshow variants
The system SHALL support: plain export at the baseline frame interval or at the current slow-motion interval, duplicating frames when the interval exceeds 125 ms so the file rate stays at or above 8 fps; slideshow export of key images only with a slide duration; and export with pauses where each keyframe is repeated `slide duration / frame interval` times. Entry points: File > Export > Video > Video / Slideshow / With pauses; `kinovea export video|slideshow|with-pauses`; MCP `export_video`.

#### Scenario: Slideshow of three key images at 2 s each
- **WHEN** a document with three keyframes is exported as slideshow with 2000 ms per slide
- **THEN** the output has 3 × ⌈2000/125⌉ frames at 8 fps and each distinct frame equals the corresponding key image export

#### Scenario: Slow motion below 8 fps
- **WHEN** the playback interval is 500 ms
- **THEN** each frame is written 4 times and the file frame rate is 8 fps

### Requirement: Side by side video
The system SHALL compose two documents frame by frame along their common timeline, horizontally or vertically, with optional merge, and encode the result. Entry point: File > Export > Video > Side by side; `kinovea export video-side-by-side`.

#### Scenario: Two clips of different length
- **WHEN** a 20-frame and a 30-frame clip are exported side by side
- **THEN** the output has 30 frames and the shorter clip holds its last frame after frame 20
