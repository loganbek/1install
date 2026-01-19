//! Shim registry for tracking installed shims

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use chrono::{DateTime, Utc};

/// A single shim entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShimEntry {
    /// Name of the shim (binary name)
    pub name: String,
    /// Path to the actual binary
    pub target: PathBuf,
    /// Backend that installed this package
    pub installed_by: String,
    /// When the shim was created
    pub created_at: DateTime<Utc>,
}

/// Registry of all shims
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ShimRegistry {
    shims: HashMap<String, ShimEntry>,
}

impl ShimRegistry {
    /// Load the registry from disk
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let path = Self::registry_path();
        
        if path.exists() {
            let content = fs::read_to_string(&path)?;
            let registry: ShimRegistry = toml::from_str(&content)?;
            Ok(registry)
        } else {
            Ok(Self::default())
        }
    }
    
    /// Save the registry to disk
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::registry_path();
        
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let content = toml::to_string_pretty(self)?;
        fs::write(&path, content)?;
        Ok(())
    }
    
    /// Get the path to the registry file
    fn registry_path() -> PathBuf {
        let home = dirs::home_dir().expect("Could not find home directory");
        home.join(".local")
            .join("share")
            .join("1install")
            .join("shims.toml")
    }
    
    /// Add a shim to the registry
    pub fn add(&mut self, name: String, target: PathBuf, installed_by: String) {
        let entry = ShimEntry {
            name: name.clone(),
            target,
            installed_by,
            created_at: Utc::now(),
        };
        self.shims.insert(name, entry);
    }
    
    /// Remove a shim from the registry
    pub fn remove(&mut self, name: &str) -> Option<ShimEntry> {
        self.shims.remove(name)
    }
    
    /// Get a shim by name
    pub fn get(&self, name: &str) -> Option<&ShimEntry> {
        self.shims.get(name)
    }
    
    /// List all shims
    pub fn list(&self) -> impl Iterator<Item = &ShimEntry> {
        self.shims.values()
    }
    
    /// Get the number of shims
    pub fn len(&self) -> usize {
        self.shims.len()
    }
    
    /// Check if registry is empty
    pub fn is_empty(&self) -> bool {
        self.shims.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_registry_add_get() {
        let mut registry = ShimRegistry::default();
        registry.add(
            "test".to_string(),
            PathBuf::from("/usr/bin/test"),
            "apt".to_string(),
        );
        
        let entry = registry.get("test").unwrap();
        assert_eq!(entry.name, "test");
        assert_eq!(entry.installed_by, "apt");
    }
}
