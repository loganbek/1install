//! 1install library - core functionality
//!
//! This module provides the main `run` function and re-exports
//! all public types for the CLI.

pub mod cli;
pub mod context;
pub mod backends;
pub mod search;

use cli::{Cli, Commands, render_search_results, render_backends};
use context::OsContext;
use backends::{get_backend_for_context, get_all_available_backends, Backend};
use search::{SearchAggregator, PackageResult};

/// Main entry point for 1install operations
pub fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    match cli.command {
        Commands::Search { query, limit } => {
            search_packages(&query, limit)?;
        }
        Commands::Install { package, backend } => {
            install_package(&package, backend.as_deref())?;
        }
        Commands::Backends => {
            list_backends();
        }
    }
    Ok(())
}

/// Search for packages across all available backends
fn search_packages(query: &str, limit: usize) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Searching for '{}'...\n", query);
    
    let backends = get_all_available_backends();
    
    if backends.is_empty() {
        println!("No package managers available on this system.");
        return Ok(());
    }
    
    println!("   Searching {} backends: {}", 
        backends.len(),
        backends.iter().map(|b| b.name()).collect::<Vec<_>>().join(", ")
    );
    println!();
    
    let mut all_results: Vec<PackageResult> = Vec::new();
    
    // Search each backend (sequentially for now, async in future)
    for backend in &backends {
        match backend.search(query) {
            Ok(mut results) => {
                all_results.append(&mut results);
            }
            Err(e) => {
                eprintln!("   Warning: {} search failed: {}", backend.name(), e);
            }
        }
    }
    
    // Rank and deduplicate results
    SearchAggregator::rank_results(query, &mut all_results);
    
    // Render table
    render_search_results(&all_results, limit);
    
    Ok(())
}

/// Install a package using the appropriate backend
fn install_package(package: &str, backend_name: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Detecting system...");
    
    let backend: Box<dyn Backend> = if let Some(name) = backend_name {
        // User specified a backend
        let all = get_all_available_backends();
        all.into_iter()
            .find(|b| b.name() == name)
            .ok_or_else(|| format!("Backend '{}' not available", name))?
    } else {
        // Auto-detect based on OS
        let context = OsContext::detect();
        println!("   OS: {:?}", context.os_type);
        get_backend_for_context(&context)?
    };
    
    println!("   Backend: {}", backend.name());
    
    // Check if backend is available
    if !backend.is_available() {
        return Err(format!(
            "Backend '{}' is not available on this system",
            backend.name()
        ).into());
    }
    println!("   Status: ✓ Available\n");
    
    // Execute installation
    println!("📦 Installing {}...", package);
    backend.install(package)?;
    
    println!("\n✓ {} installed successfully!", package);
    
    Ok(())
}

/// List available backends
fn list_backends() {
    let backends = get_all_available_backends();
    let names: Vec<&str> = backends.iter().map(|b| b.name()).collect();
    render_backends(&names);
}
