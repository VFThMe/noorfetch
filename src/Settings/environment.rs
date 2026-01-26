use std::env;

pub fn get_wm() -> Option<String> {
    if let Ok(desktop) = env::var("XDG_CURRENT_DESKTOP") {
        if !desktop.is_empty() { return Some(desktop); }
    }
    if let Ok(session) = env::var("DESKTOP_SESSION") {
        if !session.is_empty() { return Some(session); }
    }
    None
}
