# Subsystem I: The Context Detection Engine

The Context Detection Engine is responsible for determining the operating environment and available package managers.

## 2.1 Operating System and Distro Heuristics

The detection logic profiles the host environment:

### Linux Detection

```rust
// Parses /etc/os-release to identify:
// - ID (e.g., "arch", "ubuntu", "fedora")
// - VERSION_ID
// - ID_LIKE (for derivative distros)
```

Based on the ID, selects the primary backend:

- `arch` → `pacman`
- `ubuntu`, `debian` → `apt`
- `fedora` → `dnf`

### Windows Detection

```rust
// Checks for package managers via:
// - Registry keys
// - CLI execution (winget --version)
// - File existence checks
```

Supported Windows managers:

- `winget` (preferred)
- `choco` (Chocolatey)
- `scoop`

### macOS Detection

```rust
// Verifies brew presence in:
// - /opt/homebrew/bin (Apple Silicon)
// - /usr/local/bin (Intel)
```

## 2.2 Dynamic Backend Registration

1install iterates through a registry of "Backend Providers":

```rust
trait Backend {
    fn name(&self) -> &str;
    fn is_available(&self) -> bool;
    fn search(&self, query: &str) -> Vec<PackageResult>;
    fn install(&self, package: &str) -> Result<(), InstallError>;
}
```

### Provider Examples

| Provider         | Liveness Check     |
| ---------------- | ------------------ |
| `AptProvider`    | `apt --version`    |
| `NpmProvider`    | `npm --version`    |
| `WingetProvider` | `winget --version` |

The engine builds an `active_backends` list based on successful liveness checks.

### Lockfile Detection

Integrates logic similar to `package-manager-detector` to infer preferred managers from lockfiles:

| Lockfile            | Inferred Manager |
| ------------------- | ---------------- |
| `package-lock.json` | `npm`            |
| `yarn.lock`         | `yarn`           |
| `Cargo.lock`        | `cargo`          |
| `Pipfile.lock`      | `pipenv`         |

## 2.3 Privilege Escalation Strategy

Different backends require different privilege levels:

### Root Required (APT/Pacman)

```rust
// Automatically prepends sudo if os.geteuid() != 0
if !is_root() && backend.requires_root() {
    command = format!("sudo {}", command);
}
```

### UserSpace (Brew/Pip/Npm)

```rust
// Enforces non-root execution to prevent permission drift
if is_root() && !backend.allows_root() {
    return Err(Error::RootNotAllowed);
}
```

### Hybrid (Winget)

Requests UAC elevation only when the package manifest demands it.
