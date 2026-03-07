// use font_kit::source::SystemSource;
// use font_kit::family_name::FamilyName;
// use font_kit::properties::Properties;

// pub fn nerd_font() -> bool {
//     // Создаем массив с названиями шрифтов Nerd Font
//     let families = [
//         FamilyName::Title("JetBrainsMono Nerd Font".to_string()),
//         FamilyName::Title("JetBrainsMono NF".to_string()),
//     ];
//     // С помощью Font_Kit определяем ОС и ищем в ней шрифты из массива
//     SystemSource::new()
//         .select_best_match(&families, &Properties::new())
//         .is_ok()
// }