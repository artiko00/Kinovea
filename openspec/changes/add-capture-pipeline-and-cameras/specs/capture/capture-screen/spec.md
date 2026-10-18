# Spec Delta

## Purpose

Provides the capture screen of the Linux application: live view with delay, recording and snapshot controls, annotations over the live image, and dual capture.

## ADDED Requirements

### Requirement: Open a camera in a capture screen
The user SHALL open a camera from the camera list in the file browser panel, or through View > Capture screen, and see the live feed with the camera alias in the screen header.

#### Scenario: Camera loads
- **WHEN** the user double-clicks a camera in the camera list
- **THEN** a capture screen opens, grabbing starts, and the header shows the alias and the measured frame rate

### Requirement: Delay control
The screen SHALL offer a delay slider and numeric field in frames and seconds; changing it changes the displayed age immediately without interrupting grabbing.

#### Scenario: Delay change
- **WHEN** the user sets a 3 second delay at 30 fps
- **THEN** the displayed frame is 90 frames old and the recording start button records the delayed stream

### Requirement: Recording controls
The screen SHALL expose Start/Stop recording, Snapshot, Arm trigger, and the recording-mode indicator, with keyboard shortcuts configurable in Preferences > Keyboard, mirroring the original defaults.

#### Scenario: Toggle recording
- **WHEN** the user presses the record button or its shortcut
- **THEN** recording starts, the button shows the recording state and elapsed time, and pressing again stops it

### Requirement: Annotations on the live image
The user SHALL draw the same drawings as in the player over the live image; the annotations persist across frames and are saved as a KVA file next to each recording when the option is enabled in preferences.

#### Scenario: Linked annotations saved
- **WHEN** a line is drawn and a recording finishes with "Save annotations with recordings" enabled
- **THEN** a KVA 2.0 file with the same base name as the recording contains the line

### Requirement: Dual capture
Two capture screens SHALL run side by side with a shared record/snapshot command from the joint controls.

#### Scenario: Joint record
- **WHEN** two cameras are open and the user uses the joint record button
- **THEN** both screens start recording within one frame interval of each other and both files appear in their captured-files strips
