use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Destination {
    pub name: String,
    pub url: String,
    pub memo: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configure {
    #[serde(default)]
    pub destination: Vec<Destination>,
}

fn config_path() -> Result<PathBuf, String> {
    let home_dir =
        dirs::home_dir().ok_or_else(|| "could not determine home directory".to_string())?;
    Ok(home_dir.join(".config").join("pesn").join("config.toml"))
}

impl Configure {
    pub fn load() -> Result<Configure, String> {
        let path = config_path()?;
        if !path.exists() {
            return Ok(Configure {
                destination: vec![],
            });
        }
        let content = std::fs::read_to_string(&path)
            .map_err(|e| format!("failed to read config file: {e}"))?;
        let config: Configure =
            toml::from_str(&content).map_err(|e| format!("failed to parse config file: {e}"))?;
        Ok(config)
    }

    pub fn resolve_destination(&self, name: &str) -> Option<&Destination> {
        self.destination.iter().find(|d| d.name == name)
    }

    pub fn save(&self) -> Result<(), String> {
        let path = config_path()?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("failed to create config directory: {e}"))?;
        }
        let content =
            toml::to_string(self).map_err(|e| format!("failed to serialize config: {e}"))?;
        std::fs::write(&path, content).map_err(|e| format!("failed to write config file: {e}"))?;
        Ok(())
    }
}
