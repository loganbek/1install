//! Backend providers for different package managers

mod winget;
mod apt;
mod brew;
mod npm;
mod pip;

pub use winget::WingetBackend;
pub use apt::AptBackend;
pub use brew::BrewBackend;
pub use npm::NpmBackend;
pub use pip::PipBackend;

use crate::context::{OsContext, OsType, LinuxDistro};
use crate::search::PackageResult;
use std::process::{Command, Stdio};

/// Trait for package manager backends
pub trait Backend: Send + Sync {
    /// Get the name of this backend
    fn name(&self) -> &str;
    
    /// Check if this backend is available on the system
    fn is_available(&self) -> bool;
    
    /// Search for packages matching the query
    fn search(&self, query: &str) -> Result<Vec<PackageResult>, Box<dyn std::error::Error + Send + Sync>>;
    
    /// Install a package
    fn install(&self, package: &str) -> Result<(), Box<dyn std::error::Error>>;
}

/// Get the appropriate backend for the detected OS context
pub fn get_backend_for_context(context: &OsContext) -> Result<Box<dyn Backend>, Box<dyn std::error::Error>> {
    match &context.os_type {
        OsType::Windows => {
            Ok(Box::new(WingetBackend::new()))
        }
        OsType::Linux { distro } => {
            match distro {
                LinuxDistro::Debian => Ok(Box::new(AptBackend::new())),
                LinuxDistro::Arch => {
                    Err("Pacman backend not yet implemented".into())
                }
                LinuxDistro::Fedora => {
                    Err("DNF backend not yet implemented".into())
                }
                LinuxDistro::Unknown => {
                    let apt = AptBackend::new();
                    if apt.is_available() {
                        Ok(Box::new(apt))
                    } else {
                        Err("Could not detect a supported package manager".into())
                    }
                }
            }
        }
        OsType::MacOS => {
            Ok(Box::new(BrewBackend::new()))
        }
        OsType::Unknown => {
            Err("Unknown operating system".into())
        }
    }
}

/// Get all available backends on this system
pub fn get_all_available_backends() -> Vec<Box<dyn Backend>> {
    let all_backends: Vec<Box<dyn Backend>> = vec![
        Box::new(WingetBackend::new()),
        Box::new(AptBackend::new()),
        Box::new(BrewBackend::new()),
        Box::new(NpmBackend::new()),
        Box::new(PipBackend::new()),
    ];
    
    all_backends
        .into_iter()
        .filter(|b| b.is_available())
        .collect()
}

/// Helper to check if a command exists
pub(crate) fn command_exists(cmd: &str) -> bool {
    #[cfg(target_os = "windows")]
    {
        Command::new("where")
            .arg(cmd)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        Command::new("which")
            .arg(cmd)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }
}

/// Helper to run a command and capture output
pub(crate) fn run_command_output(cmd: &str, args: &[&str]) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let output = Command::new(cmd)
        .args(args)
        .output()?;
    
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Command failed: {}", stderr).into())
    }
}
