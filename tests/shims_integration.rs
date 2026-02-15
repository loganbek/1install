use std::env;
use std::fs;

use tempfile::tempdir;

use oneinstall::shims::create_shim;

#[test]
fn integration_create_shim_unix() -> Result<(), Box<dyn std::error::Error>> {
    // Use a temporary HOME so we don't touch the real user directory
    let tmp = tempdir()?;
    env::set_var("HOME", tmp.path());

    // Create a fake target binary
    let target = tmp.path().join("fake-bin");
    fs::write(&target, "#!/bin/sh\necho hello")?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut p = fs::metadata(&target)?.permissions();
        p.set_mode(0o755);
        fs::set_permissions(&target, p)?;
    }

    let shim_path = create_shim("fakebin", &target)?;
    assert!(shim_path.exists(), "shim file should exist: {:?}", shim_path);

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = fs::metadata(&shim_path)?.permissions().mode();
        assert!(mode & 0o111 != 0, "shim should be executable");
    }

    Ok(())
}
