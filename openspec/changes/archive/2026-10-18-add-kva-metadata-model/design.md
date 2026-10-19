# Design

## Context

See proposal.md for motivation. Constraints that shape this design:

- The original model lives in `Kinovea.ScreenManager/Metadata` (Metadata.cs, 104 KB; 22 `AbstractDrawing` subclasses plus multi-drawing items), with GDI+ painting, WinForms context menus and serialization interleaved in each drawing class. Rust must keep only data, geometry and serialization here; painting moves to `add-headless-renderer` behind a trait.
- KVA is the contract with existing users: the current writer emits `FormatVersion` 2.1 (see `tests/fixtures/kva/*/roundtrip.kva`), readers accept 2.0 and 2.1 directly and convert 1.3 to 1.5 through `xslt/kva-1.5to2.0.xsl`; 1.2 is rejected (`MetadataConverter.cs`).
- Drawing element names are fixed by `[XmlType]` attributes: Angle, Bitmap, Chrono, ChronoMulti, Circle, CoordinateSystem, Counter, CrossMark, DistortionGrid, GenericPosture, Line, AutoNumbers/AutoNumber, Pencil, Plane, Polyline, Rectangle, Spotlights/Spotlight, SVG, TestGrid, Label, TimePolyline, TimeSegment, Track.
- Drawings without an `id` attribute get `Guid.NewGuid()` in C#; fixtures normalize ids by order of appearance (`Kinovea.FixtureGen/Areas/KvaArea.cs`). Rust tests must apply the same normalization before byte comparison.
- `kinovea-core` (timecodes, units, `VideoSection`, `TimedPoint`) and `kinovea-geometry` (points, quadrilaterals, projective mapping) exist from changes 01 and 02; this crate must not duplicate them.

## Goals / Non-Goals

**Goals:**
- One `Metadata` type usable without any UI: load, edit, save, undo, query.
- Byte-level parity of KVA output with the oracle on every fixture.
- A drawing representation that the renderer (06), exporters (12) and MCP server (13) consume through plain data and small traits.

**Non-Goals:**
- Painting, hit testing against rendered geometry, cursors, menus.
- The tracking algorithms behind `Track` and trackable points; only their persisted state.
- Preferences and tool presets persistence beyond reading the shipped tool descriptions.

## Decisions

### D1. Drawings are an enum of structs, not a trait object hierarchy
`Drawing` is `enum Drawing { Line(Line), Angle(Angle), ... }` with a common header struct (`id`, `name`, `reference_timestamp`, `fading`, `style`) embedded in each variant. Cross-cutting behavior (`content_hash`, `kva_name`, `trackable_points`, `write_kva`, `read_kva`) is implemented by small traits on each struct and dispatched by `match`. Alternative: `Box<dyn Drawing>` mirroring `AbstractDrawing`. The enum keeps serialization and pattern matching exhaustive (the compiler fails when a drawing kind is forgotten), gives `Clone`/`PartialEq` for mementos and tests for free, and avoids downcasts the renderer would otherwise need.

### D2. Multi-drawings and singletons share the model
`Spotlights`, `AutoNumbers` are multi-drawings holding items; `CoordinateSystem`, `TestGrid`, `Spotlights`, `AutoNumbers` are singletons stored directly in `Metadata`; chronos and tracks are detached lists. Managers keep the C# ids (`Keyframe.id`, fixed manager ids for singleton/detached collections) so mementos can address `(manager_id, drawing_id)`.

### D3. XML through `quick-xml` with hand-written readers and writers
Alternative: `serde` derive over the KVA schema. The format has element-order dependencies (image size and timing must be read before anything else to compute remapping), `<xsd:any>` drawing blocks, optional elements written conditionally and formatting rules (`R` doubles, `A;R;G;B`). Hand-written `read_kva(&mut Reader)` / `write_kva(&mut Writer)` per type mirror the C# `ReadXml`/`WriteXml` pairs one-to-one, which eases auditing parity. Unknown elements are consumed with `read_to_end` and logged. Dependency decision requiring approval: `quick-xml`.

### D4. Ids
`uuid` crate (`v4` for fresh ids, parse and `hyphenated` display to match `Guid.ToString()`). Dependency requiring approval: `uuid`.

### D5. Migration in Rust, not XSLT
The two stylesheets are 89 and 325 lines of straightforward element renaming plus the unit enum mapping and relative-to-absolute track times. A Rust `migrate_1x_to_2(doc) -> doc` over a DOM (`roxmltree` read, `quick-xml` write) reproduces them and removes a runtime dependency on an XSLT engine on Linux. Parity is proven by a fixture holding the XSLT output produced on the oracle host for a corpus of 1.5 files. Dependency requiring approval: `roxmltree`.

### D6. Content hash
Rust `content_hash()` implements the same field combination as each C# `ContentHash` getter using the same integer arithmetic (`i32` wrapping XOR/add), so hashes can be compared against the oracle summaries. Floats are hashed through their `f32` bit patterns as C# does.

### D7. Undo as serialized mementos
`HistoryStack` plus `Memento` enum (AddDrawing, DeleteDrawing, ModifyDrawing, AddKeyframe, DeleteKeyframe, ModifyKeyframe, AddMultiItem, DeleteMultiItem). Modify mementos store the drawing's KVA XML string (as `DrawingSerializer.SerializeMemento` does) and restore by deserializing in place, keeping undo independent of per-drawing diffing.

### D8. Styles
`StyleElements` is an ordered map (`IndexMap`-like behavior implemented with `Vec<(String, StyleElement)>` to avoid a dependency) preserving KVA element order. Defaults are parsed from `Tools/DrawingTools/Standard/*.xml` embedded at compile time with `include_dir`-style `include_str!` lists so the binary does not depend on the install layout. The 26 standard tools and 16 custom postures are shipped as data in the crate.

### D9. Generic posture
`GenericPosture` is parsed from the XML into data; `ConstraintEngine::move_handle(posture, handle, point, modifiers)` ports `GenericPostureConstraintEngine` and the impact classes. OpenPose index order is the order of `<Point>` elements in `openpose_body25.xml`.

### D10. Tests and parity
`proptest` generates random drawings for round-trip properties (read(write(d)) == d); `insta` is not used, byte comparison against fixtures is done through `kinovea-fixtures`. Dependency requiring approval: `proptest` (dev-dependency).

### Legacy superseded
Replaced by this change and deleted in `remove-dotnet-solution`: `Kinovea.ScreenManager/Metadata/Metadata.cs` (model part), `Metadata/DrawingManager/*`, `Metadata/Drawings/*.cs` data and serialization parts (painting parts are superseded by 06), `Metadata/Drawings/{Cadence,Chrono,CoordinateSystem,GenericPosture}/*`, `Metadata/Styling/**`, `Metadata/Serialization/{MetadataSerializer,DrawingSerializer,KeyframeSerializer,MultiDrawingItemSerializer,MetadataConverter,SerializationFilter}.cs`, `Metadata/Interfaces/*`, `Metadata/ToolManager.cs` (tool loading part), `UndoRedo/**`, `Tracking/Trackability/{TrackablePoint,TrackabilityManager}.cs` (state part), `Kinovea.Services/Types/{InfosFading,KVAExportFlags}.cs`, `Tools/XML/Kva to Kva/*.xsl` (kept as test inputs until then).

### Deviations recorded during apply
- XML: the `kinovea-core` DOM (`Element`) replaces `quick-xml`/`roxmltree`; it now normalizes `\r\n` to `\n` in text and keeps text edge spaces, like `XmlReader` with `IgnoreWhitespace`.
- Properties: seeded loops replace `proptest`; no new dependency besides `uuid`.
- Oracle: the original never sets `ParentMetadata` on the spotlight and number-sequence managers, so importing a KVA with `Spotlights` or `NumberSequence` throws and aborts the import at that element. `KvaArea` sets both before loading; Rust reproduces the fixed behaviour.
- Fuzz corpus: `KvaFuzz` (seeded, 2.1 shapes) instead of the legacy 2.0 `KVAFuzzer`.
- Validity: SVG and file bitmaps are dropped on import when the file is missing, as `IsValid` does; stickers and embedded bitmaps stay.

## Risks / Trade-offs

- [Byte parity is brittle to float formatting] → port `R` formatting as shortest round-trip (`ryu`-style via `format!("{}")` matches for most values); a dedicated test over the fixture corpus catches mismatches; fall back to a declared tolerance only for `Measure` attributes computed from calibration.
- [Fixtures cover only 6 drawing kinds today] → task 1 extends the kva cases with a hand-written `all-drawings.kva` exercising every element name, generated by the oracle itself from the fuzzer, before any Rust code.
- [C# `ContentHash` uses platform `string.GetHashCode`] → hash of strings is excluded from parity comparisons; only numeric components are compared.
- [Posture constraint engine is 27 KB of geometry] → ported after the move fixtures exist; each constraint and impact gets its own fixture case.
- [Embedded tool XML bloats the binary] → ~200 KB total, acceptable.

## Migration Plan

1. Extend oracle cases and areas, regenerate fixtures, commit.
2. Land model and style with unit tests.
3. Land KVA 2.x reader and writer with byte parity.
4. Land migration, undo, posture.
5. Cross-load check on the host.
Rollback: the crate is additive; nothing else depends on it until 06.

## Open Questions

- Whether `Bitmap` and `SVG` drawings embed the image bytes or only the path in 2.1 files produced by the fork; decided when the renderer needs the pixels (06). Both forms are read.
