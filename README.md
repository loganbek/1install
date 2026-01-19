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
curl -fsSL https://raw.githubusercontent.com/loganbek/1install/main/scripts/install.sh | sh
```

### Windows (PowerShell)

```powershell
iwr -useb https://raw.githubusercontent.com/loganbek/1install/main/scripts/install.ps1 | iex
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
