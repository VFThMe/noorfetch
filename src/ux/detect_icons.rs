use font_kit::source::SystemSource;
use font_kit::family_name::FamilyName;
use font_kit::properties::Properties;

pub fn nerd_font() -> bool {
    // Search for common Nerd Font family names
    let families = [
        FamilyName::Title("JetBrainsMono Nerd Font".to_string()),
        FamilyName::Title("JetBrainsMono NF".to_string()),
    ];
    // Check if any of the Nerd Font families are available in the system fonts
    SystemSource::new()
        .select_best_match(&families, &Properties::new())
        .is_ok()
}