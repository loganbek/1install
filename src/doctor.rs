//! System health and diagnostic tools

use crate::backends::get_all_available_backends;
use crate::shims::{get_shim_dir, ShimRegistry};
use std::path::PathBuf;

pub struct Doctor;

impl Doctor {
    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        println!("🩺 Running 1install diagnostics...");
        
        Self::check_shim_dir()?;
        Self::check_shim_registry()?;
        Self::check_conflicts()?;
        
        println!("\n✓ Diagnostics complete.");
        Ok(())
    }

    fn check_shim_dir() -> Result<(), Box<dyn std::error::Error>> {
        let shim_dir = get_shim_dir();
        print!("   Shim directory... ");
        
        if shim_dir.exists() {
            println!("✓ Exist ({})", shim_dir.display());
        } else {
            println!("✗ Missing");
            println!("     Try running '1i self-install' to fix.");
        }
        
        // Check if in PATH
        let path = std::env::var("PATH").unwrap_or_default();
        let in_path = path.contains(shim_dir.to_str().unwrap_or(""));
        
        print!("   PATH integration... ");
        if in_path {
            println!("✓ Found");
        } else {
            println!("✗ Not found in $PATH");
        }
        
        Ok(())
    }

    fn check_shim_registry() -> Result<(), Box<dyn std::error::Error>> {
        let registry = ShimRegistry::load()?;
        println!("   Shim registry... ✓ Loaded ({} shims)", registry.len());
        
        let mut broken = 0;
        for shim in registry.list() {
            if !shim.target.exists() {
                broken += 1;
                println!("     ✗ Broken shim: {} -> {} (target missing)", shim.name, shim.target.display());
            }
        }
        
        if broken == 0 && !registry.is_empty() {
            println!("     ✓ All shims point to valid targets.");
        }
        
        Ok(())
    }

    fn check_conflicts() -> Result<(), Box<dyn std::error::Error>> {
        println!("   Conflict detection...");
        
        let backends = get_all_available_backends();
        if backends.len() < 2 {
            println!("     ℹ Only one backend available, no conflicts possible.");
            return Ok(());
        }
        
        let common_tools = ["git", "node", "python", "python3", "jq", "rg", "fd", "npm", "pip", "docker", "curlie", "bat"];
        let mut conflicts_found = 0;

        for tool in common_tools {
            let mut providers: Vec<String> = Vec::new();
            
            // Check via 'which' / 'where' to see if multiple paths exist
            // This is a proxy for detecting if different managers installed it to different locations
            #[cfg(windows)]
            let paths = {
                let output = std::process::Command::new("where").arg(tool).output().ok();
                output.map(|o| String::from_utf8_lossy(&o.stdout).lines().map(String::from).collect::<Vec<String>>()).unwrap_or_default()
            };
            #[cfg(not(windows))]
            let paths = {
                // On Unix, 'which -a' shows all occurrences
                let output = std::process::Command::new("which").args(["-a", tool]).output().ok();
                output.map(|o| String::from_utf8_lossy(&o.stdout).lines().map(String::from).collect::<Vec<_>>()).unwrap_or_default()
            };

            if paths.len() > 1 {
                conflicts_found += 1;
                println!("     ⚠️ Found potential conflict for '{}':", tool);
                for path in paths {
                    println!("       - {}", path);
                }
            }
        }

        if conflicts_found == 0 {
            println!("     ✓ No obvious binary name collisions detected.");
        } else {
            println!("\n     ℹ Recommendation: Use '1i shims refresh' or '1i install' with --backend to prioritize a specific version.");
        }
        
        Ok(())
    }
}
