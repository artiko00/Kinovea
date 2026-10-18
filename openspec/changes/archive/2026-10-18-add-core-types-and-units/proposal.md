# Proposal

## Why

Every later crate (geometry, kinematics, metadata, video) needs the same primitives the C# code keeps in `Kinovea.Services`: timecode formatting, measurement units and their conversions, video timestamp ranges, timed points and the preferences model. Porting them first, against fixtures already produced by the oracle, gives the workspace its first real parity tests and fixes the vocabulary the rest of the migration builds on.

## What Changes

- Port `TimecodeFormat`, `TimecodeType`, `TimeHelper.GetTimestring` and `TimeHelper.MillisecondsToTimecode` to `kinovea-core`, byte-for-byte against `tests/fixtures/timecode/formats/*.csv`.
- Port the unit enums (`LengthUnit`, `SpeedUnit`, `AccelerationUnit`, `AngleUnit`, `AngularVelocityUnit`, `AngularAccelerationUnit`, `CadenceUnit`) with their abbreviations and the numeric conversions of `UnitHelper` (velocity, acceleration, angular velocity and acceleration, frequency, cross length-unit conversion, pixel size text, significant-digit rounding).
- Port `VideoSection` (timestamp range with empty and wrapped states), `TimedPoint` (with its `x;y;t;r` text form), `ThreePartsVersion` ordering and the timestamp mapper function type.
- Port the preferences model: typed general, player, capture and file-explorer preferences with the C# defaults, read and written as the existing `Preferences.xml` format 3.0, without any UI or save-on-set behaviour.
- Add two oracle areas to `Kinovea.FixtureGen`: `units` (conversion tables) and `preferences` (default preferences file), with cases under `tests/cases/`.

## Capabilities

### New Capabilities
- `core/timecode-formatting`: rendering of a time position in every supported timecode format, with the precision rules derived from the frame rate.
- `core/measurement-units`: unit enumerations, abbreviations and conversions between length, speed, acceleration, angular and frequency units.
- `core/video-time-primitives`: timestamp sections, timed points, version ordering and timestamp mapping used across the player and metadata.
- `core/preferences-model`: typed preferences with defaults and a file format compatible with the original `Preferences.xml`.

### Modified Capabilities
None.

## Impact

- **Assemblies touched**: none of the shipped C# assemblies change. `Kinovea.FixtureGen` gains two areas (`Areas/UnitsArea.cs`, `Areas/PreferencesArea.cs`) and `tests/cases/{units,preferences}/` are added. `tests/fixtures/` grows accordingly.
- **Rust**: `rust/kinovea-core` gets its first modules; `kinovea-fixtures` is a dev-dependency for parity tests. No new third-party dependency except `serde` (already a workspace dependency) for the preferences model.
- **Verification**: Rust side fully verified on Linux (`cd rust && make verify`, CI `rust.yml`). The new FixtureGen areas are verified by `tools/oracle/fixtures.sh --verify` on the Windows host; `build.yml` remains the secondary check that the solution still compiles.

## Non-goals

- Any UI, including the preferences dialog and the timecode display in the player.
- `TimeMapper` and slow-motion mapping (lives in the player and is ported in `add-video-reader-and-cache`).
- Localized unit names: only the abbreviations and enum names are ported; translations arrive with `add-i18n-fluent`.
- Logging configuration and settings directories handled by `Software.cs`.
