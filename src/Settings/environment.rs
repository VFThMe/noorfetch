use std::env;

pub fn get_wm() -> Option<String> {
    ["XDG_SESSION_DESKTOP", "DESKTOP_SESSION", "XDG_CURRENT_DESKTOP"]
        .iter()
        .find_map(|var| env::var(var).ok().filter(|v| !v.is_empty()))
}