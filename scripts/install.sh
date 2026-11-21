#!/usr/bin/env bash
# Install the awesome-schema CLI binary from GitHub Releases.
# Usage (POSIX-safe; works when piped to sh or bash):
#   curl -fsSL https://raw.githubusercontent.com/davp00/awsome-schema/main/scripts/install.sh | sh
#   VERSION=0.1.0-alpha.1 INSTALL_DIR=$HOME/.local/bin ./scripts/install.sh
set -eu
# pipefail when the shell supports it (bash); plain sh may not.
(set -o pipefail) 2>/dev/null && set -o pipefail

REPO="davp00/awsome-schema"
INSTALL_DIR="${INSTALL_DIR:-${HOME}/.local/bin}"
BIN_NAME="awesome-schema"
UA="awesome-schema-install/0.1 (+https://github.com/${REPO})"

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

# GET url → stdout. Fails with a clear message on non-2xx (avoids silent empty pipes).
gh_get() {
  local url="$1"
  local tmp http
  tmp="$(mktemp)"
  http="$(
    curl -sS -A "$UA" -H "Accept: application/vnd.github+json" \
      -o "$tmp" -w "%{http_code}" "$url" || true
  )"
  if [ "$http" != "200" ]; then
    rm -f "$tmp"
    echo "GitHub API ${url} returned HTTP ${http:-error}" >&2
    return 1
  fi
  cat "$tmp"
  rm -f "$tmp"
}

# Extract tag_name values from a GitHub releases JSON array / object.
extract_tags() {
  grep -oE '"tag_name"[[:space:]]*:[[:space:]]*"[^"]+"' \
    | sed -E 's/.*"([^"]+)".*/\1/'
}

is_usable_tag() {
  local tag="$1"
  case "$tag" in
    v*) ;;
    *) return 1 ;;
  esac
  case "$tag" in
    *canary*|*test*) return 1 ;;
  esac
  return 0
}

# First usable tag from stdin (one tag per line). Avoids bash process
# substitution so `curl … | sh` works on macOS /dash.
first_usable_tag() {
  local tag
  while IFS= read -r tag; do
    if is_usable_tag "$tag"; then
      echo "$tag"
      return 0
    fi
  done
  return 1
}

latest_stable_tag() {
  local body tag

  # 1) /releases (includes prereleases such as v0.1.0-alpha.1)
  if body="$(gh_get "https://api.github.com/repos/${REPO}/releases?per_page=30")"; then
    if tag="$(printf '%s\n' "$body" | extract_tags | first_usable_tag)"; then
      echo "$tag"
      return 0
    fi
  fi

  # 2) /releases/latest (non-prerelease only — may 404 while still on alpha)
  if body="$(gh_get "https://api.github.com/repos/${REPO}/releases/latest" 2>/dev/null)"; then
    tag="$(printf '%s\n' "$body" | extract_tags | head -n 1 || true)"
    if is_usable_tag "$tag"; then
      echo "$tag"
      return 0
    fi
  fi

  # 3) git tags (no assets guarantee — download may still 404)
  if body="$(gh_get "https://api.github.com/repos/${REPO}/tags?per_page=30")"; then
    if tag="$(
      printf '%s\n' "$body" \
        | grep -oE '"name"[[:space:]]*:[[:space:]]*"v[^"]+"' \
        | sed -E 's/.*"([^"]+)".*/\1/' \
        | first_usable_tag
    )"; then
      echo "$tag"
      return 0
    fi
  fi

  echo "could not resolve a release tag from GitHub for ${REPO}." >&2
  echo "Create a GitHub Release with platform binaries, or pin one:" >&2
  echo "  VERSION=0.1.0-alpha.1 curl -fsSL https://raw.githubusercontent.com/${REPO}/main/scripts/install.sh | sh" >&2
  echo "Or install from git / npm:" >&2
  echo "  cargo install --git https://github.com/${REPO} --locked awesome-schema" >&2
  echo "  pnpm add -D awesome-schema@alpha surrealdb@^2.2.2" >&2
  exit 1
}

main() {
  local target version tag url dest tmp http
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
  http="$(
    curl -sS -A "$UA" -L -o "$tmp" -w "%{http_code}" "$url" || true
  )"
  if [ "$http" != "200" ]; then
    rm -f "$tmp"
    echo "download ${url} failed: HTTP ${http:-error}" >&2
    echo "That tag may have no binary assets yet. Try npm or cargo install --git (see README)." >&2
    exit 1
  fi
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
