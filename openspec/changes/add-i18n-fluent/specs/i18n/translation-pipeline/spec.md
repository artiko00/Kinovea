# Spec Delta

## Purpose

Guarantees that the locale files used by the application are complete, consistent with the code and derived from the existing translations, so volunteers keep working in Weblate without re-translating.

## ADDED Requirements

### Requirement: Conversion from the original resources
The conversion tool SHALL produce one locale file per language from the original English and translated resource files, preserving every translated string, mapping positional placeholders to named variables and reporting any resource it cannot convert.

#### Scenario: Full conversion
- **WHEN** the tool runs over the five localized projects
- **THEN** 42 locale directories are produced, the English file has as many messages as the union of the English resources, and the tool exits 0 listing counts per language

#### Scenario: Placeholder mapping
- **WHEN** a resource contains `{0}` and `{1}`
- **THEN** the Fluent message declares two variables and the translated text keeps them in the translated order

#### Scenario: Unconvertible resource reported
- **WHEN** a resource has unbalanced braces
- **THEN** the tool reports file, key and reason and exits non-zero

### Requirement: Locale files are complete relative to English
A test SHALL fail when a locale file contains an identifier absent from English, when English contains a syntax error, or when a message's variables differ between English and a translation.

#### Scenario: Orphan identifier
- **WHEN** a translation defines a message that English no longer has
- **THEN** the test fails naming the language and identifier

#### Scenario: Missing identifier is allowed
- **WHEN** a translation lacks a message that English has
- **THEN** the test passes and the missing count is printed per language

### Requirement: Code references only existing identifiers
A test SHALL fail when the UI code references a message identifier that English does not define.

#### Scenario: Typo in code
- **WHEN** code requests `mnu-file-opn` and English defines `mnu-file-open`
- **THEN** the test fails naming the source file and identifier

### Requirement: Translation tool hand-off
The repository SHALL document how the translation platform consumes the locale directory, and the locale directory layout SHALL be the one that platform expects for Fluent.

#### Scenario: Layout
- **WHEN** a translator adds a language on the platform
- **THEN** a new `<lang>/kinovea.ftl` appears under the locale directory and the completeness test passes with that language partially translated
