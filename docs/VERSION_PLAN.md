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

- [x] Integrity verification (SHA256)
- [x] Transactional rollbacks
- [x] Full lifecycle management (update/uninstall)
- [x] Shim Engine (v0.5.0)
- [x] Git Backend (v0.5.0)
- [x] Configuration System (v0.5.0)

---

## v1.1.0 — The Distributor (Next Release)

**Target**: Broad availability and seamless onboarding.

### Features

- **One-Line Installer**: Shell/PS1 installers (`curl | sh`) that handles the bootstrap process.
- **Bootstrapper**: `1i self-install` to set up initial shims and PATH.
- **Backend Expansion**: Added `pacman`, `dnf`, `snap`, `flatpak`, `cargo`, and `go` backends.
- **Deep OS Support**: Improved Linux distro detection for Arch, Fedora, and openSUSE.
- **Parallel Search v2**: Non-blocking IO for even faster results.

---

## v1.2.0 — The Global Auditor

**Target**: Conflict resolution and environmental sanitization.

### Features

- **The Doctor**: `1i doctor` to detect duplicate installs across managers (e.g., `git` installed by both apt and brew).
- **Conflict Resolution TUI**: Interactive choice of which version to use for the primary shim.
- **Environment Sanitizer**: Scans and fixes ghost binaries and broken shims.
- **🛡️ Cybersecurity Review**:
  - Security audit of transaction rollbacks and shim logic.
  - Automated dependency vulnerability scanning in CI.
  - Security hardening of the `self-install` process.

## v1.2.0 Release Protocol (The Global Auditor)

1. **Verification**: Ensure `cargo test` and `cargo audit` pass locally.
2. **Tagging**: Create a semantic version tag: `git tag -a v1.2.0 -m "Release v1.2.0 - The Global Auditor"`
3. **Push**: `git push origin v1.2.0`
4. **GitHub Actions**:
   - `CI` workflow will build multi-platform binaries and create a GitHub Release.
   - `PyPI Release` workflow will build and publish the `1install` package to PyPI.
5. **Validation**: Test `pip install 1install` and download binaries from the release page.

- **🏗️ Technical Architecture Review**:
  - Performance profiling of the `Parallel Search v2` engine.
  - Refactoring for modularity and future plugin support (v1.5.0).
- **📊 Anonymized Telemetry**:
  - Opt-in performance metrics collection.
  - Backend latency and success rate monitoring.
  - Privacy-first ID generation (no PII storage).
- **Robust Test Suite**: Integration tests running against real package managers in Docker containers.

---

## v1.5.0 — The Orchestrator

**Target**: Project-level dependency management and reproducible environments.

### Key Features

- **`.1i` Manifests**: Simple project-level files to define required tools.
  - Example: `1i setup` installs everything in the manifest.
- **Lockfile Support (`1i.lock`)**: Pin versions to ensure everyone on the team has the exact same tool versions.
- **Environment Snapshots**: Export your entire system setup (all managers) to a single portable file.
- **Dependency Graphs**: Visualise which managers are providing which tools and identify overlaps.

### Engineering Deliverables

- [ ] Support for `.1i` manifest parsing (YAML/TOML).
- [ ] `1i init` to bootstrap a project environment.
- [ ] Version pinning logic for backends that support it (NPM, Brew, Cargo).
- [ ] `1i env export/import` for portability.

---

## v2.0.0 — Enterprise Grade

- Central package policy enforcement.
- Audit logging of all installations.
- Private registry/mirroring support.

---

## Release History

| Version      | Date       | Status      |
| ------------ | ---------- | ----------- |
| v0.0.1-alpha | 2026-01-18 | ✅ Released |
| v0.1.0-beta  | 2026-01-18 | ✅ Released |
| v0.5.0       | 2026-01-18 | ✅ Released |
| v1.0.0       | 2026-01-18 | ✅ Released |
| v1.1.0       | 2026-01-19 | ✅ Released |
| v1.2.0       | 2026-01-19 | ✅ Released |
| v1.5.0       | Q1 2026    | ⏳ Planned  |

```

```
