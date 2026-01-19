//! APT backend for Debian-based Linux distributions

use super::{Backend, command_exists, run_command_output};
use crate::search::PackageResult;
use std::process::Command;

/// APT package manager backend (Debian, Ubuntu, etc.)
pub struct AptBackend;

impl AptBackend {
    pub fn new() -> Self {
        Self
    }
    
    /// Parse apt-cache search output into PackageResults
    fn parse_search_output(&self, output: &str) -> Vec<PackageResult> {
        output.lines()
            .filter_map(|line| {
                // apt-cache search format: "package-name - Description text"
                let parts: Vec<&str> = line.splitn(2, " - ").collect();
                if parts.len() == 2 {
                    let name = parts[0].trim().to_string();
                    let desc = parts[1].trim().to_string();
                    Some(
                        PackageResult::new(name, "apt".to_string())
                            .with_description(desc)
                    )
                } else {
                    None
                }
            })
            .collect()
    }
}

impl Default for AptBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl Backend for AptBackend {
    fn name(&self) -> &str {
        "apt"
    }
    
    fn is_available(&self) -> bool {
        command_exists("apt-cache")
    }
    
    fn search(&self, query: &str) -> Result<Vec<PackageResult>, Box<dyn std::error::Error + Send + Sync>> {
        let output = run_command_output("apt-cache", &["search", query])?;
        Ok(self.parse_search_output(&output))
    }
    
    fn install(&self, package: &str) -> Result<(), Box<dyn std::error::Error>> {
        let need_sudo = !is_root();
        
        let cmd_display = if need_sudo {
            format!("sudo apt-get install -y {}", package)
        } else {
            format!("apt-get install -y {}", package)
        };
        
        println!("   Running: {}", cmd_display);
        
        let status = if need_sudo {
            Command::new("sudo")
                .args(["apt-get", "install", "-y", package])
                .env("DEBIAN_FRONTEND", "noninteractive")
                .status()?
        } else {
            Command::new("apt-get")
                .args(["install", "-y", package])
                .env("DEBIAN_FRONTEND", "noninteractive")
                .status()?
        };
        
        if status.success() {
            Ok(())
        } else {
            Err(format!("apt-get install failed with exit code: {:?}", status.code()).into())
        }
    }

    fn update(&self, package: &str) -> Result<(), Box<dyn std::error::Error>> {
        let need_sudo = !is_root();
        
        if need_sudo {
            println!("   Running: sudo apt-get update && sudo apt-get install --only-upgrade -y {}", package);
            Command::new("sudo").args(["apt-get", "update"]).status()?;
            let status = Command::new("sudo")
                .args(["apt-get", "install", "--only-upgrade", "-y", package])
                .env("DEBIAN_FRONTEND", "noninteractive")
                .status()?;
            if status.success() { Ok(()) } else { Err(format!("apt-get update failed: {:?}", status.code()).into()) }
        } else {
            println!("   Running: apt-get update && apt-get install --only-upgrade -y {}", package);
            Command::new("apt-get").args(["apt-get", "update"]).status()?;
            let status = Command::new("apt-get")
                .args(["install", "--only-upgrade", "-y", package])
                .env("DEBIAN_FRONTEND", "noninteractive")
                .status()?;
            if status.success() { Ok(()) } else { Err(format!("apt-get update failed: {:?}", status.code()).into()) }
        }
    }

    fn uninstall(&self, package: &str) -> Result<(), Box<dyn std::error::Error>> {
        let need_sudo = !is_root();
        
        if need_sudo {
            println!("   Running: sudo apt-get remove -y {}", package);
            let status = Command::new("sudo")
                .args(["apt-get", "remove", "-y", package])
                .status()?;
            if status.success() { Ok(()) } else { Err(format!("apt-get remove failed: {:?}", status.code()).into()) }
        } else {
            println!("   Running: apt-get remove -y {}", package);
            let status = Command::new("apt-get")
                .args(["remove", "-y", package])
                .status()?;
            if status.success() { Ok(()) } else { Err(format!("apt-get remove failed: {:?}", status.code()).into()) }
        }
    }
}

/// Check if running as root
fn is_root() -> bool {
    #[cfg(unix)]
    {
        unsafe { libc::geteuid() == 0 }
    }
    
    #[cfg(not(unix))]
    {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_apt_name() {
        let backend = AptBackend::new();
        assert_eq!(backend.name(), "apt");
    }
    
    #[test]
    fn test_parse_search_output() {
        let backend = AptBackend::new();
        let output = "python3 - Interactive high-level OO language\npython3-pip - Python package installer";
        let results = backend.parse_search_output(output);
        
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].name, "python3");
        assert_eq!(results[1].name, "python3-pip");
    }
}
