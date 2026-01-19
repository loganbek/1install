# Subsystem IV: Path Management via Shims

To fulfill **"added to the path"**, 1install avoids modifying shell configs (`.bashrc`, `.zshrc`) for every tool. Instead, it uses **Shims**.

## The Problem with Traditional PATH Management

Traditional approaches have significant drawbacks:

1. **Shell Config Bloat**: Adding PATH entries for each tool clutters configs
2. **Restart Required**: Changes require shell restart or manual sourcing
3. **Version Conflicts**: Multiple versions of the same tool collide
4. **Platform Inconsistency**: Different shells need different configs

## The Shim Solution

### Single PATH Entry

Only one directory is added to `$PATH`:

```bash
# Added once during 1install setup
export PATH="$HOME/.local/share/1install/shims:$PATH"
```

### Shim Generation

Upon installation, 1install creates a small executable shim:

```rust
// shim_generator.rs
fn create_shim(binary_name: &str, actual_path: &Path) -> Result<(), Error> {
    let shim_path = get_shim_dir().join(binary_name);

    #[cfg(unix)]
    {
        let script = format!(r#"#!/bin/sh
exec "{}" "$@"
"#, actual_path.display());
        fs::write(&shim_path, script)?;
        fs::set_permissions(&shim_path, Permissions::from_mode(0o755))?;
    }

    #[cfg(windows)]
    {
        // Create batch file shim
        let script = format!(r#"@echo off
"{}" %*
"#, actual_path.display());
        fs::write(shim_path.with_extension("cmd"), script)?;
    }

    Ok(())
}
```

### Directory Structure

```
~/.local/share/1install/
├── shims/           # All shims live here
│   ├── node         # → /usr/bin/node
│   ├── python       # → ~/.pyenv/shims/python
│   ├── rg           # → /usr/bin/rg
│   └── code         # → /usr/bin/code
├── bin/             # 1install's own binaries
│   └── 1i           # The main binary
└── cache/           # Download cache
```

## Shim Types

### Simple Passthrough Shim

For straightforward binary forwarding:

```bash
#!/bin/sh
exec "/actual/path/to/binary" "$@"
```

### Version-Aware Shim

For tools with multiple versions:

```bash
#!/bin/sh
VERSION="${TOOL_VERSION:-default}"
exec "$HOME/.local/share/1install/versions/tool/$VERSION/bin/tool" "$@"
```

### Intelligent Shim (Rust-based)

For complex routing logic:

```rust
// Compiled shim that can:
// - Check .tool-versions files
// - Select version based on project context
// - Log usage for analytics
// - Perform pre-execution checks
```

## Benefits

| Feature              | Traditional PATH | Shim Approach |
| -------------------- | ---------------- | ------------- |
| Shell restart needed | Yes              | No            |
| Config file changes  | Per tool         | Once          |
| Version switching    | Manual           | Automatic     |
| Cross-platform       | Complex          | Unified       |
| Transparency         | Low              | High          |

## Implementation Notes

### Immediate Availability

Shims ensure newly installed tools are immediately available without shell restarts:

```
$ 1i install ripgrep
✓ ripgrep installed

$ rg --version  # Works immediately!
ripgrep 13.0.0
```

### Shim Registry

1install maintains a registry of all shims:

```toml
# ~/.local/share/1install/shims.toml
[[shims]]
name = "rg"
target = "/usr/bin/rg"
installed_by = "apt"
installed_at = "2024-01-15T10:30:00Z"

[[shims]]
name = "node"
target = "/usr/local/bin/node"
installed_by = "brew"
installed_at = "2024-01-14T14:22:00Z"
```
