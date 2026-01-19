//! Homebrew backend for macOS (and Linux)

use super::{Backend, command_exists, run_command_output};
use crate::search::PackageResult;
use std::process::Command;

/// Homebrew package manager backend
pub struct BrewBackend;

impl BrewBackend {
    pub fn new() -> Self {
        Self
    }
    
    /// Parse brew search output into PackageResults
    fn parse_search_output(&self, output: &str) -> Vec<PackageResult> {
        output.lines()
            .filter(|line| !line.trim().is_empty() && !line.starts_with("==>"))
            .map(|line| {
                let name = line.trim().to_string();
                PackageResult::new(name, "brew".to_string())
            })
            .collect()
    }
}

impl Default for BrewBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl Backend for BrewBackend {
    fn name(&self) -> &str {
        "brew"
    }
    
    fn is_available(&self) -> bool {
        command_exists("brew")
    }
    
    fn search(&self, query: &str) -> Result<Vec<PackageResult>, Box<dyn std::error::Error + Send + Sync>> {
        let output = run_command_output("brew", &["search", query])?;
        Ok(self.parse_search_output(&output))
    }
    
    fn install(&self, package: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("   Running: brew install {}", package);
        
        let status = Command::new("brew")
            .args(["install", package])
            .status()?;
        
        if status.success() {
            Ok(())
        } else {
            Err(format!("brew install failed with exit code: {:?}", status.code()).into())
        }
    }

    fn update(&self, package: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("   Running: brew upgrade {}", package);
        
        let status = Command::new("brew")
            .args(["upgrade", package])
            .status()?;
        
        if status.success() {
            Ok(())
        } else {
            Err(format!("brew upgrade failed with exit code: {:?}", status.code()).into())
        }
    }

    fn uninstall(&self, package: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("   Running: brew uninstall {}", package);
        
        let status = Command::new("brew")
            .args(["uninstall", package])
            .status()?;
        
        if status.success() {
            Ok(())
        } else {
            Err(format!("brew uninstall failed with exit code: {:?}", status.code()).into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_brew_name() {
        let backend = BrewBackend::new();
        assert_eq!(backend.name(), "brew");
    }
}
