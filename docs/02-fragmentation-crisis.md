# The Fragmentation Crisis and Theoretical Framework

## 1.1 The Taxonomy of Package Management Systems

To design an effective unification layer, one must dissect the distinct architectural paradigms of the target systems. 1install spans four distinct categories:

### System Package Managers

| Manager  | Platform      | Architecture             |
| -------- | ------------- | ------------------------ |
| `apt`    | Debian/Ubuntu | Debian dependency models |
| `pacman` | Arch Linux    | Binary packages with AUR |
| `winget` | Windows       | MSIX and EXE installers  |

These maintain OS integrity and require elevated privileges.

### Language-Specific Managers

| Manager | Language | Challenge                                                                                                                        |
| ------- | -------- | -------------------------------------------------------------------------------------------------------------------------------- |
| `pip`   | Python   | Installs into language-specific subdirectories, complicating `$PATH` management. PEP 668 blocks global installs on some distros. |
| `npm`   | Node.js  | Global installs in non-standard locations                                                                                        |

### Universal/Containerized Formats

| Manager | Mechanism                                                               |
| ------- | ----------------------------------------------------------------------- |
| `snap`  | Packages mount as loop devices                                          |
| `brew`  | Operates as secondary system manager in `/usr/local` or `/opt/homebrew` |

### Source Distribution

| Method | Implication                                                                                                |
| ------ | ---------------------------------------------------------------------------------------------------------- |
| `git`  | Installing from source implies cloning and building (e.g., `make`, `cargo`), lacking standardized metadata |

## 1.2 The "1install" Philosophy: The Adapter Pattern

1install adopts the **Adapter Pattern**, wrapping each manager in a translation layer that normalizes:

- **Inputs**: `search`, `install` commands
- **Outputs**: Metadata, exit codes

The system evaluates the hypergraph of available backends to solve dependency constraints across ecosystems.

### Example Flow

When a user runs `1i install python`, the system determines whether to trigger:

- `apt-get` (system) for system-wide installation
- A version manager like `mise` (user) for user-space installation

The decision is based on context and configuration.
