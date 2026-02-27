use std::fs;
use std::time::SystemTime;

pub fn get_install_days() -> String {
    let now = SystemTime::now();
    
    fs::metadata("/usr/")
        .ok()
        .and_then(|m| m.created().or_else(|_| m.modified()).ok())
        .and_then(|t| now.duration_since(t).ok())
        .map(|d| format!("{} days", d.as_secs() / 86400))
        .unwrap_or_else(|| "unknown".to_string())
}