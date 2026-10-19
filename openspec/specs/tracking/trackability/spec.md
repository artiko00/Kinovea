# tracking/trackability Specification

## Purpose
Specifies how the trackable points of drawings obtain their position at any time, from object tracks, camera motion or their reference value, and how that binding is stored in annotation files.

## Requirements

### Requirement: Trackable point reference
A trackable point SHALL hold a reference timestamp and value; setting a new reference SHALL clear any cached camera-transformed positions.

#### Scenario: Camera transform cached
- **WHEN** a camera transformer is initialized and the position at a time is asked twice
- **THEN** the transform is computed once and the same value is returned

### Requirement: Drawing tracker modes
A drawing tracker SHALL be in one of three modes: object tracking (each trackable point bound to a track id), camera tracking (positions transformed from the reference time), or none (reference value); object tracking SHALL take precedence when initialized and a bound track has data at the requested time.

#### Scenario: Object tracking position
- **WHEN** a point is bound to a track with a position at time 40 and the time 40 is requested
- **THEN** the drawing receives the track position and the age 0

#### Scenario: Camera tracking without object tracking
- **WHEN** object tracking is not initialized and a camera transformer is initialized
- **THEN** the drawing receives the camera-transformed reference value

#### Scenario: Object tracking without a sample
- **WHEN** object tracking is initialized and the bound track has no position at the requested time
- **THEN** that point is not updated (the original sends nothing for it, and no camera fallback applies)

#### Scenario: No tracking
- **WHEN** neither object tracking nor camera motion is available
- **THEN** no position is sent to the drawing, and asking a point position at a time returns its reference value

### Requirement: Manager lifecycle
The manager SHALL add a tracker for every trackable drawing, assign drawings loaded from a file to their tracker by id, forget trackers of deleted drawings, trim tracks after a time, report whether any drawing is actively tracking, and expose a content hash that changes when object tracking is initialized for a drawing or a tracker is forgotten.

#### Scenario: Assign after load
- **WHEN** trackers are read from a file before their drawings and a drawing with a matching id is assigned
- **THEN** the tracker reports assigned and the drawing's points resolve through it

#### Scenario: Forget
- **WHEN** a drawing is deleted
- **THEN** its tracker is removed and the content hash changes

### Requirement: Serialization
The manager SHALL write a `Trackers` section with one element per tracker holding the drawing id and its `TrackablePoint` entries (key, reference timestamp, reference value, bound track id when any) and SHALL read it back identically; tracking parameters SHALL serialize as `TrackerParameters` with the exact element names of the original format.

#### Scenario: Round trip
- **WHEN** a manager with two trackers without object tracking is written and read back
- **THEN** the content hash is unchanged (the object-tracking flag is part of the hash but not of the file, as in the original)

#### Scenario: Sample files
- **WHEN** the `Trackers` and `TrackerParameters` sections of the sample annotation files are parsed
- **THEN** no element is reported unknown and re-serialization reproduces them

### Requirement: Timeline container
A time-indexed timeline SHALL insert values at times, return the closest value from a time, the last value, trim after a time, clear with a disposer, and enumerate in time order.

#### Scenario: Closest from
- **WHEN** values exist at 0, 10, 20 and the closest from 14 is asked
- **THEN** the value at 10 is returned
