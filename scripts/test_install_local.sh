#!/bin/bash
set -e
cd "$(dirname "$0")/.."
mkdir -p target/debug
cat > target/debug/1i <<'EOF'
#!/bin/sh
echo dummy 1i
EOF
chmod +x target/debug/1i
echo "created:"
ls -la target/debug/1i
bash scripts/install.sh
echo "after run:"
if [ -f "$HOME/.local/bin/1i" ]; then
  ls -la "$HOME/.local/bin/1i"
  "$HOME/.local/bin/1i" || true
else
  echo "notfound"
fi
# Verify aliases
for a in 1install oneinstall; do
  if [ -e "$HOME/.local/bin/$a" ]; then
    echo "alias $a ->" $(readlink -f "$HOME/.local/bin/$a" || true)
  else
    echo "alias $a not found"
  fi
done
