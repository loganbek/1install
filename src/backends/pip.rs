//! Pip/pipx backend for Python packages

use super::{Backend, command_exists, run_command_output};
use crate::search::PackageResult;
use std::process::Command;

/// Pip package manager backend (prefers pipx for global installs)
pub struct PipBackend {
    use_pipx: bool,
}

impl PipBackend {
    pub fn new() -> Self {
        Self {
            use_pipx: command_exists("pipx"),
        }
    }
    
    /// Parse pip search output
    /// Note: pip search is disabled on PyPI, so we use pip index versions
    fn parse_search_output(&self, output: &str) -> Vec<PackageResult> {
        // pip index versions output format varies
        output.lines()
            .filter(|line| !line.trim().is_empty())
            .filter_map(|line| {
                // Try to extract package name and version
                let parts: Vec<&str> = line.split_whitespace().collect();
                if !parts.is_empty() {
                    let name = parts[0].trim_matches(|c| c == '(' || c == ')').to_string();
                    let version = parts.get(1).map(|s| s.trim_matches(|c| c == '(' || c == ')').to_string());
                    
                    let mut result = PackageResult::new(name, "pip".to_string());
                    if let Some(v) = version {
                        result = result.with_version(v);
                    }
                    Some(result)
                } else {
                    None
                }
            })
            .collect()
    }
}

impl Default for PipBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl Backend for PipBackend {
    fn name(&self) -> &str {
        if self.use_pipx { "pipx" } else { "pip" }
    }
    
    fn is_available(&self) -> bool {
        command_exists("pip") || command_exists("pip3") || command_exists("pipx")
    }
    
    fn search(&self, query: &str) -> Result<Vec<PackageResult>, Box<dyn std::error::Error + Send + Sync>> {
        // pip search is deprecated, try pip index versions for exact package
        let pip_cmd = if command_exists("pip3") { "pip3" } else { "pip" };
        
        match run_command_output(pip_cmd, &["index", "versions", query]) {
            Ok(output) => Ok(self.parse_search_output(&output)),
            Err(_) => {
                // Fallback: return a single result for the query (assume it exists)
                Ok(vec![PackageResult::new(query.to_string(), "pip".to_string())])
            }
        }
    }
    
    fn install(&self, package: &str) -> Result<(), Box<dyn std::error::Error>> {
        if self.use_pipx {
            println!("   Running: pipx install {}", package);
            
            let status = Command::new("pipx")
                .args(["install", package])
                .status()?;
            
            if status.success() {
                Ok(())
            } else {
                Err(format!("pipx install failed with exit code: {:?}", status.code()).into())
            }
        } else {
            let pip_cmd = if command_exists("pip3") { "pip3" } else { "pip" };
            println!("   Running: {} install --user {}", pip_cmd, package);
            
            let status = Command::new(pip_cmd)
                .args(["install", "--user", package])
                .status()?;
            
            if status.success() {
                Ok(())
            } else {
                Err(format!("pip install failed with exit code: {:?}", status.code()).into())
            }
        }
    }

    fn update(&self, package: &str) -> Result<(), Box<dyn std::error::Error>> {
        if self.use_pipx {
            println!("   Running: pipx upgrade {}", package);
            let status = Command::new("pipx").args(["upgrade", package]).status()?;
            if status.success() { Ok(()) } else { Err(format!("pipx upgrade failed: {:?}", status.code()).into()) }
        } else {
            let pip_cmd = if command_exists("pip3") { "pip3" } else { "pip" };
            println!("   Running: {} install --user --upgrade {}", pip_cmd, package);
            let status = Command::new(pip_cmd)
                .args(["install", "--user", "--upgrade", package])
                .status()?;
            if status.success() { Ok(()) } else { Err(format!("pip upgrade failed: {:?}", status.code()).into()) }
        }
    }

    fn uninstall(&self, package: &str) -> Result<(), Box<dyn std::error::Error>> {
        if self.use_pipx {
            println!("   Running: pipx uninstall {}", package);
            let status = Command::new("pipx").args(["uninstall", package]).status()?;
            if status.success() { Ok(()) } else { Err(format!("pipx uninstall failed: {:?}", status.code()).into()) }
        } else {
            let pip_cmd = if command_exists("pip3") { "pip3" } else { "pip" };
            println!("   Running: {} uninstall -y {}", pip_cmd, package);
            let status = Command::new(pip_cmd)
                .args(["uninstall", "-y", package])
                .status()?;
            if status.success() { Ok(()) } else { Err(format!("pip uninstall failed: {:?}", status.code()).into()) }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pip_name() {
        let backend = PipBackend::new();
        // Name depends on pipx availability
        let name = backend.name();
        assert!(name == "pip" || name == "pipx");
    }
}
