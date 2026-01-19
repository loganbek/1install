//! Configuration settings

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Backend preferences
    pub backends: BackendConfig,
    /// Behavior settings
    pub behavior: BehaviorConfig,
    /// Shim settings
    pub shims: ShimConfig,
    /// Telemetry settings
    pub telemetry: TelemetryConfig,
}

/// Backend-related configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct BackendConfig {
    /// Priority order for backends (first = highest priority)
    pub priority: Vec<String>,
    /// Disabled backends
    pub disabled: Vec<String>,
}

/// Behavior configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct BehaviorConfig {
    /// Show verbose output
    pub verbose: bool,
    /// Automatically confirm prompts
    pub auto_confirm: bool,
    /// Create shims for installed binaries
    pub create_shims: bool,
}

/// Shim configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ShimConfig {
    /// Automatically refresh shims after install
    pub auto_refresh: bool,
}

impl Default for BehaviorConfig {
    fn default() -> Self {
        Self {
            verbose: false,
            auto_confirm: true,
            create_shims: true,
        }
    }
}

impl Default for ShimConfig {
    fn default() -> Self {
        Self {
            auto_refresh: true,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            backends: BackendConfig::default(),
            behavior: BehaviorConfig::default(),
            shims: ShimConfig::default(),
            telemetry: TelemetryConfig::default(),
        }
    }
}

/// Telemetry configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TelemetryConfig {
    /// Enable anonymized telemetry
    #[serde(default = "default_telemetry_enabled")]
    pub enabled: bool,
    /// Permanent anonymous client ID
    pub client_id: Option<String>,
}

fn default_telemetry_enabled() -> bool { true }


impl Default for BackendConfig {
    fn default() -> Self {
        Self {
            priority: vec![
                "apt".to_string(),
                "winget".to_string(),
                "brew".to_string(),
                "snap".to_string(),
                "npm".to_string(),
                "pip".to_string(),
            ],
            disabled: vec![],
        }
    }
}



/// Get the config file path
pub fn get_config_path() -> PathBuf {
    let home = dirs::home_dir().expect("Could not find home directory");
    
    #[cfg(windows)]
    {
        home.join(".config").join("1install").join("config.toml")
    }
    
    #[cfg(not(windows))]
    {
        home.join(".config").join("1install").join("config.toml")
    }
}

/// Load configuration from disk
pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let path = get_config_path();
    
    if path.exists() {
        let content = fs::read_to_string(&path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    } else {
        Ok(Config::default())
    }
}

/// Save configuration to disk
pub fn save_config(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let path = get_config_path();
    
    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    let content = toml::to_string_pretty(config)?;
    fs::write(&path, content)?;
    Ok(())
}

impl Config {
    /// Get a config value by dot-notation path
    pub fn get(&self, key: &str) -> Option<String> {
        match key {
            "backends.priority" => Some(self.backends.priority.join(",")),
            "backends.disabled" => Some(self.backends.disabled.join(",")),
            "behavior.verbose" => Some(self.behavior.verbose.to_string()),
            "behavior.auto_confirm" => Some(self.behavior.auto_confirm.to_string()),
            "behavior.create_shims" => Some(self.behavior.create_shims.to_string()),
            "shims.auto_refresh" => Some(self.shims.auto_refresh.to_string()),
            "telemetry.enabled" => Some(self.telemetry.enabled.to_string()),
            "telemetry.client_id" => self.telemetry.client_id.clone(),
            _ => None,
        }
    }
    
    /// Set a config value by dot-notation path
    pub fn set(&mut self, key: &str, value: &str) -> Result<(), String> {
        match key {
            "backends.priority" => {
                self.backends.priority = value.split(',').map(|s| s.trim().to_string()).collect();
                Ok(())
            }
            "backends.disabled" => {
                self.backends.disabled = value.split(',').map(|s| s.trim().to_string()).collect();
                Ok(())
            }
            "behavior.verbose" => {
                self.behavior.verbose = value.parse().map_err(|_| "Invalid boolean")?;
                Ok(())
            }
            "behavior.auto_confirm" => {
                self.behavior.auto_confirm = value.parse().map_err(|_| "Invalid boolean")?;
                Ok(())
            }
            "behavior.create_shims" => {
                self.behavior.create_shims = value.parse().map_err(|_| "Invalid boolean")?;
                Ok(())
            }
            "shims.auto_refresh" => {
                self.shims.auto_refresh = value.parse().map_err(|_| "Invalid boolean")?;
                Ok(())
            }
            "telemetry.enabled" => {
                self.telemetry.enabled = value.parse().map_err(|_| "Invalid boolean")?;
                Ok(())
            }
            "telemetry.client_id" => {
                self.telemetry.client_id = Some(value.to_string());
                Ok(())
            }
            _ => Err(format!("Unknown config key: {}", key)),
        }
    }
    
    /// List all config keys
    pub fn list_keys() -> Vec<&'static str> {
        vec![
            "backends.priority",
            "backends.disabled",
            "behavior.verbose",
            "behavior.auto_confirm",
            "behavior.create_shims",
            "shims.auto_refresh",
            "telemetry.enabled",
            "telemetry.client_id",
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(config.backends.priority.contains(&"apt".to_string()));
        assert!(config.behavior.auto_confirm);
    }
    
    #[test]
    fn test_get_set() {
        let mut config = Config::default();
        config.set("behavior.verbose", "true").unwrap();
        assert_eq!(config.get("behavior.verbose"), Some("true".to_string()));
    }
}
