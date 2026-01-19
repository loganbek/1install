//! CLI command definitions using clap

use clap::{Parser, Subcommand};

/// 1install - Unified cross-platform package manager
///
/// Install packages from any source with a single command.
/// Automatically detects your OS and selects the best package manager.
#[derive(Parser, Debug)]
#[command(name = "1i")]
#[command(author = "Logan Bek")]
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
    ///
    /// Searches winget, apt, brew, npm, pip and more simultaneously,
    /// then displays results in a unified, ranked table.
    Search {
        /// Search query
        #[arg(value_name = "QUERY")]
        query: String,
        
        /// Maximum number of results to display
        #[arg(short, long, default_value = "20")]
        limit: usize,
    },
    
    /// Install a package
    ///
    /// Automatically detects your OS and uses the appropriate
    /// package manager (apt, winget, brew, etc.)
    Install {
        /// Name of the package to install
        #[arg(value_name = "PACKAGE")]
        package: String,
        
        /// Specify which backend to use (e.g., apt, winget, npm)
        #[arg(short, long)]
        backend: Option<String>,
    },
    
    /// List available backends on this system
    Backends,
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
    fn test_install_command() {
        let cli = Cli::parse_from(["1i", "install", "git"]);
        match cli.command {
            Commands::Install { package, .. } => {
                assert_eq!(package, "git");
            }
            _ => panic!("Expected Install command"),
        }
    }
    
    #[test]
    fn test_search_command() {
        let cli = Cli::parse_from(["1i", "search", "python"]);
        match cli.command {
            Commands::Search { query, limit } => {
                assert_eq!(query, "python");
                assert_eq!(limit, 20);
            }
            _ => panic!("Expected Search command"),
        }
    }
}
