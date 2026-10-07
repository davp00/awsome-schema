#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

OUTPUT_DIR="${COVERAGE_OUTPUT_DIR:-target/llvm-cov}"
# Soft baseline for todos 1–3 (not a 100% gate). Override with COVERAGE_FAIL_UNDER_LINES.
FAIL_UNDER="${COVERAGE_FAIL_UNDER_LINES:-}"

echo "==> cargo llvm-cov --workspace (exclude e2e)"
mkdir -p "${OUTPUT_DIR}"
cargo llvm-cov clean --workspace

ARGS=(--workspace --locked --exclude e2e --html --output-dir "${OUTPUT_DIR}")
if [[ -n "${FAIL_UNDER}" ]]; then
  case "${FAIL_UNDER}" in
    *.*) FAIL_UNDER_FLOAT="${FAIL_UNDER}" ;;
    *) FAIL_UNDER_FLOAT="${FAIL_UNDER}.0" ;;
  esac
  ARGS+=(--fail-under-lines "${FAIL_UNDER_FLOAT}")
  echo "    fail-under-lines=${FAIL_UNDER_FLOAT}"
fi

cargo llvm-cov "${ARGS[@]}"
cargo llvm-cov report --lcov --output-path "${OUTPUT_DIR}/lcov.info"
cargo llvm-cov report --summary-only

echo "Coverage report written to ${OUTPUT_DIR}/index.html"
