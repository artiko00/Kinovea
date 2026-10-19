# metadata/undo-history Specification

## Purpose
Provides undo and redo over the annotation model through serialized mementos, so any consumer (UI or MCP server) can revert and replay edits without knowing drawing internals.

## Requirements

### Requirement: History stack
A history stack SHALL accept mementos, expose can-undo, can-redo, undo and redo action names, step backward and forward, and clear the redo stack when a new memento is pushed.

#### Scenario: Push clears redo
- **WHEN** two mementos are pushed, one undo is performed, then a third memento is pushed
- **THEN** redo is unavailable and undo is available twice

### Requirement: Mementos for drawings and keyframes
Mementos SHALL exist for add drawing, delete drawing, modify drawing, add keyframe, delete keyframe, modify keyframe, add and delete multi-drawing item, and modify video filter. Modify-drawing mementos SHALL capture the drawing's KVA serialization restricted to a serialization filter (core, style, fading) and restore only that part on undo. Keyframe additions and deletions SHALL refresh track keyframe labels.

#### Scenario: Undo restores exact state
- **WHEN** a Line is moved after a core modify memento was pushed and undo is performed
- **THEN** the Line's KVA serialization equals the one captured before the move

#### Scenario: Redo re-applies
- **WHEN** undo then redo is performed on a delete-drawing memento
- **THEN** the drawing is absent again from its keyframe

#### Scenario: Filtered undo keeps the rest
- **WHEN** a fading modify memento of a track is pushed, points are appended to the track and undo is performed
- **THEN** the fading is restored and the appended points remain

#### Scenario: Deleting a keyframe drops its track label
- **WHEN** a keyframe inside a track span is deleted
- **THEN** the track no longer has a keyframe label at that time
