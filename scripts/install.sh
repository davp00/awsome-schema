#!/usr/bin/env bash
# Install the awesome-schema CLI binary from GitHub Releases.
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/davp00/awsome-schema/main/scripts/install.sh | sh
#   VERSION=0.1.0-alpha.1 INSTALL_DIR=$HOME/.local/bin ./scripts/install.sh
set -euo pipefail

REPO="davp00/awsome-schema"
INSTALL_DIR="${INSTALL_DIR:-${HOME}/.local/bin}"
BIN_NAME="awesome-schema"

rust_target() {
  local os arch
  os="$(uname -s | tr '[:upper:]' '[:lower:]')"
  arch="$(uname -m)"
  case "${os}-${arch}" in
    linux-x86_64|linux-amd64) echo "x86_64-unknown-linux-gnu" ;;
    darwin-arm64|darwin-aarch64) echo "aarch64-apple-darwin" ;;
    darwin-x86_64) echo "x86_64-apple-darwin" ;;
    mingw*|msys*|cygwin*)
      echo "windows is not supported by this shell installer; download the .exe from GitHub Releases" >&2
      exit 1
      ;;
    *)
      echo "unsupported platform: ${os} ${arch}" >&2
      exit 1
      ;;
  esac
}

asset_name() {
  local target="$1"
  echo "awesome-schema-${target}"
}

normalize_version() {
  local v="$1"
  v="${v#v}"
  echo "$v"
}

latest_stable_tag() {
  # Prefer the newest non-canary, non-test release tag (v*).
  local tag
  tag="$(
    curl -fsSL "https://api.github.com/repos/${REPO}/releases" \
      | grep -oE '"tag_name":[[:space:]]*"[^"]+"' \
      | sed -E 's/.*"([^"]+)".*/\1/' \
      | grep -E '^v' \
      | grep -Ev 'canary|test' \
      | head -n 1 || true
  )"
  if [ -z "$tag" ]; then
    echo "could not resolve latest release tag from GitHub" >&2
    exit 1
  fi
  echo "$tag"
}

main() {
  local target version tag url dest tmp
  target="$(rust_target)"

  if [ -n "${VERSION:-}" ]; then
    version="$(normalize_version "$VERSION")"
    tag="v${version}"
  else
    tag="$(latest_stable_tag)"
    version="$(normalize_version "$tag")"
  fi

  url="https://github.com/${REPO}/releases/download/${tag}/$(asset_name "$target")"
  dest="${INSTALL_DIR}/${BIN_NAME}"
  tmp="$(mktemp)"

  echo "Downloading awesome-schema ${version} (${target})..."
  curl -fsSL "$url" -o "$tmp"
  mkdir -p "$INSTALL_DIR"
  mv "$tmp" "$dest"
  chmod 755 "$dest"

  echo "Installed ${dest}"
  case ":${PATH}:" in
    *":${INSTALL_DIR}:"*) ;;
    *)
      echo "Add ${INSTALL_DIR} to your PATH, then run: awesome-schema --version"
      ;;
  esac
  echo "Upgrade later with: awesome-schema upgrade"
}

main "$@"
