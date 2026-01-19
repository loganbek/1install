//! Snap backend for Linux

use super::{Backend, command_exists};
use crate::search::PackageResult;
use std::process::Command;

pub struct SnapBackend;

impl SnapBackend {
    pub fn new() -> Self {
        Self
    }
}

impl Default for SnapBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl Backend for SnapBackend {
    fn name(&self) -> &str {
        "snap"
    }
    
    fn is_available(&self) -> bool {
        command_exists("snap")
    }
    
    fn search(&self, query: &str) -> Result<Vec<PackageResult>, Box<dyn std::error::Error + Send + Sync>> {
        let output = Command::new("snap")
            .args(["find", query])
            .output()?;
            
        let mut results = Vec::new();
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        let mut lines = stdout.lines();
        let _header = lines.next();
        
        for line in lines {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 5 {
                results.push(PackageResult {
                    name: parts[0].to_string(),
                    version: Some(parts[1].to_string()),
                    description: Some(parts[4..].join(" ")),
                    source: "snap".to_string(),
                    score: 0.0,
                });
            }
        }
        
        Ok(results)
    }
    
    fn install(&self, package: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("   Running: sudo snap install {}", package);
        
        let status = Command::new("sudo")
            .args(["snap", "install", package])
            .status()?;
            
        if status.success() {
            Ok(())
        } else {
            Err(format!("snap install failed with exit code: {:?}", status.code()).into())
        }
    }

    fn update(&self, package: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("   Running: sudo snap refresh {}", package);
        
        let status = Command::new("sudo")
            .args(["snap", "refresh", package])
            .status()?;
            
        if status.success() {
            Ok(())
        } else {
            Err(format!("snap refresh failed with exit code: {:?}", status.code()).into())
        }
    }

    fn uninstall(&self, package: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("   Running: sudo snap remove {}", package);
        
        let status = Command::new("sudo")
            .args(["snap", "remove", package])
            .status()?;
            
        if status.success() {
            Ok(())
        } else {
            Err(format!("snap remove failed with exit code: {:?}", status.code()).into())
        }
    }
}
