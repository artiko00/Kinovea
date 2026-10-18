# Spec Delta

## Purpose

Specifies the capture-side frame pipeline: a fixed set of pre-allocated slots shared between one producer (camera) and several consumers (display, recorder, delay) without locks, with explicit frame drops when consumers fall behind.

## ADDED Requirements

### Requirement: Ring buffer allocation
The ring buffer SHALL pre-allocate a power-of-two number of slots of a fixed byte size, reject a non power-of-two capacity, and report whether allocation succeeded.

#### Scenario: Invalid capacity
- **WHEN** a ring buffer with capacity 6 is requested
- **THEN** construction fails with an error naming the power-of-two constraint

### Requirement: Producer claim and commit
The producer SHALL claim the next slot only if every active consumer has read past the slot that would be overwritten; a refused claim SHALL count as one dropped frame. Commit SHALL make the slot visible to consumers by advancing the producer position.

#### Scenario: Drop when a consumer lags
- **WHEN** capacity is 4, one active consumer is at position 0 and the producer has committed positions 0..3
- **THEN** the next claim is refused and the drop count becomes 1

#### Scenario: Inactive consumers do not block
- **WHEN** the only lagging consumer is inactive
- **THEN** the claim succeeds

### Requirement: Consumer barrier
A consumer waiting for a position SHALL return as soon as the producer position is at or beyond it, and SHALL return the current producer position so a slow consumer can drain several frames in one pass.

#### Scenario: Fast consumer waits
- **WHEN** a consumer waits for position 5 and the producer commits position 5 later
- **THEN** the wait returns 5 or more only after that commit

### Requirement: Measured frequency
The pipeline SHALL publish the measured frame frequency of the producer, computed over a sliding window of recent frame arrival times.

#### Scenario: Steady producer
- **WHEN** frames arrive every 20 ms for 48 frames
- **THEN** the published frequency is 50 Hz within 5 percent

### Requirement: Batch consumer lifecycle
A consumer SHALL expose started, active and position states; it SHALL process every committed slot in order while active, skip processing while inactive, and stop cleanly on request.

#### Scenario: Deactivate and reactivate
- **WHEN** a consumer is deactivated during production and reactivated later
- **THEN** it resumes from the current producer position without processing the skipped slots
