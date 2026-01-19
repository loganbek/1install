//! Go backend for Go tools

use super::{Backend, command_exists};
use crate::search::PackageResult;
use std::process::Command;

pub struct GoBackend;

impl GoBackend {
    pub fn new() -> Self {
        Self
    }
}

impl Default for GoBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl Backend for GoBackend {
    fn name(&self) -> &str {
        "go"
    }
    
    fn is_available(&self) -> bool {
        command_exists("go")
    }
    
    fn search(&self, _query: &str) -> Result<Vec<PackageResult>, Box<dyn std::error::Error + Send + Sync>> {
        // Go search is not built into the CLI in a way that returns package info easily.
        // We return empty for now as go install usually requires the full path anyway.
        Ok(Vec::new())
    }
    
    fn install(&self, package: &str) -> Result<(), Box<dyn std::error::Error>> {
        let pkg_path = if package.contains('@') {
            package.to_string()
        } else {
            format!("{}@latest", package)
        };
        
        println!("   Running: go install {}", pkg_path);
        
        let status = Command::new("go")
            .args(["install", &pkg_path])
            .status()?;
            
        if status.success() {
            Ok(())
        } else {
            Err(format!("go install failed with exit code: {:?}", status.code()).into())
        }
    }

    fn update(&self, package: &str) -> Result<(), Box<dyn std::error::Error>> {
        // re-installing with @latest updates it
        self.install(package)
    }

    fn uninstall(&self, _package: &str) -> Result<(), Box<dyn std::error::Error>> {
        Err("Go does not support a native 'uninstall' command. Please manually remove the binary from your $GOPATH/bin.".into())
    }
}
