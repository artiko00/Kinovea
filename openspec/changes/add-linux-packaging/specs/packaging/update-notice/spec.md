# Spec Delta

## Purpose

Tells users when a newer release exists, without downloading or installing anything, replacing the original updater dialog.

## ADDED Requirements

### Requirement: Opt-in release check
When "Check for updates at startup" is enabled in Preferences > General, the application SHALL query the release feed once per start, in the background, and SHALL never block the UI or fail startup on network errors.

#### Scenario: Newer release available
- **WHEN** the feed lists a version greater than the running one
- **THEN** a dismissible notice shows the new version and a link to the release page, and Help > Check for updates shows the same information on demand

#### Scenario: Offline
- **WHEN** the feed is unreachable
- **THEN** startup completes normally and no error dialog is shown; the Help menu entry reports "could not reach the release feed"

#### Scenario: Disabled
- **WHEN** the preference is off
- **THEN** no network request is made at startup

### Requirement: Version comparison
Release versions SHALL be compared numerically by components (year.minor.patch), ignoring a leading `v` and pre-release suffixes, so that `2026.10.0` is newer than `2026.9.3`.

#### Scenario: Component comparison
- **WHEN** the running version is `2026.9.3` and the feed has `v2026.10.0`
- **THEN** the notice is shown
