use std::env;
use std::path::Path;

pub fn get_wm() -> Option<String> {
    let from_env = [
        "XDG_CURRENT_DESKTOP",
        "XDG_SESSION_DESKTOP",
        "DESKTOP_SESSION",
    ]
    .iter()
    .find_map(|var| env::var(var).ok().filter(|v| !v.is_empty()));

    if let Some(wm) = from_env {
        return Some(wm);
    }

    // fallback
    let known_wm = [
        "sway",
        "i3",
        "bspwm",
        "openbox",
        "fluxbox",
        "icewm",
        "xfwm4",
        "kwin_wayland",
        "kwin_x11",
        "mutter",
        "marco",
        "compiz",
        "enlightenment",
        "hyprland",
        "river",
        "dwm",
        "awesome",
        "qtile",
        "herbstluftwm",
        "leftwm",
        "wayfire",
        "labwc",
        "niri",
    ];

    std::fs::read_dir("/proc").ok()?.find_map(|entry| {
        let entry = entry.ok()?;
        let name = entry.file_name();
        let pid_str = name.to_str()?;
        if !pid_str.chars().all(|c| c.is_ascii_digit()) {
            return None;
        }
        let comm = std::fs::read_to_string(entry.path().join("comm")).ok()?;
        let comm = comm.trim();
        known_wm
            .iter()
            .find(|&&wm| comm == wm)
            .map(|&wm| wm.to_string())
    })
}

pub fn get_shell() -> Option<String> {
    let raw = env::var("SHELL").ok().filter(|v| !v.is_empty())?;
    Path::new(&raw)
        .file_name()
        .and_then(|n| n.to_str())
        .map(str::to_string)
}
