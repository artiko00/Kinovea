# Tasks

## 1. Converter

- [ ] 1.1 Add workspace member `tools/i18n/resx2ftl` (Cargo crate, `quick-xml`) with `src/main.rs`, `src/convert.rs`, `src/convert_test.rs`; record the approval for the new crates and dependencies in `.agent/progress.md`; verify `cd rust && make verify`.
- [ ] 1.2 Write `src/convert_test.rs` cases first: identifier scheme, `{0}`/`{1}` to named variables, Fluent escaping, unbalanced braces error with file/key; then implement `convert.rs`; verify `cargo test -p resx2ftl`.
- [ ] 1.3 Write `src/languages_test.rs` (parse `LanguageManager.cs` to `languages.toml`: 41 entries plus English, low-coverage flags, `zh-CHS`→`zh-Hans`) and implement `src/languages.rs`; verify test.
- [ ] 1.4 Run the converter over the five `Languages/` folders into `rust/kinovea-i18n/locales/`, commit the output, add `make i18n` and `resx2ftl --check`; verify English message count equals the union of English resources (203+565+31+33+10 minus duplicates reported by the tool) and `--check` exits 0 on a clean tree.

## 2. Runtime crate

- [ ] 2.1 Add crate `rust/kinovea-i18n` with `fluent-bundle`, `fluent-langneg`, `unic-langid`, `include_dir`; write `src/localizer_test.rs` (lookup, fallback to English for Tamil, variable formatting in German, `KINOVEA_LOCALES_DIR` override) then implement `src/localizer.rs` and the `tr!` macro in `src/macros.rs` with `src/macros_test.rs`; verify `cargo test -p kinovea-i18n`.
- [ ] 2.2 Write `src/negotiate_test.rs` (system locale `fr-CA` → `fr`, unsupported → `en`, preference wins over system) and implement `src/negotiate.rs`; verify test.
- [ ] 2.3 Write `src/lint_test.rs` (orphan identifier fails, missing identifier passes with count, variable mismatch fails, English syntax error fails) and implement `src/lint.rs` used by the test; verify it passes on the committed locales.
- [ ] 2.4 Write `tests/references_test.rs` scanning `rust/**/*.rs` for `tr!` literals against English; verify it fails on an injected typo and passes after removal.

## 3. UI integration

- [ ] 3.1 Replace English literals in `rust/kinovea-ui/src/**` and the capture module with `tr!` identifiers from the converted English file, adding to `locales/en/kinovea.ftl` only the strings that have no original counterpart; verify `tests/references_test.rs` and `egui_kittest` snapshots in `kinovea-ui` still pass.
- [ ] 3.2 Write `rust/kinovea-ui/src/menus/language_test.rs` (Options > Language lists native names from `languages.toml`, low-coverage marker on the six languages, selecting French re-renders the File menu in French without restart, preference persisted as `fr`) and implement `src/menus/language.rs`; verify test.
- [ ] 3.3 Write `rust/kinovea-ui/src/fonts_test.rs` rendering one sample string per script (CJK, Arabic, Tamil, Thai, Cyrillic, Greek) and asserting no missing-glyph fallback; implement font loading per design D6/risk note; verify test.

## 4. Documentation and hand-off

- [ ] 4.1 Rewrite `localization.md` for Fluent: directory layout, adding a language, adding strings (`locales/en/kinovea.ftl`), Weblate component settings, the freeze procedure; update `AGENTS.md` rules (English `.ftl` instead of English `.resx`); verify `npm run lint` and `make verify` exit 0.
- [ ] 4.2 Add the `resx2ftl --check` step to `.github/workflows/rust.yml` (workflow change requires approval) and append the `.agent/progress.md` entry; verify the CI job is green on the branch.

## Workflow follow-up

- Human: lock Weblate, merge pending `.resx` PRs, re-run `make i18n`, switch the Weblate component to `rust/kinovea-i18n/locales`, unlock.
- Archive the change; tick 18 in `openspec/roadmap.md`.
