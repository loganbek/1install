//! Shim generator for cross-platform binary wrappers

use std::fs;
use std::path::{Path, PathBuf};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

/// Get the shim directory path
pub fn get_shim_dir() -> PathBuf {
    let home = dirs::home_dir().expect("Could not find home directory");
    
    #[cfg(windows)]
    {
        home.join(".local").join("share").join("1install").join("shims")
    }
    
    #[cfg(not(windows))]
    {
        home.join(".local").join("share").join("1install").join("shims")
    }
}

/// Ensure the shim directory exists
pub fn ensure_shim_dir() -> std::io::Result<PathBuf> {
    let dir = get_shim_dir();
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

/// Create a shim for a binary
pub fn create_shim(binary_name: &str, target_path: &Path) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let shim_dir = ensure_shim_dir()?;
    
    #[cfg(windows)]
    {
        create_windows_shim(&shim_dir, binary_name, target_path)
    }
    
    #[cfg(not(windows))]
    {
        create_unix_shim(&shim_dir, binary_name, target_path)
    }
}

/// Create a Unix shell script shim
#[cfg(not(windows))]
fn create_unix_shim(shim_dir: &Path, binary_name: &str, target_path: &Path) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let shim_path = shim_dir.join(binary_name);
    
    let script = format!(r#"#!/bin/sh
# 1install shim for {}
exec "{}" "$@"
"#, binary_name, target_path.display());
    
    fs::write(&shim_path, script)?;
    
    // Make executable
    let mut perms = fs::metadata(&shim_path)?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&shim_path, perms)?;
    
    Ok(shim_path)
}

/// Create a Windows batch file shim
#[cfg(windows)]
fn create_windows_shim(shim_dir: &Path, binary_name: &str, target_path: &Path) -> Result<PathBuf, Box<dyn std::error::Error>> {
    // Create both .cmd and .ps1 shims for maximum compatibility
    let cmd_path = shim_dir.join(format!("{}.cmd", binary_name));
    let ps1_path = shim_dir.join(format!("{}.ps1", binary_name));
    
    // Batch file shim
    let cmd_script = format!(r#"@echo off
rem 1install shim for {}
"{}" %*
"#, binary_name, target_path.display());
    
    fs::write(&cmd_path, cmd_script)?;
    
    // PowerShell shim
    let ps1_script = format!(r#"# 1install shim for {}
& "{}" $args
"#, binary_name, target_path.display());
    
    fs::write(&ps1_path, ps1_script)?;
    
    Ok(cmd_path)
}

/// Get the path setup instruction for the user's shell
pub fn get_path_instruction() -> String {
    let shim_dir = get_shim_dir();
    
    #[cfg(windows)]
    {
        format!(
            r#"Add this to your PATH (one-time setup):

PowerShell (add to $PROFILE):
    $env:PATH = "{};$env:PATH"

Or add permanently via System Properties > Environment Variables
"#, 
            shim_dir.display()
        )
    }
    
    #[cfg(not(windows))]
    {
        format!(
            r#"Add this to your shell config (one-time setup):

bash (~/.bashrc):
    export PATH="{}:$PATH"

zsh (~/.zshrc):
    export PATH="{}:$PATH"

fish (~/.config/fish/config.fish):
    set -gx PATH {} $PATH
"#, 
            shim_dir.display(),
            shim_dir.display(),
            shim_dir.display()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_shim_dir() {
        let dir = get_shim_dir();
        assert!(dir.to_string_lossy().contains("1install"));
        assert!(dir.to_string_lossy().contains("shims"));
    }
}
