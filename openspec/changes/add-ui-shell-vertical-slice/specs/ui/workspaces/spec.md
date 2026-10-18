# Spec Delta

## Purpose

Covers window descriptors and workspaces: how one Kinovea instance remembers its screens and layout, and how users name, reopen and group windows, compatible with the XML files the original application writes under the settings directory.

## ADDED Requirements

### Requirement: Window descriptor persisted on exit
Each running instance SHALL own a window descriptor (id, name, last save time, startup mode, screen list, explorer visibility and splitter ratios, window rectangle and state, dual layout) stored in the `Windows` settings directory as XML, written on quit and on layout changes.

#### Scenario: Startup mode `Continue`
- **WHEN** the startup mode is `Continue` and the user quits with two player screens loaded
- **THEN** the next launch recreates two player screens with the same files, positions and layout

#### Scenario: Startup mode `Explorer`
- **WHEN** the startup mode is `Explorer`
- **THEN** the next launch opens with the navigation pane and no screen regardless of the previous session

### Requirement: Descriptor files are compatible with the original format
Window descriptor XML written by the Rust application SHALL be readable by the original application and vice versa for the fields both implement; unknown elements SHALL be preserved on round-trip.

#### Scenario: Round-trip of an original descriptor
- **WHEN** a descriptor file produced by the original Windows application is loaded and saved again without changes
- **THEN** the saved file contains every element of the original with identical values

### Requirement: Manage windows
`Options > Manage windows…` SHALL open a dialog listing known window descriptors with name, last save, screen summary and startup mode, offering `Rename`, `Delete`, `Open in new instance` (`File > Open new window`) and `Properties` (`Options > Window properties…`) to edit name and startup mode.

#### Scenario: Rename a window
- **WHEN** the user renames a window to `Sprint analysis` and presses `OK`
- **THEN** the title bar shows `Kinovea - Sprint analysis` and the descriptor file holds the new name

#### Scenario: Open in new instance
- **WHEN** the user picks a saved window and `Open in new instance`
- **THEN** a second `kinovea` process starts with that descriptor and the current instance is unchanged

### Requirement: Workspaces group windows
`Options > Manage workspaces…` SHALL let the user create a named workspace from the currently open windows, list workspaces, and launch one, which starts one instance per window in the workspace. Workspaces SHALL be stored in the `Workspaces` settings directory as XML compatible with the original format.

#### Scenario: Create and launch a workspace
- **WHEN** two instances are open and the user creates workspace `Two players` then quits and launches it from the dialog
- **THEN** two instances start, each restoring its own window descriptor

#### Scenario: Workspace with missing window
- **WHEN** a workspace references a window descriptor that was deleted
- **THEN** the launch skips it and reports the missing window in the dialog without failing the others
