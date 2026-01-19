//! 1install library - core functionality

pub mod cli;
pub mod context;
pub mod backends;
pub mod search;
pub mod shims;
pub mod config;

use cli::{Cli, Commands, ConfigAction, ShimsAction, render_search_results, render_backends};
use context::OsContext;
use backends::{get_backend_for_context, get_all_available_backends, Backend};
use search::{SearchAggregator, PackageResult};
use config::{load_config, save_config, get_config_path, Config};
use shims::{get_shim_dir, ShimRegistry};

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
        Commands::Config { action } => {
            handle_config(action)?;
        }
        Commands::Shims { action } => {
            handle_shims(action)?;
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
    
    SearchAggregator::rank_results(query, &mut all_results);
    render_search_results(&all_results, limit);
    
    Ok(())
}

/// Install a package using the appropriate backend
fn install_package(package: &str, backend_name: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Detecting system...");
    
    let backend: Box<dyn Backend> = if let Some(name) = backend_name {
        let all = get_all_available_backends();
        all.into_iter()
            .find(|b| b.name() == name)
            .ok_or_else(|| format!("Backend '{}' not available", name))?
    } else {
        let context = OsContext::detect();
        println!("   OS: {:?}", context.os_type);
        get_backend_for_context(&context)?
    };
    
    println!("   Backend: {}", backend.name());
    
    if !backend.is_available() {
        return Err(format!(
            "Backend '{}' is not available on this system",
            backend.name()
        ).into());
    }
    println!("   Status: ✓ Available\n");
    
    println!("📦 Installing {}...", package);
    backend.install(package)?;
    
    // Handle shim creation
    let config = load_config().unwrap_or_default();
    if config.behavior.create_shims {
        match try_create_shim(package, backend.name()) {
            Ok(Some(path)) => println!("   ✓ Created shim: {}", path.display()),
            Ok(None) => if config.behavior.verbose { println!("   ℹ Could not locate binary for shim creation"); },
            Err(e) => eprintln!("   ⚠ Shim creation failed: {}", e),
        }
    }
    
    println!("\n✓ {} installed successfully!", package);
    
    Ok(())
}

/// Try to create a shim for an installed package
fn try_create_shim(package: &str, backend_name: &str) -> Result<Option<std::path::PathBuf>, Box<dyn std::error::Error>> {
    // Try to find the binary in PATH
    let binary_name = package; // Simplification: assume binary name = package name
    
    let path = if cfg!(windows) {
        let output = std::process::Command::new("where")
            .arg(binary_name)
            .output()?;
        if output.status.success() {
            let s = String::from_utf8_lossy(&output.stdout);
            s.lines().next().map(std::path::PathBuf::from)
        } else {
            None
        }
    } else {
        let output = std::process::Command::new("which")
            .arg(binary_name)
            .output()?;
        if output.status.success() {
            let s = String::from_utf8_lossy(&output.stdout);
            Some(std::path::PathBuf::from(s.trim()))
        } else {
            None
        }
    };
    
    if let Some(target_path) = path {
        // Create the shim
        let shim_path = shims::create_shim(binary_name, &target_path)?;
        
        // Register in shim registry
        let mut registry = ShimRegistry::load()?;
        registry.add(binary_name.to_string(), target_path, backend_name.to_string());
        registry.save()?;
        
        Ok(Some(shim_path))
    } else {
        Ok(None)
    }
}

/// List available backends
fn list_backends() {
    let backends = get_all_available_backends();
    let names: Vec<&str> = backends.iter().map(|b| b.name()).collect();
    render_backends(&names);
}

/// Handle config subcommands
fn handle_config(action: ConfigAction) -> Result<(), Box<dyn std::error::Error>> {
    match action {
        ConfigAction::Get { key } => {
            let config = load_config()?;
            match config.get(&key) {
                Some(value) => println!("{} = {}", key, value),
                None => eprintln!("Unknown config key: {}", key),
            }
        }
        ConfigAction::Set { key, value } => {
            let mut config = load_config()?;
            config.set(&key, &value).map_err(|e| e)?;
            save_config(&config)?;
            println!("✓ Set {} = {}", key, value);
        }
        ConfigAction::List => {
            let config = load_config()?;
            println!("Configuration:\n");
            for key in Config::list_keys() {
                if let Some(value) = config.get(key) {
                    println!("  {} = {}", key, value);
                }
            }
        }
        ConfigAction::Path => {
            println!("{}", get_config_path().display());
        }
    }
    Ok(())
}

/// Handle shims subcommands
fn handle_shims(action: ShimsAction) -> Result<(), Box<dyn std::error::Error>> {
    match action {
        ShimsAction::List => {
            let registry = ShimRegistry::load()?;
            if registry.is_empty() {
                println!("No shims registered.");
            } else {
                println!("Registered shims ({}):\n", registry.len());
                for shim in registry.list() {
                    println!("  {} → {} ({})", 
                        shim.name, 
                        shim.target.display(),
                        shim.installed_by
                    );
                }
            }
        }
        ShimsAction::Path => {
            println!("{}", get_shim_dir().display());
        }
        ShimsAction::Setup => {
            println!("{}", shims::get_path_instruction());
        }
    }
    Ok(())
}
