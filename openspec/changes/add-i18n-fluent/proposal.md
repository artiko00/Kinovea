# Proposal

## Why

Kinovea ships in 42 languages maintained by volunteers on Weblate through `.resx` files. The Rust UI (changes 14 to 17) has introduced its text in English only. Without a localization layer that reuses the existing translations, the Linux fork would lose years of translation work and the Weblate workflow described in `localization.md`. This change converts the `.resx` corpus to Fluent, wires it into the `egui` UI and keeps Weblate as the translation tool.

## What Changes

- Add a converter `tools/i18n/resx2ftl` that reads the English and translated `.resx` of the five localized projects (`Kinovea` 203 strings, `Kinovea.ScreenManager` 565, `Kinovea.FileBrowser` 31, `Kinovea.Camera` 33, `Kinovea.Updater` 10) and writes Fluent files per language under `rust/kinovea-i18n/locales/<lang>/`, mapping .NET `{0}` placeholders to Fluent variables.
- Add crate `kinovea-i18n`: bundle loading, locale negotiation, fallback to English, and a `tr!` macro used by every UI crate.
- Add the language picker (Options > Language, as in the original menu) with the language names in their own script taken from the original `LanguageManager`, including the low-coverage marking.
- Add tests: every locale file has the same message identifiers as English (missing ones fall back and are reported), no stale identifier is referenced from code, placeholders match.
- Document the Weblate hand-off: Weblate supports Fluent natively; the component points to `rust/kinovea-i18n/locales`.

## Capabilities

### New Capabilities
- `i18n/localized-ui`: UI text is resolved from locale files with fallback, selected by the user, and persisted.
- `i18n/translation-pipeline`: the conversion and consistency rules that keep locale files complete and in sync with code.

### Modified Capabilities
None.

## Impact

- **Assemblies touched**: none modified. `Languages/*.resx` are read only; translated `.resx` are never edited. `Kinovea.Services/Language/LanguageManager.cs` is the source for language names and low-coverage list.
- **New crate** `kinovea-i18n` with dependencies `fluent-bundle`, `fluent-langneg`, `unic-langid` (decision requiring approval). The converter is a Rust binary in `tools/i18n` sharing the workspace (no Python added to the toolchain).
- **Verification**: Linux only, `make verify`; the converter runs in CI to assert the committed `.ftl` files are up to date with the `.resx` sources until `remove-dotnet-solution` deletes the `.resx`, after which the `.ftl` files are the source of truth.
- **KVA**: unchanged. Fluent is UI-only; KVA keeps its invariant-culture serialization.

## Non-goals

- Translating new Rust UI strings into other languages; only English is authored here and Weblate fills the rest.
- Right-to-left layout for Arabic and Persian beyond what `egui` provides.
- Localized documentation or help pages.
