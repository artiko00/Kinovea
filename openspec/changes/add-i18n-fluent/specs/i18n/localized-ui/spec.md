# Spec Delta

## Purpose

Shows the Linux application's interface in the user's language using the existing Kinovea translations, with English as fallback, and lets the user switch language without restarting.

## ADDED Requirements

### Requirement: Language selection
The user SHALL choose the interface language from Options > Language, where each language is listed by its name in its own script; the choice SHALL persist in preferences and apply immediately to all open windows.

#### Scenario: Switching language
- **WHEN** the user selects "Français" in Options > Language
- **THEN** menus, dialogs and tooltips show French text without restart and the preference file records `fr`

#### Scenario: First start
- **WHEN** no language preference exists
- **THEN** the application uses the system locale when it is among the supported languages, otherwise English

### Requirement: Fallback to English
When a message is missing or malformed in the selected locale, the application SHALL display the English message instead of an identifier or an empty string.

#### Scenario: Partial translation
- **WHEN** the selected locale is Tamil, which covers a fraction of the strings
- **THEN** untranslated strings render in English and no message identifier is visible to the user

### Requirement: Placeholders
Messages with values (counts, names, units) SHALL format the values in the selected language's order and number formatting.

#### Scenario: Parameterized message
- **WHEN** the message "Frame {0} of {1}" is shown in German with values 3 and 10
- **THEN** the German translation renders with both numbers in the positions defined by the German text

### Requirement: Low-coverage languages are flagged
Languages marked as low coverage in the original application SHALL appear in the language menu with the same marking so users know most text will be English.

#### Scenario: Menu entry
- **WHEN** the language menu is opened
- **THEN** Slovak, Estonian, Tamil, Azerbaijani, Latvian and Danish are shown with the low-coverage indicator
