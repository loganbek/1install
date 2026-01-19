//! Winget backend for Windows

use super::{Backend, command_exists, run_command_output};
use crate::search::PackageResult;
use std::process::Command;

/// Windows Package Manager (winget) backend
pub struct WingetBackend;

impl WingetBackend {
    pub fn new() -> Self {
        Self
    }
    
    /// Parse winget search output into PackageResults
    fn parse_search_output(&self, output: &str) -> Vec<PackageResult> {
        let mut results = Vec::new();
        let lines: Vec<&str> = output.lines().collect();
        
        // Find the header line to determine column positions
        let header_idx = lines.iter().position(|l| l.contains("Name") && l.contains("Id"));
        if header_idx.is_none() {
            return results;
        }
        
        let header = lines[header_idx.unwrap()];
        let separator_idx = header_idx.unwrap() + 1;
        
        // Skip header and separator
        for line in lines.iter().skip(separator_idx + 1) {
            if line.trim().is_empty() {
                continue;
            }
            
            // winget output is column-aligned, try to parse it
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let name = parts[0].to_string();
                let id = parts.get(1).map(|s| s.to_string());
                let version = parts.get(2).map(|s| s.to_string());
                
                let mut result = PackageResult::new(name, "winget".to_string());
                if let Some(v) = version {
                    result = result.with_version(v);
                }
                if let Some(id) = id {
                    result = result.with_description(format!("ID: {}", id));
                }
                results.push(result);
            }
        }
        
        results
    }
}

impl Default for WingetBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl Backend for WingetBackend {
    fn name(&self) -> &str {
        "winget"
    }
    
    fn is_available(&self) -> bool {
        command_exists("winget")
    }
    
    fn search(&self, query: &str) -> Result<Vec<PackageResult>, Box<dyn std::error::Error + Send + Sync>> {
        let output = run_command_output("winget", &["search", query, "--accept-source-agreements"])?;
        Ok(self.parse_search_output(&output))
    }
    
    fn install(&self, package: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("   Running: winget install {} --accept-source-agreements --accept-package-agreements", package);
        
        let status = Command::new("winget")
            .args([
                "install",
                package,
                "-e",
                "--accept-source-agreements",
                "--accept-package-agreements",
            ])
            .status()?;
        
        if status.success() {
            Ok(())
        } else {
            Err(format!("winget install failed with exit code: {:?}", status.code()).into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_winget_name() {
        let backend = WingetBackend::new();
        assert_eq!(backend.name(), "winget");
    }
}
