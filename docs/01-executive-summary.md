# Executive Summary

The contemporary software distribution landscape is characterized by a fragmented ecosystem of package managers, each operating within isolated silos of operating system constraints, language runtimes, and distribution philosophies.

## The Problem

**1install** (binary alias `1i`) addresses a critical inefficiency in modern workflows—a unified install utility that amalgamates disparate systems like:

- winget, chocolatey (Windows)
- snap, apt, dpkg (Debian-based Linux)
- pacman (Arch-based Linux)
- brew (macOS/Linux)
- pip, npm (Language-specific)
- git (Source distribution)

## The Solution

Unlike traditional package managers, **1install** functions as a high-level orchestration layer (a "Hyper-Manager") that leverages existing tools as backend providers.

### Key Features

1. **Unified Search Interface** - Search across all package managers with a single command
2. **Intelligent Backend Selection** - Automatically selects the best package manager for the context
3. **Rigorous Path/Integrity Management** - Shim-based PATH management and integrity verification

By synthesizing the "shim" architecture of tools like mise with the federated metadata aggregation of Repology, 1install provides a seamless "install and forget" experience.

## Repository

- **GitHub**: https://github.com/loganbek/1install
- **Binary**: `1i`
