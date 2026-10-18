//! `General` preferences group (port of `GeneralPreferences.cs`).

use super::fields::xml_struct;

#[derive(Debug, Clone, PartialEq)]
pub struct GeneralPreferences {
    /// UI culture name; empty means the invariant culture (the oracle's default).
    pub culture: String,
    pub enable_debug_log: bool,
    pub enable_all_languages: bool,
    pub preference_page: i32,
    pub pointer_key: String,
}

impl Default for GeneralPreferences {
    fn default() -> Self {
        Self {
            culture: String::new(),
            enable_debug_log: false,
            enable_all_languages: false,
            preference_page: 0,
            pointer_key: "::default".to_owned(),
        }
    }
}

xml_struct!(GeneralPreferences {
    culture => "Culture",
    enable_debug_log => "EnableDebugLog",
    enable_all_languages => "EnableAllLanguages",
    preference_page => "PreferencesPage",
    pointer_key => "Pointer",
});
