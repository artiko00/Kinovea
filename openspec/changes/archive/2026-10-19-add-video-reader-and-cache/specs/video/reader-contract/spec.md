# Spec Delta

## Purpose

Defines the contract every video source fulfils so that players, exporters and the headless server can open a source, learn its timing and geometry, navigate by timestamp and enumerate frames without knowing the decoder behind it.

## ADDED Requirements

### Requirement: Video info describes the source timing model
A reader SHALL expose, after a successful open, the file path, original image size and rotation, pixel and sample aspect ratios, frames per second, frame interval in milliseconds, timestamps per second, average timestamps per frame, first and last timestamp and duration in timestamps.

#### Scenario: Synthetic source opened
- **WHEN** a source with 25 fps, 1 timestamp per frame and 250 frames is opened
- **THEN** the info reports frame interval 40 ms, average timestamps per frame 1, first timestamp 0 and duration 250 timestamps

#### Scenario: Single image
- **WHEN** the duration in timestamps is 1
- **THEN** the reader reports the source as a single frame

### Requirement: Capabilities are declared as flags
A reader SHALL declare which optional behaviors it supports: changing the working zone, decoding on demand, prebuffering, caching, changing aspect ratio, deinterlacing, duration, frame rate, rotation, demosaicing and stabilization. Convenience accessors SHALL derive from the flags.

#### Scenario: Decoding mode switch is validated against capabilities
- **WHEN** a switch to prebuffering is requested on a reader without the prebuffer capability
- **THEN** the switch is refused and the reader stays in its current decoding mode

#### Scenario: Not-initialized mode is always allowed
- **WHEN** a switch to the not-initialized decoding mode is requested
- **THEN** it is accepted regardless of capabilities

### Requirement: Timestamp relation classifies a request against a reference
Given a requested and a reference timestamp, the reader SHALL classify the request as match (within half an average frame), behind, far behind (more than 50 frames behind), ahead, far ahead (more than 50 frames ahead), or unknown when either value is negative.

#### Scenario: Within tolerance
- **WHEN** the request differs from the reference by less than half a frame of timestamps
- **THEN** the relation is match

#### Scenario: Far ahead
- **WHEN** the request is 51 frames of timestamps after the reference
- **THEN** the relation is far ahead

#### Scenario: Negative input
- **WHEN** either timestamp is negative
- **THEN** the relation is unknown

### Requirement: Playback clock projects the expected timestamp
During playback the reader SHALL compute the expected timestamp from the reference timestamp, the elapsed real time since the playback epoch, the playback frame interval and the average timestamps per frame; outside playback it SHALL return 0.

#### Scenario: Two frame intervals elapsed
- **WHEN** the playback state references timestamp 100 with a 40 ms frame interval, 1 timestamp per frame, and 80 ms of real time elapsed
- **THEN** the expected timestamp is 102

### Requirement: Navigation requests
A reader SHALL accept player requests (playback, timestamp, step forward, step backward) and move requests (to a timestamp, or to the next frame) and report whether the target frame was acquired. Moving next past the end of the working zone SHALL report not acquired.

#### Scenario: Move to timestamp
- **WHEN** a move to a timestamp inside the working zone is requested on a synthetic source
- **THEN** the current frame carries that timestamp and the request reports acquired

#### Scenario: Move next at the end
- **WHEN** the current frame is the last of the working zone and move next is requested
- **THEN** the request reports not acquired and the current frame is unchanged

### Requirement: Frame enumeration walks the working zone
Enumerating frames with interval 0 SHALL yield every frame from the working zone start to its end; a positive interval SHALL yield the frame closest to each multiple of the interval and stop before exceeding the working zone end. Enumeration SHALL be synchronous and the caller SHALL be able to disable prebuffering around it.

#### Scenario: Full enumeration
- **WHEN** a 10-frame working zone is enumerated with interval 0
- **THEN** exactly 10 frames are yielded in increasing timestamp order

#### Scenario: Interval enumeration
- **WHEN** a 10-frame zone with 1 timestamp per frame is enumerated with interval 3
- **THEN** frames at timestamps 0, 3, 6 and 9 are yielded

### Requirement: Summary extraction
A reader SHALL extract a summary of a file (image flag, size, duration in milliseconds, frame rate and up to N thumbnails bounded by a maximum size) without leaving the file open, and SHALL report whether a companion annotation file with the same stem exists.

#### Scenario: Thumbnails bounded
- **WHEN** 3 thumbnails are requested with maximum width 100 from a 320x240 source
- **THEN** 3 images are returned, each no wider than 100 pixels, and the source is closed afterwards

### Requirement: Reader registry resolves sources by extension
The registry SHALL map lower-cased extensions to reader factories, fall back to the wildcard reader when an extension is unknown, expose whether an extension is supported, and return the most recently written supported file matching a path pattern.

#### Scenario: Unknown extension with wildcard
- **WHEN** a wildcard reader is registered and a source with an unregistered extension is requested
- **THEN** the wildcard reader is instantiated

#### Scenario: Case-insensitive extension
- **WHEN** a reader is registered for `.gif` and `.GIF` is requested
- **THEN** that reader is returned
