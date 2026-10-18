# Spec Delta

## Purpose

Determines which drawing, and which of its handles, lies under a point in image coordinates, so headless callers and the UI can select and manipulate annotations consistently with the original tolerances.

## ADDED Requirements

### Requirement: Hit result contract
Hit testing a drawing at a point and timestamp SHALL return miss, body (the drawing as a whole), or a handle index greater than zero, using the same handle numbering as the original drawing kind.

#### Scenario: Line handles
- **WHEN** a Line from (100,100) to (300,100) is hit tested at (100,100), (300,100), (200,100) and (200,150) at viewport scale 1
- **THEN** the results are handle 1, handle 2, body, and miss

### Requirement: Tolerances follow the viewport scale
Point hits SHALL use a box of 10 viewport pixels converted to image pixels (minimum 1); path hits SHALL widen the path by the line size plus 10 viewport pixels; filled areas SHALL hit anywhere inside.

#### Scenario: Zoomed-in viewport shrinks tolerance
- **WHEN** a point 8 image pixels away from a handle is tested at viewport scale 2
- **THEN** the result is miss, and at viewport scale 0.5 the result is that handle

### Requirement: Parity with the oracle hit tables
Hit results for the probe grid recorded by the oracle SHALL match for every drawing kind.

#### Scenario: Probe grid
- **WHEN** every probe in `tests/fixtures/render/<case>/hits.csv` is evaluated
- **THEN** the returned code equals the recorded one for at least 99 percent of probes, and all mismatches lie within 2 image pixels of a region boundary

### Requirement: Invisible drawings are not hit
A drawing whose opacity factor is zero at the timestamp SHALL return miss.

#### Scenario: Faded out
- **WHEN** a drawing fully faded at the current timestamp is tested at its own anchor point
- **THEN** the result is miss
