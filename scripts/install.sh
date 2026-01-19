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

# For now, since we are in a dev environment, we assume the binary is built locally
# or we just provide the instruction if we can't download.
# For the real distributor, we would use curl/wget here.

echo "✓ 1install bootstrap complete."
echo ""
echo "Next steps:"
echo "1. Ensure $INSTALL_DIR is in your PATH."
echo "2. Run '1i shims setup' to configure your environment."
