use std::fs;
use std::time::SystemTime;

pub fn get_install_days() -> Option<String> {
    let now = SystemTime::now();

    fs::metadata("/usr/")
        .ok()
        .and_then(|m| m.created().or_else(|_| m.modified()).ok())
        .and_then(|t| now.duration_since(t).ok())
        .map(|d| {
            let days = d.as_secs() / 86400;
            if days == 0 {
                return None;
            }
            Some(format!("{} days", days))
        })
        .flatten()
}
