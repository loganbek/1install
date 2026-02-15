//! Shim generator for cross-platform binary wrappers

use std::fs;
use std::path::{Path, PathBuf};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

/// Get the shim directory path
pub fn get_shim_dir() -> PathBuf {
    // Prefer XDG bin dir if set, otherwise default to ~/.local/bin
    if let Ok(xdg) = std::env::var("XDG_BIN_HOME") {
        return PathBuf::from(xdg);
    }

    let home = dirs::home_dir().expect("Could not find home directory");

    #[cfg(windows)]
    {
        home.join(".local").join("bin")
    }

    #[cfg(not(windows))]
    {
        home.join(".local").join("bin")
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

/// Remove shim files for a given binary name. Returns true if any file was removed.
pub fn remove_shim(binary_name: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let shim_dir = get_shim_dir();
    let mut removed = false;

    #[cfg(windows)]
    {
        let cmd = shim_dir.join(format!("{}.cmd", binary_name));
        let ps1 = shim_dir.join(format!("{}.ps1", binary_name));
        if cmd.exists() { let _ = std::fs::remove_file(&cmd); removed = true; }
        if ps1.exists() { let _ = std::fs::remove_file(&ps1); removed = true; }
    }

    #[cfg(not(windows))]
    {
        let shim = shim_dir.join(binary_name);
        if shim.exists() { let _ = std::fs::remove_file(&shim); removed = true; }
    }

    Ok(removed)
}

/// Create a Unix shell script shim
#[cfg(not(windows))]
fn create_unix_shim(shim_dir: &Path, binary_name: &str, target_path: &Path) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let shim_path = shim_dir.join(binary_name);
    let script = format!(r#"#!/bin/sh
# 1install shim for {name}
exec "{target}" "$@"
"#, name = binary_name, target = target_path.display());

    // Ensure the shim directory exists
    fs::create_dir_all(shim_dir)?;

    // Write atomically: write to a temp file then rename
    let mut tmp = shim_path.with_extension(".tmp");
    fs::write(&tmp, script.as_bytes())?;

    // Set exec perms on tmp first
    let mut perms = fs::metadata(&tmp)?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&tmp, perms)?;

    fs::rename(&tmp, &shim_path)?;

    Ok(shim_path)
}

/// Create a Windows batch file shim
#[cfg(windows)]
fn create_windows_shim(shim_dir: &Path, binary_name: &str, target_path: &Path) -> Result<PathBuf, Box<dyn std::error::Error>> {
    // Create both .cmd and .ps1 shims for maximum compatibility
    let cmd_path = shim_dir.join(format!("{}.cmd", binary_name));
    let ps1_path = shim_dir.join(format!("{}.ps1", binary_name));
    
    // Batch file shim
    let cmd_script = format!("@echo off\r\nrem 1install shim for {}\r\n\"{}\" %*\r\n", binary_name, target_path.display());

    // write atomically for both files
    let mut cmd_tmp = cmd_path.with_extension("cmd.tmp");
    fs::write(&cmd_tmp, cmd_script.as_bytes())?;
    fs::rename(&cmd_tmp, &cmd_path)?;

    // PowerShell shim: ensure single quotes around path and escape any single quotes
    let target_str = target_path.display().to_string();
    let escaped = target_str.replace("'", "''");
    let ps1_script = format!("# 1install shim for {}\r\n& '{}' $args\r\n", binary_name, escaped);
    let mut ps1_tmp = ps1_path.with_extension("ps1.tmp");
    fs::write(&ps1_tmp, ps1_script.as_bytes())?;
    fs::rename(&ps1_tmp, &ps1_path)?;

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
        // By default the shim dir should be the user's XDG_BIN_HOME or ~/.local/bin
        let s = dir.to_string_lossy().to_string();
        assert!(s.ends_with(".local/bin") || std::env::var("XDG_BIN_HOME").is_ok());
    }
}
