# Spec Delta

## Purpose

Describes how file-based videos are opened, probed, seeked and decoded into frames the player can consume, with timing information that stays stable across containers and codecs.

## ADDED Requirements

### Requirement: Opening a file
Opening a supported file SHALL succeed and expose original size, rotation, sample aspect ratio and the timing model; opening an unreadable or unsupported file SHALL return a typed failure without panicking and leave the reader unloaded.

#### Scenario: Lossless synthetic clip
- **WHEN** a 25 fps, 50-frame clip synthesized in test setup is opened
- **THEN** the info reports 25 fps, 50 frames of duration, first timestamp 0 and an image size equal to the synthesized size

#### Scenario: Missing file
- **WHEN** a non-existent path is opened
- **THEN** the result is a file-not-opened failure and the reader stays unloaded

### Requirement: Rotation from stream metadata
Rotation SHALL be taken from the stream display matrix, rounded to a multiple of 90 degrees; other angles SHALL be ignored.

#### Scenario: Rotation metadata
- **WHEN** a clip tagged with a 90 degree display rotation is opened
- **THEN** the reference size is the transposed original size and frames are rotated accordingly

### Requirement: Timing model derivation
Frames per second SHALL come from the average frame rate, else the real frame rate, else a measurement of the first frames; timestamps per second from the stream time base; the first timestamp from the container start time clamped to zero; the duration from the stream, else the container, else a measurement; the last timestamp from the frame count when known.

#### Scenario: Container without stream duration
- **WHEN** a clip whose stream reports no duration but whose container does is opened
- **THEN** the duration in timestamps is derived from the container duration and the timestamps per second

### Requirement: Exact frame delivery
A move to a timestamp SHALL deliver the frame whose decoded timestamp is within half a frame of the target, regardless of the group-of-pictures structure, by seeking to the preceding keyframe and decoding forward.

#### Scenario: Seek inside a GOP
- **WHEN** a clip encoded with keyframes every 12 frames, where every frame encodes its index in pixel values, is opened and frame 17 is requested
- **THEN** the delivered frame decodes to index 17

#### Scenario: Backward step after seek
- **WHEN** frame 17 is delivered and a step backward is requested
- **THEN** frame 16 is delivered without a visible re-seek failure

### Requirement: Decoding modes
The reader SHALL support on-demand decoding, prebuffering on a background thread, and full caching of the working zone, SHALL refuse caching when the estimated memory exceeds the requested budget, and SHALL switch modes without losing the current frame.

#### Scenario: Working zone fits in memory
- **WHEN** a working zone of 20 frames of 64x64 is requested with a 64 MB budget
- **THEN** the reader switches to caching mode and the cache snapshot covers the whole zone

#### Scenario: Working zone exceeds budget
- **WHEN** the working zone requires more than the budget
- **THEN** the reader stays in prebuffering mode and reports the required amount

### Requirement: Playback with frame skipping
During playback the prebuffer thread SHALL follow the player clock, SHALL drop scaling of frames when behind, skip non-reference frames when further behind, and seek ahead to the next keyframe when the lag exceeds the configured seconds; disabling frame skipping SHALL decode every frame.

#### Scenario: Frame skipping disabled
- **WHEN** frame skipping is disallowed and playback runs faster than the decoder
- **THEN** every frame of the zone is delivered in order and no frame is skipped

### Requirement: Geometry requests
The reader SHALL apply a geometry request (presentation size and scale, pre-scaling allowance, aspect ratio override, rotation, demosaicing, deinterlacing, stabilization data) and publish the resulting geometry: reference size, output size, scale, flags.

#### Scenario: Pre-scaled output
- **WHEN** pre-scaling is allowed with a presentation size half of the reference size
- **THEN** delivered frames have the half size and the geometry reports pre-scaled with scale 0.5

#### Scenario: Forced aspect ratio
- **WHEN** the 16:9 aspect ratio override is requested on a 4:3 source
- **THEN** the reference size width is adjusted to 16:9 while the height is unchanged

### Requirement: Summary and image sequences
Summary extraction SHALL return evenly spaced thumbnails without leaving the file open; opening a numbered image pattern SHALL expose the sequence as a video at the configured frame rate.

#### Scenario: Thumbnails
- **WHEN** 4 thumbnails are requested from a 100-frame clip
- **THEN** 4 images are returned, taken from increasing positions, and the reader reports not loaded afterwards

#### Scenario: Image sequence
- **WHEN** a directory with `img_0001.png` to `img_0010.png` is opened through the sequence pattern
- **THEN** the info reports 10 frames

### Requirement: Supported containers and codecs are discoverable
The reader SHALL expose the libav version it was built against and SHALL fail at startup with a clear message when the runtime libav is older than the minimum supported version.

#### Scenario: Version reporting
- **WHEN** the version is queried
- **THEN** it names the libavcodec, libavformat and libswscale versions
