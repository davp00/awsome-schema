#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
HOOKS_DIR="$ROOT/.git/hooks"
HOOK_SRC="$ROOT/scripts/hooks/pre-commit"
HOOK_DST="$HOOKS_DIR/pre-commit"

if [[ ! -d "$ROOT/.git" ]]; then
  echo "error: not a git repository ($ROOT)" >&2
  exit 1
fi

mkdir -p "$HOOKS_DIR"
ln -sf "../../scripts/hooks/pre-commit" "$HOOK_DST"
chmod +x "$HOOK_SRC" "$ROOT/scripts/check.sh"

echo "Installed pre-commit hook -> $HOOK_DST"
