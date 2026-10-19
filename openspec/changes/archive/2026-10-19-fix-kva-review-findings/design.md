# Design

## Context
Findings come from the Hermes review of `add-kva-metadata-model`; each is verified against the cited C# file before it is fixed, with a test that reproduces it first.

## Decisions
- Parity with the original wins over the earlier spec wording: the 1.2 rejection was a misreading of `MetadataConverter.Convert`, which logs and returns the document unchanged.
- The modify-drawing filter reuses `kva_io::Filter`; restoring a filtered part reads the captured element into a copy of the current drawing so the fields outside the filter keep their current values.

- Track keyframe labels are refreshed where the original calls `UpdateTrajectoriesForKeyframes`: adding and deleting keyframes and the undo of a deletion; `MergeInsertKeyframe` itself does not (Metadata.cs), so neither does Rust.
- Deviation: add and delete on singleton managers return an explicit error; the original silently does nothing.

## Risks / Trade-offs
- Reading 1.2 files as 2.x keeps whatever the 2.x reader understands, as the original does; nothing more is promised.
