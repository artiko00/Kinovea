# Spec Delta

## ADDED Requirements

### Requirement: Oracle rasterizes annotations and records hits
The oracle SHALL provide a `render` area that paints each case KVA over a flat frame with the original GDI+ code at the sizes and timestamps declared in the case, writes one PNG per timestamp, and writes a `hits.csv` table of hit-test results over a probe grid, recording the tolerance for Rust comparison in the manifest.

#### Scenario: Render fixtures are reproducible
- **WHEN** the render area runs twice on the host
- **THEN** PNGs and hit tables are byte-identical
