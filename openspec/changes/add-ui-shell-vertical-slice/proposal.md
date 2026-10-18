# Proposal

## Why

After phases A to D the Rust core is reachable only from tests and the MCP server. This change adds the `egui` desktop shell and proves the stack end to end with one vertical slice, retiring the toolkit risk while the surface is small.

## What Changes

- Add the `kinovea-ui` crate (egui application); the `kinovea` binary becomes its launcher, keeping `--version` and reserving `--headless` for the MCP server.
- Main window with the original `File / Edit / View / Image / Video / Tools / Options / Help` menus; entries without a Rust implementation are shown disabled.
- Navigation pane: file explorer, favourites, thumbnails of the current folder, recent history; cameras tab disabled until change 17.
- Screen manager hosting one or two screens (side by side or stacked), swap and close. The only screen type is a minimal player: show the frame, scrub, draw a `Line`, select and move it.
- Window and workspace descriptors persisted as the original XML files so an instance restores its layout; `Manage windows` and `Manage workspaces` dialogs.
- Preferences dialog with the General and Player tabs backed by the change-01 model; other tabs disabled.
- Acceptance: launch, open a video from the explorer, draw a line, `File > Save` writes a KVA 2.0 file the original Kinovea opens unchanged.

## Capabilities

### New Capabilities
- `ui/main-window`: application window, menu bar, hotkey dispatch, status and layout persistence.
- `ui/file-browser`: navigation pane with explorer, favourites, thumbnails and history that opens files into a screen.
- `ui/screen-manager`: one or two screens, screen descriptors, swap/close, dual layout, and the minimal player screen used by the vertical slice.
- `ui/workspaces`: windows and workspaces descriptors persisted and restored, with their management dialogs.

### Modified Capabilities
None.

## Impact

- **Rust**: new crate `rust/kinovea-ui`; `rust/kinovea` becomes the launcher; `kinovea-metadata`, `kinovea-video`, `kinovea-render`, `kinovea-core` are consumed, not modified.
- **Dependencies (approval required)**: `eframe`/`egui`, `wgpu`, `rfd`, `egui_extras`, `egui_kittest` (dev); `.github/workflows/rust.yml` gains system packages for headless UI tests.
- **Assemblies touched**: none on the C# side. `Kinovea/`, `Kinovea.FileBrowser/`, `Kinovea.Services/Window/` and `Kinovea.ScreenManager/ScreenManager*.cs` are superseded, removed in `remove-dotnet-solution`.
- **Verification**: Linux only, `cd rust && make verify` (view-model tests, `egui_kittest`). The slice's KVA is opened once in the original app on the `windows` host (manual, in tasks).
- **KVA format**: unchanged; the slice writes through the serializer of change 05.

## Non-goals

- Full player controls, drawing toolbar, timeline, magnifier, dual synchronisation (change 15).
- Measurement dialogs and plots (change 16), capture screens (change 17), localisation (change 18), packaging (change 19).
- Pixel parity with the WinForms layout: `Designer.cs` files are not ported, the layout is redesigned in egui.
- Crash recovery, replay watcher, custom pointers, splash screen, help system links.
