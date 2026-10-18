# Spec Delta

## Purpose

Describes how sequences of frames are written to a video file for capture recording and export, with deterministic timing and container metadata.

## ADDED Requirements

### Requirement: Open a recording context
Opening a recording SHALL accept a file path, image size, frame interval for the file, quality, pixel format of incoming frames, an uncompressed flag and a rotation, SHALL pick the container from the file extension, SHALL use MJPEG unless uncompressed raw video is requested, and SHALL fail with a typed error when the container or codec is unavailable.

#### Scenario: MJPEG in MKV
- **WHEN** a recording is opened to `out.mkv` with quality 90 and 40 ms interval
- **THEN** the file is created with one MJPEG video stream at 25 fps

#### Scenario: Uncompressed
- **WHEN** the uncompressed flag is set
- **THEN** the stream uses raw video with the incoming pixel format

### Requirement: Save frames
Saving a frame SHALL accept either already-encoded JPEG bytes (wrapped without re-encoding) or raw BGRA/RGB24 pixels (converted and encoded), SHALL stamp the frame duration in stream time base, and SHALL report encoding and write durations for diagnostics.

#### Scenario: Raw frames round trip
- **WHEN** 10 synthesized raw frames are saved as uncompressed and the file is re-opened with the reader
- **THEN** 10 frames are read back with identical pixel values

#### Scenario: JPEG passthrough
- **WHEN** JPEG-encoded frames are saved
- **THEN** the packets are written without decoding and the output frame count equals the input count

### Requirement: Rotation metadata
When a rotation is requested the writer SHALL store it as stream display metadata so that readers present the frames rotated.

#### Scenario: Rotation preserved
- **WHEN** a recording with 90 degree rotation is re-opened
- **THEN** the reader reports a 90 degree rotation

### Requirement: Close finalizes the file
Closing SHALL flush the encoder, write the trailer and release resources; closing after a failure SHALL still produce a readable file for the frames written so far.

#### Scenario: Close after success
- **WHEN** 25 frames are saved and the context is closed
- **THEN** the file duration is 1 s at 25 fps
