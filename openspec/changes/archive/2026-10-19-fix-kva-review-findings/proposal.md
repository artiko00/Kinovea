# Proposal

## Why

An adversarial review of the archived `add-kva-metadata-model` change (Hermes, verified against the C# sources) found places where the Rust port of the KVA loader, the legacy migration and the undo history diverge from the original. Fixtures did not cover them. The most visible: legacy track keyframe labels computed before relative times are fixed, files of format 1.2 and older rejected while the original reads them, and modify mementos that restore the whole drawing instead of the serialized part they captured.

## What Changes

- Legacy import fixes relative trajectory times before the post-import pass, so track keyframe labels use absolute times.
- Formats 1.2 and older are read as they are, with the version reported, instead of rejected (the original logs and loads them).
- Adding or deleting keyframes refreshes track keyframe labels.
- Modify-drawing mementos carry a serialization filter (core, style, fading) and restore only that part; video-filter mementos restore one filter; deleting a multi-drawing item or a track cleans the trackability state like the original; duplicate drawing ids are not added.
- The migration writes empty elements produced by `xsl:value-of` with an explicit end tag and compares unit codes without trimming; timestamp remapping rounds ties to even.

## Capabilities

### Modified Capabilities
- `metadata/kva-migration`: format detection reads 1.2 and older instead of rejecting them.
- `metadata/undo-history`: modify mementos are filtered; keyframe edits refresh track labels.

## Impact

- `rust/kinovea-metadata` only. No fixture changes; every existing parity test keeps passing byte for byte.
