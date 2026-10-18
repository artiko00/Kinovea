# Spec Delta

## Purpose

Produces an annotated image of a video frame and its metadata without any window, so tests can compare against the original rendering and the MCP server can return annotated frames.

## ADDED Requirements

### Requirement: Scene composition order
Rendering a frame SHALL paint the source image (with mirroring when set and the metadata background color), then detached chronos, tracks, singleton drawings, then attached drawings: with default fading enabled all keyframes in fading z-order, otherwise only the current keyframe, each keyframe's drawings from last to first, exactly as the original player.

#### Scenario: Overlap order
- **WHEN** two attached drawings overlap on the same keyframe
- **THEN** the first drawing in the keyframe list is painted on top

### Requirement: CPU backend produces buffers and PNG
A CPU backend SHALL implement the renderer trait and output an RGBA8 buffer of the requested size and encode it as PNG.

#### Scenario: Snapshot of a sample
- **WHEN** the `complete` sample KVA is rendered at 848×480 over a flat grey frame at each fixture timestamp
- **THEN** the PNG decodes to the same size as the fixture and the per-channel mean absolute difference against `tests/fixtures/render/<case>/<timestamp>.png` is below the tolerance stored in the fixture manifest, and at most 1 percent of pixels differ by more than 64

#### Scenario: Deterministic output
- **WHEN** the same scene is rendered twice
- **THEN** the two buffers are byte-identical

### Requirement: Headless operation
Rendering SHALL require no display server, GPU or font installed on the system.

#### Scenario: CI run
- **WHEN** the test suite runs on a Linux CI runner without a display
- **THEN** all snapshot tests pass using the bundled font
