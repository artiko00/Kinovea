# Spec Delta

## Purpose

Lets a single raster image be opened, navigated and annotated as a short fixed-length video, matching the behavior the original application exposes for photos.

## ADDED Requirements

### Requirement: Synthetic timeline for still images
A still image SHALL be exposed as a video of 250 frames at 25 fps with 1 timestamp per frame, first timestamp 0, a working zone from 0 to 249, and a summary flagged as image with zero duration in milliseconds.

#### Scenario: Timeline constants
- **WHEN** a PNG is opened
- **THEN** the info reports 25 fps, 40 ms interval, duration 250 timestamps and the working zone ends at 249

#### Scenario: Any timestamp inside the zone yields the image
- **WHEN** a move to timestamp 137 is requested
- **THEN** the current frame carries timestamp 137 and the image pixels equal the file content

### Requirement: Move next from the initial state
The first move-next request SHALL deliver frame 0; subsequent move-next requests SHALL advance one frame; moving past the working zone end SHALL report not acquired.

#### Scenario: First next is frame zero
- **WHEN** an image is opened and move next is requested once
- **THEN** the current timestamp is 0

### Requirement: Orientation
EXIF orientation tags SHALL be applied when decoding so the original size reports the displayed size; a user rotation request SHALL re-generate the image rotated and swap the reference size for 90 and 270 degrees. Mirroring tags SHALL be treated as their rotation-only counterpart.

#### Scenario: EXIF rotated JPEG
- **WHEN** a 40x30 JPEG with EXIF orientation 6 (rotate 90) is opened
- **THEN** the original size reports 30x40

#### Scenario: User rotation
- **WHEN** a 40x30 PNG is opened and a 90 degree rotation geometry is requested
- **THEN** the reference size is 30x40 and the delivered frame is 30x40

### Requirement: Supported extensions
The reader SHALL register for `.jpg`, `.jpeg`, `.png` and `.bmp`, case-insensitive, and SHALL return a typed failure for a corrupt file.

#### Scenario: Corrupt file
- **WHEN** a file with a `.png` extension containing random bytes is opened
- **THEN** the result is a not-supported failure and the reader reports not loaded

### Requirement: Thumbnail
Summary extraction SHALL return one thumbnail scaled to the requested maximum width keeping the aspect ratio.

#### Scenario: Thumbnail width
- **WHEN** a 400x300 image is summarized with maximum width 100
- **THEN** one 100x75 thumbnail is returned

### Requirement: Info matches the original readers
The video info computed for generated images SHALL equal the values recorded by the original readers in the `imagesource` fixtures.

#### Scenario: Fixture comparison
- **WHEN** the `imagesource` fixture cases are replayed on images generated with the same dimensions and frame delays
- **THEN** every info field matches exactly
