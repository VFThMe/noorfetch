// use serde::{Deserialize, Serialize};
// use std::fs;
// use std::path::PathBuf;
// use directories::ProjectDirs;
// use std::collections::HashMap;

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct ModuleConfig {
//     pub display: bool,
// }

//
// impl Default for ModuleConfig {
//     fn default() -> Self {
//         Self {
//             display: true,
//         }
//     }
// }

// #[derive(Serialize, Deserialize, Debug)]
// #[serde(rename_all = "PascalCase")]
// pub struct Config {
//     // Используем HashMap для хранения конфигураций модулей по их именам
//     pub modules: HashMap<String, ModuleConfig>,
// }

// // Создаем Default для Config
// impl Default for Config {
//     fn default() -> Self {
//         let mut modules = HashMap::new();
//         modules.insert("os".to_string(), ModuleConfig::default());
//         modules.insert("user".to_string(), ModuleConfig {
//             ..ModuleConfig::default()
//         });
//         modules.insert("hostname".to_string(), ModuleConfig::default());
//         modules.insert("ram".to_string(), ModuleConfig::default());
//         modules.insert("swap".to_string(), ModuleConfig::default());
//         modules.insert("cpu".to_string(), ModuleConfig::default());
//         modules.insert("krnl".to_string(), ModuleConfig::default());

//         Config { modules }
//     }
// }

// pub fn get_path() -> PathBuf {
//     // Получаем директорию конфигурации с помощью directories crate
//     if let Some(proj_dirs) = ProjectDirs::from("", "", "rsfetch") {
//         let config_dir = proj_dirs.config_dir();
//         if !config_dir.exists() {
//             let _ = fs::create_dir_all(config_dir);
//         }
//         config_dir.join("config.json") // Возвращаем путь к config.json
//     } else {
//         PathBuf::from("config.json")
//     }
// }

// pub fn load_config() -> Config {
//     let path = get_path();

//     if !path.exists() {
//         let default_config = Config::default();
//         let json = serde_json::to_string_pretty(&default_config).unwrap();
//         fs::write(&path, json).expect("Failed to create default configuration in config.json");
//         return default_config;
//     }

//     let content = fs::read_to_string(&path).expect("Failed to read configuration file");

//     match serde_json::from_str::<Config>(&content) {
//         Ok(config) => config,
//         Err(e) => {
//             eprintln!("Configuration error: {}, reverting to default", e);
//             Config::default()
//         }
//     }
// }