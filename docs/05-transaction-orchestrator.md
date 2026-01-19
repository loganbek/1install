# Subsystem III: The Transaction Orchestrator

The core user requirement: **"I just want it added"** — drives the transactional layer.

## 4.1 Command Translation

Each backend requires specific flags for non-interactive, zero-touch installation:

### APT (Debian/Ubuntu)

```bash
DEBIAN_FRONTEND=noninteractive apt-get install -y <package>
```

- `DEBIAN_FRONTEND=noninteractive` - Suppresses prompts
- `-y` - Assumes "yes" to all prompts

### Pacman (Arch Linux)

```bash
pacman -S --noconfirm <package>
```

- `--noconfirm` - Bypasses all confirmation prompts

### Winget (Windows)

```powershell
winget install --id <ID> -e --silent --accept-source-agreements --accept-package-agreements
```

- `--id <ID>` - Uses exact package identifier
- `-e` - Exact match
- `--silent` - Silent installation
- `--accept-*-agreements` - Auto-accept licenses

### Brew (macOS/Linux)

```bash
brew install <package>
```

Brew is already non-interactive by default.

### Pip (Python)

```bash
# Preferred: Use pipx for global tool installation
pipx install <package>

# Fallback: Direct pip with safety flags
pip install --user <package>

# Last resort (if explicitly configured)
pip install --break-system-packages <package>
```

### NPM (Node.js)

```bash
npm install -g <package>
```

### Git (Source)

```bash
git clone <url>
cd <repo>
# Heuristic build detection (see Phase 3)
```

## 4.2 Transaction Model

Each installation is wrapped in a transaction:

```rust
struct Transaction {
    package: String,
    backend: Box<dyn Backend>,
    status: TransactionStatus,
    started_at: DateTime<Utc>,
    completed_at: Option<DateTime<Utc>>,
}

enum TransactionStatus {
    Pending,
    InProgress,
    Completed,
    Failed(String),
    RolledBack,
}
```

### Rollback Support

For backends that support it, failed installations trigger rollback:

```rust
impl Transaction {
    fn rollback(&mut self) -> Result<(), Error> {
        match self.backend.uninstall(&self.package) {
            Ok(_) => {
                self.status = TransactionStatus::RolledBack;
                Ok(())
            }
            Err(e) => Err(e)
        }
    }
}
```

## 4.3 Progress Reporting

```
$ 1i install ripgrep

[1/3] Detecting backends...     ✓ apt, snap, cargo available
[2/3] Selecting best source...  ✓ Using apt (system priority)
[3/3] Installing ripgrep...     ✓ Installed ripgrep 13.0.0

✓ ripgrep installed successfully
  Binary location: /usr/bin/rg
  Shim created:    ~/.local/share/1install/shims/rg
```
