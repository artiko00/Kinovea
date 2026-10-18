# Spec Delta

## Purpose

Makes the Linux build installable by end users as an AppImage or a Flatpak, integrated with the desktop (launcher entry, icon, `.kva` file association), without requiring a Rust toolchain.

## ADDED Requirements

### Requirement: AppImage runs on a clean system
The AppImage SHALL start on a stock x86_64 Linux desktop without additional packages beyond the base system libraries declared in the packaging documentation, and SHALL embed all locale files, drawing tool definitions and schemas the application needs.

#### Scenario: Version from the bundle
- **WHEN** the AppImage is executed with `--version` on a clean container image of the oldest supported distribution
- **THEN** it prints `kinovea <version>` matching the version single source and exits 0

#### Scenario: Open a video from the bundle
- **WHEN** the AppImage is launched with a path to a sample video
- **THEN** the main window opens the video in a player screen

### Requirement: Flatpak bundle
A Flatpak manifest with application id `org.kinovea.Kinovea` SHALL build with the declared freedesktop runtime and SHALL request only the permissions needed: display, audio input for the trigger, camera devices, host file access for videos, network for cameras and the update notice.

#### Scenario: Build and install locally
- **WHEN** the manifest is built and the resulting bundle installed for the user
- **THEN** the application appears in the desktop launcher and runs with the same `--version` output as the AppImage

### Requirement: Desktop integration
The bundles SHALL install a desktop entry, icons at 16 to 512 px plus scalable, a MIME definition for `application/x-kinovea-kva` matching `*.kva` and the KVA XML root element, and AppStream metadata.

#### Scenario: Validators pass
- **WHEN** the desktop file and the metainfo file are validated with the standard freedesktop tools
- **THEN** both report no errors

#### Scenario: File association
- **WHEN** the user opens a `.kva` file from the file manager after installation
- **THEN** Kinovea starts, locates the video referenced by the KVA and loads the annotations, or shows the "video not found" dialog from the player when the video is missing
