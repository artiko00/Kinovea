# Tasks

Every task writes the failing test first (`src/<file>_test.rs` with `#[cfg(test)] mod <file>_test;`, or `tests/<name>_test.rs`), then the code, and ends with `cd rust && make verify` and `npm run lint` green. Strings go through `kinovea-ui/src/strings.rs`; no `.resx` is edited (change 18 localises).

## 1. Oracle fixtures and pure models

- [ ] 1.1 Add `PlayerArea` to `Kinovea.FixtureGen` emitting `timemapper.csv` (inputs 0..maxInput step, nominal, real, rounded, up/down large/small) for three timebases (25/25, 30/240, 59.94/1000) and `viewport.csv` (container size × stretch × zoom → rendering rect) from `ViewportManipulator`; add `tests/cases/player/*.json`; run `tools/oracle/fixtures.sh`; commit fixtures. Verify `--verify` identical.
- [ ] 1.2 Port `TimeMapper` to `rust/kinovea-video/src/time_mapper.rs` with parity test over the fixture (tol 1e-9). Verify.
- [ ] 1.3 `rust/kinovea-ui/src/player/viewport.rs`: `ImageTransform` (zoom, pan, rotation, mirror, stretch/fill) and `fit(container, stretch, force_fit)` with parity test over `viewport.csv`; unit tests for zoom-at-cursor invariance. Verify.

## 2. Egui renderer and parity

- [ ] 2.1 `src/render/text_measurer.rs` implementing `kinovea_render::TextMeasurer` over egui fonts; test compares measured widths of fixed strings with a tolerance table. Verify.
- [ ] 2.2 `src/render/egui_renderer.rs`: full `Renderer` impl (paths, strokes with dash/squiggly, text, images, clip, opacity) replacing the change-14 subset; `tests/render_parity_test.rs` renders every scene in `tests/fixtures/render/*/scene.json` via `egui_kittest` and compares to `expected.png` within the declared tolerances. Verify.
- [ ] 2.3 Hit-test parity test: for each scene and the handle positions listed in `scene.json`, `kinovea_render::hit_test` with the egui transform returns the same handle as the headless run. Verify.

## 3. Player state and playback

- [ ] 3.1 `src/player/state.rs`: `PlayerState` (video, metadata, time origin, selection, modified, timecode format) and command handlers for every `PlayerScreenCommands` variant of change 14's `CommandId`; unit tests per command with a fake reader. Verify.
- [ ] 3.2 `src/player/playback.rs`: play loop with monotonic clock, frame drop policy, working-zone loop; tests with a fake clock assert frame sequence at 50%, 100%, 300%. Verify.
- [ ] 3.3 `src/player/speed_slider.rs`: widget bound to `TimeMapper`, labels `x% (y% real)`, `Up/Down`, `Shift+Up/Down`, stickiness; `egui_kittest` test presses keys and asserts label text. Verify.
- [ ] 3.4 `src/player/info_bar.rs` and time-origin command; tests cover negative times and all timecode formats via `kinovea_core::timecode`. Verify.
- [ ] 3.5 Image menu: mirror, rotation, aspect ratio, deinterlace/demosaic toggles, background colour routed to `kinovea_video` frame post-processing and `ImageTransform`; test rotation keeps KVA coordinates. Verify.
- [ ] 3.6 `src/player/magnifier.rs`: model (source rect, factor, inset placement) + painter using `EguiRenderer`; `egui_kittest` snapshot test. Verify.

## 4. Timeline widgets

- [ ] 4.1 `src/timeline/frame_tracker.rs`: `FrameTrackerModel` (`pos_to_x`, `x_to_pos`, markers, cache spans, seek progress, `scrub`, `commit`) + painter; unit tests on mapping and marker hit, `egui_kittest` test clicks a keyframe marker. Verify.
- [ ] 4.2 `src/timeline/selection_tracker.rs`: zone handles, lock, set start/end at current; tests for shrink, lock, and persistence through `kinovea_metadata` working zone. Verify.
- [ ] 4.3 `src/timeline/keyframe_strip.rs`: `KeyframeStripModel` (order, selection, rename, delete, collapse) + painter with thumbnails from the cache; `egui_kittest` test renames with `F2`. Verify.

## 5. Toolbar, drawings, side panel

- [ ] 5.1 `src/toolbar/toolbar_xml.rs`: parse `Tools/DrawingTools/Toolbars/player.xml` (entries, subentries, separators); test asserts the exact button list from the spec. Verify.
- [ ] 5.2 Copy tool icons into `rust/kinovea-ui/assets/icons/` with a `SOURCES.md`; `src/toolbar/icons.rs` loads by tool name; test every toolbar entry has an icon. Verify.
- [ ] 5.3 `src/toolbar/state.rs`: `ToolbarState` (armed tool, sticky rule, creation gesture kind per tool from `ToolManager`) + `src/player/gestures.rs` (click/drag/freehand/multi-click creation, `Enter`/double-click finish); tests create one drawing of each gesture kind. Verify.
- [ ] 5.4 Manipulation: hover handles, drag handle/body, `Delete`, cut/copy/paste/paste-in-place through `kinovea_metadata::clipboard`, undo/redo through `HistoryStack`; `egui_kittest` tests move + undo. Verify.
- [ ] 5.5 Dialogs `Configure drawing` (style, fading), `Configure drawing name`, `Tool presets` (colour profile save/load via `kinovea_metadata::styling`), `Configure video timing`, `Time sections`; `egui_kittest` test changes a colour and asserts the drawing style and undo. Verify.
- [ ] 5.6 `src/player/side_panel/{keyframes,drawing,tracking}.rs` bound to selection; visibility/splitter persisted in `ScreenDescriptorPlayback`; tests on binding and persistence. Verify.
- [ ] 5.7 Context menus and annotation file menu (`Save`, `Save as…`, `Load…` merge, `Unload`, default annotations, `Copy image`, `Copy time`, `Mark as time origin`, export entries disabled until change 12); test load-merge keyframe count. Verify.
- [ ] 5.8 Replace the minimal player in `ScreenManagerState` with `PlayerState`; keep `tests/vertical_slice_test.rs` green and assert the saved KVA equals the previous output. Verify.

## 6. Dual playback

- [ ] 6.1 `src/dual/sync.rs`: `CommonTime` mapping (per-player offsets, extents, union), `DualController` (sync point, merge, alpha, linked speeds, swap sync); unit tests for the spec scenarios. Verify.
- [ ] 6.2 Common controls bar and common timeline widget (`F10` toggle, dual commands from the command table); `egui_kittest` test sets a sync point and steps once. Verify.
- [ ] 6.3 Merge compositor: `kinovea_render::compose(frame_a, frame_b, alpha)` shared with change 12; snapshot test at alpha 0.5. Verify.

## 7. Integration and documentation

- [ ] 7.1 Integration test `tests/player_flow_test.rs`: open synthetic clip, set working zone, add two keyframes, draw one drawing of every toolbar tool, save, reload, compare drawing count and types; run in CI. Verify.
- [ ] 7.2 Manual check on the `windows` host: open the KVA from 7.1 in the original app, confirm every drawing type appears; record in `.agent/progress.md`. Verify entry present.
- [ ] 7.3 Update `AGENTS.md`, `architecture.md` (player screen in Rust), `openspec/roadmap.md` tick 15; verify `npm run lint` and `make verify`.
