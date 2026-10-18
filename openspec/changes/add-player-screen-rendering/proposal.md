# Proposal

## Why

Change 14 opens a video and draws one line. Users rely on the full player: timeline with keyframes and working zone, speed control for slow motion, the 29-tool drawing toolbar, zoom, magnifier, image adjustments and two synchronised players. This change ports that screen onto the `Renderer` trait for playback and annotation parity.

## What Changes

- Player controls: frame tracker with keyframe markers, working-zone selection, cache and seek progress; speed slider with the original nominal/real mapping; keyframe strip; info bar; side panel (keyframes, drawing, tracking setup).
- Drawing toolbar generated from `Tools/DrawingTools/Toolbars/player.xml` and the tool XML files, with sub-tool drop-downs, presets and style editor.
- `EguiRenderer`: `kinovea-render::Renderer` backend on `egui::Painter`, so all 29 drawings render and hit-test as the headless renderer of change 06; create, move, resize, configure, cut/copy/paste/delete, undo/redo.
- Viewport: zoom, pan, stretch/fill, rotation, mirror, aspect ratio, deinterlace/demosaic, magnifier.
- Dual player: common controls and timeline, sync point (`F8`), sync merge (`F9`) with alpha, swap sync.
- Player context and `Video`/`Image` menus: time origin, background, save/load/unload annotations, copy image/time; export entries wired when change 12 lands (this change provides the frame compositor both use).

## Capabilities

### New Capabilities
- `ui/player-screen`: playback, navigation, speed, working zone, keyframes, info bar, side panels and image transforms of one player screen.
- `ui/drawing-toolbar`: toolbar generated from the toolbar XML, tool selection, presets, style editing and drawing manipulation on the viewport.
- `ui/timeline`: frame tracker, selection tracker and keyframe strip behaviour, including the common timeline of dual playback.
- `ui/dual-playback`: two synchronised players, sync point, merge and common controls.

### Modified Capabilities
- `ui/screen-manager`: the minimal player screen of change 14 is replaced by the full player screen; the `Minimal player screen for the vertical slice` requirement is superseded.

## Impact

- **Rust**: `rust/kinovea-ui` gains `player/`, `toolbar/`, `timeline/`, `dual/`, `render/`; `kinovea-video` gains the time mapper. `kinovea-metadata`, `kinovea-render`, `kinovea-core` are consumed.
- **Dependencies**: none beyond change 14. Snapshot PNGs under `rust/kinovea-ui/tests/snapshots/`.
- **Assemblies touched**: none on the C# side. `Kinovea.ScreenManager/PlayerScreen/*`, `DualPlayer/*`, `Metadata/DrawingToolbar/*`, `Magnifier.cs` are superseded.
- **Verification**: Linux only: view-model tests, `egui_kittest`, snapshot parity against the change-06 headless renderer (declared tolerance), oracle fixtures for the time mapper and viewport maths.
- **KVA format**: unchanged.

## Non-goals

- Measurement dialogs, kinematics plots, calibration forms (change 16).
- Export dialogs and file writers (change 12), capture screens (change 17), localisation (change 18).
- Video filters beyond deinterlace/demosaic toggles (camera motion and lens filters are change 11 and surface here only as menu entries).
- Replay watcher and crash recovery.
