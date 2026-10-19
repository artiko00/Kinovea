# Tasks

## 1. Loader and migration
- [x] 1.1 Fix relative trajectories before the post-import pass (track keyframe labels in absolute time); test with a 1.5 track and a keyframe inside its span.
- [x] 1.2 Read formats 1.2 and older as they are (find `FormatVersion` as a descendant); replace the rejection test by a reading test.
- [x] 1.3 Write empty `xsl:value-of` elements with an explicit end tag everywhere in the migration and compare unit codes without trimming; the migration fixtures still match byte for byte.
- [x] 1.4 Round remapped timestamps half to even, with a tie test.

## 2. Undo history and edits
- [x] 2.1 Refresh track keyframe labels when keyframes are added or deleted (also through undo).
- [x] 2.2 Modify-drawing mementos carry a serialization filter, restore only that part and compare state within the filter; port the parameterless `IsSameState`.
- [x] 2.3 Video-filter mementos capture and restore one filter.
- [x] 2.4 Deleting a multi-drawing item removes its tracker (and the memento restores it); deleting a track forgets its track bindings; adding a drawing whose id is already present is ignored; the modify-keyframe undo does not create a missing keyframe; singleton managers support modify mementos and reject add/delete explicitly.
- [x] 2.5 Tighten the two weak tests found by the review.

## 3. Integration
- [ ] 3.1 `cd rust && make verify`, `rustfmt --check` on macro-declared modules and `npm run lint` green; progress entry; PR and CI.
