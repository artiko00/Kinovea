# Proposal

## Why

Every screen, exporter and the future MCP server consume video through one abstraction: the `VideoReader` contract, its frame containers (full cache and bounded prebuffer) and, on the capture side, the lock-free frame pipeline. Porting that layer first, with a synthetic reader instead of FFmpeg, lets the player logic, the renderer and the tests be written against a stable, deterministic source before any native decoder exists.

## What Changes

- Port `Kinovea.Video` core types to `kinovea-video`: video info, capabilities, decoding modes, player state and actions, working zone requests, cache snapshots, video geometry and its request, video summary, open/export/recording results.
- Define the `VideoReader` trait with the same semantics as the C# abstract class: open/close/summary, player and move requests, timestamp relation, playback clock, working zone and decoding mode switching, frame enumeration.
- Port the two frame containers: the dense `Cache` (working zone fully in memory) and the sparse, bounded `PreBuffer` (capacity, retention window behind current, contiguity, blocking add with interruption, eviction rules, published snapshot).
- Port `Kinovea.Pipeline`: fixed-slot ring buffer with power-of-two capacity, producer claim/commit, consumer barrier, drop counting and measured frequency, plus the batch consumer base.
- Add a reader registry mapping extensions to reader factories with the wildcard fallback used by the FFmpeg plugin.
- Add a synthetic reader for tests (frames encode their own timestamp) and a `videocache` oracle area replaying cache operation scripts.

## Capabilities

### New Capabilities
- `video/reader-contract`: how any video source exposes info, capabilities, navigation requests, working zone and frame enumeration.
- `video/frame-cache`: behavior of the dense cache and the bounded prebuffer (acquisition, eviction, contiguity, snapshots, interruption).
- `video/frame-pipeline`: producer/consumer ring buffer semantics for capture (claims, drops, barriers, frequency).

### Modified Capabilities
None.

## Impact

- **Assemblies superseded** (deleted in `remove-dotnet-solution`): `Kinovea.Video` (except reader implementations living in sibling projects) and `Kinovea.Pipeline`. No C# file is modified except `Kinovea.FixtureGen` (new `videocache` area).
- **Rust**: `rust/kinovea-video` gains the code; depends on `kinovea-core` (timestamps, `VideoSection`, `TimedPoint`, rotation and aspect enums from `add-core-types-and-units`). No external crate is required; the concurrency primitives come from `std`.
- **Fixtures**: new `tests/cases/videocache/*.json` and `tests/fixtures/videocache/`.
- **Verification**: the Rust side is fully verified on Linux (`cd rust && make verify`, CI `rust.yml`). The C# oracle area is verified by `tools/oracle/fixtures.sh --verify` on the Windows host; `build.yml` only confirms the solution still compiles.
- No UI, no KVA format impact, no user-facing feature.

## Non-goals

- Decoding real files (`add-ffmpeg-video-reader`, `add-image-gif-svg-readers`).
- Player screen logic (clock, synchronization, rendering): `add-player-screen-rendering`.
- Capture consumers that write files or display frames: `add-capture-pipeline-and-cameras`.
- Benchmark modes of the original pipeline (dead code, commented out in the C# source).
