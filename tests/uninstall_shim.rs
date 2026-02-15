use std::env;
use tempfile::tempdir;
use std::path::PathBuf;

#[test]
fn uninstall_removes_shim_and_registry_entry() -> Result<(), Box<dyn std::error::Error>> {
    // Use a temporary HOME so registry and shim dir are isolated
    let td = tempdir()?;
    env::set_var("HOME", td.path());

    // Prepare fake binary target
    let bin_dir = td.path().join(".local").join("bin");
    std::fs::create_dir_all(&bin_dir)?;
    let target = bin_dir.join("fakebin");
    std::fs::write(&target, b"#!/bin/sh\necho fake\n")?;
    #[cfg(unix)]
    {
        let mut perms = std::fs::metadata(&target)?.permissions();
        use std::os::unix::fs::PermissionsExt;
        perms.set_mode(0o755);
        std::fs::set_permissions(&target, perms)?;
    }

    // Create shim pointing to the fake target
    let shim_path = oneinstall::shims::create_shim("fakebin", &target)?;
    assert!(shim_path.exists());

    // Register in the ShimRegistry
    let mut registry = oneinstall::shims::ShimRegistry::load()?;
    registry.add("fakebin".to_string(), target.clone(), "test-backend".to_string());
    registry.save()?;

    // Ensure registry has entry
    let registry = oneinstall::shims::ShimRegistry::load()?;
    assert!(registry.get("fakebin").is_some());

    // Now remove shim using helper
    let removed = oneinstall::shims::remove_shim("fakebin")?;
    assert!(removed);

    // Remove from registry and save
    let mut registry = oneinstall::shims::ShimRegistry::load()?;
    assert!(registry.remove("fakebin").is_some());
    registry.save()?;

    // Verify physical file gone and registry empty
    assert!(!shim_path.exists());
    let registry = oneinstall::shims::ShimRegistry::load()?;
    assert!(registry.is_empty());

    Ok(())
}
