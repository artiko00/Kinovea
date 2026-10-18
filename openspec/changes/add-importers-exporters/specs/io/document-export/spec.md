# Spec Delta

## Purpose

Produces a report of the key images and their comments as Markdown, optionally converted to ODT or DOCX, so an analysis can be shared as a document.

## ADDED Requirements

### Requirement: Markdown report of the key images
The system SHALL write a Markdown file titled with the video name and, per keyframe inside the selection in time order, a level-2 heading with its name, the time code in italics, the key image reference when images are enabled, and the comment as plain text with Markdown line breaks. Entry points: File > Export > Document > Markdown; `kinovea export markdown`; MCP `export_report`.

#### Scenario: Image files
- **WHEN** images are enabled
- **THEN** each key image is rendered with its drawings into `images/<index>.png` next to the report, indexes zero-padded to the digit count of the keyframe total, and referenced as `![](images/<index>.png){width=100%}`

#### Scenario: Three key images
- **WHEN** a document with three keyframes inside the selection is exported with images enabled
- **THEN** `report.md` matches the oracle `report.md` fixture and `images/0.png`, `images/1.png`, `images/2.png` exist

#### Scenario: Keyframe outside the selection
- **WHEN** a keyframe lies before the selection start
- **THEN** it is absent from the report and the image indexes are still consecutive

### Requirement: ODT and DOCX delegate to pandoc
The system SHALL produce ODT or DOCX by first writing the Markdown report and then invoking `pandoc` with `--resource-path` set to the report directory. When `pandoc` is not found on `PATH` or at the configured path, the export SHALL fail before writing anything with a message naming the tool. Entry points: File > Export > Document > ODT / DOCX; `kinovea export odt|docx`.

#### Scenario: pandoc missing
- **WHEN** `pandoc` is unavailable and a DOCX export is requested
- **THEN** no file is created and the error message contains `pandoc`

#### Scenario: pandoc available
- **WHEN** `pandoc` is available
- **THEN** the output file exists, is a valid zip with `word/document.xml` (DOCX) or `content.xml` (ODT), and the intermediate Markdown is kept next to it
