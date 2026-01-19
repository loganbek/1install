# Engineering Roadmap: MVP to v1.0.0

This roadmap outlines the development phases for https://github.com/loganbek/1install.

---

## Phase 1: The Walking Skeleton (v0.0.1-alpha)

**Objective**: Validate Context Detection and Single-Backend Passthrough.

### Deliverables

| Component       | Description                                                      |
| --------------- | ---------------------------------------------------------------- |
| CLI Skeleton    | Implement using Rust with `clap`                                 |
| ContextDetector | Identify OS (Linux/Windows/macOS) and one primary system manager |
| Basic Install   | `1i install <pkg>` passes through to native manager              |
| Manual PATH     | User manually adds `~/.local/share/1install/bin` to PATH         |

### Implementation Checklist

- [ ] Set up Rust project with Cargo
- [ ] Add `clap` for CLI parsing
- [ ] Implement `ContextDetector` struct:
  - [ ] Detect Linux distro via `/etc/os-release`
  - [ ] Detect Windows via `winget --version`
  - [ ] Detect macOS via `brew --version`
- [ ] Create `AptProvider` as first backend
- [ ] Wire up `1i install <pkg>` → `sudo apt-get install -y <pkg>`
- [ ] Create `WingetProvider` for Windows testing
- [ ] Basic error handling and user feedback

### Success Criteria

A binary that compiles and runs a simple installation (e.g., `1i install git`) on a single OS.

---

## Phase 2: The Aggregator (v0.1.0-beta)

**Objective**: Multiple Backends and Unified Search.

### Deliverables

| Component        | Description                                                     |
| ---------------- | --------------------------------------------------------------- |
| Backend Trait    | Define `Backend` trait with `search`, `install`, `is_available` |
| Multiple Drivers | Implement `apt`, `brew`, `winget`, `snap`, `npm`                |
| Parallel Search  | Async searching using `tokio`                                   |
| Unified Output   | Render results using `comfy-table`                              |

### Implementation Checklist

- [ ] Define `Backend` trait in Rust
- [ ] Implement backend drivers:
  - [ ] `AptProvider`
  - [ ] `BrewProvider`
  - [ ] `WingetProvider`
  - [ ] `SnapProvider`
  - [ ] `NpmProvider`
- [ ] Add `tokio` for async runtime
- [ ] Implement parallel backend searching
- [ ] Parse CLI output into structured data (jc-inspired)
- [ ] Render unified table with `comfy-table`
- [ ] Rank results by match quality and backend priority

### Success Criteria

`1i search <query>` returns aggregated results from multiple sources; `1i install` works for system and npm packages.

---

## Phase 3: The Hyper-Manager (v0.5.0)

**Objective**: Shims, Configuration, and Git Support.

### Deliverables

| Component              | Description                                           |
| ---------------------- | ----------------------------------------------------- |
| Shim Engine            | Create shims for installed binaries                   |
| Git Driver             | Heuristic builder for git URLs                        |
| Configuration          | `~/.config/1install/config.toml` for user preferences |
| Immediate Availability | No shell restart needed after installs                |

### Implementation Checklist

- [ ] Implement Shim Engine:
  - [ ] Detect binaries after npm/pip global installs
  - [ ] Create shell/batch shims in shim directory
  - [ ] Maintain shim registry
- [ ] Implement Git Driver:
  - [ ] Clone repository
  - [ ] Detect build system (`Cargo.toml`, `Makefile`, `package.json`)
  - [ ] Auto-build and install
- [ ] Create configuration system:
  - [ ] Parse TOML config
  - [ ] Backend priority settings
  - [ ] User preferences
- [ ] Add `1i config` command for managing settings

### Success Criteria

Seamless "install and run" without shell restarts; support for source-based installs from git.

---

## Phase 4: Production Release (v1.0.0)

**Objective**: Integrity, Conflict Resolution, and Polish.

### Deliverables

| Component           | Description                       |
| ------------------- | --------------------------------- |
| Integrity Checker   | SHA256 verification pre-install   |
| Conflict Resolution | TUI for name collisions           |
| Self-Update         | `1i self-update` command          |
| CI/CD               | Full test suite on GitHub Actions |

### Implementation Checklist

- [ ] Implement IntegrityChecker:
  - [ ] Fetch SHA256 from PyPI/NPM/etc.
  - [ ] Verify downloaded artifacts
  - [ ] Hard-abort on mismatches
- [ ] Add interactive TUI (using `ratatui`):
  - [ ] Resolve name collisions
  - [ ] Backend selection when ambiguous
- [ ] Implement self-update mechanism
- [ ] Set up GitHub Actions:
  - [ ] Build matrix (Linux/Windows/macOS)
  - [ ] Unit tests
  - [ ] Integration tests
  - [ ] Release automation

### Success Criteria

Production-ready release with security verification, polished UX, and automated releases.

---

## Timeline Estimate

| Phase                  | Duration  | Cumulative |
| ---------------------- | --------- | ---------- |
| Phase 1 (v0.0.1-alpha) | 2-3 weeks | 3 weeks    |
| Phase 2 (v0.1.0-beta)  | 3-4 weeks | 7 weeks    |
| Phase 3 (v0.5.0)       | 4-5 weeks | 12 weeks   |
| Phase 4 (v1.0.0)       | 3-4 weeks | 16 weeks   |

**Total estimated time to v1.0.0**: ~4 months
