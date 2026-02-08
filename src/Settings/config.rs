use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use directories::ProjectDirs;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModuleConfig {
    pub display: bool,
}

impl Default for ModuleConfig {
    fn default() -> Self {
        Self { display: true }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    // Используем #[serde(rename)], чтобы в JSON было "Modules", а в коде - "modules"
    #[serde(rename = "Modules")]
    pub modules: HashMap<String, ModuleConfig>,
}

impl Default for Config {
    fn default() -> Self {
        let mut modules = HashMap::new();
        // Список модулей по умолчанию
        let keys = vec!["os", "user", "hostname", "ram", "swap", "cpu", "krnl"];
        for key in keys {
            modules.insert(key.to_string(), ModuleConfig::default());
        }

        Config { modules }
    }
}

pub fn get_path() -> PathBuf {
    if let Some(proj_dirs) = ProjectDirs::from("", "", "noorfetch") {
        let config_dir = proj_dirs.config_dir();
        if !config_dir.exists() {
            let _ = fs::create_dir_all(config_dir);
        }
        config_dir.join("config.json")
    } else {
        PathBuf::from("config.json")
    }
}

pub fn load_config() -> Config {
    let path = get_path();

    if !path.exists() {
        let default_config = Config::default();
        let json = serde_json::to_string_pretty(&default_config).unwrap();
        fs::write(&path, json).expect("Failed to write default config");
        return default_config;
    }

    let content = fs::read_to_string(&path).expect("Failed to read config file");

    // Пытаемся распарсить JSON. Если структура не совпадает — возвращаем дефолт
    serde_json::from_str::<Config>(&content).unwrap_or_else(|e| {
        eprintln!("Error parsing config: {}, using defaults", e);
        Config::default()
    })
}
