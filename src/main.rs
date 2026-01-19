//! 1install - Unified cross-platform package manager
//!
//! This is the main entry point for the `1i` CLI tool.

use oneinstall::cli::Cli;
use clap::Parser;

fn main() {
    let cli = Cli::parse();
    
    if let Err(e) = oneinstall::run(cli) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
