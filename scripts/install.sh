#!/usr/bin/env bash
# 1install Unix Bootstrap Installer
set -euo pipefail

# Detect OS
OS="$(uname -s)"
ARCH="$(uname -m)"

echo "🚀 Installing 1install for $OS ($ARCH)..."

# Determine download URL (placeholder for now, pointing to latest release)
# In production, this would use the GitHub API to find the latest asset
REPO="loganbek/1install"

# If running under sudo, prefer the original user (SUDO_USER) home
if [ -n "${SUDO_USER-}" ] && [ "${SUDO_USER-}" != "root" ]; then
    TARGET_USER="$SUDO_USER"
    TARGET_HOME="$(eval echo "~$SUDO_USER")"
else
    TARGET_USER="$(id -un)"
    TARGET_HOME="$HOME"
fi

INSTALL_DIR="$TARGET_HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

# Ensure ownership when run as root so the user can execute files there
if [ "$(id -u)" -eq 0 ]; then
    chown -R "$TARGET_USER":"$TARGET_USER" "$TARGET_HOME/.local" 2>/dev/null || true
fi

echo "   Target directory: $INSTALL_DIR"

# If there's a locally built binary (from cargo build), install it directly for a one-line developer install
LOCAL_BIN_CANDIDATES=("./target/release/1i" "./target/debug/1i")
for candidate in "${LOCAL_BIN_CANDIDATES[@]}"; do
    if [ -f "$candidate" ]; then
        echo "   Found local binary: $candidate — installing to $INSTALL_DIR"
        cp "$candidate" "$INSTALL_DIR/1i"
        chmod 755 "$INSTALL_DIR/1i"
        # Ensure ownership when run under sudo
        if [ -n "${SUDO_USER-}" ] && [ "${SUDO_USER-}" != "root" ]; then
            chown "$SUDO_USER":"$SUDO_USER" "$INSTALL_DIR/1i" 2>/dev/null || true
        fi
        # Create convenient aliases (symlinks) on Unix
        if [ "$(uname -s)" != "" ] && [ "$(expr substr $(uname -s) 1 5)" != "MINGW" ]; then
            for alias in 1install oneinstall; do
                alias_path="$INSTALL_DIR/$alias"
                if [ -e "$alias_path" ]; then
                    rm -f "$alias_path" || true
                fi
                ln -s "1i" "$alias_path" || true
                chown -h "$TARGET_USER":"$TARGET_USER" "$alias_path" 2>/dev/null || true
            done
        fi
        break
    fi
done

# Add to PATH in the user's shell config if not already present
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo "   Adding $INSTALL_DIR to PATH for user $TARGET_USER..."

    # Try to detect the user's login shell from /etc/passwd, fallback to $SHELL
    USER_SHELL="$(getent passwd "$TARGET_USER" 2>/dev/null | cut -d: -f7 || echo "$SHELL")"
    case "$USER_SHELL" in
        */zsh) SHELL_CONFIG="$TARGET_HOME/.zshrc" ;;
        */bash) SHELL_CONFIG="$TARGET_HOME/.bashrc" ;;
        *) SHELL_CONFIG="$TARGET_HOME/.profile" ;;
    esac

    # Only append if the config doesn't already contain the path
    if [ -f "$SHELL_CONFIG" ]; then
        if ! grep -Fq "$INSTALL_DIR" "$SHELL_CONFIG"; then
            echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$SHELL_CONFIG"
            echo "   Updated $SHELL_CONFIG"
        else
            echo "   $SHELL_CONFIG already updates PATH"
        fi
    else
        # Create the config file with the PATH addition
        echo "export PATH=\"\$PATH:$INSTALL_DIR\"" > "$SHELL_CONFIG"
        chown "$TARGET_USER":"$TARGET_USER" "$SHELL_CONFIG" 2>/dev/null || true
        echo "   Created $SHELL_CONFIG"
    fi

    # Update current environment (best-effort; won't affect parent shell)
    export PATH="$PATH:$INSTALL_DIR"
fi

echo "✓ 1install bootstrap complete."

echo
echo "⚙️  Configuring environment..."
if command -v 1i >/dev/null 2>&1; then
    1i shims setup
    echo "✓ Environment configured successfully."
else
    echo "⚠ Could not find '1i' in PATH. You may need to 'source $SHELL_CONFIG' or re-open your shell."
fi

echo
echo "🚀 You are ready to go! Try running: 1i search ripgrep"
