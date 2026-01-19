//! Git backend for source-based installations

use super::{Backend, command_exists};
use crate::search::PackageResult;
use std::process::Command;
use std::path::PathBuf;
use std::fs;

/// Git backend clones repositories and builds from source
pub struct GitBackend;

impl GitBackend {
    pub fn new() -> Self {
        Self
    }
    
    /// Detect build system and install
    fn build_and_install(&self, repo_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        if repo_path.join("Cargo.toml").exists() {
            println!("   Detected Rust project (Cargo)");
            self.install_cargo(repo_path)
        } else if repo_path.join("Makefile").exists() || repo_path.join("makefile").exists() {
            println!("   Detected Makefile");
            self.install_make(repo_path)
        } else if repo_path.join("package.json").exists() {
            println!("   Detected Node.js project (NPM)");
            self.install_npm(repo_path)
        } else {
            Err("Could not detect build system for this repository".into())
        }
    }
    
    fn install_cargo(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let status = Command::new("cargo")
            .args(["install", "--path", "."])
            .current_dir(path)
            .status()?;
        
        if status.success() {
            Ok(())
        } else {
            Err(format!("cargo install failed with exit code: {:?}", status.code()).into())
        }
    }
    
    fn install_make(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        // Run make
        println!("   Running: make");
        let status = Command::new("make")
            .current_dir(path)
            .status()?;
            
        if !status.success() {
            return Err(format!("make failed with exit code: {:?}", status.code()).into());
        }
        
        // Run make install (might need sudo)
        println!("   Running: make install");
        let status = Command::new("make")
            .arg("install")
            .current_dir(path)
            .status()?;
            
        if status.success() {
            Ok(())
        } else {
            Err(format!("make install failed with exit code: {:?}", status.code()).into())
        }
    }
    
    fn install_npm(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let status = Command::new("npm")
            .args(["install", "-g", "."])
            .current_dir(path)
            .status()?;
            
        if status.success() {
            Ok(())
        } else {
            Err(format!("npm install failed with exit code: {:?}", status.code()).into())
        }
    }
}

impl Default for GitBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl Backend for GitBackend {
    fn name(&self) -> &str {
        "git"
    }
    
    fn is_available(&self) -> bool {
        command_exists("git")
    }
    
    fn search(&self, _query: &str) -> Result<Vec<PackageResult>, Box<dyn std::error::Error + Send + Sync>> {
        // Git "search" doesn't really make sense in a unified way without an index
        // We could implement github API search here in the future
        Ok(vec![])
    }
    
    fn install(&self, repo_url: &str) -> Result<(), Box<dyn std::error::Error>> {
        if !repo_url.starts_with("http") && !repo_url.starts_with("git@") {
            return Err("Git installation requires a repository URL (e.g., https://github.com/user/repo)".into());
        }
        
        let temp_dir = std::env::temp_dir().join("1install-git").join(
            repo_url.rsplit('/').next().unwrap_or("repo").trim_end_matches(".git")
        );
        
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir)?;
        }
        fs::create_dir_all(&temp_dir)?;
        
        println!("   Cloning {} into {}...", repo_url, temp_dir.display());
        
        let status = Command::new("git")
            .args(["clone", "--depth", "1", repo_url, "."])
            .current_dir(&temp_dir)
            .status()?;
            
        if !status.success() {
            return Err(format!("git clone failed with exit code: {:?}", status.code()).into());
        }
        
        self.build_and_install(&temp_dir)?;
        
        // Clean up
        let _ = fs::remove_dir_all(&temp_dir);
        
        Ok(())
    }

    fn update(&self, repo_url: &str) -> Result<(), Box<dyn std::error::Error>> {
        // For git, update is just re-installing
        self.install(repo_url)
    }

    fn uninstall(&self, _repo_url: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Git "uninstall" is complex because we don't know where the build system installed things.
        // For cargo, we could try `cargo uninstall`.
        // For now, return an error explaining the limitation.
        Err("Uninstall for git source-installs is not yet supported. Please uninstall manually using the build tool (e.g., cargo uninstall)".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_git_name() {
        let backend = GitBackend::new();
        assert_eq!(backend.name(), "git");
    }
}
