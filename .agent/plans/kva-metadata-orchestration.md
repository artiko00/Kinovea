# Orquestación del change 05 `add-kva-metadata-model`

Pedido del usuario: hasta 3 agentes Haiku (esfuerzo xhigh) desarrollan; el orquestador coordina y valida;
un agente adversarial con la skill `hermes` valida el change al final.

Tipo de agente: `haiku-rust-dev` (`~/.claude/agents/haiku-rust-dev.md`, `model: haiku`). El registro de
agentes no se recarga a mitad de sesión: hay que reiniciar Claude Code antes de lanzarlos.

## Estado de partida

- [x] Fixtures regeneradas con el oráculo corregido (focos con `ParentMetadata`, sin `ContentHash` en `drawings.json`)
- [x] `tools/oracle/fixtures.sh --verify` determinista tras quitar el hash
- [ ] Lanzar los tres agentes en worktrees aislados (`isolation: worktree`, `effort: xhigh`)
- [ ] Aplicar los diffs en el checkout principal: primero A, luego B y C
- [ ] Validar: `git diff --stat -- tests/fixtures tests/cases` vacío, revisar cada hunk de `*_test.rs`, `make verify`, `npm run lint`
- [ ] Agente adversarial Hermes sobre el SHA final (worktree desechable en `/tmp/kinovea-hermes`)
- [ ] Verificar sus hallazgos, corregir, `tasks.md` 29/29, `progress.md`, `AGENTS.md`, PR, CI, merge, archive, roadmap

## Reglas comunes a los tres briefs

- Nunca editar `tests/fixtures/`, `tests/cases/`, `Kinovea.FixtureGen/`, `openspec/changes/archive/`.
- Nunca ejecutar `tools/oracle/fixtures.sh`; nunca commit ni push; sin crates nuevos.
- Nunca debilitar una aserción; si el oráculo parece equivocado, reportarlo.
- 150 líneas por archivo, `_test.rs` por cada `.rs`, sin duplicados (dupelens).
- `cd rust && make verify` en verde; linters con `/home/artiko/Desarrollos/Kinovea/node_modules/.bin/<lens> check` desde la raíz del worktree.
- Informe final: archivos cambiados, causa raíz de cada arreglo, salida resumida de pruebas.

## A — Paridad (dueño de `src/kva/`, `src/multi.rs`, `src/drawing/`, `src/style/`, `src/migration/`)

Hacer pasar, byte a byte, las pruebas que fallan hoy. Diferencias ya diagnosticadas:

1. `multi_test::{number_sequence_writes_items_then_style, spotlights_round_trip_and_replace_known_ids}`:
   el escritor ahora usa nombres constantes (`DrawingSpotlightItem.Name` = `"Spotlight"`,
   `DrawingNumberSequenceItem.Name` = `"AutoNumber"`, confirmados por las fixtures). Ajustar la expectativa
   del atributo `name` y eliminar el campo `name` si queda sin uso.
2. `kva_roundtrip_test::{every_drawing_kind_round_trips, export_flags_select_the_sections}`: el estilo del
   `Plane` de entrada trae `<GridDivisions Key="cols">6`; el oráculo escribe `<Int Key="cols"><Value>10`:
   un elemento cuyo tipo no coincide con el del estilo por defecto no se importa
   (`Kinovea.ScreenManager/Metadata/Styling/StyleElements.cs`, `ImportXML`). Seguir con las diferencias
   siguientes (765 vs 715 líneas) hasta que sean idénticos.
3. `kva_remap_test`, `kva_fuzz_test`: los ítems de foco son rastreables. El original registra cada foco
   (`DrawingSpotlight.Add` → `TrackableDrawingAdded` → `TrackabilityManager.Add(item, ReferenceTimestamp)`)
   y escribe `<TrackableDrawing name="Spotlight"><TrackablePoint key="o">`. Rust solo registra dibujos de
   keyframe (`kva/reader.rs`: `register_keyframe`, `after_import`). Revisar `DrawingSpotlightItem.cs`
   (`GetTrackablePoints`, `ReferenceTimestamp`, `WriteXml`) y `Tracking/Trackability/*.cs`. Poner el
   registro en `kva/reader.rs`, no en `metadata.rs`.
4. `kva_migration_test::format_1_3_is_migrated_and_round_trips`: el XSLT escribe `<Origin></Origin>`
   (valor vacío de `xsl:value-of`) y Rust escribe `<Origin />`. Resolverlo sin cambiar la salida de ninguna
   otra fixture (`preferences`, `calibration/kva/*.xml`).

## B — Historial de deshacer (archivos nuevos `src/history*.rs`, `src/metadata/edit*.rs`)

Tarea 7.1. `HistoryStack` y `Memento` con AddDrawing, DeleteDrawing, ModifyDrawing, AddKeyframe,
DeleteKeyframe, ModifyKeyframe, AddMultiItem, DeleteMultiItem (y ModifyVideoFilter con el `Element` literal).
Referencias C#: `Kinovea.ScreenManager/UndoRedo/**`, `Serialization/{DrawingSerializer,KeyframeSerializer,MultiDrawingItemSerializer}.cs`,
métodos `AddDrawing`, `DeleteDrawing`, `DeleteKeyframe`, `MergeInsertKeyframe`, `AddMultidrawingItem`,
`DeleteMultiDrawingItem` de `Metadata.cs`. Las operaciones de mutación van en `src/metadata/edit.rs`.
Pruebas: para cada memento, deshacer deja `write_kva` idéntico al estado previo y rehacer al posterior;
push vacía redo; dos `ModifyDrawing` consecutivos idénticos no se apilan.

## C — Propiedades y documentación (archivos nuevos en `tests/`, más `tests/README.md`)

`rust/kinovea-metadata/tests/kva_property_test.rs` con un LCG con semilla escrito en el archivo (sin
`proptest` ni `rand`): para cada tipo de `Drawing`, focos, números, keyframe, sistema de coordenadas y
rejilla, al menos 50 instancias aleatorias con `write(read(write(d))) == write(d)`. No tocar `src/`: si un
tipo no es idempotente, documentar la reproducción mínima en el informe. Actualizar la tabla de áreas de
`tests/README.md` (salidas `kva` nuevas y área `posture`).
