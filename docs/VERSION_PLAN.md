# Version Release Plan

## Overview

This document outlines the planned releases for 1install, from initial alpha to production v1.0.0.

---

## v0.0.1-alpha — The Walking Skeleton

**Target**: Proof of concept with single-backend passthrough

### Features

- ✅ CLI skeleton with `1i` binary
- ✅ Basic `install` command
- ✅ OS detection (Linux/Windows/macOS)
- ✅ Single backend passthrough
- ✅ Basic error handling

### Supported Backends

| OS                    | Backend |
| --------------------- | ------- |
| Linux (Debian/Ubuntu) | apt     |
| Windows               | winget  |
| macOS                 | brew    |

### Limitations

- No search functionality
- No shim creation
- Manual PATH configuration required
- Single backend per OS

### Example Usage

```bash
# Install on Debian/Ubuntu
1i install git          # → sudo apt-get install -y git

# Install on Windows
1i install git          # → winget install --id Git.Git -e --silent

# Install on macOS
1i install git          # → brew install git
```

---

## v0.1.0-beta — The Aggregator

**Target**: Multi-backend search and install

### Features

- ✅ All v0.0.1-alpha features
- ✅ `search` command with aggregated results
- ✅ Multiple backend support
- ✅ Parallel async searching
- ✅ Unified output formatting
- ✅ Backend priority configuration

### Supported Backends

| Category  | Backends                 |
| --------- | ------------------------ |
| System    | apt, pacman, winget, dnf |
| Universal | brew, snap               |
| Language  | npm, pip/pipx            |

### New Commands

```bash
1i search <query>        # Search across all backends
1i install <pkg>         # Install from best source
1i list                  # List installed packages
```

### Backend Priority

The system selects backends in configurable priority order:

1. System package managers (apt, winget)
2. Universal managers (brew, snap)
3. Language managers (npm, pip)

---

## v0.5.0 — The Hyper-Manager

**Target**: Shims, configuration, and git support

### Features

- ✅ All v0.1.0-beta features
- ✅ Shim-based PATH management
- ✅ Immediate binary availability (no shell restart)
- ✅ Git source installation
- ✅ User configuration file
- ✅ `config` command

### Shim System

```bash
# After install, binary is immediately available
1i install ripgrep
rg --version             # Works without shell restart!
```

### Git Support

```bash
1i install https://github.com/user/repo
# Auto-detects: Cargo.toml, Makefile, package.json
# Builds and installs automatically
```

### Configuration

```toml
# ~/.config/1install/config.toml
[backends]
priority = ["brew", "apt", "snap", "npm"]

[behavior]
auto_update_shims = true
verbose = false
```

### New Commands

```bash
1i config get <key>      # Get configuration value
1i config set <key> <val># Set configuration value
1i shims list            # List all shims
1i shims refresh         # Refresh shim registry
```

---

## v1.0.0 — Production Release

**Target**: Security, polish, and stability

### Features

- ✅ All v0.5.0 features
- ✅ Integrity verification (SHA256)
- ✅ Digital signature checking
- ✅ Interactive conflict resolution TUI
- ✅ Self-update mechanism
- ✅ Full test coverage
- ✅ CI/CD automation

### Security Features

```bash
# Pre-install hash verification
1i install requests
[2/4] Verifying integrity...     ✓ SHA256 verified

# Signature verification
[3/4] Checking signature...      ✓ GPG signature valid
```

### Conflict Resolution

```
$ 1i install node

Multiple sources available for 'node':
  [1] apt     nodejs 18.17.0  (system package)
  [2] nvm     node   20.10.0  (version manager)
  [3] brew    node   21.0.0   (homebrew)

Select source [1-3] or press Enter for recommended:
```

### New Commands

```bash
1i self-update           # Update 1install itself
1i verify <pkg>          # Re-verify installed package
1i doctor                # Diagnose system issues
1i uninstall <pkg>       # Remove package and shim
```

---

## Future Versions (Post v1.0.0)

### v1.1.0 — Plugin System

- User-defined backend plugins
- Community backend contributions
- Plugin marketplace

### v1.2.0 — Project Integration

- `.1install` project files
- Team-shared configurations
- Lockfile support

### v2.0.0 — Enterprise Features

- Central package policy
- Audit logging
- Approval workflows
- Private registry support

---

## Release Schedule (Estimated)

| Version      | Target Date | Status            |
| ------------ | ----------- | ----------------- |
| v0.0.1-alpha | Week 3      | 🔄 In Development |
| v0.1.0-beta  | Week 7      | ⏳ Planned        |
| v0.5.0       | Week 12     | ⏳ Planned        |
| v1.0.0       | Week 16     | ⏳ Planned        |
