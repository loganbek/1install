# Technical Stack Recommendation

## Primary Language: Rust

Chosen for:

| Reason                         | Benefit                                 |
| ------------------------------ | --------------------------------------- |
| **Zero-cost abstractions**     | Performance without runtime overhead    |
| **Memory safety**              | Prevents entire classes of bugs         |
| **Single-binary distribution** | Critical for the Shim architecture      |
| **Cross-platform**             | First-class Windows/Linux/macOS support |
| **Strong ecosystem**           | Excellent crates for our use cases      |

## Core Dependencies

### CLI Framework: `clap` v4

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
```

**Why clap?**

- Derive macros for clean command definitions
- Automatic help generation
- Shell completions
- Subcommand support

### Async Runtime: `tokio`

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

**Why tokio?**

- Industry-standard async runtime
- Parallel backend searching
- Non-blocking I/O for API calls

### Serialization: `serde`

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
toml = "0.8"
```

**Why serde?**

- Parse JSON from npm/PyPI APIs
- Parse TOML configuration files
- Serialize shim registry

### HTTP Client: `reqwest`

```toml
[dependencies]
reqwest = { version = "0.11", features = ["json"] }
```

**Why reqwest?**

- Async HTTP client
- Built-in JSON support
- TLS support

### Table Rendering: `comfy-table`

```toml
[dependencies]
comfy-table = "7"
```

**Why comfy-table?**

- Terminal-width aware
- Unicode support
- Customizable styling

### TUI (Phase 4): `ratatui`

```toml
[dependencies]
ratatui = "0.25"
crossterm = "0.27"
```

**Why ratatui?**

- Modern TUI framework
- Cross-platform terminal support
- Rich widget library

## Project Structure

```
1install/
├── Cargo.toml
├── Cargo.lock
├── src/
│   ├── main.rs              # Entry point
│   ├── lib.rs               # Library root
│   ├── cli/
│   │   ├── mod.rs           # CLI module
│   │   ├── commands.rs      # Command definitions
│   │   └── output.rs        # Output formatting
│   ├── context/
│   │   ├── mod.rs           # Context detection
│   │   ├── os.rs            # OS detection
│   │   └── backends.rs      # Backend discovery
│   ├── backends/
│   │   ├── mod.rs           # Backend trait
│   │   ├── apt.rs           # APT provider
│   │   ├── brew.rs          # Homebrew provider
│   │   ├── winget.rs        # Winget provider
│   │   ├── npm.rs           # NPM provider
│   │   ├── pip.rs           # Pip/pipx provider
│   │   └── git.rs           # Git source provider
│   ├── search/
│   │   ├── mod.rs           # Search orchestration
│   │   ├── aggregator.rs    # Federated search
│   │   └── ranking.rs       # Result ranking
│   ├── shims/
│   │   ├── mod.rs           # Shim management
│   │   ├── generator.rs     # Shim creation
│   │   └── registry.rs      # Shim tracking
│   ├── integrity/
│   │   ├── mod.rs           # Integrity verification
│   │   └── hashes.rs        # Hash fetching/checking
│   └── config/
│       ├── mod.rs           # Configuration
│       └── defaults.rs      # Default settings
├── tests/
│   ├── integration/         # Integration tests
│   └── unit/                # Unit tests
└── docs/                    # Documentation
```

## Build Configuration

### Cargo.toml

```toml
[package]
name = "1install"
version = "0.0.1-alpha"
edition = "2021"
authors = ["Logan Bek"]
description = "Unified cross-platform package manager"
repository = "https://github.com/loganbek/1install"
license = "MIT"

[[bin]]
name = "1i"
path = "src/main.rs"

[profile.release]
lto = true
strip = true
codegen-units = 1
```

## Cross-Platform Considerations

### Conditional Compilation

```rust
#[cfg(target_os = "linux")]
mod linux {
    pub fn default_backend() -> &'static str { "apt" }
}

#[cfg(target_os = "windows")]
mod windows {
    pub fn default_backend() -> &'static str { "winget" }
}

#[cfg(target_os = "macos")]
mod macos {
    pub fn default_backend() -> &'static str { "brew" }
}
```

### CI Build Matrix

```yaml
# .github/workflows/build.yml
jobs:
  build:
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    runs-on: ${{ matrix.os }}
```
