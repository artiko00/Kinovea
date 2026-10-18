# Spec Delta

## MODIFIED Requirements

### Requirement: Minimal player screen for the vertical slice
The player screen SHALL be the full player screen specified in `ui/player-screen`, `ui/drawing-toolbar` and `ui/timeline`. The minimal screen of the vertical slice (frame display, slider, `Line` tool only) is replaced; its behaviours remain available as a subset of the full screen.

#### Scenario: Vertical slice still passes
- **WHEN** the vertical-slice test opens a clip, draws a line and saves
- **THEN** the produced KVA is identical to the one produced before this change for the same inputs

#### Scenario: Full toolbar present
- **WHEN** a player screen is created
- **THEN** the toolbar shows every tool of the toolbar definition, not only `Line`
