# Spec Delta

## Purpose

Specifies the two frame containers readers use to serve the player: a dense cache holding a whole working zone, and a bounded prebuffer filled asynchronously by a decoding thread, with deterministic acquisition, eviction and snapshot rules.

## ADDED Requirements

### Requirement: Frames carry timestamps and the previous timestamp
A frame SHALL hold an image, its timestamp and the timestamp of the frame decoded before it, so that stepping backward is possible without a second decode.

#### Scenario: Empty frame
- **WHEN** an empty frame is created
- **THEN** it has no image and both timestamps are -1

### Requirement: Dense cache acquisition
The dense cache SHALL store frames sorted by timestamp, reject duplicates, acquire the closest frame to a requested timestamp (ties resolved to the earlier frame), acquire the next frame when one exists, trim to a section, clear, and publish a snapshot whose version increases on every structural change.

#### Scenario: Closest between two frames
- **WHEN** frames at 10 and 20 are cached and timestamp 15 is requested
- **THEN** the frame at 10 becomes current

#### Scenario: Trim evicts outside frames and protects current
- **WHEN** frames at 0, 10, 20, 30 are cached, current is 10, and the cache is trimmed to [10, 20]
- **THEN** only 10 and 20 remain, current is still 10 and the snapshot reports one section [10, 20]

#### Scenario: Duplicate add
- **WHEN** a frame with an already cached timestamp is added
- **THEN** the add reports duplicate and the cache is unchanged

### Requirement: Prebuffer bounds and retention
The prebuffer SHALL hold at most its capacity (default 32) frames, keep at most the retention window (default 8) frames behind the current frame when acquiring, never evict the current frame, and report count, emptiness and contiguity.

#### Scenario: Retention window applied on acquire
- **WHEN** 20 contiguous frames are buffered and the frame at index 15 is acquired with retention 8
- **THEN** frames before index 7 are evicted and the frame at 15 is current

#### Scenario: Contiguity
- **WHEN** frames at 0, 1, 2 and 5 (one timestamp per frame) are buffered
- **THEN** the prebuffer reports non-contiguous

### Requirement: Blocking add with interruption
Adding a frame to a full prebuffer SHALL block the caller until space is freed or the add is interrupted; an interrupted add SHALL report interrupted and leave the frame unstored; forced add SHALL never block and SHALL evict the oldest non-current frame when full.

#### Scenario: Add unblocks after acquire
- **WHEN** the prebuffer is full and a producer thread adds a frame
- **THEN** the producer blocks until the consumer acquires a later frame and eviction frees a slot, after which the add reports added

#### Scenario: Interrupt pending add
- **WHEN** a producer is blocked in add and the consumer interrupts adds
- **THEN** the add returns interrupted within a bounded time and the frame is not in the buffer

#### Scenario: Forced add on full buffer
- **WHEN** the prebuffer is full and a frame is force-added
- **THEN** the oldest frame that is not current is evicted and the new frame is stored

### Requirement: Timestamp relation to the buffer
The prebuffer SHALL relate a target timestamp to its contents as behind, in-bounds match, in-bounds no match, ahead, or far ahead using the configured tolerance (half a frame) and far-ahead threshold (50 frames).

#### Scenario: In bounds without exact frame
- **WHEN** frames at 0 and 10 are buffered with tolerance 0.5 and timestamp 5 is related
- **THEN** the relation is in-bounds no match

### Requirement: Try acquire from player state
Given a player state, the prebuffer SHALL try to acquire its target (reference timestamp, or current ± one frame for step actions), report whether the target was acquired, the acquired timestamp and whether the buffer is contiguous, and evict frames outside the retention window of the acquired frame.

#### Scenario: Step forward acquired
- **WHEN** current is at timestamp 10 with one timestamp per frame and a step-forward state is tried
- **THEN** the frame at 11 is acquired and reported

### Requirement: Shutdown and purge
Shutdown SHALL release every frame including current and wake blocked producers; purge SHALL remove every frame except current; both SHALL publish a new snapshot.

#### Scenario: Purge keeps current
- **WHEN** 5 frames are buffered with current at the third and purge is called
- **THEN** exactly one frame remains and it is the current one

### Requirement: Cache operation scripts match the original implementation
Replaying a scripted sequence of adds, acquisitions, trims and purges SHALL produce the same sequence of snapshots, current timestamps and add results as the original implementation recorded in the `videocache` fixtures.

#### Scenario: Fixture replay
- **WHEN** every case under the `videocache` fixture area is replayed
- **THEN** each recorded step matches exactly
