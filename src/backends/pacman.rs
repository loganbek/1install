//! Pacman backend for Arch Linux

use super::{Backend, command_exists};
use crate::search::PackageResult;
use std::process::Command;

pub struct PacmanBackend;

impl PacmanBackend {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PacmanBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl Backend for PacmanBackend {
    fn name(&self) -> &str {
        "pacman"
    }
    
    fn is_available(&self) -> bool {
        command_exists("pacman")
    }
    
    fn search(&self, query: &str) -> Result<Vec<PackageResult>, Box<dyn std::error::Error + Send + Sync>> {
        let output = Command::new("pacman")
            .args(["-Ss", query])
            .output()?;
            
        let mut results = Vec::new();
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        let mut current_pkg = None;
        
        for line in stdout.lines() {
            if line.starts_with(' ') {
                if let Some((name, version)) = current_pkg.take() {
                    results.push(PackageResult {
                        name,
                        version: Some(version),
                        description: Some(line.trim().to_string()),
                        source: "pacman".to_string(),
                        score: 0.0,
                    });
                }
            } else {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let name_part = parts[0];
                    let name = name_part.split('/').last().unwrap_or(name_part).to_string();
                    let version = parts[1].to_string();
                    current_pkg = Some((name, version));
                }
            }
        }
        
        Ok(results)
    }
    
    fn install(&self, package: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("   Running: sudo pacman -S --noconfirm {}", package);
        
        let status = Command::new("sudo")
            .args(["pacman", "-S", "--noconfirm", package])
            .status()?;
            
        if status.success() {
            Ok(())
        } else {
            Err(format!("pacman install failed with exit code: {:?}", status.code()).into())
        }
    }

    fn update(&self, package: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.install(package)
    }

    fn uninstall(&self, package: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("   Running: sudo pacman -Rns --noconfirm {}", package);
        
        let status = Command::new("sudo")
            .args(["pacman", "-Rns", "--noconfirm", package])
            .status()?;
            
        if status.success() {
            Ok(())
        } else {
            Err(format!("pacman uninstall failed with exit code: {:?}", status.code()).into())
        }
    }
}
