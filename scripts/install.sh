#!/bin/sh
# 1install Unix Bootstrap Installer
set -e

# Detect OS
OS="$(uname -s)"
ARCH="$(uname -m)"

echo "🚀 Installing 1install for $OS ($ARCH)..."

# Determine download URL (placeholder for now, pointing to latest release)
# In production, this would use the GitHub API to find the latest asset
REPO="loganbek/1install"
# URL="https://github.com/$REPO/releases/latest/download/1i-linux-x86_64.tar.gz"

INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

echo "   Target directory: $INSTALL_DIR"

# Add to PATH if not already present
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo "   Adding $INSTALL_DIR to PATH..."
    SHELL_CONFIG=""
    case $SHELL in
        */zsh) SHELL_CONFIG="$HOME/.zshrc" ;;
        */bash) SHELL_CONFIG="$HOME/.bashrc" ;;
        *) SHELL_CONFIG="$HOME/.profile" ;;
    esac
    
    if [ -f "$SHELL_CONFIG" ]; then
        echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$SHELL_CONFIG"
        echo "   Updated $SHELL_CONFIG"
    fi
    export PATH="$PATH:$INSTALL_DIR"
fi

echo "✓ 1install bootstrap complete."

echo ""
echo "⚙️  Configuring environment..."
if command -v 1i >/dev/null 2>&1; then
    1i shims setup
    echo "✓ Environment configured successfully."
else
    echo "⚠ Could not find '1i' in PATH. You may need to 'source $SHELL_CONFIG'."
fi

echo ""
echo "🚀 You are ready to go! Try running: 1i search ripgrep"
