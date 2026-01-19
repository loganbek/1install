//! Cargo backend for Rust tools

use super::{Backend, command_exists};
use crate::search::PackageResult;
use std::process::Command;

pub struct CargoBackend;

impl CargoBackend {
    pub fn new() -> Self {
        Self
    }
}

impl Default for CargoBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl Backend for CargoBackend {
    fn name(&self) -> &str {
        "cargo"
    }
    
    fn is_available(&self) -> bool {
        command_exists("cargo")
    }
    
    fn search(&self, query: &str) -> Result<Vec<PackageResult>, Box<dyn std::error::Error + Send + Sync>> {
        let output = Command::new("cargo")
            .args(["search", query, "--limit", "10"])
            .output()?;
            
        let mut results = Vec::new();
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        for line in stdout.lines() {
            if line.contains(" = \"") {
                let parts: Vec<&str> = line.split(" = \"").collect();
                let name = parts[0].trim().to_string();
                
                let second_half = parts[1];
                let version_parts: Vec<&str> = second_half.split("\" # ").collect();
                let version = version_parts[0].to_string();
                let description = if version_parts.len() > 1 {
                    version_parts[1].trim().to_string()
                } else {
                    "".to_string()
                };
                
                results.push(PackageResult {
                    name,
                    version: Some(version),
                    description: Some(description),
                    source: "cargo".to_string(),
                    score: 0.0,
                });
            }
        }
        
        Ok(results)
    }
    
    fn install(&self, package: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("   Running: cargo install {}", package);
        
        let status = Command::new("cargo")
            .args(["install", package])
            .status()?;
            
        if status.success() {
            Ok(())
        } else {
            Err(format!("cargo install failed with exit code: {:?}", status.code()).into())
        }
    }

    fn update(&self, package: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.install(package)
    }

    fn uninstall(&self, package: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("   Running: cargo uninstall {}", package);
        
        let status = Command::new("cargo")
            .args(["uninstall", package])
            .status()?;
            
        if status.success() {
            Ok(())
        } else {
            Err(format!("cargo uninstall failed with exit code: {:?}", status.code()).into())
        }
    }
}
