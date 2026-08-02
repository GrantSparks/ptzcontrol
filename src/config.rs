use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tracing::warn;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectionConfig {
    Ip { address: String },
    Serial { port: String, baud_rate: u32 },
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        Self::Ip {
            address: "192.168.1.100".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraConfig {
    pub id: u64,
    pub name: String,
    pub connection: ConnectionConfig,
}

impl CameraConfig {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            connection: ConnectionConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub cameras: Vec<CameraConfig>,
    pub selected_camera_id: Option<u64>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            cameras: Vec::new(),
            selected_camera_id: None,
        }
    }
}

impl AppConfig {
    pub fn config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|p| p.join("ptz-controller").join("cameras.toml"))
    }

    pub fn load() -> Self {
        let Some(path) = Self::config_path() else {
            return Self::default();
        };

        if !path.exists() {
            return Self::default();
        }

        match fs::read_to_string(&path) {
            Ok(contents) => match toml::from_str(&contents) {
                Ok(config) => config,
                Err(e) => {
                    warn!("Failed to parse config file: {}", e);
                    Self::default()
                }
            },
            Err(e) => {
                warn!("Failed to read config file: {}", e);
                Self::default()
            }
        }
    }

    pub fn save(&self) {
        let Some(path) = Self::config_path() else {
            warn!("Could not determine config directory");
            return;
        };

        if let Some(parent) = path.parent() {
            if let Err(e) = fs::create_dir_all(parent) {
                warn!("Failed to create config directory: {}", e);
                return;
            }
        }

        match toml::to_string_pretty(self) {
            Ok(contents) => {
                if let Err(e) = fs::write(&path, contents) {
                    warn!("Failed to write config file: {}", e);
                }
            }
            Err(e) => {
                warn!("Failed to serialize config: {}", e);
            }
        }
    }

    #[allow(dead_code)]
    pub fn get_selected_camera(&self) -> Option<&CameraConfig> {
        self.selected_camera_id
            .and_then(|id| self.cameras.iter().find(|c| c.id == id))
    }

    pub fn next_id(&self) -> u64 {
        self.cameras.iter().map(|c| c.id).max().unwrap_or(0) + 1
    }

    pub fn add_camera(&mut self, name: String) -> &mut CameraConfig {
        let id = self.next_id();
        self.cameras.push(CameraConfig::new(id, name));
        if self.selected_camera_id.is_none() {
            self.selected_camera_id = Some(id);
        }
        self.cameras.last_mut().unwrap()
    }

    pub fn remove_camera(&mut self, id: u64) {
        self.cameras.retain(|c| c.id != id);
        if self.selected_camera_id == Some(id) {
            self.selected_camera_id = self.cameras.first().map(|c| c.id);
        }
    }
}

pub fn list_serial_ports() -> Vec<String> {
    let mut ports = Vec::new();

    if let Ok(entries) = fs::read_dir("/dev") {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.starts_with("ttyUSB")
                    || name.starts_with("ttyACM")
                    || name.starts_with("ttyS")
                {
                    ports.push(path.to_string_lossy().to_string());
                }
            }
        }
    }

    ports.sort();
    ports
}
