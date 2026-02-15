#!/usr/bin/env bash
set -euo pipefail

echo "Running Rust integration tests for shims..."
cargo test --test shims_integration -- --nocapture

echo "Running installer integration (local dummy)..."
bash scripts/test_install_local.sh

echo "Integration tests completed."
