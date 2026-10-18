//! User preferences model: typed groups with the original defaults and the
//! `Preferences.xml` (format 3.0) reader and writer. No implicit persistence.

macro_rules! modules {
    ($($m:ident),* $(,)?) => { $(pub mod $m;)* };
}

macro_rules! test_modules {
    ($($m:ident),* $(,)?) => { $(#[cfg(test)] mod $m;)* };
}

modules!(
    capture,
    capture_automation,
    capture_folders,
    dom,
    enums,
    explorer,
    explorer_items,
    fading,
    fields,
    general,
    helpers,
    kinogram,
    names,
    player,
    player_params,
    player_xml,
    recent_colors,
    vision_params,
    xml,
);

test_modules!(
    mod_test,
    capture_test,
    capture_automation_test,
    capture_folders_test,
    dom_test,
    enums_test,
    explorer_test,
    explorer_items_test,
    fading_test,
    fields_test,
    general_test,
    helpers_test,
    kinogram_test,
    names_test,
    player_test,
    player_params_test,
    player_xml_test,
    recent_colors_test,
    vision_params_test,
    xml_test,
);

pub use xml::{PreferencesError, FORMAT_VERSION};

/// All preference groups. Setting a value never touches the disk; call `save`.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Preferences {
    pub general: general::GeneralPreferences,
    pub file_explorer: explorer::FileExplorerPreferences,
    pub player: player::PlayerPreferences,
    pub capture: capture::CapturePreferences,
    /// Keyboard shortcuts, kept verbatim until the UI change models them.
    pub keyboard: Option<dom::Element>,
}
