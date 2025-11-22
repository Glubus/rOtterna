use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub hp_drain_rate: f64,
    pub overall_difficulty: f64,
    pub song_path: String,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            hp_drain_rate: 8.0,
            overall_difficulty: 9.0,
            song_path: String::new(),
        }
    }
}

impl Settings {
    fn get_config_path() -> Result<PathBuf, String> {
        let mut config_dir = std::env::current_dir()
            .map_err(|e| format!("Error getting current directory: {}", e))?;
        config_dir.push("config");
        std::fs::create_dir_all(&config_dir)
            .map_err(|e| format!("Error creating config directory: {}", e))?;
        config_dir.push("settings.json");
        Ok(config_dir)
    }

    pub fn load() -> Result<Settings, String> {
        let config_path = Self::get_config_path()?;
        
        if !config_path.exists() {
            let default = Settings::default();
            default.save()?;
            return Ok(default);
        }

        let content = fs::read_to_string(&config_path)
            .map_err(|e| format!("Error reading settings file: {}", e))?;
        
        let settings: Settings = serde_json::from_str(&content)
            .map_err(|e| format!("Error parsing settings file: {}", e))?;
        
        Ok(settings)
    }

    pub fn save(&self) -> Result<(), String> {
        let config_path = Self::get_config_path()?;
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Error serializing settings: {}", e))?;
        
        fs::write(&config_path, content)
            .map_err(|e| format!("Error writing settings file: {}", e))?;
        
        Ok(())
    }
}

#[tauri::command]
pub fn get_settings() -> Result<Settings, String> {
    Settings::load()
}

#[tauri::command]
pub fn set_settings(settings: Settings) -> Result<(), String> {
    settings.save()
}

