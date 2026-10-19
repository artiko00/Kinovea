# Spec Delta

## Purpose

Guarantees that the Rust implementation reads and writes Kinovea Video Analysis files (KVA, XML, format 2.0 and 2.1) with full fidelity to the original application, so analyses move between the C# application and the Rust fork without loss.

## ADDED Requirements

### Requirement: Reading KVA 2.x
The reader SHALL accept KVA documents with `FormatVersion` 2.0 or 2.1 and populate the metadata: general information, calibration, keyframes with drawings, chronos, tracks, spotlights, number sequence, coordinate system, test grid, trackability, video filters and camera motion.

#### Scenario: Sample files load completely
- **WHEN** `Tools/XML/Schema/samples/{minimal,simple,complete}.kva` are loaded
- **THEN** keyframe, attached drawing and track counts equal the `summary.json` of the matching fixture

#### Scenario: Unknown drawing elements are skipped without failure
- **WHEN** a `Drawings` block contains an element whose name matches no drawing kind
- **THEN** loading succeeds, the element is skipped and a warning naming the element is reported

#### Scenario: Elements without id receive stable ids
- **WHEN** a drawing or keyframe element has no `id` attribute
- **THEN** it is assigned a fresh id, and the test normalization maps ids to `00000000-0000-4000-8000-<n>` by order of appearance before comparing with fixtures

### Requirement: Writing KVA 2.1
The writer SHALL produce a `FormatVersion` 2.1 document with the element order and formatting of the original application: UTF-8, two-space indentation, invariant culture numbers, `R` round-trip doubles, booleans in lower case, colors as `A;R;G;B`, points as `X;Y`.

#### Scenario: Byte-level parity on round trip
- **WHEN** each sample is loaded and written with the producer set to `Kinovea.0.0`
- **THEN** the output equals `tests/fixtures/kva/<case>/roundtrip.kva` byte for byte after id normalization

#### Scenario: Export flag subsets
- **WHEN** writing with the capture-recording flags (basics, drawings, calibration)
- **THEN** the document omits chronos, tracks, spotlights, number sequence, trackability, video filters and camera motion, and contains `CaptureRecording` instead of the file path elements

### Requirement: Context remapping on import
When the imported file was created on a different video, the reader SHALL scale coordinates by the ratio of image sizes and remap timestamps using the file and current average timestamps per frame and first timestamps, as the original application does.

#### Scenario: Same video, no remapping
- **WHEN** the current context matches the file's image size and timing
- **THEN** every coordinate and timestamp is loaded unchanged

#### Scenario: Different image size
- **WHEN** the file declares 1280×720 and the current video is 640×360
- **THEN** every point coordinate is halved and the `kva/remap` fixture values are matched within 1e-4

### Requirement: Interoperability with the original application
Files written by the Rust writer SHALL load in the original C# application with the same drawings, and files written by the original application SHALL load in Rust.

#### Scenario: Cross-load on the oracle host
- **WHEN** a Rust-written KVA for each sample is loaded by `Kinovea.FixtureGen` on the Windows host
- **THEN** the oracle reports identical keyframe, drawing and track counts and an identical content hash list
