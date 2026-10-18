# Spec Delta

## Purpose

Defines how live frames become files: recording modes, triggers, snapshots, dynamic file naming and capture folders, matching the behavior users rely on in the original capture screen.

## ADDED Requirements

### Requirement: Recording modes
Recording SHALL support the two original modes: Delay (frames are pulled from the delay buffer at the current delay and encoded on the fly) and Scheduled (recording stops automatically after the configured duration).

#### Scenario: Delay mode records the delayed stream
- **WHEN** the delay is 2 seconds and recording starts
- **THEN** the first frame written is the frame that was displayed 2 seconds earlier and the file has no gap relative to the delayed view

#### Scenario: Scheduled mode stops on time
- **WHEN** the maximum duration is 5 seconds and recording starts
- **THEN** recording stops after 5 seconds and the file contains the frames of that window

### Requirement: Output formats
Recording SHALL write MJPEG in MP4, MKV or AVI containers, or raw uncompressed frames in MKV or AVI, with the quality and container chosen in Preferences > Capture > Recording.

#### Scenario: MJPEG MKV
- **WHEN** container MKV, codec MJPEG and 100 frames at 50 fps are recorded
- **THEN** the file opens in the Rust video reader with 100 frames and a declared frame rate of 50 fps

### Requirement: Snapshot
The capture screen SHALL save the currently displayed frame as an image when the snapshot button is pressed or the configured shortcut is used, in the format chosen in preferences (JPEG, PNG, BMP).

#### Scenario: Snapshot during delay
- **WHEN** delay is active and a snapshot is requested
- **THEN** the saved image is the delayed frame currently shown

### Requirement: Dynamic file naming
File names and sub-folders SHALL support the original variables (`%date%`, `%dateb%`, `%time%`, `%year%`, `%month%`, `%day%`, `%hour%`, `%minute%`, `%second%`, `%camalias%`, `%camfps%`, user variables from the profile CSV) and SHALL compute the next name when a file exists.

#### Scenario: Default pattern
- **WHEN** the pattern is `%dateb%-%time%` at 2026-10-18 14:05:09
- **THEN** the resolved file name is `20261018-140509` plus the configured extension

#### Scenario: Counter increment
- **WHEN** `run-07` exists and the pattern resolves to `run-07`
- **THEN** the next file is named `run-08`

### Requirement: Triggers
The capture screen SHALL start recording or save a snapshot (Preferences > Capture > Trigger) when the audio input level exceeds the configured threshold or when a UDP message arrives on the configured port, honoring the quiet period between triggers.

#### Scenario: Audio trigger
- **WHEN** the trigger is armed, the action is Record Video, and the audio level exceeds the threshold
- **THEN** recording starts and a second peak during the quiet period does not retrigger

#### Scenario: UDP trigger
- **WHEN** any UDP datagram arrives on the configured port while armed
- **THEN** the configured action is executed once

### Requirement: Capture folders
Recordings and snapshots SHALL go to the capture folder selected in the screen, from the list configured in Preferences > Capture > Paths, and the resulting file SHALL appear in the captured-files strip of the screen.

#### Scenario: Recorded file listed
- **WHEN** a recording finishes
- **THEN** a thumbnail of it is appended to the captured-files strip and clicking it opens the file in a player screen
