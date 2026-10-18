# Tasks

Every task writes the failing test first (`rust/<crate>/src/<file>_test.rs` declared with `#[cfg(test)] mod <file>_test;`, or `rust/<crate>/tests/<name>_test.rs`), then the code, and ends with `cd rust && make verify` and `npm run lint` green. UI text goes through `kinovea-ui/src/strings.rs`; no `.resx` is edited (localisation is change 18).

## 1. Crate and launcher (dependencies need approval before 1.1)

- [ ] 1.1 Add `rust/kinovea-ui` to the workspace with `eframe`, `egui`, `egui_extras`, `rfd` and dev-dependency `egui_kittest` pinned in `[workspace.dependencies]`; `lib.rs` exposes `run(args: ShellArgs) -> Result<(), ShellError>`; test in `src/lib_test.rs` asserts `ShellArgs::parse` handles a file path argument. Verify `cargo build -p kinovea-ui` links on Linux.
- [ ] 1.2 Rewrite `rust/kinovea/src/main.rs` to delegate to `kinovea_ui::run` except for `--version` and `--headless` (reserved, prints "headless mode is not available yet", exit 2); update `src/main_test.rs` and `tests/main_test.rs`. Verify both exit codes.
- [ ] 1.3 Add `kinovea-ui/src/strings.rs` with English constants keyed like the original `.resx` keys used by this change and a test (`strings_test.rs`) asserting no key is empty or duplicated. Verify.

## 2. Main window, menus, hotkeys

- [ ] 2.1 `src/menus.rs`: static menu table (`File`, `Edit`, `View`, `Image`, `Video`, `Tools`, `Options`, `Help`) with `enabled_when` predicates; test `menus_test.rs` asserts order and that every entry maps to a `CommandId` or a `Disabled` marker. Verify.
- [ ] 2.2 `src/commands.rs`: `CommandId` enum (union of `PlayerScreenCommands`, `DualPlayerCommands`, file explorer commands) and `CommandTable::defaults()` with the original key bindings; hand-write `tests/cases/ui/hotkeys.json` from `HotkeySettingsManager.cs`; test `commands_test.rs` loads the json and compares. Verify.
- [ ] 2.3 `src/shell.rs`: `ShellState` with `handle_hotkey`, `active_screen`, `dialogs: DialogStack`; tests cover hotkey routing to the active screen and no-op when none. Verify.
- [ ] 2.4 `src/app.rs`: `eframe::App` impl drawing menu bar, navigation pane placeholder, screen area; `egui_kittest` test `tests/menu_test.rs` opens `File` and asserts `Open video…` enabled and `Open replay folder…` disabled. Verify headless.
- [ ] 2.5 `src/preferences_dialog.rs`: General and Player tabs bound to `kinovea_core::prefs`, other tabs disabled; OK writes, Cancel discards; test with a temp settings dir. Verify.

## 3. Descriptors and workspaces

- [ ] 3.1 Add `WindowArea` to `Kinovea.FixtureGen` writing `window.xml`, `screen-playback.xml`, `workspace.xml` through the original `WriteXML`; add `tests/cases/window/*.json`; run `tools/oracle/fixtures.sh`; commit `tests/fixtures/window/`. Verify `fixtures.sh --verify` identical.
- [ ] 3.2 `kinovea-core/src/window/{window_descriptor,screen_descriptor,workspace_descriptor}.rs` with quick-xml read/write preserving unknown elements; parity tests read the fixtures and round-trip byte-equal after normalisation. Verify.
- [ ] 3.3 `kinovea-ui/src/workspace.rs`: `WorkspaceState` listing descriptors from the settings dirs, rename, delete, create workspace, launch (spawns `kinovea --window <id>`); tests on a temp settings dir. Verify.
- [ ] 3.4 Dialogs `Manage windows`, `Window properties`, `Manage workspaces`, `Workspace name` in `src/dialogs/`; `egui_kittest` test renames a window and asserts the descriptor file. Verify.
- [ ] 3.5 Window geometry persistence and off-screen correction in `shell.rs` (`restore_geometry(monitors)`); unit tests for the off-screen case. Verify.

## 4. Navigation pane

- [ ] 4.1 `src/file_browser/state.rs`: `FileBrowserState` with folder tree model (home, mounted volumes from `/proc/mounts`), supported-extension filter from `kinovea_video::supported_extensions()`, hidden-file preference, favourites and history (10 entries) persisted via `kinovea_core::prefs`; tests on a temp tree. Verify.
- [ ] 4.2 `src/file_browser/thumbnails.rs`: worker pool with bounded queue and LRU cache keyed by (path, mtime) producing RGBA thumbnails via `kinovea_video`; test with the synthetic clip (`ffmpeg -f lavfi testsrc`, generated in `build.rs` of the test crate as in change 08). Verify.
- [ ] 4.3 `src/file_browser/view.rs`: tabs `Files`/`Cameras` (disabled placeholder), tree, thumbnail grid with size selector (`Ctrl+wheel`), `F2` rename, context menu `Add to favourites`; `egui_kittest` test selects a folder and double-clicks a file emitting `UiEvent::OpenFile`. Verify.

## 5. Screen manager and minimal player

- [ ] 5.1 `src/screen_manager/state.rs`: `ScreenManagerState` with 0..2 screens, `set_count`, `swap`, `close(idx)` with unsaved check, active index, layout (horizontal/vertical, ratio); unit tests for every transition in the spec. Verify.
- [ ] 5.2 `src/player/minimal.rs`: `MinimalPlayerState` wrapping `kinovea_video::Player` (open, seek, play/pause), `kinovea_metadata::Metadata`, tool enum `{Pointer, Line}`, `begin_drag/drag/end_drag` creating a keyframe with a `Line` drawing or moving a hit drawing via `kinovea_render::hit_test`; tests with a fake reader. Verify.
- [ ] 5.3 `src/player/view.rs`: viewport painting through an `EguiRenderer` implementing `kinovea_render::Renderer` for the subset needed (line, handles), frame slider, play button, tool buttons; `egui_kittest` test drags a line and asserts one keyframe with one line. Verify.
- [ ] 5.4 Save path: `File > Save`/`Save as…` through `kinovea_metadata::serializer::save`, sidecar `<video>.kva` auto-load on open, modified flag in title; test saves, validates against `Tools/XML/Schema/kva.2.0.xsd` (via `kinovea-metadata` validator), reloads. Verify.
- [ ] 5.5 Vertical-slice integration test `tests/vertical_slice_test.rs`: launch `App` headless, open the synthetic clip from the browser state, draw a line, save, reload, compare. Verify in CI.

## 6. CI and documentation

- [ ] 6.1 After approval, edit `.github/workflows/rust.yml` to install `libxkbcommon-dev libwayland-dev libxcb-*-dev libgtk-3-dev` (rfd) before the build; verify the job passes on a PR.
- [ ] 6.2 Manual parity check: copy the KVA produced by 5.5 to the `windows` host, open it in the original Kinovea with the same clip, confirm the line and keyframe; record the result in `.agent/progress.md`. Verify entry present.
- [ ] 6.3 Update `AGENTS.md` structure (`rust/kinovea-ui`), `architecture.md` (new crate replaces `Kinovea`, `Kinovea.FileBrowser`), `openspec/roadmap.md` tick 14; verify `npm run lint` and `make verify`.
