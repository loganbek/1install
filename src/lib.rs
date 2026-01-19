//! 1install library - core functionality

pub mod cli;
pub mod context;
pub mod backends;
pub mod search;
pub mod shims;
pub mod config;
pub mod integrity;
pub mod telemetry;
pub mod doctor;

use cli::{Cli, Commands, ConfigAction, ShimsAction, render_search_results, render_backends};
use context::OsContext;
use backends::{get_backend_for_context, get_all_available_backends, Backend};
use search::{SearchAggregator, PackageResult};
use config::{load_config, save_config, get_config_path, Config};
use shims::{get_shim_dir, ShimRegistry};
use integrity::verify_file_hash;
use telemetry::{TelemetryClient, TelemetryEvent};
use doctor::Doctor;
use std::path::{Path, PathBuf};
use std::time::Instant;

/// Transactional state for installation
struct Transaction<'a> {
    package: &'a str,
    backend: Box<dyn Backend>,
    installed: bool,
    shim_created: bool,
}

impl<'a> Transaction<'a> {
    fn new(package: &'a str, backend: Box<dyn Backend>) -> Self {
        Self {
            package,
            backend,
            installed: false,
            shim_created: false,
        }
    }

    fn rollback(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.installed {
            println!("   ⚠️ Rolling back installation of {}...", self.package);
            let _ = self.backend.uninstall(self.package);
        }
        if self.shim_created {
            let mut registry = ShimRegistry::load()?;
            if registry.remove(self.package).is_some() {
                let _ = registry.save();
                let shim_dir = get_shim_dir();
                #[cfg(windows)]
                {
                    let _ = std::fs::remove_file(shim_dir.join(format!("{}.cmd", self.package)));
                    let _ = std::fs::remove_file(shim_dir.join(format!("{}.ps1", self.package)));
                }
                #[cfg(not(windows))]
                {
                    let _ = std::fs::remove_file(shim_dir.join(self.package));
                }
            }
        }
        Ok(())
    }
}

/// Main entry point for 1install operations
pub async fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    match cli.command {
        Commands::Search { query, limit } => {
            search_packages(query, limit).await?;
        }
        Commands::Install { package, backend, verify } => {
            install_package(&package, backend.as_deref(), verify.as_deref())?;
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
        Commands::Update { package, backend } => {
            update_package(&package, backend.as_deref())?;
        }
        Commands::Uninstall { package, backend } => {
            uninstall_package(&package, backend.as_deref())?;
        }
        Commands::SelfInstall => {
            handle_self_install()?;
        }
        Commands::Doctor => {
            Doctor::run()?;
        }
    }
    Ok(())
}

/// Search for packages across all available backends
pub async fn search_packages(query: String, limit: usize) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Searching for '{}'...\n", query);
    let start_time = Instant::now();
    
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
    
    TelemetryClient::track_event(TelemetryEvent::SearchStarted { 
        query_length: query.len(), 
        backends_count: backends.len() 
    });
    
    let mut join_set: tokio::task::JoinSet<(String, Result<Vec<PackageResult>, Box<dyn std::error::Error + Send + Sync>>)> = tokio::task::JoinSet::new();
    let query_shared = std::sync::Arc::new(query.clone());
    
    for backend in backends {
        let q = query_shared.clone();
        join_set.spawn(async move {
            match backend.search(&q) {
                Ok(results) => (backend.name().to_string(), Ok(results)),
                Err(e) => (backend.name().to_string(), Err(e)),
            }
        });
    }
    
    let mut all_results: Vec<PackageResult> = Vec::new();
    
    while let Some(res) = join_set.join_next().await {
        match res {
            Ok((_name, Ok(mut results))) => {
                all_results.append(&mut results);
            }
            Ok((name, Err(e))) => {
                eprintln!("   Warning: {} search failed: {}", name, e);
            }
            Err(e) => {
                eprintln!("   Error: Search task panicked: {}", e);
            }
        }
    }
    
    let duration = start_time.elapsed();
    TelemetryClient::track_event(TelemetryEvent::SearchFinished { 
        total_results: all_results.len(), 
        duration_ms: duration.as_millis() 
    });
    
    SearchAggregator::rank_results(&query, &mut all_results);
    render_search_results(&all_results, limit);
    
    Ok(())
}

/// Install a package using the appropriate backend
fn install_package(package: &str, backend_name: Option<&str>, verify_hash: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Detecting system...");
    let start_time = Instant::now();
    let backend = get_backend(backend_name)?;
    let backend_name_str = backend.name().to_string();
    
    println!("   Backend: {}", backend_name_str);
    println!("   Status: ✓ Available\n");
    
    TelemetryClient::track_event(TelemetryEvent::InstallStarted { 
        backend: backend_name_str.clone() 
    });
    
    let mut tx = Transaction::new(package, backend);
    
    println!("📦 Installing {}...", package);
    let install_result = tx.backend.install(package);
    
    match install_result {
        Ok(_) => tx.installed = true,
        Err(e) => {
            let duration = start_time.elapsed();
            TelemetryClient::track_event(TelemetryEvent::InstallFinished { 
                backend: backend_name_str, 
                success: false, 
                duration_ms: duration.as_millis() 
            });
            let _ = tx.rollback();
            return Err(e);
        }
    }
    
    // Find binary for verification and shims
    let binary_path = find_binary(package);
    
    // Integrity Verification
    if let Some(hash) = verify_hash {
        if let Some(ref path) = binary_path {
            println!("🛡️ Verifying integrity...");
            if let Err(e) = verify_file_hash(path, hash) {
                println!("   ❌ Integrity verification failed: {}", e);
                let _ = tx.rollback();
                return Err("Security check failed: Package hash mismatch!".into());
            }
            println!("   ✓ Hash verified.");
        } else {
            println!("   ⚠️ Could not locate binary for integrity verification.");
            let _ = tx.rollback();
            return Err("Security check failed: Could not locate binary to verify!".into());
        }
    }
    
    // Handle shim creation
    let config = load_config().unwrap_or_default();
    if config.behavior.create_shims {
        if let Some(ref path) = binary_path {
            match create_shim_internal(package, path, tx.backend.name()) {
                Ok(shim_path) => {
                    println!("   ✓ Created shim: {}", shim_path.display());
                    tx.shim_created = true;
                }
                Err(e) => eprintln!("   ⚠ Shim creation failed: {}", e),
            }
        } else if config.behavior.verbose {
            println!("   ℹ Could not locate binary for shim creation");
        }
    }
    
    println!("\n✓ {} installed successfully!", package);
    
    let duration = start_time.elapsed();
    TelemetryClient::track_event(TelemetryEvent::InstallFinished { 
        backend: backend_name_str, 
        success: true, 
        duration_ms: duration.as_millis() 
    });
    
    Ok(())
}

/// Update a package
fn update_package(package: &str, backend_name: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Detecting system...");
    let backend = get_backend(backend_name)?;
    
    println!("   Backend: {}", backend.name());
    println!("   Status: ✓ Available\n");
    
    println!("🔄 Updating {}...", package);
    backend.update(package)?;
    
    println!("\n✓ {} updated successfully!", package);
    Ok(())
}

/// Uninstall a package
fn uninstall_package(package: &str, backend_name: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Detecting system...");
    let backend = get_backend(backend_name)?;
    
    println!("   Backend: {}", backend.name());
    println!("   Status: ✓ Available\n");
    
    println!("🗑️ Uninstalling {}...", package);
    backend.uninstall(package)?;
    
    // Remove shim if it exists
    let mut registry = ShimRegistry::load()?;
    if registry.remove(package).is_some() {
        println!("   ✓ Removed associated shim");
        registry.save()?;
        
        let shim_dir = get_shim_dir();
        #[cfg(windows)]
        {
            let _ = std::fs::remove_file(shim_dir.join(format!("{}.cmd", package)));
            let _ = std::fs::remove_file(shim_dir.join(format!("{}.ps1", package)));
        }
        #[cfg(not(windows))]
        {
            let _ = std::fs::remove_file(shim_dir.join(package));
        }
    }
    
    println!("\n✓ {} uninstalled successfully!", package);
    Ok(())
}

/// Helper to get a backend
fn get_backend(backend_name: Option<&str>) -> Result<Box<dyn Backend>, Box<dyn std::error::Error>> {
    if let Some(name) = backend_name {
        let all = get_all_available_backends();
        all.into_iter()
            .find(|b| b.name() == name)
            .ok_or_else(|| format!("Backend '{}' not available", name).into())
    } else {
        let context = OsContext::detect();
        get_backend_for_context(&context)
    }
}

/// Find a binary in PATH
fn find_binary(name: &str) -> Option<PathBuf> {
    if cfg!(windows) {
        let output = std::process::Command::new("where")
            .arg(name)
            .output().ok()?;
        if output.status.success() {
            let s = String::from_utf8_lossy(&output.stdout);
            s.lines().next().map(PathBuf::from)
        } else {
            None
        }
    } else {
        let output = std::process::Command::new("which")
            .arg(name)
            .output().ok()?;
        if output.status.success() {
            let s = String::from_utf8_lossy(&output.stdout);
            Some(PathBuf::from(s.trim()))
        } else {
            None
        }
    }
}

/// Internal shim creation logic
fn create_shim_internal(name: &str, target: &Path, backend_name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let shim_path = shims::create_shim(name, target)?;
    let mut registry = ShimRegistry::load()?;
    registry.add(name.to_string(), target.to_path_buf(), backend_name.to_string());
    registry.save()?;
    Ok(shim_path)
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

/// Handle 1install self-installation/bootstrapping
fn handle_self_install() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Bootstrapping 1install...");
    
    // 1. Ensure shim directory exists
    let shim_dir = get_shim_dir();
    println!("   Ensuring shim directory exists: {}", shim_dir.display());
    let _ = std::fs::create_dir_all(&shim_dir);
    
    // 2. Guide user on PATH setup
    println!("\nNext steps to complete setup:");
    println!("{}", shims::get_path_instruction());
    
    println!("\n✓ 1install is ready for action!");
    Ok(())
}
