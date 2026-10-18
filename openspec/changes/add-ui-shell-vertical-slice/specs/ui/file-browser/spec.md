# Spec Delta

## Purpose

Describes the navigation pane: the file explorer, favourite folders, thumbnail viewer and recent history through which users open videos into screens, replacing the WinForms `FileBrowserUserInterface`.

## ADDED Requirements

### Requirement: Explorer tab lists folders and supported files
The navigation pane SHALL show a `Files` tab with a folder tree rooted at the user home and mounted volumes, and, for the selected folder, a thumbnail grid of files whose extension is in the supported video/image list of the video crate. Hidden files SHALL be omitted unless the preference `Show hidden files` is on.

#### Scenario: Select a folder
- **WHEN** the user clicks a folder in the tree
- **THEN** the thumbnail grid lists its supported files sorted by name, each with file name, duration when known and a first-frame thumbnail generated in the background

#### Scenario: Unsupported files are hidden
- **WHEN** the folder contains `notes.txt` and `clip.mp4`
- **THEN** only `clip.mp4` appears in the grid

### Requirement: Opening a file loads it into a screen
Double-clicking a thumbnail, pressing `Enter` on the selected thumbnail or using `File > Open video…` SHALL load the file into the active player screen, creating one if none exists. When two screens exist, the file goes to the active screen and the other is left untouched.

#### Scenario: Open from thumbnail
- **WHEN** the user double-clicks `clip.mp4`
- **THEN** the active player screen shows the first frame of `clip.mp4` and the window title includes `clip.mp4`

#### Scenario: Open with no screen
- **WHEN** no screen exists and the user opens a file
- **THEN** one player screen is created and the file is loaded into it

### Requirement: Favourites and history
The pane SHALL show a `Favourites` list of user-chosen folders (add/remove through the explorer context menu, persisted in preferences) and `File > History` SHALL list the last 10 opened files, most recent first, with `Reset history`.

#### Scenario: Add favourite
- **WHEN** the user right-clicks a folder and chooses `Add to favourites`
- **THEN** the folder appears in `Favourites` and survives a relaunch

#### Scenario: History updates
- **WHEN** the user opens `a.mp4` then `b.mp4`
- **THEN** `File > History` lists `b.mp4` first and `a.mp4` second

### Requirement: Thumbnail size and keyboard navigation
The thumbnail grid SHALL offer sizes `Extra small` to `Extra large` through a selector and `Ctrl+Mouse wheel`, persisted per window, and SHALL support arrow keys to move the selection and `F2` to rename the selected file.

#### Scenario: Resize thumbnails
- **WHEN** the user selects `Large`
- **THEN** thumbnails re-layout to the larger size without regenerating images from the video

#### Scenario: Rename
- **WHEN** the user selects a thumbnail, presses `F2`, types a new name and presses `Enter`
- **THEN** the file is renamed on disk and the grid shows the new name

### Requirement: Cameras tab placeholder
The pane SHALL show a `Cameras` tab that is disabled with the text `Capture is not available yet` until the capture change is implemented.

#### Scenario: Cameras tab disabled
- **WHEN** the user clicks the `Cameras` tab
- **THEN** the tab does not activate and the placeholder text is visible as tooltip
