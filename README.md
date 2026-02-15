# 1install (1i) 🚀

### One command to rule them all.

[![Version](https://img.shields.io/badge/version-1.0.0-blue.svg)](https://github.com/loganbek/1install)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)]()

**1install** is a high-performance, unified package meta-orchestrator. It provides a single, consistent interface for searching, installing, updating, and managing software across multiple package managers and ecosystems.

---

## ⚡ Quick Start (v1.1.0)

Get up and running in seconds. Choose the one-liner for your OS:

### Unix (Linux/macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/loganbek/1install/main/install.sh | sh
```

### Windows (PowerShell)

```powershell
iwr -useb https://raw.githubusercontent.com/loganbek/1install/main/install.ps1 | iex
```

---

## ✨ Key Features

- **🔍 Unified Federated Search**: Search across `apt`, `winget`, `brew`, `npm`, `pip`, `snap`, `flatpak`, and more—simultaneously.
- **🛡️ Integrity Verification**: Automated SHA-256 hash checking for secure installations.
- **🔄 Transactional Rollbacks**: If an installation fails, 1install rolls back changes to keep your system clean.
- **🚀 Shim Engine**: Instant binary availability without shell restarts or PATH tinkering.
- **📦 Multi-Backend Support**:
  - **System**: `apt`, `pacman`, `dnf`, `winget`, `brew`
  - **Universal**: `snap`, `flatpak`
  - **Language**: `npm`, `pip`, `cargo`, `go`
  - **Source**: Directly install from `git` repositories.

---

## 🛠️ Usage

### Search for anything

```bash
1i search ripgrep
```

### Install with a specific backend (or let 1i decide)

```bash
1i install jq
1i install node --backend npm
```

### Secure installation (v1.0.0+)

```bash
1i install ripgrep --verify <SHA256_HASH>
```

### Manage Lifecycle

```bash
1i update 1i
1i uninstall git
```

### System Health

```bash
1i doctor    # Detect conflicts and broken shims
```

---

## 🏗️ Architecture

1install is built in **Rust** for maximum speed and safety. It uses the **Adapter Pattern** to wrap existing package managers, providing a normalized data model and transactional execution.

For detailed architectural specs, see the [docs/](file:///c:/Users/logan/1install/docs/) directory.

---

## ☕ Support & Donations

If 1install has saved you time and improved your workflow, consider supporting its future development! Your contributions help recoup server costs and maintenance time.

- **PayPal**: [Donate via PayPal](mailto:logan@bekconsulting.info) (logan@bekconsulting.info)
<!-- - **Bitcoin (BTC)**: `bc1q7lzkzlnv8zvz8vzv8vzv8vzv8vzv8vzv8vzv8v` -->
- **Ethereum (ETH)**: `loganbek.eth`
<!-- - **Monero (XMR)**: `44AFFq5kSiGBoZ4NMD2AsLQsXYRIAnT188A019bA188A019bA188A019bA188A019bA188A019bA188A0` -->

---

## 📊 Telemetry & Active Users

To help us prioritize features and understand our impact, 1install includes anonymized, privacy-first telemetry. We track:

- Total number of active users (anonymized UUID)
- Command success/failure rates
- Backend performance metrics

**Telemetry is Opt-Out.** If you prefer not to share this data, you can disable it with:

```bash
1i config set telemetry.enabled false
```

---

## 🖋️ Authors

- **Logan Bek**
- **Antigravity** (AI Co-Author)
- **Claude Opus** (Architectural Planning)

---

## ⚖️ License

Distributed under the MIT License. See `LICENSE` for more information.

## TODO/TASKS
## Installer notes

- The one-line bootstrap installer (`scripts/install.sh`) installs the `1i` binary into the invoking user's `~/.local/bin` directory and will add that directory to the user's shell config if needed. It also handles being run under `sudo` (it targets the original user via `SUDO_USER`).
- For developer convenience, the installer will detect a locally-built `1i` binary at `./target/release/1i` or `./target/debug/1i` and copy it into `~/.local/bin` so the one-liner can be used during development.

If you want to reproduce the developer flow locally, build with `cargo build` and then run the bootstrap script:

```bash
cargo build --release   # optional
bash scripts/install.sh
```

These changes aim to keep the oneliner experience simple and reliable: the installer will place the binary where common user-local binaries live and try not to require a manual PATH edit.

## Simple examples (install common tools)

Once `1i` is installed and on your PATH, installing common developer tooling is as easy as:

```bash
# Install Node.js (uses the appropriate backend for your OS)
1i install node

# Install npm (language-level package manager)
1i install npm

# Install pip (Python package installer)
1i install pip
```

Notes:
- Backend selection is automatic where possible; use `--backend` to force a specific backend (for example `--backend apt` on Debian/Ubuntu).
- If a tool is provided by multiple backends, `1i` will prefer the system-configured priority or the most appropriate backend for your OS.

## Developer: run tests and integration checks

To run the unit and integration tests locally:

```bash
cargo test
scripts/tests/run_integration_tests.sh
```

If you want to test the oneliner against a locally-built `1i`, build and then run the installer which will pick up the local binary:

```bash
cargo build --release
bash scripts/install.sh
```

## Output styling (future)

We plan to introduce rich, colorized, and per-backend output styling (tables, icons, and contextual colors) in a future release. The codebase already contains a basic table renderer for `search` results; upcoming work will centralize styling and provide a runtime feature flag to enable/disable color for CI environments.

