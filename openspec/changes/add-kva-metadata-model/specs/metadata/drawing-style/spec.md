# Spec Delta

## Purpose

Describes how drawings carry their visual style as a dictionary of named elements with typed values and defaults, exactly as persisted in KVA `DrawingStyle` blocks and in the tool description files, so styles survive round trips and presets.

## ADDED Requirements

### Requirement: Style elements are typed and keyed
A style SHALL be a map from element key to a typed element among: color, line size, line shape, line ending, pen shape, pen size, track shape, font size, toggle, integer with bounds. Each element SHALL expose its value, its default, a display name and a stable KVA element name.

#### Scenario: Round trip of a style block
- **WHEN** a `DrawingStyle` XML block from any fixture KVA is parsed and written back
- **THEN** the written XML equals the input after whitespace normalization

#### Scenario: Unknown element kinds are preserved
- **WHEN** a `DrawingStyle` block contains an element kind the model does not know
- **THEN** reading succeeds, the element is kept verbatim and written back unchanged

### Requirement: Tool defaults come from the standard tool descriptions
The default style of each drawing kind SHALL be loaded from the `KinoveaDrawingTool` descriptions shipped in `Tools/DrawingTools/Standard`, keyed by tool name.

#### Scenario: Every standard tool loads
- **WHEN** the standard tool directory is loaded
- **THEN** 26 tools are available and each exposes its drawing kind, default style and attachment flag

### Requirement: Style data binds to elements
Style elements SHALL be bindable to a style data record (color, background color, font, line size, line shape, line ending, track shape, pen shape, grid rows and columns, decimal places, curved, perspective, clock) so that changing an element updates the bound field and vice versa.

#### Scenario: Binding propagates
- **WHEN** the element with key `line color` is set to a new color
- **THEN** the bound style data color equals that color and the style content hash changes

### Requirement: Memorize and restore
A style SHALL support memorizing its current values and restoring them, used by undo of style edits.

#### Scenario: Restore after edits
- **WHEN** a style is memorized, three elements are changed, then restored
- **THEN** every element equals its memorized value
