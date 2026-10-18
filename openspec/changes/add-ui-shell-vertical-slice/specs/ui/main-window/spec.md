# Spec Delta

## Purpose

Defines the Kinovea desktop application window on Linux: how it starts, its menu bar and hotkeys, and how it persists its own geometry, so every screen added later is hosted by one consistent shell.

## ADDED Requirements

### Requirement: Application starts into a main window
The `kinovea` executable SHALL open a resizable main window titled `Kinovea` when launched without `--version` or `--headless`, and SHALL exit 0 when the window is closed through the window manager or `File > Quit` (`Ctrl+Q`).

#### Scenario: Default launch
- **WHEN** the user runs `kinovea` with no arguments on a Linux desktop session
- **THEN** a window appears within 2 seconds showing the menu bar, the navigation pane and one empty screen area

#### Scenario: Launch with a file argument
- **WHEN** the user runs `kinovea path/to/video.mp4`
- **THEN** the window opens with that video loaded in the first player screen

#### Scenario: Command-line version still works
- **WHEN** the user runs `kinovea --version`
- **THEN** no window opens and the version string is printed with exit code 0

### Requirement: Menu bar mirrors the original structure
The main window SHALL show the top-level menus `File`, `Edit`, `View`, `Image`, `Video`, `Tools`, `Options`, `Help` in this order. Entries whose feature is not implemented yet SHALL be visible but disabled, so the menu structure matches the original application from the first release.

#### Scenario: Implemented entries are enabled
- **WHEN** the user opens the `File` menu
- **THEN** `Open video…` (`Ctrl+O`), `Open replay folder…` (disabled), `History`, `Save` (`Ctrl+S`), `Save as…`, `Close`, `Quit` are listed and the implemented ones are enabled

#### Scenario: Unimplemented entry is disabled
- **WHEN** the user opens `Tools`
- **THEN** `Linear kinematics`, `Angular kinematics`, `Angle-angle diagram`, `Scatter diagram`, `Calibration validation` are listed and disabled with no action on click

### Requirement: Hotkeys dispatch to the active screen
The window SHALL route key presses to the active screen through a command table equivalent to the original defaults (for example `Space` toggles play, `Left`/`Right` step one frame, `Ctrl+O` opens a file, `Ctrl+S` saves annotations, `Escape` resets the viewport).

#### Scenario: Hotkey reaches the active player
- **WHEN** a video is loaded in the active player screen and the user presses `Space`
- **THEN** playback toggles in that screen and no other screen reacts

#### Scenario: Hotkey with no active screen
- **WHEN** no screen is loaded and the user presses `Space`
- **THEN** nothing happens and no error is logged

### Requirement: Window geometry and panel state persist
The window SHALL persist its position, size, maximised state, navigation-pane visibility and splitter ratio in the window descriptor so that the next launch restores them.

#### Scenario: Restore after relaunch
- **WHEN** the user resizes the window, hides the navigation pane with `View > Navigation pane` (`F4`), quits, and launches again
- **THEN** the window reopens with the same size and the navigation pane hidden

#### Scenario: Off-screen geometry is corrected
- **WHEN** the persisted rectangle lies entirely outside the current monitors
- **THEN** the window opens centred on the primary monitor at its default size

### Requirement: Preferences dialog exposes the implemented tabs
`Options > Preferences…` SHALL open a modal dialog with the tabs `General`, `Player`, `Drawings`, `Capture`, `Keyboard`; tabs without a Rust implementation SHALL be disabled. Changes SHALL be written to the preferences file on `OK` and discarded on `Cancel`.

#### Scenario: Change a general preference
- **WHEN** the user changes the UI language or the time format in `General` and presses `OK`
- **THEN** the preferences file contains the new value and the main window applies it without restart

#### Scenario: Cancel discards
- **WHEN** the user edits a value and presses `Cancel`
- **THEN** the preferences file is unchanged
