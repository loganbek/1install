//! CLI command definitions using clap

use clap::{Parser, Subcommand};

/// 1install - Unified cross-platform package manager
#[derive(Parser, Debug)]
#[command(name = "1i")]
#[command(author = "Logan Bek, Antigravity & Claude Opus")]
#[command(version)]
#[command(about = "Unified cross-platform package manager", long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Available commands
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Search for packages across all available package managers
    Search {
        /// Search query
        #[arg(value_name = "QUERY")]
        query: String,
        
        /// Maximum number of results to display
        #[arg(short, long, default_value = "20")]
        limit: usize,
    },
    
    /// Install a package
    Install {
        /// Name of the package to install
        #[arg(value_name = "PACKAGE")]
        package: String,
        
        /// Specify which backend to use (e.g., apt, winget, npm)
        #[arg(short, long)]
        backend: Option<String>,
        
        /// Expected SHA-256 hash of the package (binary) for integrity verification
        #[arg(long, value_name = "HASH")]
        verify: Option<String>,
    },
    
    /// List available backends on this system
    Backends,
    
    /// Manage configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
    
    /// Manage shims
    Shims {
        #[command(subcommand)]
        action: ShimsAction,
    },
    
    /// Update a package to the latest version
    Update {
        /// Name of the package to update
        #[arg(value_name = "PACKAGE")]
        package: String,
        
        /// Specify which backend to use
        #[arg(short, long)]
        backend: Option<String>,
    },
    
    /// Uninstall a package
    Uninstall {
        /// Name of the package to uninstall
        #[arg(value_name = "PACKAGE")]
        package: String,
        
        /// Specify which backend to use
        #[arg(short, long)]
        backend: Option<String>,
    },
    
    /// Install 1install to the local system (bootstrap)
    SelfInstall,
    
    /// Diagnose system health and package conflicts
    Doctor,
}

/// Config subcommands
#[derive(Subcommand, Debug)]
pub enum ConfigAction {
    /// Get a configuration value
    Get {
        /// Configuration key (e.g., backends.priority)
        key: String,
    },
    /// Set a configuration value
    Set {
        /// Configuration key
        key: String,
        /// Value to set
        value: String,
    },
    /// List all configuration values
    List,
    /// Show config file path
    Path,
}

/// Shims subcommands
#[derive(Subcommand, Debug)]
pub enum ShimsAction {
    /// List all shims
    List,
    /// Show the shim directory path
    Path,
    /// Show PATH setup instructions
    Setup,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        Cli::command().debug_assert();
    }
    
    #[test]
    fn test_config_get() {
        let cli = Cli::parse_from(["1i", "config", "get", "backends.priority"]);
        match cli.command {
            Commands::Config { action: ConfigAction::Get { key } } => {
                assert_eq!(key, "backends.priority");
            }
            _ => panic!("Expected Config Get command"),
        }
    }
}
