# Spec Delta

## Purpose

Separates what a drawing paints from how pixels are produced, so the same painting code renders annotations in tests, in a headless server and in the desktop UI through interchangeable backends.

## ADDED Requirements

### Requirement: Backend-neutral primitives
A renderer SHALL provide the primitives used by the original drawings: stroke line, polyline, cardinal-spline curve with tension, ellipse (stroke and fill), arc and pie, rectangle (stroke and fill), rounded rectangle, filled polygon, path fill and stroke, squiggly line, arrow head, text measure and text draw, image blit with source and destination rectangles and optional mirroring, clip to rectangle, and translate and rotate transforms with reset.

#### Scenario: Every drawing paints with the trait only
- **WHEN** the render crate is compiled
- **THEN** the painting module depends only on the renderer trait, the metadata crate and the geometry crate, and no backend type appears in it

#### Scenario: Primitive coverage test
- **WHEN** a recording backend that logs calls is used to paint every drawing kind from the `all-drawings` fixture
- **THEN** every drawing kind issues at least one call and no call outside the primitive set is attempted

### Requirement: Paint derives from style and opacity
Pens, brushes and fonts SHALL be derived from a drawing's style data, the fading opacity factor and the viewport scale as the original does: line width scaled by the viewport, dash patterns for dashed line shapes, alpha multiplied by opacity, fonts scaled by the viewport stretch factor.

#### Scenario: Pen width scales with the viewport
- **WHEN** a Line with line size 4 is painted at viewport scale 0.5
- **THEN** the recorded stroke width is 2

#### Scenario: Faded drawing is skipped
- **WHEN** a drawing's opacity factor at the current timestamp is 0
- **THEN** no primitive is issued for it

### Requirement: Viewport transform
Painting SHALL go through an image-to-viewport transform (location and scale) with transform and untransform of points, distances, sizes and rectangles equivalent to the original transformer.

#### Scenario: Round trip
- **WHEN** a point is transformed then untransformed with scale 1.75 and offset (10, 20)
- **THEN** the result equals the input within 1e-4
