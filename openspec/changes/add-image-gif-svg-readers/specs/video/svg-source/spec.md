# Spec Delta

## Purpose

Exposes a static SVG drawing as a fixed-length video rasterized at the presentation size, so zooming keeps vector sharpness.

## ADDED Requirements

### Requirement: Synthetic timeline for SVG
An SVG SHALL use the same timeline as still images (250 frames, 25 fps, 1 timestamp per frame).

#### Scenario: Timeline
- **WHEN** an SVG is opened
- **THEN** the info reports duration 250 and 25 fps

### Requirement: Size from the document
The original size SHALL come from the SVG width and height attributes, else from the viewBox, else default to 800x600; documents without a usable size SHALL still open.

#### Scenario: Width and height attributes
- **WHEN** an SVG declaring width 300 and height 200 is opened
- **THEN** the original size is 300x200

#### Scenario: viewBox only
- **WHEN** an SVG declaring only a viewBox of 0 0 120 80 is opened
- **THEN** the original size is 120x80

### Requirement: Rasterization at the output size
When a geometry request changes the output size, the reader SHALL re-rasterize the document at that size instead of scaling the previous raster; the rendering SHALL be anti-aliased with a transparent background composited over white.

#### Scenario: Re-rasterize on zoom
- **WHEN** a 100x100 SVG containing a single diagonal line is requested at 400x400
- **THEN** the delivered frame is 400x400 and the line edge is not block-scaled (no 4x4 constant pixel blocks along the edge)

### Requirement: Invalid document
A malformed SVG SHALL return a typed failure on open.

#### Scenario: Malformed XML
- **WHEN** a file containing `<svg><rect` is opened
- **THEN** the result is a not-supported failure
