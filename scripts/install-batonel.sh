#!/usr/bin/env bash
set -euo pipefail

REPO_OWNER="Arcflect"
REPO_NAME="batonel"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"
REQUESTED_VERSION="${1:-latest}"
TMP_DIR=""

cleanup_tmp_dir() {
  if [[ -n "${TMP_DIR:-}" && -d "$TMP_DIR" ]]; then
    rm -rf "$TMP_DIR"
  fi
}

require_cmd() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "missing required command: $1" >&2
    exit 1
  fi
}

resolve_version() {
  if [[ "$REQUESTED_VERSION" != "latest" ]]; then
    echo "$REQUESTED_VERSION"
    return
  fi

  local latest_url
  latest_url="$(curl -fsSL -o /dev/null -w '%{url_effective}' "https://github.com/${REPO_OWNER}/${REPO_NAME}/releases/latest")"
  local latest_tag
  latest_tag="${latest_url##*/}"

  if [[ ! "$latest_tag" =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo "failed to resolve latest version tag" >&2
    exit 1
  fi

  echo "$latest_tag"
}

detect_target() {
  local os
  local arch
  os="$(uname -s)"
  arch="$(uname -m)"

  case "$os" in
    Linux)
      case "$arch" in
        x86_64)
          echo "x86_64-unknown-linux-gnu"
          ;;
        aarch64|arm64)
          echo "aarch64-unknown-linux-gnu"
          ;;
        *)
          echo "unsupported Linux architecture: $arch (supported: x86_64, aarch64/arm64)" >&2
          exit 1
          ;;
      esac
      ;;
    Darwin)
      case "$arch" in
        x86_64)
          echo "x86_64-apple-darwin"
          ;;
        arm64|aarch64)
          echo "aarch64-apple-darwin"
          ;;
        *)
          echo "unsupported macOS architecture: $arch (supported: x86_64, arm64/aarch64)" >&2
          exit 1
          ;;
      esac
      ;;
    *)
      echo "unsupported OS: $os" >&2
      exit 1
      ;;
  esac
}

verify_checksum() {
  local file="$1"
  local checksum_file="$2"

  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum -c "$checksum_file"
    return
  fi

  if command -v shasum >/dev/null 2>&1; then
    local expected
    local actual
    expected="$(awk '{print $1}' "$checksum_file")"
    actual="$(shasum -a 256 "$file" | awk '{print $1}')"
    if [[ "$expected" != "$actual" ]]; then
      echo "checksum mismatch" >&2
      exit 1
    fi
    return
  fi

  echo "missing checksum tool: sha256sum or shasum is required" >&2
  exit 1
}

install_binary() {
  local binary_path="$1"
  local destination="$INSTALL_DIR/batonel"

  chmod +x "$binary_path"

  if [[ -w "$INSTALL_DIR" ]]; then
    mv "$binary_path" "$destination"
  else
    sudo mv "$binary_path" "$destination"
  fi

  echo "installed: $destination"
}

main() {
  require_cmd curl
  require_cmd tar

  local version
  version="$(resolve_version)"
  if [[ ! "$version" =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo "invalid version tag: $version (expected: vX.Y.Z or latest)" >&2
    exit 1
  fi

  local target
  target="$(detect_target)"

  local archive_name
  local checksum_name
  local base_url
  archive_name="batonel-${version}-${target}.tar.gz"
  checksum_name="${archive_name}.sha256"
  base_url="https://github.com/${REPO_OWNER}/${REPO_NAME}/releases/download/${version}"

  TMP_DIR="$(mktemp -d)"
  trap cleanup_tmp_dir EXIT

  echo "downloading ${archive_name}"
  curl -fsSL -o "$TMP_DIR/$archive_name" "$base_url/$archive_name"
  curl -fsSL -o "$TMP_DIR/$checksum_name" "$base_url/$checksum_name"

  echo "verifying checksum"
  (cd "$TMP_DIR" && verify_checksum "$archive_name" "$checksum_name")

  tar -xzf "$TMP_DIR/$archive_name" -C "$TMP_DIR"

  if [[ ! -f "$TMP_DIR/batonel" ]]; then
    echo "archive does not contain expected binary: batonel" >&2
    exit 1
  fi

  install_binary "$TMP_DIR/batonel"
  batonel --version
}

main
