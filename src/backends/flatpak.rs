//! Flatpak backend for Linux

use super::{Backend, command_exists};
use crate::search::PackageResult;
use std::process::Command;

pub struct FlatpakBackend;

impl FlatpakBackend {
    pub fn new() -> Self {
        Self
    }
}

impl Default for FlatpakBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl Backend for FlatpakBackend {
    fn name(&self) -> &str {
        "flatpak"
    }
    
    fn is_available(&self) -> bool {
        command_exists("flatpak")
    }
    
    fn search(&self, query: &str) -> Result<Vec<PackageResult>, Box<dyn std::error::Error + Send + Sync>> {
        let output = Command::new("flatpak")
            .args(["search", query])
            .output()?;
            
        let mut results = Vec::new();
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        for line in stdout.lines() {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 3 {
                results.push(PackageResult {
                    name: parts[0].trim().to_string(),
                    version: Some(parts[3].trim().to_string()),
                    description: Some(parts[1].trim().to_string()),
                    source: "flatpak".to_string(),
                    score: 0.0,
                });
            } else {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                     results.push(PackageResult {
                        name: parts[0].to_string(),
                        version: if parts.len() > 3 { Some(parts[3].to_string()) } else { None },
                        description: Some(parts[1].to_string()),
                        source: "flatpak".to_string(),
                        score: 0.0,
                    });
                }
            }
        }
        
        Ok(results)
    }
    
    fn install(&self, package: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("   Running: flatpak install -y flathub {}", package);
        
        let status = Command::new("flatpak")
            .args(["install", "-y", "flathub", package])
            .status()?;
            
        if status.success() {
            Ok(())
        } else {
            Err(format!("flatpak install failed with exit code: {:?}", status.code()).into())
        }
    }

    fn update(&self, package: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("   Running: flatpak update -y {}", package);
        
        let status = Command::new("flatpak")
            .args(["update", "-y", package])
            .status()?;
            
        if status.success() {
            Ok(())
        } else {
            Err(format!("flatpak update failed with exit code: {:?}", status.code()).into())
        }
    }

    fn uninstall(&self, package: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("   Running: flatpak uninstall -y {}", package);
        
        let status = Command::new("flatpak")
            .args(["uninstall", "-y", package])
            .status()?;
            
        if status.success() {
            Ok(())
        } else {
            Err(format!("flatpak uninstall failed with exit code: {:?}", status.code()).into())
        }
    }
}
