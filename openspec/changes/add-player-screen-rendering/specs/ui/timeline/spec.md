# Spec Delta

## Purpose

Describes the frame tracker, working-zone selector and keyframe strip: the time navigation widgets of the player and of dual playback.

## ADDED Requirements

### Requirement: Frame tracker
The timeline SHALL show the working zone extent, the playhead, keyframe markers (clickable), the sync point marker, cache coverage and seek progress; dragging the playhead SHALL scrub (display frames while dragging) and releasing SHALL commit the position.

#### Scenario: Click a keyframe marker
- **WHEN** the user clicks a keyframe marker on the timeline
- **THEN** the player seeks exactly to that keyframe

#### Scenario: Scrub shows frames
- **WHEN** the user drags the playhead across 50 frames
- **THEN** at least every cached frame under the cursor is displayed during the drag and the final position is the frame under release

### Requirement: Working zone selection
A selection tracker SHALL let the user set start and end of the working zone by dragging handles, by `Set start`/`Set end` buttons at the current frame, by `Lock`; the zone SHALL be persisted in the KVA, and changing it SHALL trigger cache reload per the video crate policy.

#### Scenario: Shrink the zone
- **WHEN** the user drags the end handle to the middle of the video
- **THEN** playback loops within the new zone and the duration label updates

#### Scenario: Locked zone
- **WHEN** the zone is locked and the user drags a handle
- **THEN** the handles do not move

### Requirement: Keyframe strip
A horizontal strip SHALL list keyframes in time order with thumbnail, title and colour, support selection, rename (`F2`), delete, `Show comments`, drag to reorder in time, and collapse/expand persisted in the screen descriptor.

#### Scenario: Rename in strip
- **WHEN** the user presses `F2` on a keyframe box and types `Take-off`
- **THEN** the KVA keyframe title becomes `Take-off` and the timeline marker tooltip shows it

### Requirement: Common timeline for dual playback
When two players are synchronised the common controls SHALL show one common timeline spanning both working zones aligned at the sync point, with both playheads and keyframe markers of each player in its colour.

#### Scenario: Common timeline extents
- **WHEN** player A has a 10 s zone, player B a 6 s zone, synced at A=4 s, B=1 s
- **THEN** the common timeline spans from the earliest start to the latest end in common time and both playheads coincide at the sync point
