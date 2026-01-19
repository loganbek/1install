//! NPM backend for Node.js packages

use super::{Backend, command_exists, run_command_output};
use crate::search::PackageResult;
use std::process::Command;

/// NPM package manager backend
pub struct NpmBackend;

impl NpmBackend {
    pub fn new() -> Self {
        Self
    }
    
    /// Parse npm search output into PackageResults
    fn parse_search_output(&self, output: &str) -> Vec<PackageResult> {
        // npm search --json returns JSON array
        // For simplicity, parse the default tabular output
        output.lines()
            .skip(1) // Skip header
            .filter(|line| !line.trim().is_empty())
            .filter_map(|line| {
                let parts: Vec<&str> = line.split('|').collect();
                if parts.len() >= 3 {
                    let name = parts[0].trim().to_string();
                    let desc = parts[1].trim().to_string();
                    let version = parts.get(4).map(|s| s.trim().to_string());
                    
                    let mut result = PackageResult::new(name, "npm".to_string())
                        .with_description(desc);
                    if let Some(v) = version {
                        result = result.with_version(v);
                    }
                    Some(result)
                } else {
                    // Fallback: just use the line as package name
                    let name = line.split_whitespace().next()?.to_string();
                    Some(PackageResult::new(name, "npm".to_string()))
                }
            })
            .collect()
    }
}

impl Default for NpmBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl Backend for NpmBackend {
    fn name(&self) -> &str {
        "npm"
    }
    
    fn is_available(&self) -> bool {
        command_exists("npm")
    }
    
    fn search(&self, query: &str) -> Result<Vec<PackageResult>, Box<dyn std::error::Error + Send + Sync>> {
        // Use npm search with limited results for speed
        let output = run_command_output("npm", &["search", query, "--long", "--parseable"])?;
        Ok(self.parse_search_output(&output))
    }
    
    fn install(&self, package: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("   Running: npm install -g {}", package);
        
        let status = Command::new("npm")
            .args(["install", "-g", package])
            .status()?;
        
        if status.success() {
            Ok(())
        } else {
            Err(format!("npm install failed with exit code: {:?}", status.code()).into())
        }
    }

    fn update(&self, package: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("   Running: npm update -g {}", package);
        
        let status = Command::new("npm")
            .args(["update", "-g", package])
            .status()?;
        
        if status.success() {
            Ok(())
        } else {
            Err(format!("npm update failed with exit code: {:?}", status.code()).into())
        }
    }

    fn uninstall(&self, package: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("   Running: npm uninstall -g {}", package);
        
        let status = Command::new("npm")
            .args(["uninstall", "-g", package])
            .status()?;
        
        if status.success() {
            Ok(())
        } else {
            Err(format!("npm uninstall failed with exit code: {:?}", status.code()).into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_npm_name() {
        let backend = NpmBackend::new();
        assert_eq!(backend.name(), "npm");
    }
}
