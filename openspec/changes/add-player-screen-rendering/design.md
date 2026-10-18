# Design

## Context

See proposal.md for motivation. Constraints:

- `PlayerScreenUserInterface2.cs` is 6 630 lines mixing playback state, drawing manipulation, menus, rendering and WinForms plumbing; `PlayerScreen.cs` (1 130) is the presenter, `FrameServerPlayer.cs` the video facade, `TimeMapper.cs` the speed mapping, `ViewportManipulator.cs` the zoom/pan maths, `FrameTracker.cs`/`SelectionTracker.cs`/`KeyframeBox.cs` the custom controls, `DualPlayerController.cs` the sync logic. The behaviour is the contract; the file split is not.
- Change 06 defines `kinovea-render::Renderer` (paths, text, images, hit testing) and a `tiny-skia` backend with snapshot fixtures of every drawing type. Change 05 owns drawings, styling, keyframes and undo (`HistoryStack`). Change 07/08 own the frame cache, working zone and `supported_extensions`.
- Change 14 provides `ShellState`, `ScreenManagerState`, `CommandTable`, `EguiRenderer` for the subset `Line`, and the test harness (`egui_kittest`, synthetic clips).
- egui is immediate mode: there is no retained control tree. Timeline, speed slider and keyframe strip become custom widgets painted each frame from state.

## Goals / Non-Goals

**Goals:**
- Full player parity in behaviour and hotkeys; one `Renderer` implementation shared by every drawing.
- Rendering parity checked mechanically: the same scene rendered by `tiny-skia` (change 06) and by `EguiRenderer` through `egui_kittest` differ only within a declared tolerance.
- Every widget's logic unit-testable without egui.

**Non-Goals:**
- Pixel-identical appearance to WinForms controls.
- Re-implementing OxyPlot plots or measurement dialogs (change 16).

## Decisions

### D1. Split by responsibility, not by the original files
Modules in `kinovea-ui/src/player/`: `state.rs` (`PlayerState`: video, metadata, time origin, selection, tools, modified flag), `playback.rs` (play loop driven by egui `request_repaint_after`, frame scheduling from `TimeMapper`), `viewport.rs` (zoom/pan/rotation/mirror as one `ImageTransform` matrix, port of `ImageToViewportTransformer` + `ViewportManipulator` + `ImageTransform.cs`), `toolbar/`, `timeline/`, `keyframes.rs`, `side_panel/`, `menus.rs`, `dual/`. Alternative: one big `player.rs` mirroring the C#; rejected because `linelens` (150 lines) and testability.

### D2. `TimeMapper` and speed slider are ported verbatim and oracle-checked
`TimeMapper` is pure maths with non-obvious stickiness and rounding. It is ported to `kinovea-video::time_mapper` (not UI) and a new `PlayerArea` in `Kinovea.FixtureGen` emits `timemapper.csv` (input → nominal, real, rounded, next step up/down) for several timebases. The slider widget only reads the mapper.

### D3. `EguiRenderer` is the single drawing backend
`kinovea-ui/src/render/egui_renderer.rs` implements `kinovea_render::Renderer` over `egui::Painter`: `fill_path`, `stroke_path`, `text` (via egui fonts with a `TextMeasurer` adapter so layout decisions made in drawings match the headless backend), `image`, `clip`, with the `ImageTransform` applied once at the top. Hit testing stays in `kinovea-render::hit_test` (pure geometry) so it is identical in both backends. Alternative: let each drawing paint egui directly; rejected because it duplicates 29 drawings and breaks the MCP headless path.

### D4. Toolbar from XML through `kinovea-metadata::tools`
Change 05 parses the standard/custom tool XML into `ToolManager`. This change adds the toolbar XML parser (`toolbar.rs`: `KinoveaToolbar { entries: Vec<Entry { tool, sub: Vec<SubEntry|Separator> }> }`) and a `ToolbarState` (`armed_tool`, sticky rule, presets). Icons: the original PNG resources are reused (GPL, same repo) via `egui_extras` image loading, keyed by tool name, loaded from `Tools/DrawingTools/Icons/` copied into the crate's `assets/` (decision: copy once, document source).

### D5. Timeline widgets as pure models + painters
`FrameTrackerModel` (min, max, position, keyframes, sync marker, cache spans, seek progress; methods `hit(x)`, `pos_to_x`, `x_to_pos`, `scrub(x)`, `commit()`), `SelectionTrackerModel` (zone, lock, handles), `KeyframeStripModel` (ordering, selection, collapse). Painters are thin. The common timeline reuses `FrameTrackerModel` with two playheads and a `CommonTime` mapping from `dual/sync.rs`.

### D6. Dual playback as a controller over two `PlayerState`s
`DualController` (port of `DualPlayerController`) holds sync offsets, merge flag and alpha, linked speeds; it drives both players through their command API and composes the merged frame with the compositor (`kinovea-render::compose` with alpha) that change 12 also uses for side-by-side export.

### D7. Rendering parity tests
For each drawing type the snapshot fixtures of change 06 (`tests/fixtures/render/<drawing>/scene.json` + `expected.png` produced by `tiny-skia`) are rendered through `EguiRenderer` in `egui_kittest` at the same size; the test asserts mean absolute difference ≤ 2/255 and max difference ≤ 48/255 on anti-aliased edges (tolerances declared in `tests/render_parity_test.rs`, tuned once and committed).

## Legacy superseded

Removed in `remove-dotnet-solution`:
- `Kinovea.ScreenManager/PlayerScreen/*` (all controls, forms `FormConfigureDrawing2`, `FormConfigureDrawingName`, `FormConfigureSpeed`, `FormConfigureVisibility`, `FormTimeSections`, `FormToolPresets`, `FrameServerPlayer`, `TimeMapper`, `ViewportManipulator`, `PlayerScreen`, `MemoPlayerScreen`, `LineStyle`), except `Forms/FormCalibrate*`, `FormCalibrationValidation`, `FormConfigureTrajectoryDisplay` which belong to change 16.
- `Kinovea.ScreenManager/DualPlayer/*`, `Metadata/DrawingToolbar/*`, `Metadata/Magnifier.cs`, `Metadata/HitTester.cs`, `ImageToViewportTransformer.cs`, `IImageToViewportTransformer.cs`, `ImageTransform.cs`, `ViewportManipulator.cs`, `ScreenPointerManager.cs`, `PointerManager.cs`, `DrawingClipboard.cs`, `MessageToaster.cs`.

## Risks / Trade-offs

- [egui text rendering differs from GDI+ so label boxes size differently] → the `TextMeasurer` adapter is the only source of text metrics for both backends in the Rust app; parity with the C# output is not a goal, parity between Rust backends is.
- [Playback timing jitter in immediate mode] → the play loop schedules frames from a monotonic clock and drops frames per the original policy; a test with a fake clock asserts frame selection.
- [6 630-line UI hides behaviours not in the inventory] → each module's tasks start with a read of the matching C# region; gaps found are logged in `.agent/progress.md` and added as tasks, not silently dropped.
- [Icon assets licensing] → they are part of this GPLv2 repository; copying within the repo keeps the licence.

## Migration Plan

1. Port `TimeMapper` + oracle fixture; viewport transform; `EguiRenderer` with parity tests (no UI change visible).
2. Replace the minimal player with the full `PlayerState` and widgets (timeline, speed, keyframes).
3. Toolbar from XML, manipulation, style dialogs, side panel, context menus.
4. Dual controller and common controls.
Rollback per step: the minimal player of change 14 stays in the tree until step 2 is green.

## Open Questions

- Whether the magnifier should render through a second `EguiRenderer` pass or a texture crop; both satisfy the spec, decide by measuring frame time on a 4K clip.
