# Tasks

## 1. Timecode formatting (`rust/kinovea-core/src/timecode.rs`)

- [x] 1.1 Write `rust/kinovea-core/src/timecode_test.rs` with failing tests: enum classification table, name round-trip, precision at 25 and 1000 fps, carry case `59995 ms → 1:00.00`; declare `#[cfg(test)] mod timecode_test;` in `lib.rs`; verify `cargo test -p kinovea-core` fails to compile for the missing module.
- [x] 1.2 Implement `TimecodeFormat`, `TimecodeType`, `timecode_type()`, `ms_to_timecode(ms: f64, precision: i32)` with sign, truncation and carry rules in `timecode.rs`; verify 1.1 tests pass and `cd rust && make verify` is green.
- [x] 1.3 Add `rust/kinovea-core/tests/timecode_parity_test.rs` reading `tests/fixtures/timecode/formats/ms_to_timecode.csv` through `kinovea_fixtures::fixtures_root()` and asserting every row; add `kinovea-fixtures` as dev-dependency in `rust/kinovea-core/Cargo.toml`; verify the test passes for all rows.
- [x] 1.4 Implement `timestring(framerate, frames, ms, timestamps, duration_ts, total_frames, format, symbol)` and extend `timecode_parity_test.rs` with `timecodes.csv` (all nine formats × 3 frame rates); verify every row matches byte for byte and `make verify` is green.

## 2. Measurement units (`rust/kinovea-core/src/units.rs`)

- [x] 2.1 Add `tests/cases/units/conversions.json` (magnitude lists per conversion) and `Kinovea.FixtureGen/Areas/UnitsArea.cs` writing `abbreviations.csv`, `velocity.csv`, `acceleration.csv`, `angular_velocity.csv`, `angular_acceleration.csv`, `frequency.csv`, `pixel_size.csv`; register the area in `Program.cs`; verify `tools/oracle/fixtures.sh --verify` reports identical trees and commits `tests/fixtures/units/conversions/`.
- [x] 2.2 Write `units_test.rs` with failing tests: abbreviation of every enum value, `10 m/s → 36 km/h`, `1 km/h → 100000 cm`, minimal digits (12.34, 3.456, 0.0234); verify they fail.
- [x] 2.3 Implement the seven unit enums with `abbreviation()`, `convert_velocity`, `convert_acceleration`, `convert_angular_velocity`, `convert_angular_acceleration`, `convert_frequency`, `convert_for_length_unit`, `minimal_digits`, `pixel_size_text` in `units.rs` (split into `units/convert.rs` if over 150 lines); verify 2.2 passes.
- [x] 2.4 Add `rust/kinovea-core/tests/units_parity_test.rs` asserting every CSV of `tests/fixtures/units/conversions/` (relative tolerance 1e-6, exact text for abbreviations and pixel size); verify all rows pass and `make verify` is green.

## 3. Video time primitives

- [x] 3.1 Write `video_section_test.rs` (containment, equality of empties, text form, default/empty constructors, length of wrapped section) and `timed_point_test.rs` (four-field round-trip, three-field legacy parse, content hash stability); verify they fail.
- [x] 3.2 Implement `VideoSection` (`Copy`, `PartialOrd` following `CompareTo`) in `video_section.rs` and `TimedPoint` with `Display`/`FromStr` in `timed_point.rs`; verify 3.1 passes.
- [x] 3.3 Write `version_test.rs` (ordering of `2023.1`, `2023.1.1`, `2024.1`, two-part parse) and `timestamp_mapper_test.rs` (identity); implement `ThreePartsVersion` in `version.rs` and `TimestampMapper` type alias + `identity` in `timestamp_mapper.rs`; verify tests pass and `make verify` is green.

## 4. Preferences model

- [x] 4.1 Add `quick-xml` (serialize feature) to `rust/Cargo.toml` workspace dependencies and `kinovea-core` after approval; add `tests/cases/preferences/defaults.json`, `tests/cases/preferences/modified.json` + `modified.xml` (edited values plus an unknown `FutureSetting` element), and `Kinovea.FixtureGen/Areas/PreferencesArea.cs` that points the settings directory at the output folder, initializes preferences, loads the input when given and saves `Preferences.xml`; verify `fixtures.sh --verify` is green and `tests/fixtures/preferences/{defaults,modified}/Preferences.xml` are committed.
- [x] 4.2 Write `preferences/general_test.rs` and `preferences/explorer_test.rs` asserting defaults against the values in `GeneralPreferences.cs` and `FileExplorerPreferences.cs`; implement `GeneralPreferences` and `FileExplorerPreferences` structs with `Default`; verify tests pass.
- [x] 4.3 Write `preferences/player_test.rs` asserting player defaults (timecode format, units, filtering flags, timeline jumps, keyframe presets, export profiles); implement `PlayerPreferences` with `Default`, splitting sub-structs into `preferences/player/*.rs` files under 150 lines; verify tests pass.
- [x] 4.4 Write `preferences/capture_test.rs` asserting capture defaults (frame rate replacement thresholds, recording paths, delay, formats); implement `CapturePreferences`; verify tests pass.
- [x] 4.5 Write `preferences/xml_test.rs`: defaults serialize element-wise equal to `tests/fixtures/preferences/defaults/Preferences.xml`, `modified` round-trips known values and drops `FutureSetting`, missing path yields defaults; implement `Preferences::load(path)` / `save(path)` in `preferences/xml.rs` with `FormatVersion="3.0"`; verify tests pass and `make verify` is green.

## 5. Integration

- [x] 5.1 Re-export the new modules from `rust/kinovea-core/src/lib.rs`, remove the placeholder `crate_name()` only if no test depends on it, update `tests/README.md` (areas `units`, `preferences`) and `.agent/progress.md`; verify `cd rust && make verify`, `npm run lint` and `cargo test -p kinovea-fixtures` (manifest commit consistency) are all green.
