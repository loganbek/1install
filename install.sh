#!/usr/bin/env bash
set -euo pipefail

# Top-level installer wrapper.
# If the repository is cloned and `scripts/install.sh` exists, run it.
# Otherwise fetch the bootstrapper from raw.githubusercontent and run it.

SCRIPT_LOCAL="scripts/install.sh"
RAW_URL="https://raw.githubusercontent.com/loganbek/1install/main/scripts/install.sh"

if [ -f "$SCRIPT_LOCAL" ]; then
    echo "Running local $SCRIPT_LOCAL"
    exec bash "$SCRIPT_LOCAL" "$@"
else
    echo "Fetching installer from $RAW_URL"
    if command -v curl >/dev/null 2>&1; then
        curl -fsSL "$RAW_URL" | bash -s -- "$@"
    elif command -v wget >/dev/null 2>&1; then
        wget -qO- "$RAW_URL" | bash -s -- "$@"
    else
        echo "Error: curl or wget is required to download the installer." >&2
        exit 1
    fi
fi
