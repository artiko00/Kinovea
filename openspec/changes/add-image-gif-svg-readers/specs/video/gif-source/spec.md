# Spec Delta

## Purpose

Exposes animated GIF files as videos whose frames and timing come from the file, decoded once into memory.

## ADDED Requirements

### Requirement: GIF timing from frame delays
The reader SHALL use 100 timestamps per second, take the frame interval from the first frame delay (in hundredths of a second), report frames per second as 100 divided by that interval, duration as frame count times interval, and treat a zero delay as 10 hundredths.

#### Scenario: Three frames at 100 ms
- **WHEN** a GIF with 3 frames of 10 hundredths delay is opened
- **THEN** the info reports 10 fps, 100 ms interval, 30 timestamps of duration and 10 timestamps per frame

#### Scenario: Zero delay
- **WHEN** a GIF whose frame delay is 0 is opened
- **THEN** the interval used is 10 hundredths

### Requirement: Frames are fully decoded into the cache
Opening SHALL decode every frame with disposal methods applied (each frame is the full composited image) into the dense cache, so navigation never decodes again and the working zone covers all frames.

#### Scenario: Composited frames
- **WHEN** a GIF whose second frame is a partial update is opened and frame 1 is requested
- **THEN** the delivered image equals the full composited second frame

#### Scenario: Navigation uses the cache
- **WHEN** move to timestamp 15 is requested on the three-frame GIF
- **THEN** the frame at timestamp 10 is current (closest)

### Requirement: Single-frame GIF is an image
A GIF with exactly one frame SHALL be flagged as an image in the summary.

#### Scenario: One frame
- **WHEN** a one-frame GIF is summarized
- **THEN** the summary reports image and 3 thumbnails requested yield 1

### Requirement: Summary thumbnails
Thumbnails SHALL be taken every ceil(count / requested) frames starting at the first frame.

#### Scenario: Thumbnails spacing
- **WHEN** 2 thumbnails are requested from a 5-frame GIF
- **THEN** frames 0 and 3 are returned
