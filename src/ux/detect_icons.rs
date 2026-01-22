use font_kit::source::SystemSource;
use font_kit::family_name::FamilyName;
use font_kit::properties::Properties;

pub fn nerd_font() -> bool {
    let families = [
        FamilyName::Title("JetBrainsMono Nerd Font".to_string()),
        FamilyName::Title("JetBrainsMono NF".to_string()),
    ];

    SystemSource::new()
        .select_best_match(&families, &Properties::new())
        .is_ok()
}