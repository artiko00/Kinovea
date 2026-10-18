# Spec Delta

## Purpose

Specifies synchronised playback of two player screens for side-by-side comparison, including sync point, merged view and common controls.

## ADDED Requirements

### Requirement: Common controls appear with two players
When two player screens are hosted, `View > Common controls` (`F10`) SHALL toggle a common control bar with play/pause, first/previous/next/last, go to sync point, sync merge toggle, swap sync and the common timeline; commands SHALL act on both players.

#### Scenario: Common play
- **WHEN** two videos are loaded and the user presses play in the common controls
- **THEN** both players play at their own speed settings and the common playhead advances

### Requirement: Synchronisation point
`Set sync point` (`F8` in a player, common button) SHALL mark the current frame of each player as the common origin; navigating in common time SHALL keep the offset; the sync point SHALL persist in each screen descriptor.

#### Scenario: Sync offset kept while stepping
- **WHEN** A is synced at 2.0 s and B at 1.0 s and the user steps forward once in common controls
- **THEN** A shows 2.0 s + one frame and B shows 1.0 s + one frame

### Requirement: Sync merge
`Sync merge` (`F9`) SHALL overlay the second player's frame on the first with an alpha adjustable by `Alt++/-`; when active both viewports show the merged image and drawings of both players.

#### Scenario: Toggle merge
- **WHEN** merge is enabled with alpha 50%
- **THEN** each viewport shows both frames blended at 50% and disabling merge restores the separate frames

### Requirement: Dual speed and swap
Changing speed in one player while synced SHALL be independent per player unless `Link speeds` is enabled; `Swap sync` SHALL exchange which player leads.

#### Scenario: Linked speeds
- **WHEN** `Link speeds` is on and the user sets player A to 50%
- **THEN** player B is set to 50% as well
