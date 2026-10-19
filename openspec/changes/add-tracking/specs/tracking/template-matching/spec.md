# Spec Delta

## Purpose

Specifies the correlation-based point tracker: given the last tracked position and a stored template, locate the point in the next frame, decide success, refine to sub-pixel precision and update the template.

## ADDED Requirements

### Requirement: Tracking parameters and defaults
Tracking parameters SHALL include algorithm (correlation by default), search window (default 100x100), block window (default 20x20), similarity threshold (default 0.5), template update threshold (default 0.8), mask usage with HSV range, dilate and erode counts, and reset-on-move; window sizes SHALL be clamped to at most 400 pixels per side.

#### Scenario: Defaults
- **WHEN** parameters are created without arguments
- **THEN** they report the default values above

#### Scenario: Clamping
- **WHEN** a 900x900 search window is set
- **THEN** the stored window is 400x400

### Requirement: Readiness
The tracker SHALL be ready only when it holds a template whose size equals the block window for the last tracked point; otherwise a reference point SHALL be created from the current image before tracking.

#### Scenario: Block window changed
- **WHEN** the block window is changed after a template was stored
- **THEN** the tracker reports not ready

### Requirement: Match within the search window
A track step SHALL compute the normalized cross-correlation coefficient of the template over the search window centered on the last position (cropped to the image), take the best location, and succeed when the best similarity is at least the similarity threshold.

#### Scenario: Translated synthetic target
- **WHEN** a square target rendered by formula moves 3 pixels right and 2 down between two frames and the tracker steps
- **THEN** the new point is within 0.5 pixel of the true position and the step reports success

#### Scenario: Target lost
- **WHEN** the target leaves the search window
- **THEN** the step reports failure, the new point keeps the previous location and its score is 0

### Requirement: Sub-pixel refinement
When the best location is not on the border of the similarity map and the similarity is below 1.0, the location SHALL be refined by fitting a parabola through the three horizontal and three vertical neighbors and taking each peak; otherwise the integer location SHALL be used.

#### Scenario: Peak on the border
- **WHEN** the best match is on the first column of the similarity map
- **THEN** no refinement is applied

#### Scenario: Half-pixel motion
- **WHEN** a smooth target rendered by formula moves 0.5 pixel
- **THEN** the refined position is within 0.15 pixel of the true position

### Requirement: Template update policy
After each step the previous template SHALL be kept when the similarity is above the update threshold or below the similarity threshold (a confident match or a failure); otherwise (a fair match) the template SHALL be replaced by the image patch at the new location, as `TrackerTemplateMatching.CreateTrackPoint` does.

#### Scenario: Confident match keeps the template
- **WHEN** a step succeeds with similarity 0.95 and update threshold 0.8
- **THEN** the stored template is unchanged

#### Scenario: Fair match updates the template
- **WHEN** a step succeeds with similarity 0.7, similarity threshold 0.5 and update threshold 0.8
- **THEN** the stored template is the new patch

### Requirement: Mask
When mask usage is enabled, the correlation SHALL be restricted to the ellipse inscribed in the block window (the mask the original paints with GDI+); when the masked correlation yields an infinite score or a score above 1 the tracker SHALL fall back to the unmasked correlation. The HSV range, dilation and erosion parameters are stored but not used by the tracker, as in the original.

#### Scenario: Mask excludes the block corners
- **WHEN** a disc moves over a gradient background with mask usage enabled
- **THEN** each step succeeds with similarity above the threshold and the corners of the block do not take part in the correlation

### Requirement: Trim and clear
Trimming at a time SHALL drop templates after that time; clear SHALL drop all templates and the tracker SHALL report not ready.

#### Scenario: Trim
- **WHEN** templates exist at times 0, 10, 20 and the tracker is trimmed at 10
- **THEN** only the templates at 0 and 10 remain

### Requirement: Parity with the original tracker
On the synthetic sequences of the `tracking` oracle area the Rust tracker SHALL produce positions within 0.5 pixel and similarity scores within 0.02 of those recorded from the original implementation, with identical success/failure flags.

#### Scenario: Fixture replay
- **WHEN** every `template-*` case under `tests/fixtures/tracking` is replayed
- **THEN** all rows satisfy the tolerances
