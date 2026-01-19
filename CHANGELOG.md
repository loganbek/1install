# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned

- Advanced AI-driven backend selection
- Universal build-system generator

---

## [1.0.0] - 2026-01-18

### Added

- **Integrity Verification**: SHA-256 hash checking for all installations via `--verify <HASH>`.
- **Transaction Orchestrator**: Atomic installation with automatic rollback on failure (cleanly uninstalls failed pkgs and removes shims).
- **Package Lifecycle**: New `update` and `uninstall` commands supporting all backends.
- **Shim Cleanup**: `uninstall` now automatically cleans up associated shims from PATH.
- **Security Alerts**: Immediate failure and rollback if a package's binary hash does not match expectations.

### Changed

- Unified backend selection logic in `lib.rs`.
- Bumped version to 1.0.0.

### Technical

- Added `sha2`, `hex`, and `thiserror` dependencies.

---

## [0.5.0] - 2026-01-18

### Added

- **Shim Engine**: Immediate binary availability via cross-platform wrappers (.cmd/.ps1 on Windows, shell scripts on Unix).
- **Configuration System**: TOML-based settings in `~/.config/1install/config.toml` with dot-notation get/set.
- **Git Backend**: Clone and build from source for Rust (Cargo), Node.js (NPM), and projects with Makefiles.
- **`1i config` command**: Manage global settings (priority, disabled backends, shim defaults).
- **`1i shims` command**: List registered shims and show PATH setup instructions.
- **Shim Registry**: Persistent tracking of all shims and their source backends.

### Technical

- Added `toml`, `dirs`, and `chrono` dependencies.
- Automatic binary location and shim generation after successful installs.

---

## [0.1.0-beta] - 2026-01-18

### Added

- **Federated search** across multiple package managers with `1i search <query>`
- **NPM backend** with npm search and `npm install -g` support
- **Pip/pipx backend** with PyPI index lookup (prefers pipx for safety)
- **`1i backends` command** to list available package managers
- **`--backend` flag** for `1i install` to specify a package manager
- **Formatted table output** using comfy-table with colored backend names
- **Result ranking algorithm** prioritizing exact matches and backend priority

### Changed

- Extended `Backend` trait with `search()` method
- Made backends thread-safe with `Send + Sync` bounds

### Dependencies

- Added `tokio` for async runtime
- Added `comfy-table` for table formatting
- Added `serde` and `serde_json` for JSON parsing

---

## [0.0.1-alpha] - 2026-01-18

### Added

- Initial release - "Walking Skeleton" MVP
- **CLI skeleton** with `clap` v4 derive macros
- **`1i install <package>` command** with automatic backend selection
- **OS context detection** for Windows, Linux (with distro), and macOS
- **Winget backend** for Windows with auto-accept agreements
- **APT backend** for Debian/Ubuntu with automatic sudo and DEBIAN_FRONTEND
- **Homebrew backend** for macOS
- **`--help` and `--version` flags**

### Technical

- Rust project with optimized release profile (LTO, strip)
- Unit tests for CLI, OS detection, and backend implementations

---

[unreleased]: https://github.com/loganbek/1install/compare/v0.1.0-beta...HEAD
[0.1.0-beta]: https://github.com/loganbek/1install/compare/v0.0.1-alpha...v0.1.0-beta
[0.0.1-alpha]: https://github.com/loganbek/1install/releases/tag/v0.0.1-alpha
