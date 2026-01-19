//! Operating system detection

use std::process::Command;

/// Detected operating system type
#[derive(Debug, Clone, PartialEq)]
pub enum OsType {
    /// Windows (any version)
    Windows,
    /// Linux with specific distribution
    Linux { distro: LinuxDistro },
    /// macOS
    MacOS,
    /// Unknown or unsupported OS
    Unknown,
}

/// Linux distribution identifiers
#[derive(Debug, Clone, PartialEq)]
pub enum LinuxDistro {
    /// Debian-based (Debian, Ubuntu, Mint, etc.)
    Debian,
    /// Arch-based (Arch, Manjaro, etc.)  
    Arch,
    /// Fedora/RHEL-based
    Fedora,
    /// Unknown distribution
    Unknown,
}

/// Operating system context
#[derive(Debug, Clone)]
pub struct OsContext {
    /// The detected OS type
    pub os_type: OsType,
    /// Raw OS name if available
    pub os_name: Option<String>,
    /// OS version if available
    pub os_version: Option<String>,
}

impl OsContext {
    /// Detect the current operating system context
    pub fn detect() -> Self {
        #[cfg(target_os = "windows")]
        {
            Self::detect_windows()
        }
        
        #[cfg(target_os = "linux")]
        {
            Self::detect_linux()
        }
        
        #[cfg(target_os = "macos")]
        {
            Self::detect_macos()
        }
        
        #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
        {
            Self {
                os_type: OsType::Unknown,
                os_name: None,
                os_version: None,
            }
        }
    }
    
    #[cfg(target_os = "windows")]
    fn detect_windows() -> Self {
        Self {
            os_type: OsType::Windows,
            os_name: Some("Windows".to_string()),
            os_version: Self::get_windows_version(),
        }
    }
    
    #[cfg(target_os = "windows")]
    fn get_windows_version() -> Option<String> {
        // Try to get Windows version from systeminfo or registry
        let output = Command::new("cmd")
            .args(["/C", "ver"])
            .output()
            .ok()?;
        
        if output.status.success() {
            let version = String::from_utf8_lossy(&output.stdout);
            Some(version.trim().to_string())
        } else {
            None
        }
    }
    
    #[cfg(target_os = "linux")]
    fn detect_linux() -> Self {
        let (distro, os_name, os_version) = Self::parse_os_release();
        
        Self {
            os_type: OsType::Linux { distro },
            os_name,
            os_version,
        }
    }
    
    #[cfg(target_os = "linux")]
    fn parse_os_release() -> (LinuxDistro, Option<String>, Option<String>) {
        use std::fs;
        
        let content = match fs::read_to_string("/etc/os-release") {
            Ok(c) => c,
            Err(_) => return (LinuxDistro::Unknown, None, None),
        };
        
        let mut id = None;
        let mut id_like = None;
        let mut name = None;
        let mut version = None;
        
        for line in content.lines() {
            if let Some((key, value)) = line.split_once('=') {
                let value = value.trim_matches('"');
                match key {
                    "ID" => id = Some(value.to_lowercase()),
                    "ID_LIKE" => id_like = Some(value.to_lowercase()),
                    "NAME" => name = Some(value.to_string()),
                    "VERSION_ID" => version = Some(value.to_string()),
                    _ => {}
                }
            }
        }
        
        // Determine distro family
        let distro = match id.as_deref() {
            Some("debian") | Some("ubuntu") | Some("linuxmint") | Some("pop") => LinuxDistro::Debian,
            Some("arch") | Some("manjaro") | Some("endeavouros") => LinuxDistro::Arch,
            Some("fedora") | Some("rhel") | Some("centos") | Some("rocky") => LinuxDistro::Fedora,
            _ => {
                // Check ID_LIKE for derivative distros
                match id_like.as_deref() {
                    Some(like) if like.contains("debian") || like.contains("ubuntu") => LinuxDistro::Debian,
                    Some(like) if like.contains("arch") => LinuxDistro::Arch,
                    Some(like) if like.contains("fedora") || like.contains("rhel") => LinuxDistro::Fedora,
                    _ => LinuxDistro::Unknown,
                }
            }
        };
        
        (distro, name, version)
    }
    
    #[cfg(target_os = "macos")]
    fn detect_macos() -> Self {
        let version = Command::new("sw_vers")
            .arg("-productVersion")
            .output()
            .ok()
            .and_then(|o| {
                if o.status.success() {
                    Some(String::from_utf8_lossy(&o.stdout).trim().to_string())
                } else {
                    None
                }
            });
        
        Self {
            os_type: OsType::MacOS,
            os_name: Some("macOS".to_string()),
            os_version: version,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_os_detection() {
        let context = OsContext::detect();
        
        #[cfg(target_os = "windows")]
        assert_eq!(context.os_type, OsType::Windows);
        
        #[cfg(target_os = "macos")]
        assert_eq!(context.os_type, OsType::MacOS);
        
        #[cfg(target_os = "linux")]
        match context.os_type {
            OsType::Linux { .. } => {}
            _ => panic!("Expected Linux OS type"),
        }
    }
}
