use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

fn default_display() -> bool { true }
fn default_order() -> u32 { 99 }
fn default_format() -> String { "{value}".to_string() }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModuleConfig {
    #[serde(default = "default_display")]
    pub display: bool,

    #[serde(default)]
    pub label: Option<String>,

    #[serde(default)]
    pub color: Option<String>,

    #[serde(default = "default_order")]
    pub order: u32,

    #[serde(default = "default_format")]
    pub format: String,
}

impl Default for ModuleConfig {
    fn default() -> Self {
        Self {
            display: true,
            label: None,
            color: None,
            order: default_order(),
            format: default_format(),
        }
    }
}

impl ModuleConfig {
    pub fn from_key(key: &str) -> Self {
        let (display, color, order) = match key {
            "os"       => (true,  Some("#DC8A78"), 1),
            "user"     => (true,  Some("#DD7878"), 2),
            "hostname" => (true,  Some("#EA76CB"), 3),
            "wm"       => (true,  Some("#8839EF"), 4),
            "ram"      => (true,  Some("#E64553"), 5),
            "swap"     => (true,  Some("#FE640B"), 6),
            "cpu"      => (true,  Some("#DF8E1D"), 7),
            "krnl"     => (true,  Some("#40A02B"), 8),
            "days"     => (true,  Some("#179299"), 9),
            "init"     => (false, Some("#04A5E5"), 10),
            _          => (true,  None,            99),
        };

        let format = match key {
            "ram" | "swap" => "{used}/{total} MiB".to_string(),
            "cpu"          => "{brand} ({cores})".to_string(),
            _              => default_format(),
        };

        Self { display, label: None, color: color.map(str::to_string), order, format }
    }
    
    pub fn resolve_label<'a>(&'a self, key: &'a str) -> &'a str {
        self.label.as_deref().unwrap_or(key)
    }

    pub fn resolve_color(&self) -> (u8, u8, u8) {
        self.color
            .as_deref()
            .and_then(hex_to_rgb)
            .unwrap_or((255, 255, 255))
    }

    pub fn format_value(&self, vars: &[(&str, &str)]) -> String {
        let mut result = self.format.clone();
        for (name, val) in vars {
            result = result.replace(&format!("{{{}}}", name), val);
        }
        result
    }
}

fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
    let hex = hex.trim().trim_start_matches('#');
    if hex.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some((r, g, b))
}

fn default_logo() -> String { "default".to_string() }
fn default_custom_art() -> Option<String> { None }

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(rename = "Modules")]
    pub modules: IndexMap<String, ModuleConfig>,

    #[serde(default = "default_logo")]
    pub logo: String,

    #[serde(default = "default_custom_art")]
    pub custom_art: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        let keys = ["os", "user", "hostname", "wm", "ram", "swap", "cpu", "krnl", "days", "init"];
        let modules = keys
            .iter()
            .map(|&k| (k.to_string(), ModuleConfig::from_key(k)))
            .collect();

        Config { modules, logo: default_logo(), custom_art: None }
    }
}

impl Config {
    /// Переназначает `order` последовательно (1, 2, 3, …) по текущему
    /// относительному порядку, а затем пересортировывает IndexMap так,
    /// чтобы в сериализованном JSON ключи шли по возрастанию order.
    pub fn normalize_order(&mut self) {
        // Сортируем ключи по текущему order
        self.modules.sort_by(|_, a, _, b| a.order.cmp(&b.order));

        // Переназначаем order последовательно
        for (new_order, (_, module)) in self.modules.iter_mut().enumerate() {
            module.order = (new_order + 1) as u32;
        }
    }
}

fn get_config_path() -> PathBuf {
    let home = std::env::var_os("HOME").map(PathBuf::from);

    let mut base = match home {
        Some(p) => p,
        None    => return PathBuf::from("config.json"),
    };

    #[cfg(target_os = "macos")]
    base.push("Library/Application Support/noorfetch/config.json");

    #[cfg(any(
        target_os = "linux",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    base.push(".config/noorfetch/config.json");

    #[cfg(not(any(
        target_os = "macos",
        target_os = "linux",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd"
    )))]
    base.push("noorfetch/config.json");

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
            Ok(mut cfg) => {
                cfg.normalize_order();
                // Перезаписываем файл с нормализованным порядком
                if let Ok(json) = serde_json::to_string_pretty(&cfg) {
                    let _ = fs::write(&path, json);
                }
                cfg
            }
            Err(e)  => {
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
