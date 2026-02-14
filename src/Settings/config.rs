use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModuleConfig {
    pub display: bool,
}

impl Default for ModuleConfig {
    fn default() -> Self {
        Self { display: true }
    }
}

impl ModuleConfig {
    pub fn from_key(key: &str) -> Self {
        match key {
            "init" => Self { display: false },
            _ => Self::default(),
        }
    }
}

fn default_logo() -> String {
    "default".to_string()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(rename = "Modules")]
    pub modules: HashMap<String, ModuleConfig>,

    #[serde(default = "default_logo")]
    pub logo: String,
}

impl Default for Config {
    fn default() -> Self {
        let mut modules = HashMap::new();
        let keys = vec!["os", "user", "hostname", "ram", "swap", "cpu", "krnl", "days", "init"];

        for key in keys {
            modules.insert(key.to_string(), ModuleConfig::from_key(key));
        }

        Config {
            modules,
            logo: default_logo(),
        }
    }
}

fn get_config_path() -> PathBuf {
    let home = std::env::var_os("HOME").map(PathBuf::from);

    let mut base = match home {
        Some(p) => p,
        None => return PathBuf::from("config.json"), // крайний fallback
    };

    #[cfg(target_os = "macos")]
    {
        base.push("Library/Application Support/noorfetch/config.json");
    }

    #[cfg(any(
        target_os = "linux",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    {
        base.push(".config/noorfetch/config.json");
    }
    
    #[cfg(not(any(
        target_os = "macos",
        target_os = "linux",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd"
    )))]
    {
        base.push("noorfetch/config.json"); // fallback
    }

    base
}

pub fn load_config() -> Config {
    let path = get_config_path();

    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    if !path.exists() {
        let default_config = Config::default();
        if let Ok(json) = serde_json::to_string_pretty(&default_config) {
            let _ = fs::write(&path, json);
        }
        return default_config;
    }

    match fs::read_to_string(&path) {
        Ok(content) => match serde_json::from_str::<Config>(&content) {
            Ok(cfg) => cfg,
            Err(e) => {
                eprintln!("Error parsing config: {}, using defaults", e);
                Config::default()
            }
        },
        Err(e) => {
            eprintln!("Failed to read config file: {}, using defaults", e);
            Config::default()
        }
    }
}
