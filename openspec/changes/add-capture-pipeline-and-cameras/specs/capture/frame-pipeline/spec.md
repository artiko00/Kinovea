# Spec Delta

## Purpose

Moves frames from one camera producer to several consumers (display, delay buffer, recorder) with bounded memory, explicit drop accounting and a delay buffer that serves frames by age, so capture keeps up with the camera on Linux.

## ADDED Requirements

### Requirement: Single producer, multiple consumers over a bounded ring buffer
The pipeline SHALL hand each produced frame to every registered consumer through a ring buffer of fixed capacity, without copying the frame per consumer and without blocking the producer on a slow consumer.

#### Scenario: All consumers see every frame when they keep up
- **WHEN** a producer emits 1000 frames and two consumers each process faster than the producer
- **THEN** both consumers receive the 1000 frames in order with matching frame identifiers and the drop counter is 0

#### Scenario: Slow consumer causes counted drops, not stalls
- **WHEN** one consumer processes at half the producer rate for a buffer of 8 frames
- **THEN** the producer never blocks, the slow consumer skips frames, and the pipeline drop counter equals the number of frames that consumer did not process

### Requirement: Frame descriptors gate reallocation
The pipeline SHALL allocate buffers from an image descriptor (format, width, height, orientation, buffer size) and SHALL reallocate only when a new descriptor is incompatible with the current one.

#### Scenario: Compatible reconnect reuses buffers
- **WHEN** a camera reconnects with an identical descriptor
- **THEN** no buffer is reallocated and consumer positions are reset

#### Scenario: Resolution change reallocates
- **WHEN** the descriptor width or height changes
- **THEN** all buffers are reallocated to the new size before the next frame is claimed

### Requirement: Delay buffer serves frames by age
The delay buffer SHALL store the most recent frames up to a capacity derived from available memory and SHALL return the frame that is N frames old, or the oldest available frame when N exceeds what is stored.

#### Scenario: Exact age
- **WHEN** 50 frames have been pushed and a frame of age 10 is requested
- **THEN** the frame with identifier 40 is returned

#### Scenario: Age beyond content
- **WHEN** 5 frames have been pushed and age 20 is requested
- **THEN** the oldest frame (identifier 1) is returned and the call reports the actual age served

#### Scenario: Capacity from memory budget
- **WHEN** the memory budget is 100 MB and frames are 4 MB
- **THEN** the full capacity is 25 frames and the safe capacity leaves room for the recorder backlog as the original implementation did

### Requirement: Pipeline statistics are observable
The pipeline SHALL expose producer frequency, drop count and recorder backlog so the capture screen can display them.

#### Scenario: Frequency estimate
- **WHEN** frames arrive at a steady 100 Hz for one second
- **THEN** the reported frequency is within 5 percent of 100 Hz
