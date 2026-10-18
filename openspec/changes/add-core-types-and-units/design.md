# Design

## Context

See proposal.md for motivation. Current state that shapes the port:

- `Kinovea.Services/TimeHelper.cs` formats time with `Math.Log10(framerate)` driven precision; `MillisecondsToTimecode` truncates the fraction above two digits and carries centiseconds below. Fixtures for both already exist (`tests/fixtures/timecode/formats/`).
- `Kinovea.ScreenManager/Measurement/Calibration/UnitHelper.cs` holds abbreviations and conversions as `switch` tables over the enums in `Kinovea.Services/Types/Units/`. It lives in ScreenManager only because of history; nothing in it depends on UI.
- `VideoSection`, `TimedPoint`, `ThreePartsVersion`, `TimestampMapper` are small value types in `Kinovea.Services/Types/`. `TimedPoint` is serialized inline as `x;y;t;r` inside KVA tracks, so its text form is a file-format contract.
- Preferences: `PreferencesManager` is a static facade over four classes (`GeneralPreferences`, `PlayerPreferences`, `CapturePreferences`, `FileExplorerPreferences`) that each save on every setter. The XML file has `FormatVersion="3.0"` and one element per preference. There is no fixture yet.
- `Kinovea.Services` has no `Fraction` type despite the roadmap note; nothing to port there.

## Goals / Non-Goals

**Goals:**
- Exact parity for every formatted string (timecodes, abbreviations, pixel-size text) and tolerance-based parity for conversions.
- Value types usable from `no_std`-free plain Rust without allocation on the hot path (`VideoSection`, `TimedPoint` are `Copy`).
- Preferences as a plain data model with explicit load/save, no global state.

**Non-Goals:**
- Replicating the static `PreferencesManager` singleton or its save-on-set behaviour.
- Supporting preference formats older than 3.0 (the C# code does not migrate them either).

## Decisions

### D1. Module layout mirrors the C# files
`rust/kinovea-core/src/` gets `timecode.rs` (format enum, type classification, `timestring`, `ms_to_timecode`), `units.rs` (enums + abbreviations + conversions), `video_section.rs`, `timed_point.rs`, `version.rs`, `timestamp_mapper.rs`, and `preferences/{mod,general,player,capture,explorer,xml}.rs`. Each has a sibling `<file>_test.rs`. Alternative: one `types.rs`; rejected because `testlens` and the 150-line limit favour small files.

### D2. Floating point widths follow the original
Where C# uses `float`, the port uses `f32` and converts to `f64` only where the original does (`UnitHelper.ConvertAngularVelocity` returns `double`). This keeps parity within the declared tolerances without reproducing `.NET` rounding by accident. Comparisons in tests use `kinovea_fixtures::approx_eq` with relative tolerance 1e-6 for conversions and exact equality for strings.

### D3. Timecode precision uses `f64::log10` and `ceil` like the original
`precision = ceil(log10(framerate))` as `i32`; `Normalized` format builds its `0.000…` pattern from `ceil(log10(total_frames))`. Negative or zero frame rates are not special-cased (the original does not), but the tests document the behaviour at 1 fps (`precision = 0`, falls into the centisecond branch).

### D4. Enum names are the text form
Timecode formats, units and preference enums serialize as their C# names (`ClassicTime`, `MetersPerSecond`…) via `serde` with `rename_all = "PascalCase"` where needed, because KVA and `Preferences.xml` store those names. Parsing an unknown name is an error; callers that need a default (preferences loading) apply it explicitly.

### D5. Preferences XML via `quick-xml` + `serde`
Decision requiring approval: add `quick-xml` (with `serialize` feature) to the workspace. Alternative: hand-rolled `roxmltree` reading plus manual writing. `quick-xml` is already the planned XML backend for KVA (`add-kva-metadata-model`), so this change introduces it once. The model is written in the same element order as the C# writer so a diff against the fixture is readable; comparison in tests is order-insensitive.

### D6. Oracle areas
`Kinovea.FixtureGen/Areas/UnitsArea.cs` writes `abbreviations.csv`, `velocity.csv`, `acceleration.csv`, `angular_velocity.csv`, `angular_acceleration.csv`, `frequency.csv`, `pixel_size.csv` over a fixed list of magnitudes declared in `tests/cases/units/conversions.json`. `Areas/PreferencesArea.cs` writes the default `Preferences.xml` (case `defaults`) and a copy of a hand-written input file loaded and saved by the original (case `modified`, input stored under `tests/cases/preferences/modified.xml` and referenced by the case). `Oracle.Initialize` already calls `PreferencesManager.Initialize()`; the area redirects `Software.SettingsDirectory` to the case output via the portable `AppData` folder already used by the generator.

### Legacy superseded
`Kinovea.Services/TimeHelper.cs`, `Types/TimeCodeFormat.cs`, `Types/TimeCodeType.cs`, `Types/VideoSection.cs`, `Types/TimedPoint.cs`, `Types/Units/*.cs`, `ThreePartsVersion.cs`, `Preferences/{GeneralPreferences,PlayerPreferences,CapturePreferences,FileExplorerPreferences,PreferencesManager,PreferencesHelper,IPreferenceSerializer}.cs`, and `Kinovea.ScreenManager/Measurement/Calibration/UnitHelper.cs`. Deleted in `remove-dotnet-solution`.

### Deviations found while implementing
- `FormatVersion` is an element, not an attribute; the preferences root also carries a `Keyboard` section that the original always writes. It is kept as an opaque subtree and written back verbatim until the UI change models shortcuts.
- `quick-xml` is used through a small element tree (`preferences/dom.rs`) instead of serde derive: the file mixes conditional lists, fixed element order and custom number formats, which a DOM handles directly. `uuid` (v4) is also added for capture folder ids; the final `add-kva-metadata-model` change needs both anyway.
- `EnableHighSpeedDerivativesSmoothing` is never written or read by the original; it stays an in-memory field with default `true`.
- The default capture folders embed the user's Documents folder and fresh GUIDs. The oracle replaces them by `{MyDocuments}` and stable ids; the Rust test canonicalises both sides the same way and treats `\` and `/` in paths as equal.
- `Milliseconds`/`Microseconds` timecodes reproduce the `(int)` overflow of the original (`int.MinValue` above 2^31 µs), as the fixtures capture it.

## Risks / Trade-offs

- [`.NET` `Math.Round` is banker's rounding; Rust `f64::round` is half-away-from-zero] → use `round_ties_even` where the original rounds (`MillisecondsToTimecode` centiseconds, `Milliseconds`/`Microseconds` formats); the fixture rows at `.5` boundaries catch regressions.
- [`string.Format("{0}", double)` in C# prints up to 15 significant digits] → the units area writes with `R` through `Oracle.R`, so Rust compares numbers, not strings; only the pixel-size text relies on C# default formatting of a `float`, which is reproduced with a dedicated `format_f32_default` helper tested against the fixture.
- [Preferences surface is large (~150 values)] → port group by group with the fixture diff as the oracle; values not yet modelled fail the defaults test loudly rather than silently.
- [`quick-xml` serde support has limits with attributes] → the only attribute is `FormatVersion`; handled with `@FormatVersion` field naming.

## Migration Plan

1. Land timecode and units with existing/new fixtures (no risk to C#).
2. Land value types.
3. Land preferences model and fixtures. Rollback is deleting the new modules; nothing else depends on them yet.

## Open Questions

- Whether `CadenceUnit` abbreviations should be localized later or stay fixed; irrelevant to this change (abbreviations are the C# constants).
