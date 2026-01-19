//! DNF backend for Fedora/RHEL

use super::{Backend, command_exists};
use crate::search::PackageResult;
use std::process::Command;

pub struct DnfBackend;

impl DnfBackend {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DnfBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl Backend for DnfBackend {
    fn name(&self) -> &str {
        "dnf"
    }
    
    fn is_available(&self) -> bool {
        command_exists("dnf")
    }
    
    fn search(&self, query: &str) -> Result<Vec<PackageResult>, Box<dyn std::error::Error + Send + Sync>> {
        let output = Command::new("dnf")
            .args(["search", query])
            .output()?;
            
        let mut results = Vec::new();
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        for line in stdout.lines() {
            if line.contains(" : ") {
                let parts: Vec<&str> = line.splitn(2, " : ").collect();
                let name_part = parts[0].trim();
                let name = name_part.split('.').next().unwrap_or(name_part).to_string();
                let description = parts[1].trim().to_string();
                
                results.push(PackageResult {
                    name,
                    version: Some("latest".to_string()),
                    description: Some(description),
                    source: "dnf".to_string(),
                    score: 0.0,
                });
            }
        }
        
        Ok(results)
    }
    
    fn install(&self, package: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("   Running: sudo dnf install -y {}", package);
        
        let status = Command::new("sudo")
            .args(["dnf", "install", "-y", package])
            .status()?;
            
        if status.success() {
            Ok(())
        } else {
            Err(format!("dnf install failed with exit code: {:?}", status.code()).into())
        }
    }

    fn update(&self, package: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("   Running: sudo dnf upgrade -y {}", package);
        
        let status = Command::new("sudo")
            .args(["dnf", "upgrade", "-y", package])
            .status()?;
            
        if status.success() {
            Ok(())
        } else {
            Err(format!("dnf upgrade failed with exit code: {:?}", status.code()).into())
        }
    }

    fn uninstall(&self, package: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("   Running: sudo dnf remove -y {}", package);
        
        let status = Command::new("sudo")
            .args(["dnf", "remove", "-y", package])
            .status()?;
            
        if status.success() {
            Ok(())
        } else {
            Err(format!("dnf remove failed with exit code: {:?}", status.code()).into())
        }
    }
}
