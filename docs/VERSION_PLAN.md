# Version Release Plan

This document lists planned releases and the feature sets for each version. Each feature has a status: not started, started, or finished.

Status legend:

- [ ] not started
- [-] started
- [x] finished

---

## Recent changes

- Installer behavior updated: the `scripts/install.sh` bootstrapper now installs the `1i` binary into the invoking user's `~/.local/bin` and will update the user's shell config to add that directory to `PATH` if necessary. It also detects a locally-built binary at `./target/release/1i` or `./target/debug/1i` and copies it into `~/.local/bin` for quick developer one-liner installs. Running under `sudo` will target the original user (`SUDO_USER`).

These small changes were made to preserve the one-liner promise: `curl | sh` should be enough for end users and developers alike to get `1i` on their PATH with minimal manual steps.

## v0.0.1-alpha — The Walking Skeleton

Target: Proof of concept with single-backend passthrough

- [x] CLI skeleton with `1i` binary — finished
- [x] Basic `install` command — finished
- [x] OS detection (Linux/Windows/macOS) — finished
- [x] Single backend passthrough — finished
- [x] Basic error handling — finished

Supported backends (initial): apt (Linux), winget (Windows), brew (macOS)

---

## v0.1.0-beta — The Aggregator

Target: Multi-backend search and install

- [x] Aggregated `search` command — finished
- [x] Multiple backend support (core set) — finished
- [x] Parallel async searching — finished
- [x] Unified output formatting — finished
- [x] Backend priority configuration — finished

---

## v0.5.0 — The Hyper-Manager

Target: Shims, configuration, and git support

- [-] Shim Engine (create shims, shim registry) — started
- [-] Immediate binary availability without shell restart — started
- [-] Git source installation (clone, build heuristics) — started
- [x] User configuration file + `config` command — finished
- [-] `shims` subcommands (`list`, `refresh`) — started

Notes: core shim code exists (`src/shims`), but some behavior requires verification and additional tests. Installer bootstrapping now places `1i` into `~/.local/bin` and supports detecting a local `target/*/1i` build for developer one-liners.

---

## v1.0.0 — Production Release

Target: Security, polish, and stability

- [-] Integrity verification (SHA256) — started
- [-] Digital signature checking — started
- [-] Interactive conflict resolution TUI — started
- [-] Self-update mechanism — started
- [-] Full test coverage (unit + integration) — started
- [-] CI/CD automation — started

---

## Future / Post v1.0.0

### v1.1.0 — Plugin System

- [ ] User-defined backend plugins — not started
- [ ] Community backend contributions / marketplace — not started

### v1.2.0 — Global Auditor

- [ ] Comprehensive integration test suite — not started
- [ ] Conflict detection UI / `1i doctor` — not started
- [ ] Telemetry / performance profiling — not started

### Uninstall & Cleanup (future)

- [ ] Working uninstall command and cleanup semantics — not started
- [ ] Ensure uninstall removes shims/aliases and restores PATH entries when appropriate — not started
- [ ] Add tests for uninstall safety and rollback — not started

### Output & UX polish (future)

- [ ] Colorized and prettified CLI output (per-backend styling, icons, and table themes) — not started
- [ ] Global runtime flag to disable color for CI / logs — not started
- [ ] Consistent UX patterns for all subcommands (success/warn/error verbs) — not started

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

### Key Features

- [ ] One-Line Installer: Shell/PS1 installers (`curl | sh`) that handle the bootstrap process.
- [ ] Bootstrapper: `1i self-install` to set up initial shims and PATH.
- [ ] Backend Expansion: Added `pacman`, `dnf`, `snap`, `flatpak`, `cargo`, and `go` backends.
- [ ] Deep OS Support: Improved Linux distro detection for Arch, Fedora, and openSUSE.
- [ ] Parallel Search v2: Non-blocking IO for even faster results.

---

## v1.2.0 — The Global Auditor

**Target**: Conflict resolution and environmental sanitization.

### Core Capabilities

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

### Capabilities

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

```md
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
