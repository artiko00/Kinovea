# Spec Delta

## Purpose

Imports SubRip (`.srt`) subtitle files as timed text annotations so commentary produced elsewhere can be shown over the video.

## ADDED Requirements

### Requirement: SRT cues become timed text drawings
The system SHALL split an SRT file into blocks separated by blank lines, parse the `hh:mm:ss,mmm --> hh:mm:ss,mmm` line into start and end milliseconds (both `,` and `.` accepted as the millisecond separator), strip `<...>` formatting tags from the text lines, and create one text annotation per block visible from start to end. Blocks without a timecode line or without text SHALL be ignored. Entry points: File > Load annotations (`.srt`); `kinovea import <file>`; MCP `import_subtitles`.

#### Scenario: Two cues
- **WHEN** an SRT with cues `00:00:01,000 --> 00:00:02,500` "Hello" and `00:00:03.000 --> 00:00:04.000` "<i>World</i>" is imported into a 25 fps document
- **THEN** two text drawings exist with texts `Hello` and `World`, the first visible between 1000 ms and 2500 ms, and the KVA written afterwards matches the oracle fixture

#### Scenario: Block without text
- **WHEN** a block has only an index and a timecode line
- **THEN** no drawing is created and the following blocks are still imported

#### Scenario: Windows line endings
- **WHEN** the file uses CRLF
- **THEN** the result is identical to the same file with LF
